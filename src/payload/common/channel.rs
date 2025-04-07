use bon::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct ChannelResponse {
    pub id: Option<String>,
    pub guild_id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub channel_type: Option<ChannelType>,
    pub topic: Option<String>,
    pub bitrate: Option<u32>,
    pub user_limit: Option<u32>,
    pub position: Option<u32>,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ChannelType {
    GuildText = 0,
    Dm = 1,
    GuildVoice = 2,
    GroupDm = 3,
}

impl TryFrom<u8> for ChannelType {
    type Error = ChannelTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ChannelType::GuildText),
            1 => Ok(ChannelType::Dm),
            2 => Ok(ChannelType::GuildVoice),
            3 => Ok(ChannelType::GroupDm),
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

impl From<String> for ChannelId {
    fn from(value: String) -> Self {
        Self { channel_id: value }
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
use serde_repr::{Deserialize_repr, Serialize_repr};
use thiserror::Error;
