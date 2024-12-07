use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
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
