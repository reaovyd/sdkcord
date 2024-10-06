macro_rules! make_request_payload {
    ($request_name: ident, $(#[$request_doc:meta]),*) => {
        #[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
        $(
            #[$request_doc]
        )*
        pub struct $request_name {
            nonce: Uuid,
            args: EmptyArgs
        }

        impl $request_name {
            pub fn new() -> Self {
                Self {
                    nonce: Uuid::new_v4(),
                    args: EmptyArgs::default()
                }
            }
        }

        impl Default for $request_name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
    ($request_name: ident, $(#[$request_doc:meta]),*, $(($field_name: ident, $field_type: ty, ($(#[$field_doc: meta]),*) $(, ($(#[$addt_dctv: meta]),*))? )),*) => {
        #[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
        $(
            #[$request_doc]
        )*
        pub struct $request_name {
            nonce: Uuid,
            args: paste!([<$request_name Args>])
        }

        paste! {
            #[derive(Builder, Debug, Clone, Serialize, PartialEq, Eq, Hash)]
            #[builder(derive(Debug), setter(into))]
            // lint isn't catching a /**/ comment where the Errors section is according to
            // cargo expand
            #[allow(clippy::missing_errors_doc)]
            pub struct [<$request_name Args>] {
                $(
                    $(
                        #[$field_doc]
                    )*
                    $(
                        $(
                            #[$addt_dctv]
                        )*
                    )?
                    $field_name: $field_type
                ),*
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
