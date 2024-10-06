pub use activity::*;
pub use channel::*;
pub use guild::*;
pub use message::*;
pub use notification::*;
pub use speaking::*;
pub use voice::*;

use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(tag = "evt", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscribeRequest {
    GuildStatus(GuildStatusSubscriptionEvent),
    GuildCreate(GuildCreateSubscriptionEvent),
    ChannelCreate(ChannelCreateSubscriptionEvent),
    VoiceChannelSelect(VoiceChannelSelectSubscriptionEvent),
    VoiceStateCreate(VoiceStateCreateSubscriptionEvent),
    VoiceStateUpdate(VoiceStateUpdateSubscriptionEvent),
    VoiceStateDelete(VoiceStateDeleteSubscriptionEvent),
    VoiceSettingsUpdate(VoiceSettingsUpdateSubscriptionEvent),
    VoiceConnectionStatus(VoiceConnectionStatusSubscriptionEvent),
    SpeakingStart(SpeakingStartSubscriptionEvent),
    SpeakingStop(SpeakingStopSubscriptionEvent),
    MessageCreate(MessageCreateSubscriptionEvent),
    MessageUpdate(MessageUpdateSubscriptionEvent),
    MessageDelete(MessageDeletedSubscriptionEvent),
    NotificationCreate(NotificationCreateSubscriptionEvent),
    ActivityJoin(ActivityJoinSubscriptionEvent),
    ActivitySpectate(ActivitySpectateSubscriptionEvent),
    ActivityJoinRequest(ActivityJoinRequestSubscriptionEvent),
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(tag = "evt", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UnsubscribeRequest {
    GuildStatus(GuildStatusUnsubscriptionEvent),
    GuildCreate(GuildCreateUnsubscriptionEvent),
    ChannelCreate(ChannelCreateUnsubscriptionEvent),
    VoiceChannelSelect(VoiceChannelSelectUnsubscriptionEvent),
    VoiceStateCreate(VoiceStateCreateUnsubscriptionEvent),
    VoiceStateUpdate(VoiceStateUpdateUnsubscriptionEvent),
    VoiceStateDelete(VoiceStateDeleteUnsubscriptionEvent),
    VoiceSettingsUpdate(VoiceSettingsUpdateUnsubscriptionEvent),
    VoiceConnectionStatus(VoiceConnectionStatusUnsubscriptionEvent),
    SpeakingStart(SpeakingStartUnsubscriptionEvent),
    SpeakingStop(SpeakingStopUnsubscriptionEvent),
    MessageCreate(MessageCreateUnsubscriptionEvent),
    MessageUpdate(MessageUpdateUnsubscriptionEvent),
    MessageDelete(MessageDeletedUnsubscriptionEvent),
    NotificationCreate(NotificationCreateUnsubscriptionEvent),
    ActivityJoin(ActivityJoinUnsubscriptionEvent),
    ActivitySpectate(ActivitySpectateUnsubscriptionEvent),
    ActivityJoinRequest(ActivityJoinRequestUnsubscriptionEvent),
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash, Default)]
struct EmptyArgs {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    inner: Option<()>,
}

mod activity;
mod channel;
mod guild;
mod macros;
mod message;
mod notification;
mod speaking;
mod voice;
