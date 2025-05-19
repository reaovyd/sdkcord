//! # Discord OAuth2 Client
//!
//! Many of the commands and subscription events require an OAuth2 Authorization flow. For example,
//! for reading messages in a channel, it is required that an OAuth2 authorization flow has
//! occurred and the client has sent an access token taken from this client to the Discord IPC
//! server.
//!
//! ## Refresh
//! Refreshing the token is useful if a user plans to have long-lived connections to the Discord
//! IPC. This is because token expiration is usually 1 day from when it was requested. Two
//! things that this OAuth2 client does for refreshing is:
//!
//! 1. If the time is in the 60 seconds window from the expires_in on the next request, the
//!    expiration check would be valid and an access token would be requested again with the
//!    refresh token.
//! 2. A task that is scheduled to run every 12 hours from the time that this client was created.

use std::{sync::Arc, time::Duration};

use oauth2::{
    AccessToken, AuthorizationCode, Client as OAuth2Client, ClientId, ClientSecret, EndpointNotSet,
    EndpointSet, RefreshToken, RevocationErrorResponseType, StandardErrorResponse,
    StandardRevocableToken, TokenResponse, TokenUrl,
    basic::{BasicClient, BasicErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse},
};
use reqwest::Client as HttpClient;
use secrecy::ExposeSecret;
use thiserror::Error;
use tokio::{
    io::AsyncWrite,
    sync::RwLock,
    time::{Instant, interval_at},
};
use tracing::error;

use crate::{
    client::{ReadyState, SdkClient},
    config::OAuth2Config,
    payload::{AuthenticateArgs, AuthorizeArgs},
};

const DISCORD_TOKEN_URI: &str = "https://discord.com/api/oauth2/token";
const DEFAULT_EXPIRATION_CHECK_PERIOD: Duration = Duration::from_secs(60 * 60 * 12);

#[derive(Debug)]
pub(crate) struct TokenManager {
    client: Client,
    token_data: RwLock<RefreshTokenData>,
}

impl TokenManager {
    pub(crate) async fn new<W>(
        config: &OAuth2Config,
        client_id: &str,
        sdk_client: &SdkClient<W, ReadyState>,
    ) -> Result<Self, OAuth2Error>
    where
        W: Send + Sync + 'static,
        W: AsyncWrite + Unpin,
    {
        let client = Client::new(config, client_id)?;
        let refresh_token_data =
            handle_initial_oauth2_flow(sdk_client, &client, client_id, config.clone()).await?;

        let token_manager = Self {
            client,
            token_data: RwLock::new(refresh_token_data),
        };

        Ok(token_manager)
    }

    async fn check_token_expiration(&self) -> Result<(), OAuth2Error> {
        let token_data = self.token_data.read().await;
        let now = Instant::now();
        if now >= (token_data.expires_at - Duration::from_secs(60 * 30)) {
            return Err(OAuth2Error::TokenConversion("token expired".to_string()));
        }
        Ok(())
    }

