use super::{
    macros::make_subscription_event,
    EmptyArgs,
};
use derive_builder::Builder;
use paste::paste;
use serde::Serialize;
use uuid::Uuid;
use crate::payload::subscription::SubscribeRequest;
use crate::payload::subscription::UnsubscribeRequest;

make_subscription_event!(GuildStatus,
    #[doc = "sent when a subscribed server's state changes"],
    (guild_id, String, (#[doc = "id of guild to listen to updates of"]))
);

make_subscription_event!(GuildCreate,
    #[doc = "sent when a guild is created/joined on the client"]
);

#[cfg(test)]
mod guild_tests {
    use pretty_assertions::assert_eq;

    use super::{
        GuildStatusEventArgsBuilder,
        GuildStatusSubscriptionEvent,
    };

    #[test]
    fn guild_status_construction_subscription() {
        let guild_status = GuildStatusSubscriptionEvent::new(
            GuildStatusEventArgsBuilder::create_empty().guild_id("id1").build().unwrap(),
        );
        assert_eq!(guild_status.args.guild_id, "id1".to_owned());
    }

    #[test]
    fn guild_status_construction_subscription_serialized() {
        let guild_status = GuildStatusSubscriptionEvent::new(
            GuildStatusEventArgsBuilder::create_empty().guild_id("id1").build().unwrap(),
        );
        let ser = serde_json::to_string(&guild_status).unwrap();
        assert!(ser.contains(r#"{"guild_id":"id1"}"#))
    }
}
