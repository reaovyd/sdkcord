macro_rules! impl_event_args_type {
    ($args_name: ident) => {
        paste::paste! {
            impl $crate::payload::ArgsType for [<$args_name Args>] {
                fn args_val(self) -> crate::payload::Args {
                    crate::payload::Args::$args_name(self)
                }
            }

            impl $crate::payload::EventArgsType for [<$args_name Args>] {
                fn name(&self) -> crate::payload::Event {
                    crate::payload::Event::$args_name
                }
            }

            impl $crate::payload::sealed::Sealed for [<$args_name Args>] {}
        }
    };
}

macro_rules! impl_request_args_type {
    ($args_name: ident) => {
        paste::paste! {
            impl $crate::payload::ArgsType for [<$args_name Args>] {
                fn args_val(self) -> crate::payload::Args {
                    crate::payload::Args::$args_name(self)
                }
            }

            impl $crate::payload::RequestArgsType for [<$args_name Args>] {
                fn name(&self) -> crate::payload::Command {
                    crate::payload::Command::$args_name
                }
            }

            impl $crate::payload::sealed::Sealed for [<$args_name Args>] {}
        }
    };
}

macro_rules! impl_empty_args_type {
        ($args_name: ident) => {
            paste::paste! {
                #[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq, Hash)]
                pub struct [<$args_name Args>]($crate::payload::EmptyBracket);
            }
        };
    }

pub(crate) use impl_empty_args_type;
pub(crate) use impl_event_args_type;
pub(crate) use impl_request_args_type;
