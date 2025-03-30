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
#[cfg(unix)]
use tokio::net::unix::OwnedWriteHalf;

#[cfg(windows)]
use crate::conn::windows::ClientWriteHalf;
#[cfg(windows)]
use crate::conn::windows::connect_windows;

use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::oneshot::{self},
    time::{Instant, error::Elapsed},
};

const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

#[cfg(unix)]
pub async fn spawn_client() -> Result<SdkClient<OwnedWriteHalf, UnreadyState>, SdkClientError> {
    let (rh, wh) = connect_unix()
        .await
        .map_err(|err| SdkClientError::ConnectionFailed(err.to_string()))?;
    spawn(wh, rh).await
}

#[cfg(windows)]
pub async fn spawn_client() -> Result<SdkClient<ClientWriteHalf, UnreadyState>, SdkClientError> {
    let (rh, wh) = connect_windows()
        .await
        .map_err(|err| SdkClientError::ConnectionFailed(err.to_string()))?;
    spawn(wh, rh).await
}

async fn spawn<T, R>(wh: T, rh: R) -> Result<SdkClient<T, UnreadyState>, SdkClientError>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
    R: Send + Sync + 'static,
    R: AsyncRead + Unpin,
{
    let serializer_client = spawn_pool()
        .channel_buffer(512)
        .num_threads(16)
        .op(serialize)
        .call();
    // TODO: make this deserialization function...
    let deserialization_client = spawn_pool()
        .channel_buffer(512)
        .num_threads(16)
        .op(deserialize)
        .call();
    let codec = FrameCodec {};
    let framed_write = FramedWrite::new(wh, codec);
    let framed_read = FramedRead::new(rh, codec);

    let writer = kameo::spawn(Writer::new(serializer_client, framed_write));
    let coordinator = kameo::spawn(Coordinator::new(writer));
    kameo::spawn(Reader::new(
        deserialization_client,
        framed_read,
        coordinator.clone(),
    ));

    Ok(SdkClient {
        coordinator,
        _state: PhantomData,
    })
}

#[derive(Debug, Clone)]
pub struct SdkClient<W, S>
where
    W: Send + Sync + 'static,
    W: AsyncWrite + Unpin,
{
    coordinator: ActorRef<Coordinator<ActorRef<Writer<W>>>>,
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
