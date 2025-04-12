use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::payload::common::{channel::impl_channel_id_type, guild::impl_guild_id_type};

use super::{
    common::channel::{Channel, ChannelResponse},
    macros::{impl_empty_args_type, impl_event_args_type, impl_request_args_type},
};

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct SelectTextChannelArgs {
    #[builder(into)]
    channel_id: Option<String>,
    timeout: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ChannelCreateData(pub Channel);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct SelectTextChannelData(pub Option<GetChannelData>);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct GetChannelData(pub ChannelResponse);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct GetChannelsData {
    pub channels: Vec<Channel>,
}

impl_empty_args_type!(ChannelCreate);
impl_channel_id_type!(GetChannelArgs);
impl_guild_id_type!(GetChannelsArgs);

impl_request_args_type!(GetChannel);
impl_request_args_type!(GetChannels);
impl_request_args_type!(SelectTextChannel);

impl_event_args_type!(ChannelCreate);
