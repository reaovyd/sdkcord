use crate::payload::macros::make_subscription_event;

make_subscription_event!(GuildStatus,
    (#[doc = "sent when a subscribed server's state changes"]),
    (guild_id, String, (#[doc = "id of guild to listen to updates of"]), (#[builder(into)]))
);

make_subscription_event!(GuildCreate,
    (#[doc = "sent when a guild is created/joined on the client"])
);

#[cfg(test)]
mod guild_tests {
    use pretty_assertions::assert_eq;

    use crate::payload::GuildStatusEventRequestArgs;

    use super::GuildStatusEventRequest;

    #[test]
    fn guild_status_construction_subscription() {
        let guild_status = GuildStatusEventRequest::new(
            GuildStatusEventRequestArgs::builder().guild_id("id1").build(),
        );
        assert_eq!(guild_status.args.guild_id, "id1".to_owned());
    }

    #[test]
    fn guild_status_construction_subscription_serialized() {
        let guild_status = GuildStatusEventRequest::new(
            GuildStatusEventRequestArgs::builder().guild_id("id1").build(),
        );
        let ser = serde_json::to_string(&guild_status).unwrap();
        assert!(ser.contains(r#"{"guild_id":"id1"}"#))
    }
}
