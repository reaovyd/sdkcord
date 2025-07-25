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
    sync::RwLock,
    time::{Instant, interval_at},
};
use tracing::error;

use crate::{
    client::{InnerSdkClient, SdkClient},
    config::OAuth2Config,
    payload::{AuthenticateArgs, AuthorizeArgs},
};

const DISCORD_TOKEN_URI: &str = "https://discord.com/api/oauth2/token";
const DEFAULT_EXPIRATION_CHECK_PERIOD: Duration = Duration::from_secs(60 * 60 * 12);

#[derive(Debug)]
pub(crate) struct TokenManager {
    oauth2_client: OAuth2TokenClient,
    refresh_token: RwLock<RefreshTokenData>,
    sdk_client: Arc<InnerSdkClient>,
}

impl TokenManager {
    pub(crate) async fn new(
        config: OAuth2Config,
        client_id: &str,
        sdk_client: Arc<InnerSdkClient>,
    ) -> Result<Self, OAuth2Error> {
        let oauth2_client = OAuth2TokenClient::new(&config, client_id)?;
        // TODO: If the token file we read is corrupted OR the format is bad from parsing OR the file
        // doesn't exist, then we will handle the this by calling again and it should write to the
        // file
        let refresh_token =
            handle_initial_oauth2_flow(&sdk_client, &oauth2_client, client_id, config).await?;
        let token_manager = Self {
            oauth2_client,
            refresh_token: RwLock::new(refresh_token),
            sdk_client,
        };

        Ok(token_manager)
    }

    pub(crate) async fn is_token_expired(&self) -> bool {
        let token_data = self.refresh_token.read().await;
        is_token_expired(&token_data)
    }

    pub(crate) async fn refresh_token(&self) -> Result<(), OAuth2Error> {
        let mut write_lock = self.refresh_token.write().await;
        if !is_token_expired(&write_lock) {
            return Ok(());
        }
        let refresh_token_data = self
            .oauth2_client
            .exchange_refresh_token(&write_lock)
            .await?;
        *write_lock = RefreshTokenData::try_from(refresh_token_data)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct OAuth2TokenClient {
    http_client: HttpClient,
    oauth2_client: DiscordOAuth2Client,
}

impl OAuth2TokenClient {
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
// pub(crate) fn spawn_refresh_task(sdk_client: SdkClient) {
//     tokio::task::spawn(async move {
//         let mut interval = interval_at(
//             Instant::now() + DEFAULT_EXPIRATION_CHECK_PERIOD,
//             DEFAULT_EXPIRATION_CHECK_PERIOD,
//         );
//         loop {
//             interval.tick().await;
//             let token_manager = sdk_client.token_manager();
//             let refresh_data = {
//                 let data = token_manager.token_data.read().await;
//                 data.clone()
//             };
//             let token = match token_manager
//                 .client
//                 .exchange_refresh_token(&refresh_data)
//                 .await
//             {
//                 Ok(token) => token,
//                 Err(e) => {
//                     error!("failed to exchange refresh token: {}", e);
//                     continue;
//                 }
//             };
//             let new_access_token = token.access_token().clone();
//             if let Err(e) = token_manager
//                 .refresh_token(
//                     RefreshTokenData::try_from(token).unwrap(),
//                     new_access_token,
//                     &sdk_client,
//                 )
//                 .await
//             {
//                 error!("failed to refresh token: {}", e);
//                 continue;
//             }
//         }
//     });
// }

fn is_token_expired(refresh_token: &RefreshTokenData) -> bool {
    Instant::now() > refresh_token.expires_at
}

async fn authorize(
    sdk_client: &InnerSdkClient,
    client_id: &str,
    oauth2_config: OAuth2Config,
) -> Result<String, OAuth2Error> {
    let authorize_args = AuthorizeArgs::builder()
        .client_id(client_id)
        .scopes(oauth2_config.scopes)
        .build();

    let authorize_data = sdk_client
        .authorize(authorize_args)
        .await
        .map_err(|err| OAuth2Error::AuthorizationCodeExchange(err.to_string()))?;

    authorize_data
        .code
        .ok_or_else(|| OAuth2Error::AuthorizationCodeExchange("code missing".to_string()))
}

async fn authenticate(
    sdk_client: &InnerSdkClient,
    access_token: String,
) -> Result<(), OAuth2Error> {
    let authenticate_args = AuthenticateArgs::builder()
        .access_token(access_token)
        .build();

    sdk_client
        .authenticate(authenticate_args)
        .await
        .map_err(|err| OAuth2Error::Authenticate(err.to_string()))?;
    Ok(())
}

async fn handle_initial_oauth2_flow(
    sdk_client: &InnerSdkClient,
    oauth2_client: &OAuth2TokenClient,
    client_id: &str,
    oauth2_config: OAuth2Config,
) -> Result<RefreshTokenData, OAuth2Error> {
    let code = authorize(sdk_client, client_id, oauth2_config).await?;

    let token = oauth2_client.exchange_code(&code).await?;
    let refresh_token_data = RefreshTokenData::try_from(&token)?;
    let access_token = token.access_token().clone().into_secret();

    authenticate(sdk_client, access_token).await?;

    Ok(refresh_token_data)
}

#[derive(Debug, Clone)]
struct RefreshTokenData {
    refresh_token: RefreshToken,
    expires_at: Instant,
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
        Ok(Self {
            refresh_token,
            expires_at,
        })
    }
}

impl TryFrom<&BasicTokenResponse> for RefreshTokenData {
    type Error = OAuth2Error;

    fn try_from(token: &BasicTokenResponse) -> Result<Self, Self::Error> {
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
        Ok(Self {
            refresh_token,
            expires_at,
        })
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