    async fn refresh_token<W>(
        &self,
        refresh_token_data: RefreshTokenData,
        access_token: AccessToken,
        sdk_client: &Arc<SdkClient<W, ReadyState>>,
    ) -> Result<(), OAuth2Error>
    where
        W: Send + Sync + 'static,
        W: AsyncWrite + Unpin,
    {
        {
            let mut write_lock = self.token_data.write().await;
            *write_lock = refresh_token_data;
        }
        sdk_client
            .authenticate(
                AuthenticateArgs::builder()
                    .access_token(access_token.into_secret())
                    .build(),
            )
            .await
            .map_err(|err| OAuth2Error::Authenticate(err.to_string()))?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Client {
    http_client: HttpClient,
    oauth2_client: DiscordOAuth2Client,
}

impl Client {
    fn new(config: &OAuth2Config, client_id: &str) -> Result<Self, OAuth2Error> {
        let oauth2_client = BasicClient::new(ClientId::new(client_id.to_string()))
            .set_client_secret(ClientSecret::new(
                config.client_secret.expose_secret().to_string(),
            ))
            .set_token_uri(
                TokenUrl::new(DISCORD_TOKEN_URI.to_string())
                    .expect("DISCORD_TOKEN_URI should be a valid URI"),
            );

        let http_client = HttpClient::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|err| OAuth2Error::HttpClient(err.to_string()))?;

        Ok(Self {
            oauth2_client,
            http_client,
        })
    }

    async fn exchange_code(&self, code: &str) -> Result<BasicTokenResponse, OAuth2Error> {
        self.oauth2_client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(&self.http_client)
            .await
            .map_err(|err| OAuth2Error::AuthorizationCodeExchange(err.to_string()))
    }

    async fn exchange_refresh_token(
        &self,
        refresh_token_data: &RefreshTokenData,
    ) -> Result<BasicTokenResponse, OAuth2Error> {
        self.oauth2_client
            .exchange_refresh_token(&refresh_token_data.refresh_token)
            .request_async(&self.http_client)
            .await
            .map_err(|err| OAuth2Error::RefreshTokenExchange(err.to_string()))
    }
}
pub(crate) fn spawn_refresh_task<W>(sdk_client: Arc<SdkClient<W, ReadyState>>)
where
    W: Send + Sync + 'static,
    W: AsyncWrite + Unpin,
{
    tokio::task::spawn(async move {
        let mut interval = interval_at(
            Instant::now() + DEFAULT_EXPIRATION_CHECK_PERIOD,
            DEFAULT_EXPIRATION_CHECK_PERIOD,
        );
        loop {
            interval.tick().await;
            let token_manager = sdk_client.token_manager.as_ref().unwrap();
            let refresh_data = {
                let data = token_manager.token_data.read().await;
                data.clone()
            };
            let token = match token_manager
                .client
                .exchange_refresh_token(&refresh_data)
                .await
            {
                Ok(token) => token,
                Err(e) => {
                    error!("failed to exchange refresh token: {}", e);
                    continue;
                }
            };
            let new_access_token = token.access_token().clone();
            if let Err(e) = token_manager
                .refresh_token(
                    RefreshTokenData::try_from(token).unwrap(),
                    new_access_token,
                    &sdk_client,
                )
                .await
            {
                error!("failed to refresh token: {}", e);
                continue;
            }
        }
    });
}

async fn handle_initial_oauth2_flow<W>(
    client: &SdkClient<W, ReadyState>,
    oauth2_client: &Client,
    client_id: &str,
    oauth2_config: OAuth2Config,
) -> Result<RefreshTokenData, OAuth2Error>
where
    W: Send + Sync + 'static,
    W: AsyncWrite + Unpin,
{
    let authorize_args = AuthorizeArgs::builder()
        .client_id(client_id)
        .scopes(oauth2_config.scopes)
        .build();

    let authorize_data = client
        .authorize(authorize_args)
        .await
        .map_err(|err| OAuth2Error::AuthorizationCodeExchange(err.to_string()))?;

    let code = authorize_data
        .code
        .ok_or_else(|| OAuth2Error::AuthorizationCodeExchange("code missing".to_string()))?;

    let token = oauth2_client.exchange_code(&code).await?;
    let access_token = token.access_token().clone().into_secret();
    let refresh_token_data = RefreshTokenData::try_from(token)?;

    let authenticate_args = AuthenticateArgs::builder()
        .access_token(access_token)
        .build();

    client
        .authenticate(authenticate_args)
        .await
        .map_err(|err| OAuth2Error::Authenticate(err.to_string()))?;

    Ok(refresh_token_data)
}

#[derive(Debug, Clone)]
pub(crate) struct RefreshTokenData {
    refresh_token: RefreshToken,
    pub expires_at: Instant,
}

impl RefreshTokenData {
    pub(crate) const fn new(refresh_token: RefreshToken, expires_at: Instant) -> Self {
        Self {
            refresh_token,
            expires_at,
        }
    }
}

impl TryFrom<BasicTokenResponse> for RefreshTokenData {
    type Error = OAuth2Error;

    fn try_from(token: BasicTokenResponse) -> Result<Self, Self::Error> {
        let expires_in = token
            .expires_in()
            .ok_or("expires_in missing")
            .map_err(|err| OAuth2Error::TokenConversion(err.to_string()))?;
        let expires_at = Instant::now() + expires_in;
        let refresh_token = token
            .refresh_token()
            .ok_or("refresh_token missing")
            .map_err(|err| OAuth2Error::TokenConversion(err.to_string()))?
            .to_owned();
        Ok(Self::new(refresh_token, expires_at))
    }
}

type DiscordOAuth2Client<
    HasAuthUrl = EndpointNotSet,
    HasDeviceAuthUrl = EndpointNotSet,
    HasIntrospectionUrl = EndpointNotSet,
    HasRevocationUrl = EndpointNotSet,
    HasTokenUrl = EndpointSet,
> = OAuth2Client<
    BasicErrorResponse,
    BasicTokenResponse,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    HasAuthUrl,
    HasDeviceAuthUrl,
    HasIntrospectionUrl,
    HasRevocationUrl,
    HasTokenUrl,
>;

#[derive(Debug, Clone, Error)]
pub enum OAuth2Error {
    #[error("failed to exchange authorization code: {0}")]
    AuthorizationCodeExchange(String),
    #[error("failed to authenticate: {0}")]
    Authenticate(String),
    #[error("failed to exchange refresh token: {0}")]
    RefreshTokenExchange(String),
    #[error("http client failed to build: {0}")]
    HttpClient(String),
    #[error("token failed to convert: {0}")]
    TokenConversion(String),
}
