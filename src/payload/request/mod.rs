pub use activity::*;
pub use channel::*;
pub use device::*;
pub use guild::*;
pub use voice::*;
// TODO: subscriptions - bit of a special case since it includes the `evt` now.
// somehow want to tightly couple the evt enum Event enum and

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub enum Request {
    GetGuild(GetGuild),
    GetGuilds(GetGuilds),
    GetChannel(GetChannel),
    GetChannels(GetChannels),
    SetUserVoiceSettings(SetUserVoiceSettings),
    SelectVoiceChannel(SelectVoiceChannel),
    GetSelectedVoiceChannel(GetSelectedVoiceChannel),
    SelectTextChannel(SelectTextChannel),
    GetVoiceSettings(GetVoiceSettings),
    SetVoiceSettings(SetVoiceSettings),
    SetCertifiedDevices(SetCertifiedDevices),
    SetActivity(SetActivity),
    SendActivityJoinInvite(SendActivityJoinInvite),
    CloseActivityRequest(CloseActivityRequest)
}

use serde::Serialize;
mod macros {
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
        ($request_name: ident, $(#[$request_doc:meta]),*, $(($field_name: ident, $field_type: ty, $field_doc: expr $(,#[$skip_serial: meta])? )),*) => {
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
                #[builder(derive(Debug))]
                // lint isn't catching a /**/ comment where the Errors section is according to
                // cargo expand
                #[allow(clippy::missing_errors_doc)]
                pub struct [<$request_name Args>] {
                    $(
                        #[doc = $field_doc]
                        $(
                            #[$skip_serial]
                        )*
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
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash, Default)]
pub struct EmptyArgs {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    inner: Option<()>,
}


mod activity;
mod channel;
mod device;
mod guild;
mod voice;
