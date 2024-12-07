use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::payload::types::pan::Pan;

use super::macros::impl_request_args_type;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct SetUserVoiceSettingsArgs {
    #[builder(into)]
    user_id: String,
    #[builder(into)]
    pan: Option<Pan>,
    #[builder(into)]
    volume: Option<u32>,
    #[builder(into)]
    mute: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct SelectVoiceChannelArgs {
    #[builder(into)]
    channel_id: String,
    #[builder(into)]
    timeout: Option<Pan>,
    #[builder(into)]
    volume: Option<u32>,
    #[builder(into)]
    mute: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct GetSelectedVoiceChannelArgs;

impl_request_args_type!(SetUserVoiceSettings);
impl_request_args_type!(SelectVoiceChannel);
impl_request_args_type!(GetSelectedVoiceChannel);
