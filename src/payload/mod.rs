pub use activity::*;
pub use auth::*;
pub use channel::*;
pub use device::*;
pub use guild::*;
pub use subscription::*;
pub use voice::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "cmd", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Request {
    Authorize(Authorize),
    Authenticate(Authenticate),
    GetGuild(GetGuild),
    GetGuilds(GetGuilds),
    GetChannel(GetChannel),
    GetChannels(GetChannels),
    SetUserVoiceSettings(SetUserVoiceSettings),
    SelectVoiceChannel(SelectVoiceChannel),
    GetSelectedVoiceChannel(GetSelectedVoiceChannel),
    SelectTextChannel(SelectTextChannel),
    GetVoiceSettings(GetVoiceSettings),
    SetVoiceSettings(SetVoiceSettings),
    SetCertifiedDevices(SetCertifiedDevices),
    SetActivity(SetActivity),
    Subscribe(EventRequest),
    Unsubscribe(EventRequest),
    SendActivityJoinInvite(SendActivityJoinInvite),
    CloseActivityRequest(CloseActivityRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "evt", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventRequest {
    GuildStatus(GuildStatusEventRequest),
    GuildCreate(GuildCreateEventRequest),
    ChannelCreate(ChannelCreateEventRequest),
    VoiceChannelSelect(VoiceChannelSelectEventRequest),
    VoiceStateCreate(VoiceStateCreateEventRequest),
    VoiceStateUpdate(VoiceStateUpdateEventRequest),
    VoiceStateDelete(VoiceStateDeleteEventRequest),
    VoiceSettingsUpdate(VoiceSettingsUpdateEventRequest),
    VoiceConnectionStatus(VoiceConnectionStatusEventRequest),
    SpeakingStart(SpeakingStartEventRequest),
    SpeakingStop(SpeakingStopEventRequest),
    MessageCreate(MessageCreateEventRequest),
    MessageUpdate(MessageUpdateEventRequest),
    MessageDelete(MessageDeleteEventRequest),
    NotificationCreate(NotificationCreateEventRequest),
    ActivityJoin(ActivityJoinEventRequest),
    ActivitySpectate(ActivitySpectateEventRequest),
    ActivityJoinRequest(ActivityJoinRequestEventRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
struct EmptyArgs {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    inner: Option<()>,
}

use serde::{
    Deserialize,
    Serialize,
};

mod activity;
mod auth;
mod channel;
mod device;
mod guild;
mod macros;
mod subscription;
mod voice;

#[cfg(test)]
mod tests {
    use crate::payload::{
        CloseActivityRequest,
        CloseActivityRequestArgs,
        Device,
        DeviceType,
        EventRequest,
        GetChannel,
        GetChannelArgs,
        GetChannels,
        GetChannelsArgs,
        GetGuilds,
        GetSelectedVoiceChannel,
        GetVoiceSettings,
        GuildStatusEventRequest,
        GuildStatusEventRequestArgs,
        Model,
        Pan,
        Related,
        Request,
        SelectTextChannel,
        SelectTextChannelArgs,
        SelectVoiceChannel,
        SelectVoiceChannelArgs,
        SendActivityJoinInvite,
        SendActivityJoinInviteArgs,
        SetActivity,
        SetActivityArgs,
        SetCertifiedDevices,
        SetCertifiedDevicesArgs,
        SetUserVoiceSettings,
        SetUserVoiceSettingsArgs,
        SetVoiceSettings,
        SetVoiceSettingsArgs,
        Vendor,
        Volume,
    };
    use url::Url;
    use uuid::Uuid;

    use super::{
        Authorize,
        AuthorizeArgs,
        GetGuild,
        GetGuildArgs,
        OAuth2Scope,
        OAuth2Scopes,
    };

    #[test]
    fn test_subscribe_guild_status() {
        let req = Request::Subscribe(EventRequest::GuildStatus(GuildStatusEventRequest::new(
            GuildStatusEventRequestArgs::builder().guild_id("123").build(),
        )));
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains(r#""cmd":"SUBSCRIBE""#));
        assert!(json.contains(r#""evt":"GUILD_STATUS""#));
    }

    #[test]
    fn test_unsubscribe_guild_status() {
        let req = Request::Unsubscribe(EventRequest::GuildStatus(GuildStatusEventRequest::new(
            GuildStatusEventRequestArgs::builder().guild_id("123").build(),
        )));
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains(r#""cmd":"UNSUBSCRIBE""#));
        assert!(json.contains(r#""evt":"GUILD_STATUS""#));
    }

    #[test]
    fn test_get_guild_cmd_exists() {
        let get_guild = Request::GetGuild(GetGuild::new(
            GetGuildArgs::builder().guild_id("abc".to_owned()).timeout(200u32).build(),
        ));
        let serialized = serde_json::to_string(&get_guild).unwrap();
        assert!(serialized.contains(r#""cmd":"GET_GUILD""#));
    }

    #[test]
    fn test_get_guilds_cmd_exists() {
        let get_guilds = Request::GetGuilds(GetGuilds::new());
        let serialized = serde_json::to_string(&get_guilds).unwrap();
        assert!(serialized.contains(r#""cmd":"GET_GUILDS""#));
    }

    #[test]
    fn test_get_channel_cmd_exists() {
        let get_channel = Request::GetChannel(GetChannel::new(
            GetChannelArgs::builder().channel_id("123".to_string()).build(),
        ));
        let serialized = serde_json::to_string(&get_channel).unwrap();
        assert!(serialized.contains(r#""cmd":"GET_CHANNEL""#));
    }

    #[test]
    fn test_get_channels_cmd_exists() {
        let get_channels = Request::GetChannels(GetChannels::new(
            GetChannelsArgs::builder().guild_id("123".to_string()).build(),
        ));
        let serialized = serde_json::to_string(&get_channels).unwrap();
        assert!(serialized.contains(r#""cmd":"GET_CHANNELS""#));
    }

    #[test]
    fn test_set_user_voice_settings_cmd_exists() {
        let set_user_voice_settings = Request::SetUserVoiceSettings(SetUserVoiceSettings::new(
            SetUserVoiceSettingsArgs::builder()
                .user_id("123".to_string())
                .pan(Pan::builder().left(1.0).right(1.0).build().unwrap())
                .volume(Volume::builder().inner(1).build().unwrap())
                .mute(false)
                .build(),
        ));
        let serialized = serde_json::to_string(&set_user_voice_settings).unwrap();
        assert!(serialized.contains(r#""cmd":"SET_USER_VOICE_SETTINGS""#));
    }

    #[test]
    fn test_select_voice_channel_cmd_exists() {
        let cmd = Request::SelectVoiceChannel(SelectVoiceChannel::new(
            SelectVoiceChannelArgs::builder().build(),
        ));
        let serialized = serde_json::to_string(&cmd).unwrap();
        assert!(serialized.contains(r#""cmd":"SELECT_VOICE_CHANNEL""#));
    }

    #[test]
    fn test_get_selected_voice_channel_cmd_exists() {
        let cmd = Request::GetSelectedVoiceChannel(GetSelectedVoiceChannel::new());
        let serialized = serde_json::to_string(&cmd).unwrap();
        assert!(serialized.contains(r#""cmd":"GET_SELECTED_VOICE_CHANNEL""#));
    }

    #[test]
    fn test_select_text_channel_cmd_exists() {
        let cmd = Request::SelectTextChannel(SelectTextChannel::new(
            SelectTextChannelArgs::builder().build(),
        ));
        let serialized = serde_json::to_string(&cmd).unwrap();
        assert!(serialized.contains(r#""cmd":"SELECT_TEXT_CHANNEL""#));
    }

    #[test]
    fn test_get_voice_settings_cmd_exists() {
        let cmd = Request::GetVoiceSettings(GetVoiceSettings::new());
        let serialized = serde_json::to_string(&cmd).unwrap();
        assert!(serialized.contains(r#""cmd":"GET_VOICE_SETTINGS""#));
    }

    #[test]
    fn test_set_voice_settings_cmd_exists() {
        let cmd = Request::SetVoiceSettings(SetVoiceSettings::new(
            SetVoiceSettingsArgs::builder().build(),
        ));
        let serialized = serde_json::to_string(&cmd).unwrap();
        assert!(serialized.contains(r#""cmd":"SET_VOICE_SETTINGS""#));
    }

    #[test]
    fn test_set_activity_cmd_exists() {
        let cmd =
            Request::SetActivity(SetActivity::new(SetActivityArgs::builder().pid(3333u32).build()));
        let serialized = serde_json::to_string(&cmd).unwrap();
        assert!(serialized.contains(r#""cmd":"SET_ACTIVITY""#));
    }

    #[test]
    fn test_send_activity_join_invite_cmd_exists() {
        let cmd = Request::SendActivityJoinInvite(SendActivityJoinInvite::new(
            SendActivityJoinInviteArgs::builder().user_id("joe".to_owned()).build(),
        ));
        let serialized = serde_json::to_string(&cmd).unwrap();
        assert!(serialized.contains(r#""cmd":"SEND_ACTIVITY_JOIN_INVITE""#));
    }

    #[test]
    fn test_close_activity_request_cmd_exists() {
        let cmd = Request::CloseActivityRequest(CloseActivityRequest::new(
            CloseActivityRequestArgs::builder().user_id("tasd".to_owned()).build(),
        ));
        let serialized = serde_json::to_string(&cmd).unwrap();
        assert!(serialized.contains(r#""cmd":"CLOSE_ACTIVITY_REQUEST""#));
    }

    #[test]
    fn test_set_certified_devices_cmd_exists() {
        let cmd = Request::SetCertifiedDevices(SetCertifiedDevices::new(
            SetCertifiedDevicesArgs::builder()
                .devices(vec![Device::builder()
                    .device_type(DeviceType::AudioInput)
                    .id(Uuid::new_v4())
                    .vendor(
                        Vendor::builder()
                            .name("joe".to_owned())
                            .url(Url::parse("http://github.com").unwrap())
                            .build(),
                    )
                    .model(
                        Model::builder()
                            .name("joe".to_owned())
                            .url(Url::parse("http://github.com").unwrap())
                            .build(),
                    )
                    .related(Related(vec![Uuid::new_v4()]))
                    .echo_cancellation(true)
                    .noise_suppression(false)
                    .automatic_gain_control(true)
                    .hardware_mute(false)
                    .build()])
                .build(),
        ));
        let serialized = serde_json::to_string(&cmd).unwrap();
        assert!(serialized.contains(r#""cmd":"SET_CERTIFIED_DEVICES""#));
    }

    #[test]
    fn test_authorize_exists() {
        let cmd = Request::Authorize(Authorize::new(
            AuthorizeArgs::builder()
                .scope(
                    OAuth2Scopes::builder()
                        .add_scope(OAuth2Scope::Email)
                        .add_scope(OAuth2Scope::Voice)
                        .build(),
                )
                .client_id("client_id1".to_string())
                .build(),
        ));
        let serialized = serde_json::to_string(&cmd).unwrap();
        assert!(serialized.contains(r#""cmd":"AUTHORIZE""#));
    }
}
