use super::{
    macros::make_subscription_event,
    EmptyArgs,
};
use crate::payload::subscription::{
    SubscribeRequest,
    UnsubscribeRequest,
};
use paste::paste;
use serde::Serialize;
use uuid::Uuid;

make_subscription_event!(NotificationCreate,
    (#[doc = "No arguments. This event requires the rpc.notifications.read OAuth2 scope"])
);
