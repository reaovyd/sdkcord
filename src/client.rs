use std::{marker::PhantomData, time::Duration};

use kameo::{actor::ActorRef, error::SendError};
use thiserror::Error;
use tokio_util::codec::{FramedRead, FramedWrite};

use crate::{
    actors::{Coordinator, Reader, Writer},
    codec::FrameCodec,
    payload::{ConnectRequest, PayloadRequest, PayloadResponse, Request},
    pool::{deserialize, serialize, spawn_pool},
};

#[cfg(unix)]
use crate::conn::unix::connect_unix;

#[cfg(windows)]
use crate::conn::windows::connect_windows;

use tokio::{
    io::AsyncWrite,
    sync::oneshot::{self},
    time::{Instant, error::Elapsed},
};

const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

pub async fn spawn_client<T: Send + Sync + 'static>()
-> Result<SdkClient<T, UnreadyState>, SdkClientError> {
    #[cfg(unix)]
    let (rh, wh) = connect_unix()
        .await
        .map_err(|err| SdkClientError::ConnectionFailed(err.to_string()))?;
    #[cfg(windows)]
    let (rh, wh) = connect_windows()
        .await
        .map_err(|err| SdkClientError::ConnectionFailed(err.to_string()))?;

    let serializer_client = spawn_pool().cap(512).num_threads(16).op(serialize).call();
    // TODO: make this deserialization function...
    let deserialization_client = spawn_pool().cap(512).num_threads(16).op(deserialize).call();
    let framed_write = FramedWrite::new(wh, FrameCodec {});
    let framed_read = FramedRead::new(rh, FrameCodec {});

    let writer = kameo::spawn(Writer::new(serializer_client, framed_write));
    let coordinator = kameo::spawn(Coordinator::new(writer));
    let reader = Reader::new(deserialization_client, framed_read, coordinator);

    todo!()
}

#[derive(Debug, Clone)]
pub struct SdkClient<T: Send + Sync + 'static, S> {
    coordinator: ActorRef<Coordinator<T>>,
    _state: PhantomData<S>,
}

impl<T, S> SdkClient<T, S>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    #[inline(always)]
    pub async fn connect(
        self,
        client_id: &str,
    ) -> Result<SdkClient<T, ReadyState>, SdkClientError> {
        let (sndr, recv) = oneshot::channel::<PayloadResponse>();
        self.coordinator
            .tell((
                Request::Connect(ConnectRequest::new(client_id.to_string())),
                sndr,
            ))
            .await
            .map_err(|err| SdkClientError::ConnectionFailed(err.to_string()))?;

        tokio::time::timeout_at(Instant::now() + REQUEST_TIMEOUT, recv)
            .await
            .map_err(SdkClientError::Timeout)?
            .map_err(|err| SdkClientError::ConnectionFailed(err.to_string()))?;
        Ok(SdkClient {
            coordinator: self.coordinator,
            _state: PhantomData,
        })
    }
}

impl<T> SdkClient<T, ReadyState>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    #[inline(always)]
    pub async fn send_request(
        &self,
        request: PayloadRequest,
    ) -> Result<PayloadResponse, SdkClientError> {
        let (sndr, recv) = oneshot::channel::<PayloadResponse>();
        if let Err(send_err) = self
            .coordinator
            .tell((Request::Payload(request), sndr))
            .await
        {
            match send_err {
                SendError::ActorNotRunning(err) => {
                    if let Request::Payload(payload) = err.0 {
                        return Err(SdkClientError::SendRequest(Some(payload)));
                    } else {
                        return Err(SdkClientError::SendRequest(None));
                    }
                }
                SendError::ActorStopped => SdkClientError::SendRequest(None),
                SendError::MailboxFull(err) => {
                    if let Request::Payload(payload) = err.0 {
                        return Err(SdkClientError::SendRequest(Some(payload)));
                    } else {
                        return Err(SdkClientError::SendRequest(None));
                    }
                }
                SendError::HandlerError(err) => {
                    return Err(SdkClientError::InternalCoordinator(err.to_string()));
                }
                SendError::Timeout(_) => {
                    // server has a timeout on its end so client will always receive a response...
                    panic!(
                        "this should never happen since we don't have timeout setup from client to server!"
                    )
                }
            };
        }
        let resp = tokio::time::timeout_at(Instant::now() + REQUEST_TIMEOUT, recv)
            .await
            .map_err(SdkClientError::Timeout)?
            .map_err(|err| SdkClientError::ResponseDropped(err.to_string()))?;
        Ok(resp)
    }
}

#[derive(Debug, Error)]
pub enum SdkClientError {
    #[error("failed to send the request!")]
    SendRequest(Option<PayloadRequest>),
    #[error("internal server received the request, but failed to process it... {0}")]
    InternalCoordinator(String),
    #[error("response timeout from server! haven't received a response after {0} seconds...")]
    Timeout(Elapsed),
    #[error("server dropped response; response unrecoverable: {0}")]
    ResponseDropped(String),
    #[error("client failed to connect to ipc {0}")]
    ConnectionFailed(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReadyState;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnreadyState;
