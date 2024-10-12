use crate::payload::macros::make_subscription_event;

make_subscription_event!(ChannelCreate,
    (#[doc = "sent when a channel is created/joined on the client"])
);
