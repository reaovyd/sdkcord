use super::macros::make_command_reqres_payload;

make_command_reqres_payload!(
    GetChannel,
    (
        /// Used to retrieve channel information from the client
    ),
    (channel_id, String, (#[doc = "id of the channel to get"]), (#[builder(into)]))
);

make_command_reqres_payload!(
    GetChannels,
    (
        /// Used to retrieve a list of channels for a guild from the client
    ),
    (guild_id, String, (#[doc = "id of the guild to get channels for"]), (#[builder(into)]))
);

make_command_reqres_payload!(SelectTextChannel,
    (
        /// Used to join or leave a text channel, group dm, or dm
    ),
    (channel_id, Option<String>,
            (#[doc = "channel id to join (or null/Option::None to leave)"]),
            (
                #[builder(into)]
            )
    ),
    (timeout, Option<u32>,
            (#[doc = "asynchronously join channel with time to wait before timing out"]),
            (
                #[serde(skip_serializing_if = "Option::is_none")]
                #[builder(into)]
            )
    )
);

#[cfg(test)]
mod get_channel_tests {
    use pretty_assertions::assert_str_eq;

    use crate::payload::GetChannelArgs;

    use super::GetChannel;

    #[test]
    fn test_construction_basic() {
        let get_channel =
            GetChannel::new(GetChannelArgs::builder().channel_id("channel_id_123").build());
        assert_str_eq!(get_channel.args.channel_id, "channel_id_123");
    }

    #[test]
    fn test_serialization_contains_id() {
        let get_channel =
            GetChannel::new(GetChannelArgs::builder().channel_id("channel_id_123").build());
        assert_str_eq!(get_channel.args.channel_id, "channel_id_123");
        let serialized = serde_json::to_string(&get_channel).unwrap();
        assert!(serialized.contains("\"channel_id\":\"channel_id_123\""));
    }
}

#[cfg(test)]
mod get_channels_tests {
    use pretty_assertions::assert_str_eq;

    use crate::payload::GetChannelsArgs;

    use super::GetChannels;

    #[test]
    fn test_construction_basic() {
        let get_channels = GetChannels::new(GetChannelsArgs::builder().guild_id("hi123").build());
        assert_str_eq!(get_channels.args.guild_id, "hi123".to_owned());
    }

    #[test]
    fn test_serialization_contains_guild_id() {
        let get_channels = GetChannels::new(GetChannelsArgs::builder().guild_id("hi123").build());
        assert_str_eq!(get_channels.args.guild_id, "hi123".to_owned());
        let serialized = serde_json::to_string(&get_channels).unwrap();
        assert!(serialized.contains("\"guild_id\":\"hi123\""));
    }
}
