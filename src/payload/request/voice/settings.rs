use crate::payload::request::{
    macros::make_request_payload,
    EmptyArgs,
};
use derive_builder::Builder;
use ordered_float::OrderedFloat;
use paste::paste;
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

make_request_payload!(
    SetUserVoiceSettings,
    #[doc = "Used to change voice settings of users in voice channels"],
    (user_id, String, "user id"),
    (pan, Option<Pan>, "set the pan of the user", #[serde(skip_serializing_if = "Option::is_none")]),
    (volume, Option<Volume>, "set the volume of user (defaults to 100, min 0, max 200)", #[serde(skip_serializing_if = "Option::is_none")]),
    (mute, Option<bool>, "set the mute state of the user", #[serde(skip_serializing_if = "Option::is_none")])
);

make_request_payload!(
    GetVoiceSettings,
    #[doc = "Used to retrieve the client's voice settings"]
);

// TODO: mode objects
make_request_payload!(
    SetVoiceSettings,
    #[doc = "Used to set the client's voice settings"],
    #[doc = "When setting voice settings, all fields are optional. Only passed fields are updated."],
    (input, Option<InputVoiceSettings>, "input settings", #[serde(skip_serializing_if = "Option::is_none")]),
    (output, Option<OutputVoiceSettings>, "output settings", #[serde(skip_serializing_if = "Option::is_none")]),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume: Option<OrderedFloat<f32>>,
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
    pub const fn new(id: String, name: String) -> Self {
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
}

/// `Error`s that occur when trying to build the [`SetUserVoiceSettings`]
/// request
#[derive(Debug, Error)]
pub enum SetUserVoiceSettingsError {
    /// An error for values that did not satisfy the invariant while building
    /// the [`Pan`]
    #[error("Error setting pan; got values {left} {right}")]
    PanBoundary {
        /// The `left` value argument that may have caused failure
        left: f32,
        /// The `right` value argument that may have caused failure
        right: f32,
    },
    /// An error for values that did not satisfy the invariant while building
    /// the [`Volume`]
    #[error("Error setting volume; got value {vol}")]
    VolumeBoundary {
        /// The `vol` value argument that caused failure
        vol: u8,
    },
}

/// The `Pan` type
///
/// This is used as an argument for [`SetUserVoiceSettings`] where you can set
/// the `Pan` of the user. More information can be found in Discord's
/// [docs][discorddocs].
///
/// The pan (left and right) set by the user must be between 0 and 200.
///
/// [discorddocs]: https://discord.com/developers/docs/topics/rpc#setuservoicesettings-pan-object
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct Pan {
    left: OrderedFloat<f32>,
    right: OrderedFloat<f32>,
}

/// The `Volume` type
///
/// This is used as an argument for [`SetUserVoiceSettings`] where you can set
/// the `Volume` of the user. More information can be found in Discord's
/// [docs][discorddocs].
///
/// The volume set by the user must be between 0 and 200.
///
/// [discorddocs]: https://discord.com/developers/docs/topics/rpc#setuservoicesettings
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct Volume {
    #[serde(flatten)]
    inner: u8,
}

impl Volume {
    const MAX_VOL: u8 = 200;

    /// Creates a new [`Volume`] value
    ///
    /// The [`Default`] volume is 100.
    ///
    /// # Errors
    /// As per Discord's docs [here][discorddocs], the boundaries for the
    /// `volume` type are between 0 and 200. Since u8 can never be negative,
    /// we only need to check if it's above 200.
    ///
    /// [discorddocs]: https://discord.com/developers/docs/topics/rpc#setuservoicesettings
    pub const fn new(inner: u8) -> Result<Self, SetUserVoiceSettingsError> {
        if inner > Self::MAX_VOL {
            Err(SetUserVoiceSettingsError::VolumeBoundary { vol: inner })
        } else {
            Ok(Self { inner })
        }
    }
}

impl Default for Volume {
    fn default() -> Self {
        Self { inner: 100 }
    }
}

impl Pan {
    const MIN_PAN: OrderedFloat<f32> = OrderedFloat(0.0);
    const MAX_PAN: OrderedFloat<f32> = OrderedFloat(1.0);

    /// Creates a new [`Pan`] value
    ///
    /// # Errors
    /// As per Discord's docs [here][discorddocs], the boundaries for the fields
    /// in a `pan` type are between 0.0 and 1.0.
    ///
    /// If what is passed in as arguments for these parameters, then the
    /// function will return an [`enum@Error`], which would contain what you
    /// have passed in as well. See [`enum@Error`] for more.
    ///
    /// [discorddocs]: https://discord.com/developers/docs/topics/rpc#setuservoicesettings-pan-object
    pub fn new(left: f32, right: f32) -> Result<Self, SetUserVoiceSettingsError> {
        let ord_left = OrderedFloat(left);
        let ord_right = OrderedFloat(right);
        // TODO: maybe can get rid of this NAN check anyways since according to the
        // [`ordered_float`] docs they count nan to be the highest
        if (ord_left.is_nan() || ord_right.is_nan())
            || (ord_left < Self::MIN_PAN || ord_right < Self::MIN_PAN)
            || (ord_left > Self::MAX_PAN || ord_right > Self::MAX_PAN)
        {
            Err(SetUserVoiceSettingsError::PanBoundary { left, right })
        } else {
            Ok(Self { left: ord_left, right: ord_right })
        }
    }
}
