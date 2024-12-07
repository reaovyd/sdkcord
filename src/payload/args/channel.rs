use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::macros::impl_request_args_type;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct GetChannelArgs {
    #[builder(into)]
    channel_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct GetChannelsArgs {
    #[builder(into)]
    channel_id: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct SelectTextChannelArgs {
    #[builder(into)]
    guild_id: String,
    timeout: Option<u32>,
}

impl_request_args_type!(GetChannel);
impl_request_args_type!(GetChannels);
impl_request_args_type!(SelectTextChannel);
