use crate::payload::types::channel::impl_channel_id_type;

use super::macros::impl_event_args_type;

impl_channel_id_type!(SpeakingStartArgs);
impl_channel_id_type!(SpeakingStopArgs);

impl_event_args_type!(SpeakingStart);
impl_event_args_type!(SpeakingStop);
