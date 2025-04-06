use super::level::{
    ExplicitContentFilterLevel, MessageNotificationLevel, MfaLevel, NsfwLevel, VerificationLevel,
};
use bon::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Guild {
    pub id: Option<String>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner: Option<bool>,
    pub owner_id: Option<String>,
    pub permissions: Option<String>,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: Option<u32>,
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<String>,
    pub verification_level: Option<VerificationLevel>,
    pub default_message_notifications: Option<MessageNotificationLevel>,
    pub explicit_content_filter: Option<ExplicitContentFilterLevel>,
    // TODO: to add
    // pub roles: Option<Vec<Role>>,
    // pub emojis: Option<Vec<Emoji>>,
    // pub features: Option<Vec<String>>,
    pub mfa_level: Option<MfaLevel>,
    pub application_id: Option<String>,
    pub system_channel_id: Option<String>,
    // TODO: to add
    // pub system_channel_flags: Option<SystemChannelFlags>,
    pub rules_channel_id: Option<String>,
    pub max_presences: Option<u32>,
    pub max_members: Option<u32>,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    // TODO: to add
    // pub premium_tier: Option<PremiumTier>,
    pub premium_subscription_count: Option<u32>,
    // TODO: to add
    // pub preferred_locale:
    pub public_updates_channel_id: Option<String>,
    pub max_video_channel_users: Option<u32>,
    pub max_stage_video_channel_users: Option<u32>,
    pub approximate_member_count: Option<u32>,
    pub approximate_presence_count: Option<u32>,
    // TODO: to add
    // pub welcome_screen: Option<WelcomeScreen>,
    pub nsfw_level: Option<NsfwLevel>,
    // TODO: to add
    // pub stickers: Option<Vec<Sticker>>,
    pub premium_progress_bar_enabled: Option<bool>,
    pub safety_alerts_channel_id: Option<String>,
    // TODO: to add
    // pub incidents_data: Option<IncidentsData>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct GuildId {
    #[builder(into)]
    pub guild_id: String,
}

impl From<String> for GuildId {
    fn from(value: String) -> Self {
        Self { guild_id: value }
    }
}

mod macros {
    macro_rules! impl_guild_id_type {
        ($args_name: ident) => {
            #[serde_with::skip_serializing_none]
            #[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq, Hash)]
            pub struct $args_name(pub $crate::payload::common::guild::GuildId);
            impl From<$crate::payload::common::guild::GuildId> for $args_name {
                fn from(value: $crate::payload::common::guild::GuildId) -> Self {
                    Self(value)
                }
            }
        };
    }
    pub(crate) use impl_guild_id_type;
}

pub(crate) use macros::impl_guild_id_type;
