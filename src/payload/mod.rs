use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::EnumString;
use uuid::Uuid;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Payload {
    pub cmd: Command,
    pub nonce: Uuid,
    pub evt: Option<Event>,
    pub data: Option<()>,
    pub args: Option<Args>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Args {
    Authorize(AuthorizeArgs),
    Authenticate(AuthenticateArgs),
    GetGuild(GetGuildArgs),
    GetGuilds(GetGuildsArgs),
    GetChannel(GetChannelArgs),
    GetChannels(GetChannelsArgs),
    SetUserVoiceSettings(SetUserVoiceSettingsArgs),
    SelectVoiceChannel(SelectVoiceChannelArgs),
    GetSelectedVoiceChannel(GetSelectedVoiceChannelArgs),
    SelectTextChannel(SelectTextChannelArgs),
    GetVoiceSettings(GetVoiceSettingsArgs),
    SetVoiceSettings(SetVoiceSettingsArgs),
    #[cfg(feature = "untested")]
    SetCertifiedDevices(SetCertifiedDevicesArgs),
    SetActivity(SetActivityArgs),
    #[cfg(feature = "untested")]
    SendActivityJoinInvite(SendActivityJoinInviteArgs),
    #[cfg(feature = "untested")]
    CloseActivityRequest(CloseActivityRequestArgs),
    GuildStatus(GuildStatusArgs),
    GuildCreate(GuildCreateArgs),
    ChannelCreate(ChannelCreateArgs),
    VoiceChannelSelect(VoiceChannelSelectArgs),
    VoiceStateCreate(VoiceStateCreateArgs),
    VoiceStateUpdate(VoiceStateUpdateArgs),
    VoiceStateDelete(VoiceStateDeleteArgs),
    VoiceSettingsUpdate(VoiceSettingsUpdateArgs),
    VoiceConnectionStatus(VoiceConnectionStatusArgs),
    SpeakingStart(SpeakingStartArgs),
    SpeakingStop(SpeakingStopArgs),
    MessageCreate(MessageCreateArgs),
    MessageUpdate(MessageUpdateArgs),
    MessageDelete(MessageDeleteArgs),
    NotificationCreate(NotificationCreateArgs),
    #[cfg(feature = "untested")]
    ActivityJoin(ActivityJoinArgs),
    #[cfg(feature = "untested")]
    ActivitySpectate(ActivitySpectateArgs),
    #[cfg(feature = "untested")]
    ActivityJoinRequest(ActivityJoinRequestArgs),
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct EmptyBracket {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    _inner: Option<()>,
}

#[derive(
    Debug,
    Copy,
    Clone,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    strum_macros::Display,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Command {
    Dispatch,
    Authorize,
    Authenticate,
    GetGuild,
    GetGuilds,
    GetChannel,
    GetChannels,
    Subscribe,
    Unsubscribe,
    SetUserVoiceSettings,
    SelectVoiceChannel,
    GetSelectedVoiceChannel,
    SelectTextChannel,
    GetVoiceSettings,
    SetVoiceSettings,
    SetCertifiedDevices,
    SetActivity,
    SendActivityJoinInvite,
    CloseActivityRequest,
}

#[derive(
    Debug,
    Copy,
    Clone,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    strum_macros::Display,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Event {
    Ready,
    Error,
    GuildStatus,
    GuildCreate,
    ChannelCreate,
    VoiceChannelSelect,
    VoiceStateCreate,
    VoiceStateUpdate,
    VoiceStateDelete,
    VoiceSettingsUpdate,
    VoiceConnectionStatus,
    SpeakingStart,
    SpeakingStop,
    MessageCreate,
    MessageUpdate,
    MessageDelete,
    NotificationCreate,
    ActivityJoin,
    ActivitySpectate,
    ActivityJoinRequest,
}

pub trait ArgsType: sealed::Sealed {
    fn args_val(self) -> Args;
}

pub trait EventArgsType: ArgsType {
    fn name(&self) -> Event;
}

pub trait RequestArgsType: ArgsType {
    fn name(&self) -> Command;
}

mod sealed {
    pub trait Sealed {}
}

pub use activity::*;
pub use auth::*;
pub use channel::*;
#[cfg(feature = "untested")]
pub use device::*;
pub use guild::*;
pub use message::*;
pub use notification::*;
pub use request::*;
pub use response::*;
pub use speaking::*;
pub use voice::*;

mod activity;
mod auth;
mod channel;
mod device;
mod guild;
mod macros;
mod message;
mod notification;
mod request;
mod response;
mod speaking;
mod voice;

pub mod common;

#[cfg(test)]
mod tests {
    use super::{request::PayloadRequest, GetVoiceSettingsArgs};

    #[test]
    fn construct_args() {
        let payload = PayloadRequest::builder().request(GetVoiceSettingsArgs::default()).build();
        let s = serde_json::to_string(&payload).unwrap();
        println!("{}", s)
    }
}
