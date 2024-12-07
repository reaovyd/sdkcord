use serde::{Deserialize, Serialize};

use super::{Command, Event};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Args {
    Authorize(AuthorizeArgs),
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
pub use device::*;
pub use guild::*;
pub use message::*;
pub use notification::*;
pub use speaking::*;
pub use voice::*;

mod activity;
mod auth;
mod channel;
mod device;
mod guild;
mod message;
mod notification;
mod speaking;
mod voice;
