use std::{path::PathBuf, time::Duration};

use bon::Builder;
use secrecy::SecretString;

use crate::payload::common::oauth2::OAuth2Scope;

#[derive(Debug, Clone, Builder)]
pub struct OAuth2Config {
    #[builder(into)]
    pub(crate) client_secret: SecretString,
    #[builder(with = |scopes: impl IntoIterator<Item = OAuth2Scope>| {
        scopes.into_iter().collect()
    })]
    pub scopes: Vec<OAuth2Scope>,
    #[builder(default = {
        let mut config_path = dirs::config_dir().unwrap();
        config_path.push("sdkcord");
        std::fs::create_dir_all(&config_path).unwrap();
        config_path.push("sdkcord.json");
        config_path
    })]
    pub config_path: PathBuf,
    #[builder(default = {Duration::from_secs(5)}, with = |secs: u64| {
        Duration::from_secs(secs)
    })]
    pub refresh_token_timer: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Builder)]
pub struct Config {
    pub serializer_channel_buffer_size: usize,
    pub deserializer_channel_buffer_size: usize,
    pub serializer_num_threads: u8,
    pub deserializer_num_threads: u8,
    pub request_timeout: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            serializer_channel_buffer_size: 16,
            deserializer_channel_buffer_size: 256,
            serializer_num_threads: 4,
            deserializer_num_threads: 32,
            request_timeout: 30,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::payload::common::oauth2::OAuth2Scope;

    use super::{Config, OAuth2Config};
    use secrecy::ExposeSecret;

    #[test]
    fn test_config_build() {
        let config = Config::builder()
            .serializer_channel_buffer_size(512)
            .deserializer_channel_buffer_size(512)
            .serializer_num_threads(16)
            .deserializer_num_threads(16)
            .request_timeout(60)
            .build();
        assert_eq!(config.serializer_channel_buffer_size, 512);
        assert_eq!(config.deserializer_channel_buffer_size, 512);
    }

    #[test]
    fn test_oauth2_config_build() {
        let oauth2_config = OAuth2Config::builder()
            .client_secret("asd")
            .scopes([OAuth2Scope::ApplicationsBuildsUpload])
            .build();
        assert_eq!(oauth2_config.client_secret.expose_secret(), "asd");
    }
}
