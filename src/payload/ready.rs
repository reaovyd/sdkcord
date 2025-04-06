use serde::{Deserialize, Serialize};

use super::common::user::User;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ReadyData {
    pub v: Option<u8>,
    pub user: Option<User>,
    pub config: Option<ReadyConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ReadyConfig {
    pub cdn_host: String,
    pub api_endpoint: String,
    pub environment: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_ready_data() {
        let payload = r##"{"v":1,"config":{"cdn_host":"cdn.discordapp.com","api_endpoint":"//discord.com/api","environment":"production"},"user":{"id":"53908232506183680","username":"Mason","discriminator":"1337","avatar":null}}"##;
        let payload = serde_json::from_str::<super::ReadyData>(payload).unwrap();
        assert_eq!(payload.v, Some(1));
        assert_eq!(
            payload.config.unwrap().api_endpoint,
            "//discord.com/api".to_string()
        );
    }
}
