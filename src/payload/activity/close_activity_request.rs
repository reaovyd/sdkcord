use bon::builder;

use crate::payload::macros::make_command_reqres_payload;

make_command_reqres_payload!(CloseActivityRequest,
    (
        /// Used to reject an Ask to Join request.
    ),
    (user_id, String, (#[doc = "The id of the requesting user"]), (#[builder(into)]))
);

#[cfg(test)]
mod tests {

    use crate::payload::CloseActivityRequestArgs;

    use super::CloseActivityRequest;

    #[test]
    fn test_build_request() {
        let request =
            CloseActivityRequest::new(CloseActivityRequestArgs::builder().user_id("asd").build());
        let request = serde_json::to_string(&request).unwrap();
        assert!(request.contains(r#""user_id":"asd""#))
    }
}
