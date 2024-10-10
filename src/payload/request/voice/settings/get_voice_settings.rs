use crate::payload::request::{
    macros::make_request_payload,
    EmptyArgs,
    Request,
};
use serde::Serialize;
use uuid::Uuid;

make_request_payload!(
    GetVoiceSettings,
    (
        /// Used to retrieve the client's voice settings
    )
);
