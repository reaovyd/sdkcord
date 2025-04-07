use bon::Builder;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::payload::common::{channel::impl_channel_id_type, pan::Pan, voice::VoiceSettings};

use super::{
    GetChannelData,
    macros::{impl_empty_args_type, impl_event_args_type, impl_request_args_type},
};

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct UserVoiceSettings {
    #[builder(into)]
    pub user_id: Option<String>,
    #[builder(into)]
    pub pan: Option<Pan>,
    #[builder(with = |x: f32| {
        OrderedFloat(x)
    })]
    pub volume: Option<OrderedFloat<f32>>,
    #[builder(into)]
    pub mute: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct SetUserVoiceSettingsArgs(pub UserVoiceSettings);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct SetUserVoiceSettingsData(pub UserVoiceSettings);

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct SelectVoiceChannelArgs {
    #[builder(into)]
    channel_id: Option<String>,
    #[builder(into)]
    timeout: Option<Pan>,
    #[builder(into)]
    force: Option<bool>,
    #[builder(into)]
    navigate: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct SelectVoiceChannelData(pub Option<GetChannelData>);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct GetSelectedVoiceChannelData(pub Option<GetChannelData>);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct SetVoiceSettingsArgs(pub VoiceSettings);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct SetVoiceSettingsData(pub VoiceSettings);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct GetVoiceSettingsData(pub VoiceSettings);

impl_channel_id_type!(VoiceStateCreateArgs);
impl_channel_id_type!(VoiceStateUpdateArgs);
impl_channel_id_type!(VoiceStateDeleteArgs);

impl_empty_args_type!(GetVoiceSettings);
impl_empty_args_type!(GetSelectedVoiceChannel);
impl_empty_args_type!(VoiceChannelSelect);
impl_empty_args_type!(VoiceSettingsUpdate);
impl_empty_args_type!(VoiceConnectionStatus);

impl_request_args_type!(SetUserVoiceSettings);
impl_request_args_type!(GetVoiceSettings);
impl_request_args_type!(SetVoiceSettings);
impl_request_args_type!(SelectVoiceChannel);
impl_request_args_type!(GetSelectedVoiceChannel);

impl_event_args_type!(VoiceChannelSelect);
impl_event_args_type!(VoiceStateCreate);
impl_event_args_type!(VoiceStateUpdate);
impl_event_args_type!(VoiceStateDelete);
impl_event_args_type!(VoiceSettingsUpdate);
impl_event_args_type!(VoiceConnectionStatus);

impl From<VoiceSettings> for SetVoiceSettingsArgs {
    fn from(value: VoiceSettings) -> Self {
        Self(value)
    }
}
