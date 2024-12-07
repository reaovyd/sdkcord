use args::Args;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Payload {
    pub cmd: Command,
    pub nonce: Uuid,
    pub evt: Option<Event>,
    pub data: Option<()>,
    pub args: Option<Args>,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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

pub mod args;
pub mod request;
pub mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn test_construct_typestate() {}
}
