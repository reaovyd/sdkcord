use crate::payload::request::macros::make_request_payload;
use derive_builder::Builder;
use paste::paste;
use serde::Serialize;
use uuid::Uuid;

make_request_payload!(SendActivityJoinInvite,
    #[doc = "Used to accept an Ask to Join request."],
    (user_id, String, "The id of the requesting user")
);

#[cfg(test)]
mod tests {
    use crate::payload::request::{
        SendActivityJoinInvite,
        SendActivityJoinInviteArgsBuilder,
    };

    #[test]
    fn test_build_request() {
        let request = SendActivityJoinInvite::new(
            SendActivityJoinInviteArgsBuilder::default()
                .user_id("abcjoe".to_owned())
                .build()
                .unwrap(),
        );
        let request = serde_json::to_string(&request).unwrap();
        assert!(request.contains(r#""user_id":"abcjoe""#))
    }
}
