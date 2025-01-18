use bon::Builder;
use serde::{
    Deserialize,
    Serialize,
};
use serde_with::skip_serializing_none;

use crate::payload::common::activity::ActivityRequest;

use super::macros::{
    impl_empty_args_type,
    impl_event_args_type,
    impl_request_args_type,
};

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct SetActivityArgs {
    pid: u32,
    #[builder(into)]
    activity: Option<Box<ActivityRequest>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
#[cfg(feature = "untested")]
pub struct SendActivityJoinInviteArgs {
    #[builder(into)]
    user_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
#[cfg(feature = "untested")]
pub struct CloseActivityRequestArgs {
    #[builder(into)]
    user_id: Option<String>,
}

impl_request_args_type!(SetActivity);
#[cfg(feature = "untested")]
impl_request_args_type!(SendActivityJoinInvite);
#[cfg(feature = "untested")]
impl_request_args_type!(CloseActivityRequest);

#[cfg(feature = "untested")]
impl_empty_args_type!(ActivityJoin);
#[cfg(feature = "untested")]
impl_empty_args_type!(ActivitySpectate);
#[cfg(feature = "untested")]
impl_empty_args_type!(ActivityJoinRequest);

#[cfg(feature = "untested")]
impl_event_args_type!(ActivityJoin);
#[cfg(feature = "untested")]
impl_event_args_type!(ActivitySpectate);
#[cfg(feature = "untested")]
impl_event_args_type!(ActivityJoinRequest);

#[cfg(test)]
mod tests {
    use crate::payload::common::activity::{
        Activity,
        ActivityType,
    };

    use super::SetActivityArgs;

    #[test]
    fn construct_set_activity_args() {
        let sca = SetActivityArgs::builder()
            .pid(12)
            .activity(
                Activity::request_builder()
                    .activity_type(ActivityType::Watching)
                    .timestamps((12, 32))
                    .call(),
            )
            .build();
        let sca = serde_json::to_string(&sca).unwrap();
        assert!(sca.contains("\"type\":3"));
        assert!(sca.contains("\"pid\":12"));
    }
}
