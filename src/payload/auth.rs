use std::{collections::HashSet, hash::RandomState, iter::FromIterator};

use bon::{Builder, builder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::payload::common::oauth2::OAuth2Scope;

use super::{
    common::{application::Application, user::User},
    macros::impl_request_args_type,
};

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct AuthenticateArgs {
    #[builder(into)]
    access_token: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct AuthorizeArgs {
    #[builder(with = |scopes: impl IntoIterator<Item = OAuth2Scope>| {
        HashSet::<OAuth2Scope, RandomState>::from_iter(scopes).into_iter().collect()
    })]
    scopes: Option<Vec<OAuth2Scope>>,
    #[builder(into)]
    client_id: String,
    #[builder(into)]
    rpc_token: Option<String>,
    #[builder(into)]
    username: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct AuthorizeData {
    pub code: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct AuthenticateData {
    pub access_token: Option<String>,
    pub scopes: Option<Vec<OAuth2Scope>>,
    pub expires: Option<DateTime<Utc>>,
    pub user: Option<User>,
    pub application: Option<Application>,
}

impl_request_args_type!(Authenticate);
impl_request_args_type!(Authorize);

#[cfg(test)]
mod tests {
    use crate::payload::{
        AuthorizeData,
        common::{oauth2::OAuth2Scope, user::UserFlags},
    };

    use super::{AuthenticateData, AuthorizeArgs};

    #[test]
    fn construct_args_unique_scopes() {
        let args = AuthorizeArgs::builder()
            .scopes([OAuth2Scope::Rpc, OAuth2Scope::Email, OAuth2Scope::Rpc])
            .client_id("asd")
            .rpc_token("abc")
            .username("123")
            .build();
        for scope in [OAuth2Scope::Rpc, OAuth2Scope::Email] {
            assert!(args.scopes.as_ref().unwrap().contains(&scope));
        }
        assert_eq!(args.scopes.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn deserialize_authenticate() {
        let payload = r##"{"access_token":"6jaC1nd1NFVjAsKEJlt3j2PKlCCBsl","application":{"bot":{"accent_color":null,"avatar":"df7a5c32e954703bfe2e61ad8c14c3dc","avatar_decoration_data":null,"banner":null,"banner_color":null,"bot":true,"clan":null,"discriminator":"4588","flags":0,"global_name":null,"id":"1276759902551015485","primary_guild":null,"public_flags":0,"username":"IPCCord"},"bot_public":true,"bot_require_code_grant":false,"description":"bruh","flags":0,"hook":true,"icon":"df7a5c32e954703bfe2e61ad8c14c3dc","id":"1276759902551015485","integration_types_config":{"1":{"oauth2_install_params":{"permissions":"0","scopes":["applications.commands"]}}},"is_discoverable":false,"is_monetized":false,"is_verified":false,"name":"gameing","storefront_available":false,"summary":"","type":null,"verify_key":"02d2b7977161590c0bdc6a5e67d75dc9333ba0f469a0fd2d2171964516bcc5ac"},"expires":"2024-12-29T08:59:39.827000+00:00","scopes":["identify","rpc","guilds"],"user":{"accent_color":16711680,"avatar":"3a67ea77a12df7de202bcf0d696eeee8","avatar_decoration_data":null,"banner":null,"banner_color":"#ff0000","clan":null,"discriminator":"0","flags":256,"global_name":"day 2","id":"158284148040138752","primary_guild":null,"public_flags":256,"username":"day2"}}"##;
        let data = serde_json::from_str::<AuthenticateData>(payload).unwrap();
        assert_eq!(
            data.access_token,
            Some("6jaC1nd1NFVjAsKEJlt3j2PKlCCBsl".to_string())
        );
        assert!(
            data.user
                .unwrap()
                .flags
                .unwrap()
                .contains(UserFlags::HYPESQUAD_ONLINE_HOUSE_3),
        );
    }

    #[test]
    fn deserialize_authorize() {
        let payload = r##"{"code":"G4mPcde04btJlhn27sdoJukoXQ2vmn"}"##;
        let data = serde_json::from_str::<AuthorizeData>(payload).unwrap();
        assert_eq!(
            data.code,
            Some("G4mPcde04btJlhn27sdoJukoXQ2vmn".to_string())
        );
    }
}
