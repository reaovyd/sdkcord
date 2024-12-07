use std::{collections::HashSet, hash::RandomState, iter::FromIterator};

use bon::{builder, Builder};
use serde::{Deserialize, Serialize};

use crate::payload::{types::oauth2::OAuth2Scope, Command};

use super::{sealed::Sealed, Args, ArgsType, RequestArgsType};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct AuthorizeArgs {
    #[builder(with = |scopes: impl IntoIterator<Item = OAuth2Scope>| {
        HashSet::<OAuth2Scope, RandomState>::from_iter(scopes).into_iter().collect()
    })]
    scopes: Vec<OAuth2Scope>,
    #[builder(into)]
    client_id: String,
    #[builder(into)]
    rpc_token: String,
    #[builder(into)]
    username: String,
}

impl ArgsType for AuthorizeArgs {
    fn args_val(self) -> Args {
        Args::Authorize(self)
    }
}

impl RequestArgsType for AuthorizeArgs {
    fn name(&self) -> Command {
        Command::Authorize
    }
}

impl Sealed for AuthorizeArgs {}

#[cfg(test)]
mod tests {
    use crate::payload::types::oauth2::OAuth2Scope;

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
            assert!(args.scopes.contains(&scope));
        }
        assert_eq!(args.scopes.len(), 2);
    }
}
