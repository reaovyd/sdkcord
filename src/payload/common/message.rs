use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{attachment::Attachment, embed::Embed, user::User};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Message {
    pub attachments: Option<Vec<Attachment>>,
    pub author: Option<User>,
    pub author_color: Option<String>,
    pub blocked: Option<bool>,
    pub bot: Option<bool>,
    pub content: Option<String>,
    pub edited_timestamp: Option<DateTime<Utc>>,
    pub embeds: Option<Vec<Embed>>,
    pub id: Option<String>,
    pub mention_everyone: Option<bool>,
    pub mention_roles: Option<Vec<String>>,
    pub mentions: Option<Vec<String>>,
    pub nick: Option<String>,
    pub pinned: Option<bool>,
    pub timestamp: Option<DateTime<Utc>>,
    pub tts: Option<bool>,
    #[serde(rename = "type")]
    pub message_type: Option<MessageType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize_repr, Serialize_repr, Hash)]
#[repr(u8)]
pub enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    UserJoin = 7,
    GuildBoost = 8,
    GuildBoostTier1 = 9,
    GuildBoostTier2 = 10,
    GuildBoostTier3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    GuildDiscoveryGracePeriodInitialWarning = 16,
    GuildDiscoveryGracePeriodFinalWarning = 17,
    ThreadCreated = 18,
    Reply = 19,
    ChatInputCommand = 20,
    ThreadStarterMessage = 21,
    GuildInviteReminder = 22,
    ContextMenuCommand = 23,
    AutoModerationAction = 24,
    RoleSubscriptionPurchase = 25,
    InteractionPremiumUpsell = 26,
    StageStart = 27,
    StageEnd = 28,
    StageSpeaker = 29,
    StageTopic = 31,
    GuildApplicationPremiumSubscription = 32,
    GuildIncidentAlertModeEnabled = 36,
    GuildIncidentAlertModeDisabled = 37,
    GuildIncidentReportRaid = 38,
    GuildIncidentReportFalseAlarm = 39,
    PurchaseNotification = 44,
    PollResult = 46,
}
