use std::time::Duration;

use kameo::{actor::ActorRef, error::SendError};
use thiserror::Error;

use crate::{
    actors::Coordinator,
    payload::{PayloadRequest, PayloadResponse},
};

use tokio::{
    io::AsyncWrite,
    sync::oneshot::{self},
    time::{Instant, error::Elapsed},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientConfig {
    client_id: String,
    client_secret: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SdkClient<T: Send + Sync + 'static> {
    coordinator: ActorRef<Coordinator<T>>,
}

impl<T> SdkClient<T>
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
        if let Err(send_err) = self.coordinator.ask((request, sndr)).await {
            match send_err {
                SendError::ActorNotRunning(err) => SdkClientError::SendRequest(Some(err.0)),
                SendError::ActorStopped => SdkClientError::SendRequest(None),
                SendError::MailboxFull(err) => SdkClientError::SendRequest(Some(err.0)),
                SendError::HandlerError(err) => {
                    SdkClientError::InternalCoordinator(err.to_string())
                }
                SendError::Timeout(_) => {
                    // server has a timeout on its end so client will always receive a response...
                    panic!(
                        "this should never happen since we don't have timeout setup from client to server!"
                    )
                }
            };
        }
        let resp = tokio::time::timeout_at(Instant::now() + Duration::from_secs(5), recv)
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
}
