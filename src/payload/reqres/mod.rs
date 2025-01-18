use serde::{
    Deserialize,
    Serialize,
};

use super::{
    Command,
    Event,
};

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
    macro_rules! impl_event_args_type {
        ($args_name: ident) => {
            paste::paste! {
                impl $crate::payload::reqres::ArgsType for [<$args_name Args>] {
                    fn args_val(self) -> crate::payload::reqres::Args {
                        crate::payload::reqres::Args::$args_name(self)
                    }
                }

                impl $crate::payload::reqres::EventArgsType for [<$args_name Args>] {
                    fn name(&self) -> crate::payload::Event {
                        crate::payload::Event::$args_name
                    }
                }

                impl $crate::payload::reqres::sealed::Sealed for [<$args_name Args>] {}
            }
        };
    }

    macro_rules! impl_request_args_type {
        ($args_name: ident) => {
            paste::paste! {
                impl $crate::payload::reqres::ArgsType for [<$args_name Args>] {
                    fn args_val(self) -> crate::payload::reqres::Args {
                        crate::payload::reqres::Args::$args_name(self)
                    }
                }

                impl $crate::payload::reqres::RequestArgsType for [<$args_name Args>] {
                    fn name(&self) -> crate::payload::Command {
                        crate::payload::Command::$args_name
                    }
                }

                impl $crate::payload::reqres::sealed::Sealed for [<$args_name Args>] {}
            }
        };
    }

    macro_rules! impl_empty_args_type {
        ($args_name: ident) => {
            paste::paste! {
                #[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq, Hash)]
                pub struct [<$args_name Args>]($crate::payload::reqres::EmptyBracket);
            }
        };
    }

    pub(crate) use impl_empty_args_type;
    pub(crate) use impl_event_args_type;
    pub(crate) use impl_request_args_type;
}

pub use activity::*;
pub use auth::*;
pub use channel::*;
#[cfg(feature = "untested")]
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
