use serde::{Deserialize, Serialize};

use super::{Command, Event};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
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
    SetCertifiedDevices(SetCertifiedDevicesArgs),
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

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct EmptyBracket {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    _inner: Option<()>,
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

mod macros {
    macro_rules! impl_request_args_type {
        ($args_name: ident) => {
            paste::paste! {
                impl super::ArgsType for [<$args_name Args>] {
                    fn args_val(self) -> crate::payload::args::Args {
                        crate::payload::args::Args::$args_name(self)
                    }
                }

                impl super::RequestArgsType for [<$args_name Args>] {
                    fn name(&self) -> crate::payload::Command {
                        crate::payload::Command::$args_name
                    }
                }

                impl super::sealed::Sealed for [<$args_name Args>] {}
            }
        };
    }

    macro_rules! impl_empty_args_type {
        ($args_name: ident) => {
            paste::paste! {
                #[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
                pub struct [<$args_name Args>](crate::payload::args::EmptyBracket);
            }
        };
    }

    pub(crate) use impl_empty_args_type;
    pub(crate) use impl_request_args_type;
}

// pub use activity::*;
pub use auth::*;
pub use channel::*;
pub use device::*;
pub use guild::*;
// pub use message::*;
// pub use notification::*;
// pub use speaking::*;
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
