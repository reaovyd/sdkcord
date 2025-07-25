//! The client used to communiate with the Discord Client IPC.
//!
//! ## OAuth2
//! You can enable OAuth2 by providing an [OAuth2Config] when constructing the client. You can find
//! the scopes used in the [OAuth2Scope][crate::payload::common::oauth2::OAuth2Scope] enum. The ones you will most likely use are:
//!
//!  - `Rpc`
//!  - `Identify`
//!  - `Guilds`
//!  - `MessagesRead`
//!  - `RpcNotificationsRead`
//!
//! to get access to all of the commands in the SDK.
use std::{sync::Arc, time::Duration};

use kameo::{actor::ActorRef, error::SendError};
use thiserror::Error;
use tokio_util::codec::{FramedRead, FramedWrite};
use tracing::error;

use crate::{
    actors::{Coordinator, Reader, Writer},
    codec::FrameCodec,
    config::{Config, OAuth2Config},
    oauth2::{OAuth2Error, TokenManager},
    payload::*,
    pool::{deserialize, serialize, spawn_pool},
};

#[cfg(unix)]
use {crate::conn::unix::connect_unix, tokio::net::unix::OwnedWriteHalf as WriteHalf};

#[cfg(windows)]
use {crate::conn::windows::ClientWriteHalf as WriteHalf, crate::conn::windows::connect_windows};

use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::oneshot::{self},
    time::Instant,
};

/// The client for the Discord IPC server
#[derive(Debug, Clone)]
pub struct SdkClient {
    inner: Arc<InnerSdkClient>,
    token_manager: Option<Arc<TokenManager>>,
}

impl SdkClient {
    /// Constructs a new client and connects to Discord
    ///
    /// # OAuth2
    /// You can provide an [OAuth2Config] through an Option<OAuth2Config> to enable OAuth2.
    ///
    /// # Errors
    /// An [SdkClientError] is returned if the client fails to connect to the IPC server.
    pub async fn new(
        config: Config,
        client_id: impl Into<String>,
        oauth2_config: Option<OAuth2Config>,
    ) -> Result<Self, SdkClientError> {
        let client_id = client_id.into();
        let inner = Arc::new(InnerSdkClient::new(config, &client_id).await?);
        let token_manager = {
            if let Some(oauth2_config) = oauth2_config {
                let token_manager =
                    Arc::new(TokenManager::new(oauth2_config, &client_id, inner.clone()).await?);
                Some(token_manager)
            } else {
                None
            }
        };
        Ok(SdkClient {
            inner,
            token_manager,
        })
    }

    pub async fn read_event_queue(&self) -> EventData {
        self.inner.get_event_data().await
    }

    impl_pub_request! {
        /// Send a get guild request to the IPC server
        get_guild; GetGuild
    }

    impl_pub_request! {
        /// Send a get guilds request to the IPC server
        get_guilds; GetGuilds
    }

    impl_pub_request! {
        /// Send a get channel request to the IPC server
        get_channel; GetChannel
    }

    impl_pub_request! {
        /// Send a select voice channel request to the IPC server
        select_voice_channel; SelectVoiceChannel
    }

    impl_pub_request! {
        /// Send a get selected voice channel request to the IPC server
        get_selected_voice_channel; GetSelectedVoiceChannel
    }

    impl_pub_request! {
        /// Send a select text channel request to the IPC server
        select_text_channel; SelectTextChannel
    }

    impl_evt_req! {
        /// Send a subscribe request to the IPC server
        subscribe;
        Subscribe
    }

    impl_evt_req! {
        /// Send a unsubscribe request to the IPC server.
        unsubscribe;
        Unsubscribe
    }

    impl_pub_request! {
        /// Send a set user voice settings request to the IPC server.
        set_user_voice_settings;
        SetUserVoiceSettings
    }

    impl_pub_request! {
        /// Send a set voice settings request to the IPC server.
        set_voice_settings;
        SetVoiceSettings
    }

    impl_pub_request! {
        /// Send a get voice settings request to the IPC server.
        get_voice_settings;
        GetVoiceSettings
    }

    impl_pub_request! {
        /// Send a set activity request to the IPC server.
        set_activity;
        SetActivity
    }

    impl_pub_request! {
        /// Send a get channels request to the IPC server.
        get_channels;
        GetChannels
    }
}

#[derive(Debug)]
pub(crate) struct InnerSdkClient {
    coordinator: ActorRef<Coordinator<ActorRef<Writer<WriteHalf>>>>,
    request_timeout: Duration,
    evt_queue_rx: async_channel::Receiver<EventData>,
}

