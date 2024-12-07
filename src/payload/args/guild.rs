use bon::{builder, Builder};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::macros::impl_request_args_type;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct GetGuildArgs {
    #[builder(into)]
    guild_id: String,
    timeout: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct GetGuildsArgs;

impl_request_args_type!(GetGuild);
impl_request_args_type!(GetGuilds);

#[cfg(test)]
mod tests {
    use super::GetGuildArgs;

    #[test]
    fn construct_timeout_optional() {
        let args = GetGuildArgs::builder().guild_id("1234").build();
        assert_eq!(args.guild_id.as_str(), "1234");
        assert_eq!(args.timeout, None);
    }

    #[test]
    fn construct_timeout_existing() {
        let args = GetGuildArgs::builder().guild_id("1234").timeout(12345).build();
        assert_eq!(args.guild_id.as_str(), "1234");
        assert_eq!(args.timeout, Some(12345));
    }
}
