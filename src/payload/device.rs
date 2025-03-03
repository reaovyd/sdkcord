#![cfg(feature = "untested")]
use bon::{Builder, builder};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::payload::common::device::Device;

use super::macros::impl_request_args_type;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct SetCertifiedDevicesArgs {
    #[builder(with = |devices: impl IntoIterator<Item = Device>| {
        devices.into_iter().collect()
    })]
    devices: Vec<Device>,
}

#[cfg(feature = "untested")]
impl_request_args_type!(SetCertifiedDevices);

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::payload::common::device::{Device, DeviceType};

    use super::SetCertifiedDevicesArgs;

    #[test]
    fn construct_certified_devices() {
        let device = Device::builder()
            .device_type(DeviceType::AudioInput)
            .id(Uuid::new_v4())
            .vendor(("abcv", "asd"))
            .model(("123", "43"))
            .related([Uuid::new_v4(), Uuid::new_v4()])
            .echo_collection(true)
            .noise_suppression(true)
            .build();
        let scda = SetCertifiedDevicesArgs::builder().devices([device]).build();
        let json = serde_json::to_string(&scda).unwrap();
        assert!(json.contains("\"echo_collection\":true"));
    }
}
