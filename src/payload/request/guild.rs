use super::EmptyArgs;
use crate::payload::request::macros::make_request_payload;
use derive_builder::Builder;
use paste::paste;
use serde::Serialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

make_request_payload!(
    GetGuild,
    (guild_id, String, "Id of the guild to get"),
    (timeout, Option<u32>, "asynchronously get guild with time to wait before timing out")
);
make_request_payload!(GetGuilds);

#[cfg(test)]
mod get_guild_tests {
    use pretty_assertions::assert_eq;

    use super::{
        GetGuild,
        GetGuildArgsBuilder,
    };

    #[test]
    fn test_construction_basic() {
        let guild = GetGuild::new(
            GetGuildArgsBuilder::create_empty()
                .guild_id("guild_id".to_string())
                .timeout(Some(32))
                .build()
                .unwrap(),
        );
        assert_eq!(guild.args.guild_id, "guild_id".to_string());
        assert_eq!(guild.args.timeout, Some(32));
    }

    #[test]
    fn test_serialization_timeout_empty() {
        let guild = GetGuild::new(
            GetGuildArgsBuilder::create_empty()
                .guild_id("guild_id".to_string())
                .timeout(None)
                .build()
                .unwrap(),
        );

        assert_eq!(guild.args.timeout, None);

        let serialized = serde_json::to_string(&guild).unwrap();
        assert!(serialized.contains("\"guild_id\":"));
        assert!(!serialized.contains("\"timeout\":"))
    }

    #[test]
    fn test_serialization_timeout_filled() {
        let guild = GetGuild::new(
            GetGuildArgsBuilder::create_empty()
                .guild_id("guild_id".to_string())
                .timeout(Some(32))
                .build()
                .unwrap(),
        );

        assert_eq!(guild.args.timeout, Some(32));

        let serialized = serde_json::to_string(&guild).unwrap();
        assert!(serialized.contains("\"guild_id\":"));
        assert!(serialized.contains("\"timeout\":32"))
    }
}

#[cfg(test)]
mod get_guilds_tests {
    use pretty_assertions::assert_eq;

    use super::GetGuilds;

    #[test]
    fn test_construction_basic() {
        let guilds = GetGuilds::new();
        assert_eq!(guilds.args.inner, None);
    }

    #[test]
    fn test_serialization_empty_args() {
        let guilds = GetGuilds::new();
        let serialized = serde_json::to_string(&guilds).unwrap();
        assert!(serialized.contains("\"args\":{}"))
    }
}
