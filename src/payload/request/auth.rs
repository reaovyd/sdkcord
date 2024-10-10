use std::collections::HashSet;

use crate::payload::request::{
    macros::make_request_payload,
    Request,
};
use derive_builder::Builder;
use paste::paste;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

// TODO: definitely need more docs around here somehow...
make_request_payload!(Authorize,
    (
        /// Used to authenticate a new client with your app. By default this pops up a modal in-app that asks the user to authorize access to your app
        /// More information can be found on the Discord docs website
    ),
    (scope, OAuth2Scopes, (#[doc = "scopes to authorize"])),
    (client_id, String, (#[doc = "OAuth2 application id"])),
    (response_type, Option<ResponseType>,
        (#[doc = "Authorization Response Type"]),
        (#[builder(setter(strip_option), default)], #[serde(skip_serializing_if = "Option::is_none")])
    ),
    (prompt, Option<Prompt>,
        (#[doc = "Authorization prompt"]),
        (#[builder(setter(strip_option), default)], #[serde(skip_serializing_if = "Option::is_none")])
    ),
    (code_challenge, Option<String>,
        (#[doc = "Authorization code challenge"]),
        (#[builder(setter(strip_option), default)], #[serde(skip_serializing_if = "Option::is_none")])
    ),
    (state, Option<String>,
        (#[doc = "Authorization State"]),
        (#[builder(setter(strip_option), default)], #[serde(skip_serializing_if = "Option::is_none")])
    ),
    (code_challenge_method, Option<CodeChallengeMethod>,
        (#[doc = "Authorization code challenge method"]),
        (#[builder(setter(strip_option), default)], #[serde(skip_serializing_if = "Option::is_none")])
    )
);

make_request_payload!(Authenticate,
    (
        /// Used to authenticate an existing client with your app
    ),
    (access_token, String, (#[doc = "OAuth2 access token"]))
);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OAuth2Scopes(Vec<OAuth2Scope>);

impl OAuth2Scopes {
    pub fn builder() -> OAuth2ScopesBuilder {
        OAuth2ScopesBuilder::default()
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ResponseType {
    Code,
    Token,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub enum CodeChallengeMethod {
    S256,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Prompt {
    None,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OAuth2ScopesBuilder {
    scopes: HashSet<OAuth2Scope>,
}

impl OAuth2ScopesBuilder {
    pub fn add_scope(mut self, scope: OAuth2Scope) -> Self {
        self.scopes.insert(scope);
        self
    }

    pub fn build(self) -> OAuth2Scopes {
        OAuth2Scopes(self.scopes.into_iter().collect::<Vec<OAuth2Scope>>())
    }
}

// TODO: add comments later
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum OAuth2Scope {
    #[serde(rename = "activities.read")]
    ActivitiesRead,
    #[serde(rename = "activities.write")]
    ActivitiesWrite,
    #[serde(rename = "applications.builds.read")]
    ApplicationsBuildsRead,
    #[serde(rename = "applications.builds.upload")]
    ApplicationsBuildsUpload,
    #[serde(rename = "applications.commands")]
    ApplicationsCommands,
    #[serde(rename = "applications.commands.update")]
    ApplicationsCommandsUpdate,
    #[serde(rename = "applications.commands.present.update")]
    ApplicationsCommandsPermissionsUpdate,
    #[serde(rename = "applications.entitlements")]
    ApplicationsEntitlements,
    #[serde(rename = "applications.store.update")]
    ApplicationsStoreUpdate,
    #[serde(rename = "bot")]
    Bot,
    #[serde(rename = "connections")]
    Connections,
    #[serde(rename = "dm_channels.read")]
    DmChannelsRead,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "gdm.join")]
    GdmJoin,
    #[serde(rename = "guilds")]
    Guilds,
    #[serde(rename = "guilds.join")]
    GuildsJoin,
    #[serde(rename = "guilds.members.read")]
    GuildsMembersRead,
    #[serde(rename = "identify")]
    Identify,
    #[serde(rename = "messages.read")]
    MessagesRead,
    #[serde(rename = "relationships.read")]
    RelationshipsRead,
    #[serde(rename = "role_connections.write")]
    RoleConnectionsWrite,
    #[serde(rename = "rpc")]
    Rpc,
    #[serde(rename = "rpc.activites.write")]
    RpcActivitiesWrite,
    #[serde(rename = "rpc.notifications.read")]
    RpcNotificationsRead,
    #[serde(rename = "rpc.voice.read")]
    RpcVoiceRead,
    #[serde(rename = "rpc.voice.write")]
    RpcVoiceWrite,
    #[serde(rename = "voice")]
    Voice,
    #[serde(rename = "weboook.incoming")]
    WebhookIncoming,
}

#[cfg(test)]
mod tests {
    use crate::payload::request::{
        CodeChallengeMethod,
        Prompt,
        ResponseType,
    };

    use super::{
        Authorize,
        AuthorizeArgsBuilder,
        OAuth2Scope,
        OAuth2Scopes,
    };

    #[test]
    fn test_authorize_construct() {
        let cmd = Authorize::new(
            AuthorizeArgsBuilder::create_empty()
                .scope(
                    OAuth2Scopes::builder()
                        .add_scope(OAuth2Scope::Email)
                        .add_scope(OAuth2Scope::Voice)
                        .build(),
                )
                .client_id("client_id1")
                .response_type(ResponseType::Code)
                .code_challenge("abc")
                .code_challenge_method(CodeChallengeMethod::S256)
                .state("")
                .prompt(Prompt::None)
                .build()
                .unwrap(),
        );
        assert_eq!(cmd.args.client_id, "client_id1");
        for expected_scope in [OAuth2Scope::Email, OAuth2Scope::Voice] {
            assert!(cmd.args.scope.0.contains(&expected_scope));
        }
        assert_eq!(cmd.args.response_type, Some(ResponseType::Code));
        assert_eq!(cmd.args.code_challenge, Some("abc".to_string()));
        assert_eq!(cmd.args.code_challenge_method, Some(CodeChallengeMethod::S256));
        assert_eq!(cmd.args.state, Some("".to_string()));
        assert_eq!(cmd.args.prompt, Some(Prompt::None));
    }

    #[test]
    fn test_authorize_serialized() {
        let cmd = Authorize::new(
            AuthorizeArgsBuilder::create_empty()
                .scope(
                    OAuth2Scopes::builder()
                        .add_scope(OAuth2Scope::Email)
                        .add_scope(OAuth2Scope::Voice)
                        .add_scope(OAuth2Scope::GuildsMembersRead)
                        .build(),
                )
                .client_id("client_id1")
                .response_type(ResponseType::Code)
                .code_challenge("abc")
                .code_challenge_method(CodeChallengeMethod::S256)
                .state("")
                .prompt(Prompt::None)
                .build()
                .unwrap(),
        );
        let serialized = serde_json::to_string(&cmd).unwrap();
        assert!(serialized.contains(r#"abc"#));
        assert!(serialized.contains(r#"S256"#));
        assert!(serialized.contains(r#"client_id1"#));
        assert!(serialized.contains(r#""email""#));
        assert!(serialized.contains(r#""voice""#));
        assert!(serialized.contains(r#"none"#));
        assert!(serialized.contains(r#""guilds.members.read""#));
    }
}
