use super::{
    macros::make_subscription_event,
    EmptyArgs,
};
use paste::paste;
use serde::Serialize;
use uuid::Uuid;

make_subscription_event!(ChannelCreate,
    #[doc = "Sent when a channel is created/joined on the client"]
);
