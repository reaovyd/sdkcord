use crate::payload::macros::make_subscription_event;

make_subscription_event!(NotificationCreate,
    (#[doc = "No arguments. This event requires the rpc.notifications.read OAuth2 scope"])
);
