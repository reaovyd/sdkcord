use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(
    Debug,
    Copy,
    Clone,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    strum_macros::Display,
)]
pub enum OAuth2Scope {
    #[serde(rename = "activities.read")]
    #[strum(serialize = "activities.read")]
    ActivitiesRead,
    #[serde(rename = "activities.write")]
    #[strum(serialize = "activities.write")]
    ActivitiesWrite,
    #[serde(rename = "applications.builds.read")]
    #[strum(serialize = "applications.builds.read")]
    ApplicationsBuildsRead,
    #[serde(rename = "applications.builds.upload")]
    #[strum(serialize = "applications.builds.upload")]
    ApplicationsBuildsUpload,
    #[serde(rename = "applications.commands")]
    #[strum(serialize = "applications.commands")]
    ApplicationsCommands,
    #[serde(rename = "applications.commands.update")]
    #[strum(serialize = "applications.commands.update")]
    ApplicationsCommandsUpdate,
    #[serde(rename = "applications.commands.present.update")]
    #[strum(serialize = "applications.commands.present.update")]
    ApplicationsCommandsPermissionsUpdate,
    #[serde(rename = "applications.entitlements")]
    #[strum(serialize = "applications.entitlements")]
    ApplicationsEntitlements,
    #[serde(rename = "applications.store.update")]
    #[strum(serialize = "applications.store.update")]
    ApplicationsStoreUpdate,
    #[serde(rename = "bot")]
    #[strum(serialize = "bot")]
    Bot,
    #[serde(rename = "connections")]
    #[strum(serialize = "connections")]
    Connections,
    #[serde(rename = "dm_channels.read")]
    #[strum(serialize = "dm_channels.read")]
    DmChannelsRead,
    #[serde(rename = "email")]
    #[strum(serialize = "email")]
    Email,
    #[serde(rename = "gdm.join")]
    #[strum(serialize = "gdm.join")]
    GdmJoin,
    #[serde(rename = "guilds")]
    #[strum(serialize = "guilds")]
    Guilds,
    #[serde(rename = "guilds.join")]
    #[strum(serialize = "guilds.join")]
    GuildsJoin,
    #[serde(rename = "guilds.members.read")]
    #[strum(serialize = "guilds.members.read")]
    GuildsMembersRead,
    #[serde(rename = "identify")]
    #[strum(serialize = "identify")]
    Identify,
    #[serde(rename = "messages.read")]
    #[strum(serialize = "messages.read")]
    MessagesRead,
    #[serde(rename = "relationships.read")]
    #[strum(serialize = "relationships.read")]
    RelationshipsRead,
    #[serde(rename = "role_connections.write")]
    #[strum(serialize = "role_connections.write")]
    RoleConnectionsWrite,
    #[serde(rename = "rpc")]
    #[strum(serialize = "rpc")]
    Rpc,
    #[serde(rename = "rpc.activites.write")]
    #[strum(serialize = "rpc.activites.write")]
    RpcActivitiesWrite,
    #[serde(rename = "rpc.notifications.read")]
    #[strum(serialize = "rpc.notifications.read")]
    RpcNotificationsRead,
    #[serde(rename = "rpc.voice.read")]
    #[strum(serialize = "rpc.voice.read")]
    RpcVoiceRead,
    #[serde(rename = "rpc.voice.write")]
    #[strum(serialize = "rpc.voice.write")]
    RpcVoiceWrite,
    #[serde(rename = "voice")]
    #[strum(serialize = "voice")]
    Voice,
    #[serde(rename = "weboook.incoming")]
    #[strum(serialize = "weboook.incoming")]
    WebhookIncoming,
}
