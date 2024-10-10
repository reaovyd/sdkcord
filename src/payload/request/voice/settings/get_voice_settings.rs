use crate::payload::request::{
    macros::make_request_payload,
    EmptyArgs,
};
use serde::Serialize;
use crate::payload::request::Request;
use uuid::Uuid;

make_request_payload!(
    GetVoiceSettings,
    #[doc = "Used to retrieve the client's voice settings"]
);
