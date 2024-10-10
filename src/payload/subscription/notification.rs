use super::{
    macros::make_subscription_event,
    EmptyArgs,
};
use paste::paste;
use serde::Serialize;
use uuid::Uuid;
use crate::payload::subscription::SubscribeRequest;
use crate::payload::subscription::UnsubscribeRequest;

make_subscription_event!(NotificationCreate,
    (#[doc = "No arguments. This event requires the rpc.notifications.read OAuth2 scope"])
);
