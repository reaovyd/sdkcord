use crate::payload::request::{
    macros::make_request_payload,
    EmptyArgs,
};
use derive_builder::Builder;
use paste::paste;
use serde::Serialize;
use uuid::Uuid;

make_request_payload!(SelectVoiceChannel,
    #[doc = "Used to join or leave a voice channel, group dm, or dm"],
    (channel_id, Option<String>, 
        (#[doc = "channel id to join (or null/Option::None to leave)"]),
        (#[builder(setter(strip_option), default)])
    ),
    (timeout, Option<u32>,
        (#[doc = "asynchronously join channel with time to wait before timing out"]),
        (#[serde(skip_serializing_if = "Option::is_none")], #[builder(setter(strip_option), default)])
    ),
    (force, Option<bool>, 
        (#[doc = "forces a user to join a voice channel"]), 
        (#[serde(skip_serializing_if = "Option::is_none")], #[builder(setter(strip_option), default)])
    ),
    (navigate, Option<bool>,
        (#[doc = "after joining the voice channel, navigate to it in the client"]), 
        (#[serde(skip_serializing_if = "Option::is_none")], #[builder(setter(strip_option), default)])
    )
);

make_request_payload!(
    GetSelectedVoiceChannel,
    #[doc = "Used to get the current voice channel the client is in"]
);
