use bon::Builder;
use thiserror::Error;

use super::voice::VoiceState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ChannelResponse {
    #[serde(flatten)]
    pub channel: Option<Channel>,
    pub guild_id: Option<String>,
    pub topic: Option<String>,
    pub bitrate: Option<u32>,
    pub user_limit: Option<u32>,
    pub position: Option<u32>,
    pub voice_states: Option<Vec<VoiceState>>,
    // TODO: when we get message object type created
    // pub messages:
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Channel {
    pub id: Option<String>,
    pub name: Option<String>,
    pub channel_type: Option<ChannelType>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum ChannelType {
    GuildText = 0,
    Dm = 1,
    GuildVoice = 2,
    GroupDm = 3,
    GuildCategory = 4,
    GuildAnnouncement = 5,
    AnnouncementThread = 10,
    PublicThread = 11,
    PrivateThread = 12,
    GuildStageVoice = 13,
    GuildDirectory = 14,
    GuildForum = 15,
    GuildMedia = 16,
}

impl TryFrom<u8> for ChannelType {
    type Error = ChannelTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ChannelType::GuildText),
            1 => Ok(ChannelType::Dm),
            2 => Ok(ChannelType::GuildVoice),
            3 => Ok(ChannelType::GroupDm),
            4 => Ok(ChannelType::GuildCategory),
            5 => Ok(ChannelType::GuildAnnouncement),
            10 => Ok(ChannelType::AnnouncementThread),
            11 => Ok(ChannelType::PublicThread),
            12 => Ok(ChannelType::PrivateThread),
            13 => Ok(ChannelType::GuildStageVoice),
            14 => Ok(ChannelType::GuildDirectory),
            15 => Ok(ChannelType::GuildForum),
            16 => Ok(ChannelType::GuildMedia),
            _ => Err(ChannelTypeError::InvalidChannelType(value)),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum ChannelTypeError {
    #[error("ChannelType {0} does not exist...")]
    InvalidChannelType(u8),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct ChannelId {
    #[builder(into)]
    pub channel_id: String,
}

impl<T: Into<String>> From<T> for ChannelId {
    fn from(value: T) -> Self {
        Self {
            channel_id: value.into(),
        }
    }
}

mod macros {
    macro_rules! impl_channel_id_type {
        ($args_name: ident) => {
            #[serde_with::skip_serializing_none]
            #[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq, Hash)]
            pub struct $args_name(pub $crate::payload::common::channel::ChannelId);
            impl From<$crate::payload::common::channel::ChannelId> for $args_name {
                fn from(value: $crate::payload::common::channel::ChannelId) -> Self {
                    Self(value)
                }
            }
        };
    }
    pub(crate) use impl_channel_id_type;
}

pub(crate) use macros::impl_channel_id_type;
