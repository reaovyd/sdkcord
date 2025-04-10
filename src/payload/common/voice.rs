use bon::{Builder, builder};
use chrono::{DateTime, Utc};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use strum_macros::EnumString;

use super::{guild::GuildMember, pan::Pan};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Ping {
    pub time: Option<u64>,
    pub value: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VoiceConnectionState {
    Disconnected,
    AwaitingEndpoint,
    Authenticating,
    Connecting,
    Connected,
    VoiceDisconnected,
    VoiceConnecting,
    VoiceConnected,
    NoRoute,
    IceChecking,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct VoiceState {
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    pub user_id: Option<String>,
    pub member: Option<GuildMember>,
    pub session_id: Option<String>,
    pub nick: Option<String>,
    pub pan: Option<Pan>,
    pub voice_state: Option<State>,
    pub request_to_speak_timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct State {
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
    pub self_deaf: Option<bool>,
    pub self_mute: Option<bool>,
    pub self_stream: Option<bool>,
    pub self_video: Option<bool>,
    pub suppress: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct VoiceSettings {
    #[builder(into)]
    pub input: Option<VoiceSettingsInput>,
    #[builder(into)]
    pub output: Option<VoiceSettingsOutput>,
    #[builder(into)]
    pub mode: Option<VoiceSettingsMode>,
    #[builder(into)]
    pub automatic_gain_control: Option<bool>,
    #[builder(into)]
    pub echo_cancellation: Option<bool>,
    #[builder(into)]
    pub noise_suppression: Option<bool>,
    #[builder(into)]
    pub qos: Option<bool>,
    #[builder(into)]
    pub silence_warning: Option<bool>,
    #[builder(into)]
    pub deaf: Option<bool>,
    #[builder(into)]
    pub mute: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct VoiceSettingsInput(pub VoiceSettingsIO);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct VoiceSettingsOutput(pub VoiceSettingsIO);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct VoiceSettingsMode {
    #[builder(into)]
    pub mode_type: Option<ModeType>,
    #[builder(into)]
    pub auto_threshold: Option<bool>,
    #[builder(with = |x: f32| {
        OrderedFloat(x)
    })]
    pub threshold: Option<OrderedFloat<f32>>,
    #[builder(into)]
    pub shortcut: Option<Vec<ShortcutKeyCombo>>,
    #[builder(with = |x: f32| {
        OrderedFloat(x)
    })]
    pub delay: Option<OrderedFloat<f32>>,
}

#[derive(
    Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, EnumString, strum_macros::Display,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModeType {
    PushToTalk,
    VoiceActivity,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct VoiceSettingsIO {
    #[builder(into)]
    pub device_id: Option<String>,
    #[builder(with = |x: f32| {
        OrderedFloat(x)
    })]
    pub volume: Option<OrderedFloat<f32>>,
    #[builder(with = |devices: impl IntoIterator<Item = AvailableDevice>| {
        devices.into_iter().collect()
    })]
    pub available_devices: Option<Vec<AvailableDevice>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct AvailableDevice {
    #[builder(into)]
    pub id: String,
    #[builder(into)]
    pub name: String,
}

impl AvailableDevice {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct ShortcutKeyCombo {
    #[serde(rename = "type")]
    #[builder(into)]
    pub key_type: KeyType,
    pub code: u32,
    #[builder(into)]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, PartialEq, Eq, Hash, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum KeyType {
    KeyboardKey = 0,
    MouseButton = 1,
    KeyboardModifierKey = 2,
    GamepadButton = 3,
}

impl VoiceSettingsInput {
    pub fn new(voice_settings: impl Into<VoiceSettingsIO>) -> Self {
        Self(voice_settings.into())
    }
}

impl VoiceSettingsOutput {
    pub fn new(voice_settings: impl Into<VoiceSettingsIO>) -> Self {
        Self(voice_settings.into())
    }
}

impl VoiceSettingsIO {
    pub fn volume(&self) -> Option<f32> {
        self.volume.as_ref().map(|vol| vol.0)
    }
}

impl From<VoiceSettingsIO> for VoiceSettingsInput {
    fn from(value: VoiceSettingsIO) -> Self {
        VoiceSettingsInput(value)
    }
}

impl From<VoiceSettingsIO> for VoiceSettingsOutput {
    fn from(value: VoiceSettingsIO) -> Self {
        VoiceSettingsOutput(value)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::payload::common::voice::AvailableDevice;

    use super::{
        KeyType, ModeType, ShortcutKeyCombo, VoiceSettings, VoiceSettingsIO, VoiceSettingsInput,
        VoiceSettingsMode,
    };

    #[test]
    fn construct_shortcut_keycombo() {
        let skc = ShortcutKeyCombo::builder()
            .key_type(KeyType::KeyboardKey)
            .code(12)
            .name("123")
            .build();
        assert_eq!(skc.key_type, KeyType::KeyboardKey);
        assert_eq!(skc.code, 12);
        assert_eq!(skc.name.as_str(), "123");
    }

    #[test]
    fn construct_voice_settings() {
        let voice_settings = VoiceSettings::builder()
            .input(VoiceSettingsInput(
                VoiceSettingsIO::builder()
                    .device_id("12")
                    .volume(12.9)
                    .available_devices([AvailableDevice::new("aasd", "abc")])
                    .build(),
            ))
            .output(
                VoiceSettingsIO::builder()
                    .device_id("13")
                    .volume(14.3)
                    .available_devices([AvailableDevice::new("123", "45")])
                    .build(),
            )
            .mode(
                VoiceSettingsMode::builder()
                    .mode_type(ModeType::PushToTalk)
                    .auto_threshold(true)
                    .threshold(-23.133)
                    .shortcut([ShortcutKeyCombo::builder()
                        .key_type(KeyType::KeyboardKey)
                        .code(23)
                        .name("asdf")
                        .build()])
                    .delay(12.32)
                    .build(),
            )
            .automatic_gain_control(false)
            .echo_cancellation(true)
            .noise_suppression(true)
            .qos(false)
            .build();
        let voice_settings = serde_json::to_string(&voice_settings).unwrap();
        assert!(voice_settings.contains("\"automatic_gain_control\":false"));
    }
}
