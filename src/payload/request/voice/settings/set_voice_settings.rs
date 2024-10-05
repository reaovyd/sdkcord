use crate::payload::request::macros::make_request_payload;
use derive_builder::Builder;
use ordered_float::OrderedFloat;
use paste::paste;
use serde::Serialize;
use serde_with::skip_serializing_none;
use thiserror::Error;
use uuid::Uuid;

// TODO: mode objects
make_request_payload!(
    SetVoiceSettings,
    #[doc = "Used to set the client's voice settings"],
    #[doc = "When setting voice settings, all fields are optional. Only passed fields are updated."],
    (input, Option<InputVoiceSettings>,
        (#[doc = "input settings"]),
        (#[serde(skip_serializing_if = "Option::is_none")], #[builder(setter(strip_option), default)])
    ),
    (output, Option<OutputVoiceSettings>,
        (#[doc = "output settings"]),
        (#[serde(skip_serializing_if = "Option::is_none")], #[builder(setter(strip_option), default)])
    ),
    (mode, Option<ModeVoiceSettings>,
        (#[doc = "voice mode settings"]),
        (#[serde(skip_serializing_if = "Option::is_none")], #[builder(setter(strip_option), default)])
    ),
    (automatic_gain_control, Option<bool>,
        (#[doc = "state of automatic gain control"]),
        (#[serde(skip_serializing_if = "Option::is_none")], #[builder(setter(strip_option), default)])
    ),
    (echo_cancellation, Option<bool>,
        (#[doc = "state of echo cancellation"]),
        (#[serde(skip_serializing_if = "Option::is_none")], #[builder(setter(strip_option), default)])
    ),
    (noise_suppression, Option<bool>,
        (#[doc = "state of noise suppression"]),
        (#[serde(skip_serializing_if = "Option::is_none")], #[builder(setter(strip_option), default)])
    ),
    (qos, Option<bool>,
        (#[doc = "state of voice quality of service"]),
        (#[serde(skip_serializing_if = "Option::is_none")], #[builder(setter(strip_option), default)])
    ),
    (silence_warning, Option<bool>,
        (#[doc = "state of silence warning notice"]),
        (#[serde(skip_serializing_if = "Option::is_none")], #[builder(setter(strip_option), default)])
    ),
    (deaf, Option<bool>,
        (#[doc = "state of self-deafen"]),
        (#[serde(skip_serializing_if = "Option::is_none")], #[builder(setter(strip_option), default)])
    ),
    (mute, Option<bool>,
        (#[doc = "state of self-mute"]),
        (#[serde(skip_serializing_if = "Option::is_none")], #[builder(setter(strip_option), default)])
    )
);

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash, Default)]
pub struct InputVoiceSettings(VoiceSettings);

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash, Default)]
pub struct OutputVoiceSettings(VoiceSettings);

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash, Builder)]
#[builder(build_fn(validate = "Self::validate_boundaries"), setter(into))]
pub struct ModeVoiceSettings {
    /// Voice setting mode type
    #[serde(rename = "type")]
    mode_type: ModeType,
    /// Voice activity threshold automatically sets its threshold
    auto_threshold: bool,
    /// Threshold for voice activity (in dB) (min: -100, max: 0)
    threshold: OrderedFloat<f32>,
    /// Shortcut key combos for PTT
    shortcut: Shortcut,
    /// The PTT release delay (in ms) (min: 0, max: 2000)
    delay: OrderedFloat<f32>,
}

impl ModeVoiceSettingsBuilder {
    const MIN_THRESHOLD: OrderedFloat<f32> = OrderedFloat(-100.0);
    const MAX_THRESHOLD: OrderedFloat<f32> = OrderedFloat(0.0);

    const MIN_DELAY: OrderedFloat<f32> = OrderedFloat(0.0);
    const MAX_DELAY: OrderedFloat<f32> = OrderedFloat(2000.0);

    fn validate_boundaries(&self) -> Result<(), SetVoiceSettingsError> {
        if let (Some(delay), Some(threshold)) = (self.delay, self.threshold) {
            if delay < Self::MIN_DELAY || delay > Self::MAX_DELAY {
                return Err(SetVoiceSettingsError::DelayBoundary { delay: delay.0 });
            }

            if threshold < Self::MIN_THRESHOLD || threshold > Self::MAX_THRESHOLD {
                return Err(SetVoiceSettingsError::ThresholdBoundary { threshold: threshold.0 });
            }
        }
        Ok(())
    }
}

/// The mode type for the mode in voice settings
///
/// It must be either `PUSH_TO_TALK` or `VOICE_ACTIVITY`
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModeType {
    /// The `PUSH_TO_TALK` mode type
    PushToTalk,
    /// The `VOICE_ACTIVITY` mode type
    VoiceActivity,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct Shortcut {
    /// The key type. See [`KeyType`]
    #[serde(rename = "type")]
    key_type: KeyType,
    /// The key code
    code: u32,
    /// The key name
    name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum KeyType {
    KeyboardKey = 0,
    MouseButton = 1,
    KeyboardModifierKey = 2,
    GamepadButton = 3,
}

impl OutputVoiceSettings {
    const MIN_VOL: OrderedFloat<f32> = OrderedFloat(0.0);
    const MAX_VOL: OrderedFloat<f32> = OrderedFloat(200.0);
    /// Creates a new [`OutputVoiceSettings`] value
    ///
    /// # Errors
    /// A [`SetVoiceSettingsError`] is returned if the volume you inputted does
    /// not fall between the `MIN_VOL` and `MAX_VOL` which would result in a
    /// [`SetVoiceSettingsError::VolumeBoundary`] error
    pub fn new(
        device_id: &str,
        volume: f32,
        available_devices: &[AvailableDevices],
    ) -> Result<Self, SetVoiceSettingsError> {
        Ok(OutputVoiceSettings(VoiceSettings::new(
            device_id,
            volume,
            available_devices,
            Self::MIN_VOL,
            Self::MAX_VOL,
        )?))
    }
}

impl InputVoiceSettings {
    const MIN_VOL: OrderedFloat<f32> = OrderedFloat(0.0);
    const MAX_VOL: OrderedFloat<f32> = OrderedFloat(100.0);
    /// Creates a new [`InputVoiceSettings`] value
    ///
    /// # Errors
    /// A [`SetVoiceSettingsError`] is returned if the volume you inputted does
    /// not fall between the `MIN_VOL` and `MAX_VOL` which would result in a
    /// [`SetVoiceSettingsError::VolumeBoundary`] error
    pub fn new(
        device_id: &str,
        volume: f32,
        available_devices: &[AvailableDevices],
    ) -> Result<Self, SetVoiceSettingsError> {
        Ok(InputVoiceSettings(VoiceSettings::new(
            device_id,
            volume,
            available_devices,
            Self::MIN_VOL,
            Self::MAX_VOL,
        )?))
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash, Default)]
#[skip_serializing_none]
struct VoiceSettings {
    /// Device id
    device_id: Option<String>,
    /// Input voice level (min: 0, max: 100)
    /// Output voice level (min: 0, max: 200)
    volume: Option<OrderedFloat<f32>>,
    /// Array of read-only device objects containing `id` and `name` string keys
    available_devices: Option<Vec<AvailableDevices>>,
}

impl VoiceSettings {
    fn new(
        device_id: &str,
        volume: impl Into<f32>,
        available_devices: &[AvailableDevices],
        min_vol: impl Into<OrderedFloat<f32>>,
        max_vol: impl Into<OrderedFloat<f32>>,
    ) -> Result<Self, SetVoiceSettingsError> {
        let volume = volume.into();
        let min_vol = min_vol.into();
        let max_vol = max_vol.into();

        let volume_ord = OrderedFloat(volume);
        if volume_ord < min_vol || volume_ord > max_vol {
            return Err(SetVoiceSettingsError::VolumeBoundary { vol: volume });
        }

        let available_devices = {
            if available_devices.is_empty() {
                None
            } else {
                Some(available_devices.to_vec())
            }
        };

        Ok(Self {
            device_id: Some(device_id.to_string()),
            volume: Some(volume_ord),
            available_devices,
        })
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct AvailableDevices {
    id: String,
    name: String,
}

impl AvailableDevices {
    pub fn new(id: &str, name: &str) -> Self {
        let id = id.to_string();
        let name = name.to_string();
        Self { id, name }
    }
}

/// `Error`s that occur when trying to build the [`SetVoiceSettings`] request
#[derive(Debug, Error)]
pub enum SetVoiceSettingsError {
    /// An error for values that did not satisfy the invariant while building
    /// the list of [`AvailableDevices`]s
    #[error("Error setting available_devices: the list is empty so nothing can be set")]
    AvailableDevicesEmpty,
    /// An error for values that did not satisfy the invariant while building
    /// the [`Volume`]
    #[error("Error setting volume; got value {vol}")]
    VolumeBoundary {
        /// The `vol` value argument that caused failure
        vol: f32,
    },
    #[error("Error setting threshold; got value {threshold}")]
    ThresholdBoundary { threshold: f32 },
    #[error("Error setting delay; got value {delay}")]
    DelayBoundary { delay: f32 },
}

impl From<SetVoiceSettingsError> for ModeVoiceSettingsBuilderError {
    fn from(value: SetVoiceSettingsError) -> Self {
        ModeVoiceSettingsBuilderError::ValidationError(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ModeVoiceSettingsBuilder,
        ShortcutBuilder,
    };

    #[test]
    #[should_panic(expected = "ValidationError")]
    fn test_mode_settings_validation_threshold() {
        ModeVoiceSettingsBuilder::create_empty()
            .mode_type(super::ModeType::PushToTalk)
            .auto_threshold(false)
            .threshold(100.0)
            .shortcut(
                ShortcutBuilder::create_empty()
                    .code(23)
                    .name("a".to_owned())
                    .key_type(super::KeyType::GamepadButton)
                    .build()
                    .unwrap(),
            )
            .delay(200.0)
            .build()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "ValidationError")]
    fn test_mode_settings_validation_delay() {
        ModeVoiceSettingsBuilder::create_empty()
            .mode_type(super::ModeType::PushToTalk)
            .auto_threshold(false)
            .threshold(0.0)
            .shortcut(
                ShortcutBuilder::create_empty()
                    .code(23)
                    .name("a".to_owned())
                    .key_type(super::KeyType::GamepadButton)
                    .build()
                    .unwrap(),
            )
            .delay(30000.0)
            .build()
            .unwrap();
    }

    #[test]
    fn test_mode_settings_validation_success() {
        let mode_voice_settings = ModeVoiceSettingsBuilder::create_empty()
            .mode_type(super::ModeType::PushToTalk)
            .auto_threshold(false)
            .threshold(0.0)
            .shortcut(
                ShortcutBuilder::create_empty()
                    .code(23)
                    .name("a".to_owned())
                    .key_type(super::KeyType::GamepadButton)
                    .build()
                    .unwrap(),
            )
            .delay(150.0)
            .build()
            .unwrap();
        assert_eq!(mode_voice_settings.threshold.0, 0.0)
    }
}
