use std::{collections::HashSet, hash::RandomState, iter::FromIterator};

use bon::{builder, Builder};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::payload::common::oauth2::OAuth2Scope;

use super::macros::impl_request_args_type;

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
    pub code: String,
}

// TODO: Add Authenticate data fields
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct AuthenticateData {
    pub scopes: Vec<OAuth2Scope>,
}

impl_request_args_type!(Authenticate);
impl_request_args_type!(Authorize);

#[cfg(test)]
mod tests {
    use crate::payload::common::oauth2::OAuth2Scope;

    use super::AuthorizeArgs;

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
}
