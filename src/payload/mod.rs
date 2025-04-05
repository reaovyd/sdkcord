//! Payload that is returned and sent to the Discord IPC server
//!
//! # Discord RPC
//! The [Payload] type and the request types are sourced from the [Discord RPC] documentation.
//!
//! [Discord RPC]: https://discord.com/developers/docs/topics/rpc
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::EnumString;
use uuid::Uuid;

/// Payload that is sent/received to/from the Discord IPC server
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Payload {
    /// [Command] type
    ///
    /// # Note
    /// This defaults to `DISPATCH` if the [Payload] is a response
    pub cmd: Command,
    /// Unique nonce to identify the request
    pub nonce: Option<Uuid>,
    /// [Event] type
    ///
    /// # Note
    /// This will be [Option::None] if the response is a response rather than an event
    pub evt: Option<Event>,
    /// Data that is part of the response payload
    ///
    /// # Note
    /// This will be [Option::None] for requests, but will always exist for responses and events if
    /// there is data to be sent back from the server
    pub data: Option<Data>,
    /// Arguments that are part of the request payload
    ///
    /// # Note
    /// This will be [Option::None] for responses, but will always exist for requests
    pub args: Option<Args>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Data {
    Authorize(Box<AuthorizeData>),
    Authenticate(Box<AuthenticateData>),
    // GetGuilds(GetGuildsArgs),
    // GetChannel(GetChannelArgs),
    // GetChannels(GetChannelsArgs),
    // SetUserVoiceSettings(SetUserVoiceSettingsArgs),
    // SelectVoiceChannel(SelectVoiceChannelArgs),
    // GetSelectedVoiceChannel(GetSelectedVoiceChannelArgs),
    // SelectTextChannel(SelectTextChannelArgs),
    // GetVoiceSettings(GetVoiceSettingsArgs),
    // SetVoiceSettings(SetVoiceSettingsArgs),
    // #[cfg(feature = "untested")]
    // SetCertifiedDevices(SetCertifiedDevicesArgs),
    // SetActivity(SetActivityArgs),
    // #[cfg(feature = "untested")]
    // SendActivityJoinInvite(SendActivityJoinInviteArgs),
    // #[cfg(feature = "untested")]
    // CloseActivityRequest(CloseActivityRequestArgs),
    // GuildStatus(GuildStatusArgs),
    // GuildCreate(GuildCreateArgs),
    // ChannelCreate(ChannelCreateArgs),
    // VoiceChannelSelect(VoiceChannelSelectArgs),
    // VoiceStateCreate(VoiceStateCreateArgs),
    // VoiceStateUpdate(VoiceStateUpdateArgs),
    // VoiceStateDelete(VoiceStateDeleteArgs),
    // VoiceSettingsUpdate(VoiceSettingsUpdateArgs),
    // VoiceConnectionStatus(VoiceConnectionStatusArgs),
    // SpeakingStart(SpeakingStartArgs),
    // SpeakingStop(SpeakingStopArgs),
    // MessageCreate(MessageCreateArgs),
    // MessageUpdate(MessageUpdateArgs),
    // MessageDelete(MessageDeleteArgs),
    // NotificationCreate(NotificationCreateArgs),
    // #[cfg(feature = "untested")]
    // ActivityJoin(ActivityJoinArgs),
    // #[cfg(feature = "untested")]
    // ActivitySpectate(ActivitySpectateArgs),
    // #[cfg(feature = "untested")]
    // ActivityJoinRequest(ActivityJoinRequestArgs),
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
    use super::{GetVoiceSettingsArgs, request::PayloadRequest};

    #[test]
    fn construct_args() {
        let payload = PayloadRequest::builder()
            .request(GetVoiceSettingsArgs::default())
            .build();
        let _s = serde_json::to_string(&payload).unwrap();
    }
}
