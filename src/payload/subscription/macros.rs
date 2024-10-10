macro_rules! make_subscription_event {
    ($evt_name: ident, ($(#[$request_doc:meta])*)) => {
        paste! {
            #[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
            $(
                #[$request_doc]
            )*
            pub struct [<$evt_name SubscriptionEvent>] {
                nonce: Uuid,
                args: EmptyArgs,
                cmd: String
            }
        }

        impl paste!([<$evt_name SubscriptionEvent>]) {
            pub fn new() -> Self {
                Self {
                    nonce: Uuid::new_v4(),
                    args: EmptyArgs::default(),
                    cmd: "SUBSCRIBE".to_string()
                }
            }
            
            pub const fn make_request(self) -> SubscribeRequest {
                SubscribeRequest::$evt_name(self) 
            } 
        }

        impl Default for paste!([<$evt_name SubscriptionEvent>]) {
            fn default() -> Self {
                Self::new()
            }
        }

        paste! {
            #[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
            $(
                #[$request_doc]
            )*
            pub struct [<$evt_name UnsubscriptionEvent>] {
                nonce: Uuid,
                args: EmptyArgs,
                cmd: String
            }
        }

        impl paste!([<$evt_name UnsubscriptionEvent>]) {
            pub fn new() -> Self {
                Self {
                    nonce: Uuid::new_v4(),
                    args: EmptyArgs::default(),
                    cmd: "UNSUBSCRIBE".to_string()
                }
            }

            pub const fn make_request(self) -> UnsubscribeRequest {
                UnsubscribeRequest::$evt_name(self) 
            } 
        }

        impl Default for paste!([<$evt_name UnsubscriptionEvent>]) {
            fn default() -> Self {
                Self::new()
            }
        }
    };
    ($evt_name: ident, ($(#[$event_doc:meta])*), $(($field_name: ident, $field_type: ty, ($(#[$field_doc: meta])*) $(, ($(#[$addt_dctv: meta]),*))? )),*) => {
        paste! {
            #[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
            $(
                #[$event_doc]
            )*
            pub struct [<$evt_name SubscriptionEvent>] {
                nonce: Uuid,
                args: [<$evt_name EventArgs>],
                cmd: String
            }
        }

        paste! {
            #[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
            $(
                #[$event_doc]
            )*
            pub struct [<$evt_name UnsubscriptionEvent>] {
                nonce: Uuid,
                args: [<$evt_name EventArgs>],
                cmd: String
            }
        }

        paste! {
            #[derive(Builder, Debug, Clone, Serialize, PartialEq, Eq, Hash)]
            #[builder(derive(Debug), setter(into))]
            // lint isn't catching a /**/ comment where the Errors section is according to
            // cargo expand
            #[allow(clippy::missing_errors_doc)]
            pub struct [<$evt_name EventArgs>] {
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

        impl paste!([<$evt_name SubscriptionEvent>]) {
            pub fn new(args: paste!([<$evt_name EventArgs>])) -> Self {
                Self {
                    nonce: Uuid::new_v4(),
                    args,
                    cmd: "SUBSCRIBE".to_string()
                }
            }

            pub const fn make_request(self) -> SubscribeRequest {
                SubscribeRequest::$evt_name(self) 
            } 
        }

        impl paste!([<$evt_name UnsubscriptionEvent>]) {
            pub fn new(args: paste!([<$evt_name EventArgs>])) -> Self {
                Self {
                    nonce: Uuid::new_v4(),
                    args,
                    cmd: "UNSUBSCRIBE".to_string()
                }
            }

            pub const fn make_request(self) -> UnsubscribeRequest {
                UnsubscribeRequest::$evt_name(self) 
            } 
        }

    };
}
pub(crate) use make_subscription_event;
