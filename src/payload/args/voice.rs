use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::payload::types::{pan::Pan, voice::VoiceSettings};

use super::macros::{impl_empty_args_type, impl_request_args_type};

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct SetUserVoiceSettingsArgs {
    #[builder(into)]
    user_id: String,
    #[builder(into)]
    pan: Option<Pan>,
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
    volume: Option<u32>,
    #[builder(into)]
    mute: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct SetVoiceSettingsArgs(pub VoiceSettings);

impl_empty_args_type!(GetVoiceSettings);
impl_empty_args_type!(GetSelectedVoiceChannel);

impl_request_args_type!(SetUserVoiceSettings);
impl_request_args_type!(GetVoiceSettings);
impl_request_args_type!(SetVoiceSettings);
impl_request_args_type!(SelectVoiceChannel);
impl_request_args_type!(GetSelectedVoiceChannel);

impl From<VoiceSettings> for SetVoiceSettingsArgs {
    fn from(value: VoiceSettings) -> Self {
        Self(value)
    }
}
