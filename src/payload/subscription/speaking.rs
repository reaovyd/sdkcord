use super::macros::make_subscription_event;
use derive_builder::Builder;
use paste::paste;
use serde::Serialize;
use uuid::Uuid;
use crate::payload::subscription::SubscribeRequest;
use crate::payload::subscription::UnsubscribeRequest;

make_subscription_event!(SpeakingStart,
    #[doc = "sent when a user in a subscribed voice channel speaks"],
    (channel_id, String, (#[doc = "id of channel to listen to updates of"]))
);

make_subscription_event!(SpeakingStop,
    #[doc = "sent when a user in a subscribed voice channel stops speaking"],
    (channel_id, String, (#[doc = "id of channel to listen to updates of"]))
);
