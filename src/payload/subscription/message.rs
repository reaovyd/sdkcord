use super::macros::make_subscription_event;
use crate::payload::subscription::{
    SubscribeRequest,
    UnsubscribeRequest,
};
use derive_builder::Builder;
use paste::paste;
use serde::Serialize;
use uuid::Uuid;

make_subscription_event!(MessageCreate,
    (#[doc = "sent when a message is created in a subscribed text channel"]),
    (channel_id, String, (#[doc = "id of channel to listen to updates of"]))
);

make_subscription_event!(MessageUpdate,
    (#[doc = "sent when a message is updated in a subscribed text channel"]),
    (channel_id, String, (#[doc = "id of channel to listen to updates of"]))
);

make_subscription_event!(MessageDelete,
    (#[doc = "sent when a message is deleted in a subscribed text channel"]),
    (channel_id, String, (#[doc = "id of channel to listen to updates of"]))
);
