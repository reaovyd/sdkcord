use crate::payload::macros::make_command_reqres_payload;

make_command_reqres_payload!(SendActivityJoinInvite,
    (
        /// Used to accept an Ask to Join request
    ),
    (user_id, String, (#[doc = "The id of the requesting user"]), (#[builder(into)]))
);

#[cfg(test)]
mod tests {
    use crate::payload::{
        SendActivityJoinInvite,
        SendActivityJoinInviteArgs,
    };

    #[test]
    fn test_build_request() {
        let request = SendActivityJoinInvite::new(
            SendActivityJoinInviteArgs::builder().user_id("abcjoe").build(),
        );
        let request = serde_json::to_string(&request).unwrap();
        assert!(request.contains(r#""user_id":"abcjoe""#))
    }
}
