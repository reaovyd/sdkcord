use std::fmt::Debug;

use crate::payload::macros::make_command_reqres_payload;
use bon::{
    bon,
    Builder,
};
use ordered_float::OrderedFloat;
use serde::{
    Deserialize,
    Serialize,
};
use serde_with::skip_serializing_none;
use thiserror::Error;

// TODO: mode objects
make_command_reqres_payload!(
    SetVoiceSettings,
    (
        /// Used to set the client's voice settings
        /// When setting voice settings, all fields are optional. Only passed fields are updated
    ),
    (input, Option<InputVoiceSettings>,
        (#[doc = "input settings"]),
        (
            #[serde(skip_serializing_if = "Option::is_none")]
            #[builder(into)]
        )
    ),
    (output, Option<OutputVoiceSettings>,
        (#[doc = "output settings"]),
        (
            #[serde(skip_serializing_if = "Option::is_none")]
            #[builder(into)]
        )
    ),
    (mode, Option<ModeVoiceSettings>,
        (#[doc = "voice mode settings"]),
        (
            #[serde(skip_serializing_if = "Option::is_none")]
            #[builder(into)]
        )
    ),
    (automatic_gain_control, Option<bool>,
        (#[doc = "state of automatic gain control"]),
        (
            #[serde(skip_serializing_if = "Option::is_none")]
            #[builder(into)]
        )
    ),
    (echo_cancellation, Option<bool>,
        (#[doc = "state of echo cancellation"]),
        (
            #[serde(skip_serializing_if = "Option::is_none")]
            #[builder(into)]
        )
    ),
    (noise_suppression, Option<bool>,
        (#[doc = "state of noise suppression"]),
        (
            #[serde(skip_serializing_if = "Option::is_none")]
            #[builder(into)]
        )
    ),
    (qos, Option<bool>,
        (#[doc = "state of voice quality of service"]),
        (
            #[serde(skip_serializing_if = "Option::is_none")]
            #[builder(into)]
        )
    ),
    (silence_warning, Option<bool>,
        (#[doc = "state of silence warning notice"]),
        (
            #[serde(skip_serializing_if = "Option::is_none")]
            #[builder(into)]
        )
    ),
    (deaf, Option<bool>,
        (#[doc = "state of self-deafen"]),
        (
            #[serde(skip_serializing_if = "Option::is_none")]
            #[builder(into)]
        )
    ),
    (mute, Option<bool>,
        (#[doc = "state of self-mute"]),
        (
            #[serde(skip_serializing_if = "Option::is_none")]
            #[builder(into)]
        )
    )
);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct InputVoiceSettings(VoiceSettings);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct OutputVoiceSettings(VoiceSettings);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

#[bon]
impl ModeVoiceSettings {
    const MIN_THRESHOLD: OrderedFloat<f32> = OrderedFloat(-100.0);
    const MAX_THRESHOLD: OrderedFloat<f32> = OrderedFloat(0.0);

    const MIN_DELAY: OrderedFloat<f32> = OrderedFloat(0.0);
    const MAX_DELAY: OrderedFloat<f32> = OrderedFloat(2000.0);

    #[builder(derive(Debug))]
    pub fn new(
        #[builder(into)] mode_type: ModeType,
        #[builder(into)] auto_threshold: bool,
        #[builder(into)] threshold: OrderedFloat<f32>,
        #[builder(into)] shortcut: Shortcut,
        #[builder(into)] delay: OrderedFloat<f32>,
    ) -> Result<Self, SetVoiceSettingsError> {
        if delay < Self::MIN_DELAY || delay > Self::MAX_DELAY {
            return Err(SetVoiceSettingsError::DelayBoundary { delay: delay.0 });
        }

        if threshold < Self::MIN_THRESHOLD || threshold > Self::MAX_THRESHOLD {
            return Err(SetVoiceSettingsError::ThresholdBoundary { threshold: threshold.0 });
        }
        Ok(ModeVoiceSettings { mode_type, auto_threshold, threshold, shortcut, delay })
    }
}

/// The mode type for the mode in voice settings
///
/// It must be either `PUSH_TO_TALK` or `VOICE_ACTIVITY`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModeType {
    /// The `PUSH_TO_TALK` mode type
    PushToTalk,
    /// The `VOICE_ACTIVITY` mode type
    VoiceActivity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(derive(Debug))]
pub struct Shortcut {
    /// The key type. See [`KeyType`]
    #[serde(rename = "type")]
    #[builder(into)]
    key_type: KeyType,
    /// The key code
    #[builder(into)]
    code: u32,
    /// The key name
    #[builder(into)]
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum KeyType {
    KeyboardKey = 0,
    MouseButton = 1,
    KeyboardModifierKey = 2,
    GamepadButton = 3,
}

#[bon]
impl OutputVoiceSettings {
    const MIN_VOL: OrderedFloat<f32> = OrderedFloat(0.0);
    const MAX_VOL: OrderedFloat<f32> = OrderedFloat(200.0);
    /// Creates a new [`OutputVoiceSettings`] value
    ///
    /// # Errors
    /// A [`SetVoiceSettingsError`] is returned if the volume you inputted does
    /// not fall between the `MIN_VOL` and `MAX_VOL` which would result in a
    /// [`SetVoiceSettingsError::VolumeBoundary`] error
    #[builder(derive(Debug))]
    pub fn new(
        device_id: &str,
        volume: f32,
        available_devices: &[AvailableDevice],
    ) -> Result<Self, SetVoiceSettingsError> {
        Ok(OutputVoiceSettings(
            VoiceSettings::builder()
                .device_id(device_id)
                .volume(volume)
                .available_devices(available_devices)
                .min_vol(Self::MIN_VOL)
                .max_vol(Self::MAX_VOL)
                .build()?,
        ))
    }
}

#[bon]
impl InputVoiceSettings {
    const MIN_VOL: OrderedFloat<f32> = OrderedFloat(0.0);
    const MAX_VOL: OrderedFloat<f32> = OrderedFloat(100.0);
    /// Creates a new [`InputVoiceSettings`] value
    ///
    /// # Errors
    /// A [`SetVoiceSettingsError`] is returned if the volume you inputted does
    /// not fall between the `MIN_VOL` and `MAX_VOL` which would result in a
    /// [`SetVoiceSettingsError::VolumeBoundary`] error
    #[builder(derive(Debug))]
    pub fn new(
        device_id: &str,
        volume: f32,
        available_devices: &[AvailableDevice],
    ) -> Result<Self, SetVoiceSettingsError> {
        Ok(InputVoiceSettings(
            VoiceSettings::builder()
                .device_id(device_id)
                .volume(volume)
                .available_devices(available_devices)
                .min_vol(Self::MIN_VOL)
                .max_vol(Self::MAX_VOL)
                .build()?,
        ))
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
struct VoiceSettings {
    /// Device id
    device_id: Option<String>,
    /// Input voice level (min: 0, max: 100)
    /// Output voice level (min: 0, max: 200)
    volume: Option<OrderedFloat<f32>>,
    /// Array of read-only device objects containing `id` and `name` string keys
    available_devices: Option<Vec<AvailableDevice>>,
}

#[bon]
impl VoiceSettings {
    #[builder(derive(Debug))]
    fn new(
        device_id: &str,
        volume: impl Into<f32> + Debug,
        available_devices: &[AvailableDevice],
        min_vol: impl Into<OrderedFloat<f32>> + Debug,
        max_vol: impl Into<OrderedFloat<f32>> + Debug,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(derive(Debug))]
pub struct AvailableDevice {
    #[builder(into)]
    id: String,
    #[builder(into)]
    name: String,
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

#[cfg(test)]
mod tests {
    use super::{
        ModeVoiceSettings,
        Shortcut,
    };

    #[test]
    #[should_panic]
    fn test_mode_settings_validation_threshold() {
        ModeVoiceSettings::builder()
            .mode_type(super::ModeType::PushToTalk)
            .auto_threshold(false)
            .threshold(100.0)
            .shortcut(
                Shortcut::builder()
                    .code(23u32)
                    .name("a")
                    .key_type(super::KeyType::GamepadButton)
                    .build(),
            )
            .delay(200.0)
            .build()
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_mode_settings_validation_delay() {
        ModeVoiceSettings::builder()
            .mode_type(super::ModeType::PushToTalk)
            .auto_threshold(false)
            .threshold(0.0)
            .shortcut(
                Shortcut::builder()
                    .code(23u32)
                    .name("a")
                    .key_type(super::KeyType::GamepadButton)
                    .build(),
            )
            .delay(30000.0)
            .build()
            .unwrap();
    }

    #[test]
    fn test_mode_settings_validation_success() {
        let mode_voice_settings = ModeVoiceSettings::builder()
            .mode_type(super::ModeType::PushToTalk)
            .auto_threshold(false)
            .threshold(0.0)
            .shortcut(
                Shortcut::builder()
                    .code(23u32)
                    .name("a")
                    .key_type(super::KeyType::GamepadButton)
                    .build(),
            )
            .delay(150.0)
            .build()
            .unwrap();
        assert_eq!(mode_voice_settings.threshold.0, 0.0)
    }
}
