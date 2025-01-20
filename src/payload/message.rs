use crate::payload::common::channel::impl_channel_id_type;

use super::macros::impl_event_args_type;

impl_channel_id_type!(MessageCreateArgs);
impl_channel_id_type!(MessageUpdateArgs);
impl_channel_id_type!(MessageDeleteArgs);

impl_event_args_type!(MessageCreate);
impl_event_args_type!(MessageUpdate);
impl_event_args_type!(MessageDelete);
