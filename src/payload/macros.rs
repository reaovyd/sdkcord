macro_rules! make_request_payload {
    ($request_name: ident, ($(#[$request_doc:meta])*), { $request_type: ty; $request_enum_val: ident }) => {
        #[derive(Debug, Clone, serde::Serialize, PartialEq, Eq, Hash)]
        $(
            #[$request_doc]
        )*
        pub struct $request_name {
            nonce: uuid::Uuid,
            args: $crate::payload::EmptyArgs
        }

        impl $request_name {
            pub fn new() -> Self {
                Self {
                    nonce: uuid::Uuid::new_v4(),
                    args: $crate::payload::EmptyArgs::default()
                }
            }

            pub const fn build(self) -> $request_type {
                <$request_type>::$request_enum_val(self)
            }
        }

        impl Default for $request_name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
    ($request_name: ident, ($(#[$request_doc:meta])*), { $request_type: ty; $request_enum_val: ident }, $(($field_name: ident, $field_type: ty, ($(#[$field_doc: meta])*) $(, ($(#[$addt_dctv: meta]),*))? )),*) => {
        #[derive(Debug, Clone, serde::Serialize, PartialEq, Eq, Hash)]
        $(
            #[$request_doc]
        )*
        pub struct $request_name {
            nonce: uuid::Uuid,
            args: paste::paste!([<$request_name Args>])
        }

        paste::paste! {
            #[derive(derive_builder::Builder, Debug, Clone, serde::Serialize, PartialEq, Eq, Hash)]
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
            pub fn new(args: paste::paste!([<$request_name Args>])) -> Self {
                Self {
                    nonce: uuid::Uuid::new_v4(),
                    args
                }
            }
            pub const fn build(self) -> $request_type {
                <$request_type>::$request_enum_val(self)
            }
        }

    };
}

macro_rules! make_command_reqres_payload {
    ($request_name: ident, ($(#[$request_doc:meta])*)) => {
        $crate::payload::macros::make_request_payload!($request_name, ($(#[$request_doc])*), { $crate::payload::Request; $request_name });
    };

    ($request_name: ident, ($(#[$request_doc:meta])*), $(($field_name: ident, $field_type: ty, ($(#[$field_doc: meta])*) $(, ($(#[$addt_dctv: meta]),*))? )),*) => {
        $crate::payload::macros::make_request_payload!(
            $request_name,
            ($(#[$request_doc])*),
            { $crate::payload::Request; $request_name },
            $(($field_name, $field_type, ($(#[$field_doc])*) $(, ($(#[$addt_dctv]),*))? )),*
        );
    };
}

macro_rules! make_subscription_event {
    ($evt_name: ident, ($(#[$evt_doc:meta])*)) => {
        paste::paste! {
            $crate::payload::macros::make_request_payload!(
                [<$evt_name EventRequest>],
                ($(#[$evt_doc])*),
                { $crate::payload::EventRequest; $evt_name }
            );
        }
    };
    ($evt_name: ident, ($(#[$evt_doc:meta])*), $(($field_name: ident, $field_type: ty, ($(#[$field_doc: meta])*) $(, ($(#[$addt_dctv: meta]),*))? )),*) => {
        paste::paste! {
            $crate::payload::macros::make_request_payload!(
                [<$evt_name EventRequest>],
                ($(#[$evt_doc])*),
                { $crate::payload::EventRequest; $evt_name },
                $(($field_name, $field_type, ($(#[$field_doc])*) $(, ($(#[$addt_dctv]),*))? )),*
            );
        }
    };
}

pub(crate) use make_command_reqres_payload;
pub(crate) use make_request_payload;
pub(crate) use make_subscription_event;
