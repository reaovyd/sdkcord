use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum_macros::EnumString;
use thiserror::Error;
use url::Url;

use super::{guild::GuildId, oauth2::InstallParams, team::Team, user::User};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Application {
    pub id: Option<String>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub rpc_origins: Option<Vec<Url>>,
    pub bot: Option<User>,
    pub bot_public: Option<bool>,
    pub bot_require_code_grant: Option<bool>,
    pub terms_of_service_url: Option<Url>,
    pub privacy_policy_url: Option<Url>,
    pub owner: Option<User>,
    pub verify_key: Option<String>,
    #[serde(flatten)]
    pub guild_id: Option<GuildId>,
    pub team: Option<Team>,
    // pub guild: Option<Guild>
    pub primary_sku_id: Option<String>,
    pub slug: Option<String>,
    pub cover_image: Option<String>,
    pub flags: Option<ApplicationFlags>,
    pub hook: Option<bool>,
    pub is_discoverable: Option<bool>,
    pub is_monetized: Option<bool>,
    pub is_verified: Option<bool>,
    pub storefront_available: Option<bool>,
    pub summary: Option<String>,
    pub approximate_guild_count: Option<u32>,
    pub approximate_user_install_count: Option<u32>,
    pub redirect_uris: Option<Vec<Url>>,
    pub interaction_endpoint_url: Option<Url>,
    pub role_connections_verification_url: Option<Url>,
    pub event_webhooks_url: Option<Url>,
    pub event_webhook_status: Option<EventWebhookStatus>,
    pub event_webhook_types: Option<Vec<EventWebhookType>>,
    pub tags: Option<Vec<String>>,
    // TODO: add InstallParams
    pub integration_types_config: Option<IntegrationTypesConfig>,
    pub custom_install_url: Option<Url>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ApplicationFlags(u32);

bitflags! {
    impl ApplicationFlags: u32 {
        const APPLICATION_AUTO_MODERATION_RULE_CREATE_BADGE = 1 << 6;
        const GATEWAY_PRESENCE = 1 << 12;
        const GATEWAY_PRESENCE_LIMITED = 1 << 13;
        const GATEWAY_GUILD_MEMBERS = 1 << 14;
        const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;
        const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;
        const EMBEDDED = 1 << 17;
        const GATEWAY_MESSAGE_CONTENT = 1 << 18;
        const GATEWAY_MESSAGE_CONTENT_LIMITED = 1 << 19;
        const APPLICATION_COMMAND_BADGE = 1 << 23;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, EnumString)]
#[repr(u8)]
pub enum EventWebhookType {
    #[serde(rename = "APPLICATION_AUTHORIZED")]
    ApplicationAuthorized = 1,
    #[serde(rename = "ENTITLEMENT_CREATE")]
    EntitlementCreate = 2,
    #[serde(rename = "QUEST_USER_ENROLLMENT")]
    QuestUserEnrollment = 3,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, PartialEq, Eq, Hash, EnumString)]
#[repr(u8)]
pub enum EventWebhookStatus {
    Disabled = 1,
    Enabled = 2,
    DisabledByDiscord = 3,
}

impl TryFrom<u8> for EventWebhookStatus {
    type Error = ApplicationError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(EventWebhookStatus::Disabled),
            2 => Ok(EventWebhookStatus::Enabled),
            3 => Ok(EventWebhookStatus::DisabledByDiscord),
            _ => Err(ApplicationError::InvalidApplicationEventWebhookStatus(
                value,
            )),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum ApplicationError {
    #[error("ApplicationEventWebhookStatus {0} does not exist...")]
    InvalidApplicationEventWebhookStatus(u8),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum IntegrationTypesConfig {
    #[serde(rename = "0")]
    GuildInstall {
        oauth2_install_params: InstallParams,
    },
    #[serde(rename = "1")]
    UserInstall {
        oauth2_install_params: InstallParams,
    },
}

#[cfg(test)]
mod tests {
    use super::Application;

    #[test]
    fn deserialize_application() {
        let payload = r##"{"bot":{"accent_color":null,"avatar":"df7a5c32e954703bfe2e61ad8c14c3dc","avatar_decoration_data":null,"banner":null,"banner_color":null,"bot":true,"clan":null,"discriminator":"4588","flags":0,"global_name":null,"id":"1276759902551015485","primary_guild":null,"public_flags":0,"username":"IPCCord"},"guild_id":"abcdef","bot_public":true,"bot_require_code_grant":false,"description":"bruh","flags":64,"hook":true,"icon":"df7a5c32e954703bfe2e61ad8c14c3dc","id":"1276759902551015485","integration_types_config":{"1":{"oauth2_install_params":{"permissions":"0","scopes":["applications.commands"]}}},"is_discoverable":false,"is_monetized":false,"is_verified":false,"name":"gameing","storefront_available":false,"summary":"","type":null,"verify_key":"02d2b7977161590c0bdc6a5e67d75dc9333ba0f469a0fd2d2171964516bcc5ac"}"##;
        let app = serde_json::from_str::<Application>(payload).unwrap();
        assert_eq!(
            app.bot.as_ref().unwrap().username,
            Some("IPCCord".to_string())
        );
        assert_eq!(app.guild_id.as_ref().unwrap().guild_id, "abcdef");
    }
}
