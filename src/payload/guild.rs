use bon::{Builder, builder};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use super::{
    common::guild::Guild,
    macros::{impl_empty_args_type, impl_event_args_type, impl_request_args_type},
};

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct GetGuildArgs {
    #[builder(into)]
    guild_id: String,
    timeout: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct GuildStatusArgs {
    #[builder(into)]
    guild_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct GuildStatusData {
    pub guild: Option<Guild>,
    pub online: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct GetGuildData {
    pub icon_url: Option<Url>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub vanity_url_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct GetGuildsData {
    pub guilds: Option<Vec<Guild>>,
}

impl_empty_args_type!(GetGuilds);
impl_empty_args_type!(GuildCreate);

impl_request_args_type!(GetGuild);
impl_request_args_type!(GetGuilds);

impl_event_args_type!(GuildStatus);
impl_event_args_type!(GuildCreate);

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
        let args = GetGuildArgs::builder()
            .guild_id("1234")
            .timeout(12345)
            .build();
        assert_eq!(args.guild_id.as_str(), "1234");
        assert_eq!(args.timeout, Some(12345));
    }
}
