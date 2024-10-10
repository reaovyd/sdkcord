use crate::payload::request::{
    macros::make_request_payload,
    Request,
};
use derive_builder::Builder;
use paste::paste;
use serde::Serialize;
use uuid::Uuid;

make_request_payload!(CloseActivityRequest,
    (
        /// Used to reject an Ask to Join request.
    ),
    (user_id, String, (#[doc = "The id of the requesting user"]))
);

#[cfg(test)]
mod tests {

    use super::{
        CloseActivityRequest,
        CloseActivityRequestArgsBuilder,
    };

    #[test]
    fn test_build_request() {
        let request = CloseActivityRequest::new(
            CloseActivityRequestArgsBuilder::default().user_id("asd".to_owned()).build().unwrap(),
        );
        let request = serde_json::to_string(&request).unwrap();
        assert!(request.contains(r#""user_id":"asd""#))
    }
}