impl InnerSdkClient {
    pub(crate) async fn authenticate(
        &self,
        args: AuthenticateArgs,
    ) -> SdkClientResult<AuthenticateData> {
        let response = self
            .send_request(PayloadRequest::builder().request(args).build())
            .await?;
        if let Some(Data::Authenticate(data)) = response.0.data {
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

    pub(crate) async fn authorize(&self, args: AuthorizeArgs) -> SdkClientResult<AuthorizeData> {
        let response = self
            .send_request(PayloadRequest::builder().request(args).build())
            .await?;
        if let Some(Data::Authorize(data)) = response.0.data {
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

    async fn new(config: Config, client_id: &str) -> Result<InnerSdkClient, SdkClientError> {
        let (rh, wh) = {
            #[cfg(unix)]
            {
                connect_unix()
                    .await
                    .map_err(|err| SdkClientError::ConnectionFailed(err.to_string()))?
            }
            #[cfg(windows)]
            {
                connect_windows()
                    .await
                    .map_err(|err| SdkClientError::ConnectionFailed(err.to_string()))?
            }
        };
        let (evt_queue_tx, evt_queue_rx) = async_channel::bounded::<EventData>(1024);
        let coordinator = setup(wh, rh, &config, evt_queue_tx).await;
        let (sndr, recv) = oneshot::channel::<PayloadResponse>();
        // Setup Initial IPC connection
        {
            coordinator
                .tell((
                    Request::Connect(ConnectRequest::new(client_id.to_string())),
                    sndr,
                ))
                .await
                .map_err(|err| SdkClientError::ConnectionFailed(err.to_string()))?;

            tokio::time::timeout_at(
                Instant::now() + Duration::from_secs(config.request_timeout),
                recv,
            )
            .await
            .map_err(|_| SdkClientError::Timeout)?
            .map_err(|err| SdkClientError::ConnectionFailed(err.to_string()))?;
        }

        let request_timeout = Duration::from_secs(config.request_timeout);

        let sdk_client = InnerSdkClient {
            coordinator,
            request_timeout,
            evt_queue_rx,
        };
        Ok(sdk_client)
    }

    async fn get_event_data(&self) -> EventData {
        self.evt_queue_rx
            .recv()
            .await
            .expect("failed since channel is closed")
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

async fn setup<W, R>(
    wh: W,
    rh: R,
    config: &Config,
    evt_queue_tx: async_channel::Sender<EventData>,
) -> ActorRef<Coordinator<ActorRef<Writer<W>>>>
where
    W: Send + Sync + 'static,
    W: AsyncWrite + Unpin,
    R: Send + Sync + 'static,
    R: AsyncRead + Unpin,
{
    let serializer_client = spawn_pool()
        .channel_buffer(config.serializer_channel_buffer_size)
        .num_threads(config.serializer_num_threads)
        .op(serialize)
        .call();
    let deserialization_client = spawn_pool()
        .channel_buffer(config.deserializer_channel_buffer_size)
        .num_threads(config.deserializer_num_threads)
        .op(deserialize)
        .call();
    let codec = FrameCodec {};
    let framed_write = FramedWrite::new(wh, codec);
    let framed_read = FramedRead::new(rh, codec);

    let writer = kameo::spawn(Writer::new(serializer_client, framed_write));
    let coordinator = kameo::spawn(Coordinator::new(writer, evt_queue_tx));
    kameo::spawn(Reader::new(
        deserialization_client,
        framed_read,
        coordinator.clone(),
    ));
    coordinator
}

type SdkClientResult<T> = Result<Box<T>, SdkClientError>;

mod macros {
    macro_rules! impl_evt_req {
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
                pub async fn $request_name<E: EventArgsType>(&self, args: E) -> SdkClientResult<[<$args_name Data>]> {
                    if let Some(ref mgr) = self.token_manager
                    {
                        mgr.refresh_token().await?;
                    }
                    let response = self
                        .inner
                        .send_request(PayloadRequest::builder().event().$request_name(args).build())
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

    macro_rules! impl_pub_request {
        (
            $(#[$attr:meta])*
            $request_name: ident;
            $args_name: ident
        ) => {
            impl_request! {
                $(#[$attr])*
                $request_name;
                $args_name;
                pub
            }
        };
    }

    macro_rules! impl_priv_request {
        (
            $(#[$attr:meta])*
            $request_name: ident;
            $args_name: ident
        ) => {
            impl_request! {
                $(#[$attr])*
                $request_name;
                $args_name;
                pub(crate)
            }
        };
    }

    macro_rules! impl_request {
        (
            $(#[$attr:meta])*
            $request_name: ident;
            $args_name: ident;
            $viz: vis
        ) => {
            paste::paste! {
                $(#[$attr])*
                /// # Errors
                /// A [SdkClientError] is returned if the client fails to send the request or if the server
                /// responds with an error
                $viz async fn $request_name(&self, args: [<$args_name Args>]) -> SdkClientResult<[<$args_name Data>]> {
                    if let Some(ref mgr) = self.token_manager
                    {
                        mgr.refresh_token().await?;
                    }
                    let response = self
                        .inner
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
    pub(super) use impl_evt_req;
    pub(super) use impl_priv_request;
    pub(super) use impl_pub_request;
    pub(super) use impl_request;
}

use macros::impl_evt_req;
use macros::impl_priv_request;
use macros::impl_pub_request;
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
    /// OAuth2 Error
    #[error("oauth2 error: {0}")]
    OAuth2(#[from] OAuth2Error),
}
