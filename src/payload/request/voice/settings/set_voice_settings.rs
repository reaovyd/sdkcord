use crate::payload::request::macros::make_request_payload;
use derive_builder::Builder;
use ordered_float::OrderedFloat;
use paste::paste;
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

// TODO: mode objects
make_request_payload!(
    SetVoiceSettings,
    #[doc = "Used to set the client's voice settings"],
    #[doc = "When setting voice settings, all fields are optional. Only passed fields are updated."],
    (input, Option<InputVoiceSettings>, "input settings", #[serde(skip_serializing_if = "Option::is_none")]),
    (output, Option<OutputVoiceSettings>, "output settings", #[serde(skip_serializing_if = "Option::is_none")]),
    (mode, Option<ModeVoiceSettings>, "voice mode settings", #[serde(skip_serializing_if = "Option::is_none")]),
    (automatic_gain_control, Option<bool>, "state of automatic gain control", #[serde(skip_serializing_if = "Option::is_none")]),
    (echo_cancellation, Option<bool>, "state of echo cancellation", #[serde(skip_serializing_if = "Option::is_none")]),
    (noise_suppression, Option<bool>, "state of noise suppression", #[serde(skip_serializing_if = "Option::is_none")]),
    (qos, Option<bool>, "state of voice quality of service", #[serde(skip_serializing_if = "Option::is_none")]),
    (silence_warning, Option<bool>, "state of silence warning notice", #[serde(skip_serializing_if = "Option::is_none")]),
    (deaf, Option<bool>, "state of self-deafen", #[serde(skip_serializing_if = "Option::is_none")]),
    (mute, Option<bool>, "state of self-mute", #[serde(skip_serializing_if = "Option::is_none")])
);

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash, Default)]
pub struct InputVoiceSettings(VoiceSettings);

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash, Default)]
pub struct OutputVoiceSettings(VoiceSettings);

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash, Builder)]
#[builder(build_fn(validate = "Self::validate_boundaries"))]
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

    fn validate_boundaries(&self) -> Result<(), SetVoiceSettingsError> {
        if let (Some(delay), Some(threshold)) = (self.delay, self.threshold) {
            if delay < Self::MIN_THRESHOLD || delay > Self::MAX_THRESHOLD {
                return Err(SetVoiceSettingsError::DelayBoundary { delay: delay.0 });
            }

            if threshold < Self::MIN_THRESHOLD || threshold > Self::MAX_THRESHOLD {
                return Err(SetVoiceSettingsError::ThresholdBoundary { threshold: threshold.0 });
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// The mode type for the mode in voice settings
///
/// It must be either `PUSH_TO_TALK` or `VOICE_ACTIVITY`
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
        available_devices: &[DeviceObject],
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
        available_devices: &[DeviceObject],
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
struct VoiceSettings {
    /// Device id
    #[serde(skip_serializing_if = "Option::is_none")]
    device_id: Option<String>,
    /// Input voice level (min: 0, max: 100)
    /// Output voice level (min: 0, max: 200)
    #[serde(skip_serializing_if = "Option::is_none")]
    volume: Option<OrderedFloat<f32>>,
    /// Array of read-only device objects containing `id` and `name` string keys
    #[serde(skip_serializing_if = "Option::is_none")]
    available_devices: Option<Vec<DeviceObject>>,
}

impl VoiceSettings {
    fn new(
        device_id: &str,
        volume: f32,
        available_devices: &[DeviceObject],
        min_vol: OrderedFloat<f32>,
        max_vol: OrderedFloat<f32>,
    ) -> Result<Self, SetVoiceSettingsError> {
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
pub struct DeviceObject {
    id: String,
    name: String,
}

impl DeviceObject {
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
    /// the list of [`DeviceObject`]s
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
