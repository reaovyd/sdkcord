mod macros {
    macro_rules! make_request_payload {
        ($request_name: ident, $(($field_name: ident, $field_type: ty, $field_doc: expr)),+) => {
            use derive_builder::Builder;
            use paste::paste;
            use serde::Serialize;
            use uuid::Uuid;

            #[derive(Debug, Clone, Serialize)]
            pub struct $request_name {
                nonce: Uuid,
                args: paste!([<$request_name Args>])
            }

            paste! {
                #[derive(Builder, Debug, Clone, Serialize, PartialEq, Eq, Hash)]
                #[builder(derive(Debug))]
                // lint isn't catching a /**/ comment where the Errors section is according to
                // cargo expand
                #[allow(clippy::missing_errors_doc)]
                pub struct [<$request_name Args>] {
                    $(
                        #[doc = $field_doc]
                        $field_name: $field_type
                    ),+
                }
            }

            impl $request_name {
                pub fn new(args: paste!([<$request_name Args>])) -> Self {
                    Self {
                        nonce: Uuid::new_v4(),
                        args
                    }
                }
            }

        };
    }
    pub(crate) use make_request_payload;
}

use macros::make_request_payload;
