//! The client library for the IPC server
//!
//! # Usage
//! To actually spin up a client that can be used, you would first need to invoke the [spawn_client]
//! function to grab an [SdkClient]. To connect to the IPC server and tell Discord that this client
//! is ready to receive messages, you would call [SdkClient::connect].
//!
//! From there, you can send requests to the IPC server by calling the APIs inside [SdkClient] and
//! build the request you need with [PayloadRequest].
use std::{marker::PhantomData, time::Duration};

use kameo::{actor::ActorRef, error::SendError};
use thiserror::Error;
use tokio_util::codec::{FramedRead, FramedWrite};
use tracing::error;

use crate::{
    actors::{Coordinator, Reader, Writer},
    codec::FrameCodec,
    config::Config,
    payload::*,
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
    time::Instant,
};

/// Spawns an [SdkClient].
///
/// # Note
/// The client is not yet ready in the ready state in which you must call [SdkClient::connect] to
/// connect to the IPC server.
///
/// # Errors
/// [SdkClientError] is returned if the client fails to spawn.
#[cfg(unix)]
pub async fn spawn_client(
    config: Config,
) -> Result<SdkClient<OwnedWriteHalf, UnreadyState>, SdkClientError> {
    let (rh, wh) = connect_unix()
        .await
        .map_err(|err| SdkClientError::ConnectionFailed(err.to_string()))?;
    spawn(wh, rh, config).await
}

/// Spawns an [SdkClient].
///
/// # Note
/// The client is not yet ready in the ready state in which you must call [SdkClient::connect] to
/// connect to the IPC server.
///
/// # Errors
/// [SdkClientError] is returned if the client fails to spawn.
#[cfg(windows)]
pub async fn spawn_client(
    config: Config,
) -> Result<SdkClient<ClientWriteHalf, UnreadyState>, SdkClientError> {
    let (rh, wh) = connect_windows()
        .await
        .map_err(|err| SdkClientError::ConnectionFailed(err.to_string()))?;
    spawn(wh, rh, config).await
}

async fn spawn<T, R>(
    wh: T,
    rh: R,
    config: Config,
) -> Result<SdkClient<T, UnreadyState>, SdkClientError>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
    R: Send + Sync + 'static,
    R: AsyncRead + Unpin,
{
    let serializer_client = spawn_pool()
        .channel_buffer(config.serializer_channel_buffer_size)
        .num_threads(config.serializer_num_threads)
        .op(serialize)
        .call();
    // TODO: make this deserialization function...
    let deserialization_client = spawn_pool()
        .channel_buffer(config.deserializer_channel_buffer_size)
        .num_threads(config.deserializer_num_threads)
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
        request_timeout: Duration::from_secs(config.request_timeout),
        _state: PhantomData,
    })
}

/// The client for the Discord IPC server
#[derive(Debug, Clone)]
pub struct SdkClient<W, S>
where
    W: Send + Sync + 'static,
    W: AsyncWrite + Unpin,
{
    coordinator: ActorRef<Coordinator<ActorRef<Writer<W>>>>,
    request_timeout: Duration,
    _state: PhantomData<S>,
}

impl<T, S> SdkClient<T, S>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    /// Make a connection call to Discord
    ///
    /// This call is made to let Discord know that this client is ready to send and receive
    /// messages. It is necessary to call this function before sending any requests to the IPC
    /// server.
    ///
    /// # Errors
    /// An [SdkClientError] is returned if the client fails to connect to the IPC server.
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

        tokio::time::timeout_at(Instant::now() + self.request_timeout, recv)
            .await
            .map_err(|_| SdkClientError::Timeout)?
            .map_err(|err| SdkClientError::ConnectionFailed(err.to_string()))?;
        Ok(SdkClient {
            coordinator: self.coordinator,
            request_timeout: self.request_timeout,
            _state: PhantomData,
        })
    }
}

type SdkClientResult<T> = Result<Box<T>, SdkClientError>;

impl<T> SdkClient<T, ReadyState>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    impl_request! {
        /// Send a authenticate request to the IPC server
        authenticate; Authenticate
    }
    impl_request! {
        /// Send a authorize request to the IPC server
        authorize; Authorize
    }

    impl_request! {
        /// Send a get guild request to the IPC server
        get_guild; GetGuild
    }

    impl_request! {
        /// Send a get guilds request to the IPC server
        get_guilds; GetGuilds
    }

    impl_request! {
        /// Send a get channel request to the IPC server
        get_channel; GetChannel
    }

    impl_request! {
        /// Send a select voice channel request to the IPC server
        select_voice_channel; SelectVoiceChannel
    }

    impl_request! {
        /// Send a get selected voice channel request to the IPC server
        get_selected_voice_channel; GetSelectedVoiceChannel
    }

    impl_request! {
        /// Send a select text channel request to the IPC server
        select_text_channel; SelectTextChannel
    }

    /// Send a request to the IPC server
    ///
    /// As an end user, you would use this function to send a request to the IPC server. The
    /// request can be constructed using the [PayloadRequest] struct.
    ///
    /// # Errors
    /// A [SdkClientError] is returned if the client fails to send the request or if the server
    /// fails
    #[inline(always)]
    async fn send_request(
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
        let resp = tokio::time::timeout_at(Instant::now() + self.request_timeout, recv)
            .await
            .map_err(|_| SdkClientError::Timeout)?
            .map_err(|err| SdkClientError::ResponseDropped(err.to_string()))?;
        Ok(resp)
    }
}

mod macros {
    macro_rules! impl_request {
        (
            $(#[$attr:meta])*
            $request_name: ident;
            $args_name: ident
        ) => {
            paste::paste! {
                $(#[$attr])*
                /// # Errors
                /// A [SdkClientError] is returned if the client fails to send the request or if the server
                /// responds with an error
                pub async fn $request_name(&self, args: [<$args_name Args>]) -> SdkClientResult<[<$args_name Data>]> {
                    let response = self
                        .send_request(PayloadRequest::builder().request(args).build())
                        .await?;
                    if let Some(Data::$args_name(data)) = response.0.data {
                        Ok(data)
                    } else if let Some(Data::Error(error)) = response.0.data {
                        Err(SdkClientError::ResponseError { error })
                    } else {
                        error!(
                            "response should always have data but could not be found... panicking...; final response: {:?}",
                            response
                        );
                        panic!("some form of data should always be returned...");
                    }
                }
            }
        };
    }
    pub(super) use impl_request;
}

use macros::impl_request;

/// An Error type for when making requests to the IPC server may fail
#[derive(Debug, Error)]
pub enum SdkClientError {
    /// Sending the request has failed and the original [PayloadRequest] is returned
    #[error("failed to send the request!")]
    SendRequest(Option<PayloadRequest>),
    /// The request has been received, but the coordinator has failed to process it
    #[error("internal server received the request, but failed to process it {0}")]
    InternalCoordinator(String),
    /// The request has been sent, but the client has failed to receive a response from the server
    /// in a timely manner
    #[error("response timeout from server!")]
    Timeout,
    /// The response sender has been dropped and the response is unrecoverable
    #[error("server dropped response; response unrecoverable: {0}")]
    ResponseDropped(String),
    /// The client failed to connect to the IPC server
    #[error("client failed to connect to ipc {0}")]
    ConnectionFailed(String),
    /// Configuration error
    #[error("failed to spawn client because of config: {error}")]
    ConfigFailed { config: Config, error: String },
    /// Response is an error
    #[error("response sent back an error")]
    ResponseError { error: Box<ErrorData> },
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReadyState;
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnreadyState;
