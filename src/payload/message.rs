use serde::{Deserialize, Serialize};

use crate::payload::common::channel::impl_channel_id_type;

use super::{
    common::{channel::ChannelId, message::Message},
    macros::impl_event_args_type,
};

impl_channel_id_type!(MessageCreateArgs);
impl_channel_id_type!(MessageUpdateArgs);
impl_channel_id_type!(MessageDeleteArgs);

impl_event_args_type!(MessageCreate);
impl_event_args_type!(MessageUpdate);
impl_event_args_type!(MessageDelete);

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MessageCreateData(pub MessageEventData);
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MessageUpdateData(pub MessageEventData);
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MessageDeleteData(pub MessageEventData);

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MessageEventData {
    #[serde(flatten)]
    pub channel: ChannelId,
    pub message: Message,
}
