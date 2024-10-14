use crate::payload::macros::make_command_reqres_payload;

make_command_reqres_payload!(SelectVoiceChannel,
    (
        /// Used to join or leave a voice channel, group dm, or dm
    ),
    (channel_id, Option<String>,
        (#[doc = "channel id to join (or null/Option::None to leave)"]),
        (#[builder(into)])
    ),
    (timeout, Option<u32>,
        (#[doc = "asynchronously join channel with time to wait before timing out"]),
        (
            #[serde(skip_serializing_if = "Option::is_none")]
            #[builder(into)]
        )
    ),
    (force, Option<bool>,
        (#[doc = "forces a user to join a voice channel"]),
        (
            #[serde(skip_serializing_if = "Option::is_none")]
            #[builder(into)]
        )
    ),
    (navigate, Option<bool>,
        (#[doc = "after joining the voice channel, navigate to it in the client"]),
        (
            #[serde(skip_serializing_if = "Option::is_none")]
            #[builder(into)]
        )
    )
);

make_command_reqres_payload!(
    GetSelectedVoiceChannel,
    (
        /// Used to get the current voice channel the client is in
    )
);
