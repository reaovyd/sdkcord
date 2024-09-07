use super::macros::make_request_payload;
use derive_builder::Builder;
use paste::paste;
use serde::{
    Deserialize,
    Serialize,
};
use url::Url;
use uuid::Uuid;

make_request_payload!(SetCertifiedDevices,
    #[doc = "Used by hardware manufacturers to send information about the
current state of their certified devices that are connected to Discord."],
    (devices, DeviceList, "a list of devices for your manufacturer, in order of priority", #[builder(setter(into))])
);

/// Array of [`Device`] objects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeviceList(pub Vec<Device>);

/// The [`Device`] type that represents a device object in Discord
#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Device {
    /// The type of device
    #[serde(rename = "type")]
    device_type: DeviceType,
    /// The device's Windows UUID
    id: Uuid,
    /// The hardware vendor
    vendor: Vendor,
    /// The model of the product
    model: Model,
    /// UUIDs of related devices
    related: Related,
    /// If the device's native echo cancellation is enabled
    ///
    /// This is only available for [`DeviceType::AudioInput`] device types!
    #[serde(skip_serializing_if = "Option::is_none")]
    echo_cancellation: Option<bool>,
    /// If the device's native noise suppression is enabled
    ///
    /// This is only available for [`DeviceType::AudioInput`] device types!
    #[serde(skip_serializing_if = "Option::is_none")]
    noise_suppression: Option<bool>,
    /// If the device's native automatic gain control is enabled
    ///
    /// This is only available for [`DeviceType::AudioInput`] device types!
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_gain_control: Option<bool>,
    /// If the device is hardware muted
    ///
    /// This is only available for [`DeviceType::AudioInput`] device types!
    #[serde(skip_serializing_if = "Option::is_none")]
    hardware_mute: Option<bool>,
}

/// The type of device
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    /// Serializes into "audioinput"
    AudioInput,
    /// Serializes into "audiooutput"
    AudioOutput,
    /// Serializes into "videoinput"
    VideoInput,
}

/// The hardware vendor
#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Vendor {
    /// Name of the vendor
    name: String,
    /// Url for the vendor
    url: Url,
}

/// The model of the product
#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Model {
    /// Name of the model
    name: String,
    /// Url for the model
    url: Url,
}

/// UUIDs of related devices
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct Related(pub Vec<Uuid>);

impl From<Vec<Uuid>> for Related {
    fn from(value: Vec<Uuid>) -> Self {
        Related(value)
    }
}

impl From<&[Uuid]> for Related {
    fn from(value: &[Uuid]) -> Self {
        Related(value.into())
    }
}

impl<const N: usize> From<[Uuid; N]> for Related {
    fn from(value: [Uuid; N]) -> Self {
        Related(value.into())
    }
}

impl From<Vec<Device>> for DeviceList {
    fn from(value: Vec<Device>) -> Self {
        DeviceList(value)
    }
}
