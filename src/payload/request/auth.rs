use std::collections::HashSet;

use crate::payload::request::macros::make_request_payload;
use derive_builder::Builder;
use paste::paste;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

make_request_payload!(Authorize,
    #[doc = "Used to authenticate a new client with your app. By default this pops up a modal in-app that asks the user to authorize access to your app."],
    #[doc = "More information can be found on the Discord docs website"],
    (scopes, OAuth2Scopes, "scopes to authorize"),
    (client_id, String, "OAuth2 application id"),
    (rpc_token, String, "one-time use RPC token"),
    (username, String, "username to create a guest account with if the user does not have Discord")
);

make_request_payload!(Authenticate,
    #[doc = "Used to authenticate an existing client with your app."],
    (access_token, String, "OAuth2 access token")
);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OAuth2Scopes(Vec<OAuth2Scope>);

impl OAuth2Scopes {
    pub fn builder() -> OAuth2ScopesBuilder {
        OAuth2ScopesBuilder::default()
    }
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
    #[serde(rename = "guilds_members_read")]
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
                .scopes(
                    OAuth2Scopes::builder()
                        .add_scope(OAuth2Scope::Email)
                        .add_scope(OAuth2Scope::Voice)
                        .build(),
                )
                .username("username1".to_string())
                .rpc_token("rpc_token1".to_string())
                .client_id("client_id1".to_string())
                .build()
                .unwrap(),
        );
        println!("{:?}", serde_json::to_string(&cmd));
    }
}
