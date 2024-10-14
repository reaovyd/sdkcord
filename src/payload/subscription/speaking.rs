use crate::payload::macros::make_subscription_event;

make_subscription_event!(SpeakingStart,
    (#[doc = "sent when a user in a subscribed voice channel speaks"]),
    (channel_id, String, (#[doc = "id of channel to listen to updates of"]), (#[builder(into)]))
);

make_subscription_event!(SpeakingStop,
    (#[doc = "sent when a user in a subscribed voice channel stops speaking"]),
    (channel_id, String, (#[doc = "id of channel to listen to updates of"]), (#[builder(into)]))
);
