use bon::{builder, Builder};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::EnumString;
use uuid::Uuid;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct Device {
    #[builder(into)]
    #[serde(rename = "type")]
    pub device_type: DeviceType,
    #[builder(into)]
    pub id: Uuid,
    #[builder(into)]
    pub vendor: Vendor,
    #[builder(into)]
    pub model: Model,
    #[builder(with = |related_devices: impl IntoIterator<Item = Uuid>| {
        related_devices.into_iter().collect()
    })]
    pub related: Vec<Uuid>,
    #[builder(into)]
    pub echo_collection: Option<bool>,
    #[builder(into)]
    pub noise_suppression: Option<bool>,
    #[builder(into)]
    pub automatic_gain_control: Option<bool>,
    #[builder(into)]
    pub hardware_mute: Option<bool>,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum DeviceType {
    AudioInput,
    AudioOutput,
    VideoInput,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct Vendor {
    #[builder(into)]
    pub name: String,
    #[builder(into)]
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct Model {
    #[builder(into)]
    pub name: String,
    #[builder(into)]
    pub url: String,
}

impl<NameT, UrlT> From<(NameT, UrlT)> for Vendor
where
    NameT: Into<String>,
    UrlT: Into<String>,
{
    fn from(value: (NameT, UrlT)) -> Self {
        Self { name: value.0.into(), url: value.1.into() }
    }
}

impl<NameT, UrlT> From<(NameT, UrlT)> for Model
where
    NameT: Into<String>,
    UrlT: Into<String>,
{
    fn from(value: (NameT, UrlT)) -> Self {
        Self { name: value.0.into(), url: value.1.into() }
    }
}
