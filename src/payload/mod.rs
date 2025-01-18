use reqres::Args;
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

pub mod common;
pub mod reqres;
pub mod request_builder;

#[cfg(test)]
mod tests {
    use super::{reqres::GetVoiceSettingsArgs, request_builder::PayloadRequest};

    #[test]
    fn construct_args() {
        let payload = PayloadRequest::builder().request(GetVoiceSettingsArgs::default()).build();
        let s = serde_json::to_string(&payload).unwrap();
        println!("{}", s)
    }
}
