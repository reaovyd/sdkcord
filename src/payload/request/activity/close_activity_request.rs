use crate::payload::request::macros::make_request_payload;
use derive_builder::Builder;
use paste::paste;
use serde::Serialize;
use uuid::Uuid;

make_request_payload!(CloseActivityRequest,
    #[doc = "Used to reject an Ask to Join request."],
    (user_id, String, "The id of the requesting user")
);
