use super::macros::make_command_reqres_payload;
use bon::Builder;
use serde::{
    Deserialize,
    Serialize,
};
use serde_with::skip_serializing_none;
use url::Url;
use uuid::Uuid;

make_command_reqres_payload!(SetCertifiedDevices,
    (
        /// Used by hardware manufacturers to send information about the current state of their certified devices that are connected to Discord
    ),
    (devices, DeviceList, (#[doc = "a list of devices for your manufacturer, in order of priority"]), (#[builder(into)]))
);

/// Array of [`Device`] objects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeviceList(pub Vec<Device>);

/// The [`Device`] type that represents a device object in Discord
#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[builder(derive(Debug))]
pub struct Device {
    /// The type of device
    #[serde(rename = "type")]
    #[builder(into)]
    device_type: DeviceType,
    /// The device's Windows UUID
    #[builder(into)]
    id: Uuid,
    /// The hardware vendor
    #[builder(into)]
    vendor: Vendor,
    /// The model of the product
    #[builder(into)]
    model: Model,
    /// UUIDs of related devices
    #[builder(into)]
    related: Related,
    /// If the device's native echo cancellation is enabled
    ///
    /// This is only available for [`DeviceType::AudioInput`] device types!
    #[builder(into)]
    echo_cancellation: Option<bool>,
    /// If the device's native noise suppression is enabled
    ///
    /// This is only available for [`DeviceType::AudioInput`] device types!
    #[builder(into)]
    noise_suppression: Option<bool>,
    /// If the device's native automatic gain control is enabled
    ///
    /// This is only available for [`DeviceType::AudioInput`] device types!
    #[builder(into)]
    automatic_gain_control: Option<bool>,
    /// If the device is hardware muted
    ///
    /// This is only available for [`DeviceType::AudioInput`] device types!
    #[builder(into)]
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
#[builder(derive(Debug))]
pub struct Vendor {
    /// Name of the vendor
    #[builder(into)]
    name: String,
    /// Url for the vendor
    #[builder(into)]
    url: Url,
}

/// The model of the product
#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[builder(derive(Debug))]
pub struct Model {
    /// Name of the model
    #[builder(into)]
    name: String,
    /// Url for the model
    #[builder(into)]
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
