use serde::{Deserialize, Serialize};

use crate::payload::common::channel::impl_channel_id_type;

use super::macros::impl_event_args_type;

impl_channel_id_type!(SpeakingStartArgs);
impl_channel_id_type!(SpeakingStopArgs);

impl_event_args_type!(SpeakingStart);
impl_event_args_type!(SpeakingStop);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct SpeakingData {
    pub user_id: Option<String>,
    pub channel_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SpeakingStartData(pub Option<SpeakingData>);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SpeakingStopData(pub Option<SpeakingData>);
