use super::macros::make_request_payload;
use derive_builder::Builder;
use paste::paste;
use serde::Serialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

make_request_payload!(
    GetChannel,
    "Used to retrieve channel information from the client",
    (channel_id, String, "id of the channel to get")
);
make_request_payload!(
    GetChannels,
    "Used to retrieve a list of channels for a guild from the client",
    (guild_id, String, "id of the guild to get channels for")
);

#[cfg(test)]
mod get_channel_tests {
    use pretty_assertions::assert_str_eq;

    use super::{
        GetChannel,
        GetChannelArgsBuilder,
    };

    #[test]
    fn test_construction_basic() {
        let get_channel = GetChannel::new(
            GetChannelArgsBuilder::create_empty()
                .channel_id("channel_id_123".to_owned())
                .build()
                .unwrap(),
        );
        assert_str_eq!(get_channel.args.channel_id, "channel_id_123");
    }

    #[test]
    fn test_serialization_contains_id() {
        let get_channel = GetChannel::new(
            GetChannelArgsBuilder::create_empty()
                .channel_id("channel_id_123".to_owned())
                .build()
                .unwrap(),
        );
        assert_str_eq!(get_channel.args.channel_id, "channel_id_123");
        let serialized = serde_json::to_string(&get_channel).unwrap();
        assert!(serialized.contains("\"channel_id\":\"channel_id_123\""));
    }
}

#[cfg(test)]
mod get_channels_tests {
    use pretty_assertions::assert_str_eq;

    use super::{
        GetChannels,
        GetChannelsArgsBuilder,
    };

    #[test]
    fn test_construction_basic() {
        let get_channels = GetChannels::new(
            GetChannelsArgsBuilder::create_empty().guild_id("hi123".to_string()).build().unwrap(),
        );
        assert_str_eq!(get_channels.args.guild_id, "hi123".to_owned());
    }

    #[test]
    fn test_serialization_contains_guild_id() {
        let get_channels = GetChannels::new(
            GetChannelsArgsBuilder::create_empty().guild_id("hi123".to_string()).build().unwrap(),
        );
        assert_str_eq!(get_channels.args.guild_id, "hi123".to_owned());
        let serialized = serde_json::to_string(&get_channels).unwrap();
        assert!(serialized.contains("\"guild_id\":\"hi123\""));
    }
}
