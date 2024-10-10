use super::{
    macros::make_subscription_event,
    EmptyArgs,
};
use paste::paste;
use crate::payload::subscription::SubscribeRequest;
use crate::payload::subscription::UnsubscribeRequest;
use serde::Serialize;
use uuid::Uuid;

make_subscription_event!(ChannelCreate,
    (#[doc = "sent when a channel is created/joined on the client"])
);
