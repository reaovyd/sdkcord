use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::payload::types::channel::ChannelId;

use super::macros::{impl_empty_args_type, impl_event_args_type, impl_request_args_type};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct GetChannelArgs(pub ChannelId);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct GetChannelsArgs(pub ChannelId);

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct SelectTextChannelArgs {
    #[builder(into)]
    guild_id: String,
    timeout: Option<u32>,
}

impl_empty_args_type!(ChannelCreate);

impl_request_args_type!(GetChannel);
impl_request_args_type!(GetChannels);
impl_request_args_type!(SelectTextChannel);

impl_event_args_type!(ChannelCreate);

impl From<ChannelId> for GetChannelsArgs {
    fn from(value: ChannelId) -> Self {
        Self(value)
    }
}

impl From<ChannelId> for GetChannelArgs {
    fn from(value: ChannelId) -> Self {
        Self(value)
    }
}
