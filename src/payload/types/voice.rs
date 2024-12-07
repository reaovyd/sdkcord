use bon::{builder, Builder};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use strum_macros::EnumString;
use thiserror::Error;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct VoiceSettings {
    #[builder(into)]
    input: Option<VoiceSettingsInput>,
    #[builder(into)]
    output: Option<VoiceSettingsOutput>,
    #[builder(into)]
    mode: Option<VoiceSettingsMode>,
    #[builder(into)]
    automatic_gain_control: Option<bool>,
    #[builder(into)]
    echo_cancellation: Option<bool>,
    #[builder(into)]
    noise_suppression: Option<bool>,
    #[builder(into)]
    qos: Option<bool>,
    #[builder(into)]
    silence_warning: Option<bool>,
    #[builder(into)]
    deaf: Option<bool>,
    #[builder(into)]
    mute: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct VoiceSettingsInput(pub VoiceSettingsIO);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct VoiceSettingsOutput(pub VoiceSettingsIO);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct VoiceSettingsMode {
    #[builder(into)]
    pub mode_type: ModeType,
    #[builder(into)]
    pub auto_threshold: bool,
    #[builder(with = |x: f32| {
        OrderedFloat(x)
    })]
    threshold: OrderedFloat<f32>,
    #[builder(into)]
    pub shortcut: ShortcutKeyCombo,
    #[builder(with = |x: f32| {
        OrderedFloat(x)
    })]
    delay: OrderedFloat<f32>,
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
    volume: Option<OrderedFloat<f32>>,
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

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum KeyTypeError {
    #[error("failed to convert {0} into keytype")]
    ConvertError(u8),
}

impl VoiceSettingsMode {
    pub const fn delay(&self) -> f32 {
        self.delay.0
    }

    pub const fn threshold(&self) -> f32 {
        self.threshold.0
    }
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

impl TryFrom<u8> for KeyType {
    type Error = KeyTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(KeyType::KeyboardKey),
            1 => Ok(KeyType::MouseButton),
            2 => Ok(KeyType::KeyboardModifierKey),
            3 => Ok(KeyType::GamepadButton),
            _ => Err(KeyTypeError::ConvertError(value)),
        }
    }
}

impl<IdT, NameT> From<(IdT, NameT)> for AvailableDevice
where
    IdT: Into<String>,
    NameT: Into<String>,
{
    fn from(value: (IdT, NameT)) -> Self {
        AvailableDevice { id: value.0.into(), name: value.1.into() }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::{
        KeyType, ModeType, ShortcutKeyCombo, VoiceSettings, VoiceSettingsIO, VoiceSettingsInput,
        VoiceSettingsMode,
    };

    #[test]
    fn construct_shortcut_keycombo() {
        let skc =
            ShortcutKeyCombo::builder().key_type(KeyType::KeyboardKey).code(12).name("123").build();
        assert_eq!(skc.key_type, KeyType::KeyboardKey);
        assert_eq!(skc.code, 12);
        assert_eq!(skc.name.as_str(), "123");
    }

    #[test]
    #[should_panic]
    fn construct_shortcut_keycombo_try_key_type_fail() {
        let _skc = ShortcutKeyCombo::builder()
            .key_type(KeyType::try_from(12).unwrap())
            .code(12)
            .name("123")
            .build();
    }

    #[test]
    fn construct_shortcut_keycombo_try_key_type_pass() {
        let skc = ShortcutKeyCombo::builder()
            .key_type(KeyType::try_from(3).unwrap())
            .code(2)
            .name("123")
            .build();
        assert_eq!(skc.key_type, KeyType::GamepadButton);
        assert_eq!(skc.code, 2);
        assert_eq!(skc.name.as_str(), "123");
    }

    #[test]
    fn construct_voice_settings() {
        let voice_settings = VoiceSettings::builder()
            .input(VoiceSettingsInput(
                VoiceSettingsIO::builder()
                    .device_id("12")
                    .volume(12.9)
                    .available_devices([("aasd", "abc").into()])
                    .build(),
            ))
            .output(
                VoiceSettingsIO::builder()
                    .device_id("13")
                    .volume(14.3)
                    .available_devices([("123", "45").into()])
                    .build(),
            )
            .mode(
                VoiceSettingsMode::builder()
                    .mode_type(ModeType::PushToTalk)
                    .auto_threshold(true)
                    .threshold(-23.133)
                    .shortcut(
                        ShortcutKeyCombo::builder()
                            .key_type(KeyType::KeyboardKey)
                            .code(23)
                            .name("asdf")
                            .build(),
                    )
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
