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

use std::{fs::File, io::Read, path::PathBuf, sync::Arc};

use chrono::{DateTime, Utc};
use oauth2::{
    AccessToken, AuthorizationCode, Client as OAuth2Client, ClientId, ClientSecret, EndpointNotSet,
    EndpointSet, RefreshToken, RevocationErrorResponseType, StandardErrorResponse,
    StandardRevocableToken, TokenResponse, TokenUrl,
    basic::{BasicClient, BasicErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse},
};
use reqwest::Client as HttpClient;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::error;

use crate::{
    client::InnerSdkClient,
    config::OAuth2Config,
    payload::{AuthenticateArgs, AuthorizeArgs},
};

const DISCORD_TOKEN_URI: &str = "https://discord.com/api/oauth2/token";

#[derive(Debug)]
pub(crate) struct TokenManager {
    oauth2_client: OAuth2TokenClient,
    token_data: RwLock<TokenData>,
    sdk_client: Arc<InnerSdkClient>,
}

impl TokenManager {
    pub(crate) async fn new(
        config: OAuth2Config,
        client_id: &str,
        sdk_client: Arc<InnerSdkClient>,
    ) -> Result<Self, OAuth2Error> {
        let oauth2_client = OAuth2TokenClient::new(&config, client_id)?;
        // 1. Check if the token file exists. If it doesn't exist, then move to creating a new
        //    token and it should write to the file.
        // 2. If it does exist, then read from the file and parse the token data into TokenData.
        // 3. If the TokenData file fails to parse, then we will move to creating a new token and it
        //    should write to the file as well.
        // 4. If the TokenData file succeeds in parsing, then read from the file and deserialize it into
        //    TokenData

        let token_data = {
            match File::open(&config.config_path) {
                Ok(mut file) => {
                    let mut buf = String::new();
                    if let Ok(_) = file.read_to_string(&mut buf)
                        && let Ok(data) = serde_json::from_str::<TokenData>(&buf)
                    {
                        authenticate(&sdk_client, data.access_token.clone().into_secret()).await?;
                        data
                    } else {
                        drop(file);
                        handle_initial_oauth2_flow(&sdk_client, &oauth2_client, client_id, config)
                            .await?
                    }
                }
                Err(err) => {
                    error!("failed to open token file: {}", err);
                    handle_initial_oauth2_flow(&sdk_client, &oauth2_client, client_id, config)
                        .await?
                }
            }
        };

        let token_manager = Self {
            oauth2_client,
            token_data: RwLock::new(token_data),
            sdk_client,
        };

        Ok(token_manager)
    }

    pub(crate) async fn refresh_token(&self) -> Result<(), OAuth2Error> {
        if !self.is_self_token_data_expired().await {
            return Ok(());
        }
        // when read guards start dropping from above, many clients on other threads can contend
        // for the write lock in which case we need to do another check if one other client wins
        // the race and updates it and the other clients do another check to prevent another write
        // and call
        let mut write_lock = self.token_data.write().await;
        if !is_token_expired(&write_lock) {
            return Ok(());
        }
        let refresh_token_data = self
            .oauth2_client
            .exchange_refresh_token(&write_lock)
            .await?;
        *write_lock = TokenData::try_from(&refresh_token_data)?;
        let access_token = write_lock.access_token.clone().into_secret();
        drop(write_lock);
        authenticate(&self.sdk_client, access_token).await?;
        Ok(())
    }

    async fn is_self_token_data_expired(&self) -> bool {
        let token_data = self.token_data.read().await;
        is_token_expired(&token_data)
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
        token_data: &TokenData,
    ) -> Result<BasicTokenResponse, OAuth2Error> {
        self.oauth2_client
            .exchange_refresh_token(&token_data.refresh_token)
            .request_async(&self.http_client)
            .await
            .map_err(|err| OAuth2Error::RefreshTokenExchange(err.to_string()))
    }
}

fn is_token_expired(token_data: &TokenData) -> bool {
    Utc::now() > token_data.expires_at
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

async fn write_token_data_to_file(
    config_path: &PathBuf,
    token_data: &TokenData,
) -> Result<(), OAuth2Error> {
    let file = File::create(config_path).map_err(|err| OAuth2Error::TokenDataWrite {
        file_name: config_path.to_owned(),
        err: err.to_string(),
    })?;
    serde_json::to_writer(file, token_data).map_err(|err| OAuth2Error::TokenDataWrite {
        file_name: config_path.to_owned(),
        err: err.to_string(),
    })?;
    Ok(())
}

async fn handle_initial_oauth2_flow(
    sdk_client: &InnerSdkClient,
    oauth2_client: &OAuth2TokenClient,
    client_id: &str,
    oauth2_config: OAuth2Config,
) -> Result<TokenData, OAuth2Error> {
    let config_path = oauth2_config.config_path.clone();
    let code = authorize(sdk_client, client_id, oauth2_config).await?;

    let token = oauth2_client.exchange_code(&code).await?;
    let token_data = TokenData::try_from(&token)?;
    let access_token = token.access_token().clone().into_secret();

    authenticate(sdk_client, access_token).await?;
    write_token_data_to_file(&config_path, &token_data).await?;
    Ok(token_data)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenData {
    refresh_token: RefreshToken,
    access_token: AccessToken,
    expires_at: DateTime<Utc>,
}

impl TryFrom<BasicTokenResponse> for TokenData {
    type Error = OAuth2Error;

    fn try_from(token: BasicTokenResponse) -> Result<Self, Self::Error> {
        TokenData::try_from(&token)
    }
}

impl TryFrom<&BasicTokenResponse> for TokenData {
    type Error = OAuth2Error;

    fn try_from(token: &BasicTokenResponse) -> Result<Self, Self::Error> {
        let expires_in = token
            .expires_in()
            .ok_or("expires_in missing")
            .map_err(|err| OAuth2Error::TokenConversion(err.to_string()))?;
        let expires_at = Utc::now() + expires_in;
        let refresh_token = token
            .refresh_token()
            .ok_or("refresh_token missing")
            .map_err(|err| OAuth2Error::TokenConversion(err.to_string()))?
            .to_owned();
        let access_token = token.access_token().clone();
        Ok(Self {
            access_token,
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
    #[error("failed to write token data to {file_name}: {err}")]
    TokenDataWrite { file_name: PathBuf, err: String },
}
