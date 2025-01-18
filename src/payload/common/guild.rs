use bon::Builder;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct GuildId {
    #[builder(into)]
    guild_id: String,
}

mod macros {
    macro_rules! impl_guild_id_type {
        ($args_name: ident) => {
            #[serde_with::skip_serializing_none]
            #[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq, Hash)]
            pub struct $args_name(pub $crate::payload::common::guild::GuildId);
            impl From<$crate::payload::common::guild::GuildId> for $args_name {
                fn from(value: $crate::payload::common::guild::GuildId) -> Self {
                    Self(value)
                }
            }
        };
    }
    pub(crate) use impl_guild_id_type;
}

pub(crate) use macros::impl_guild_id_type;
