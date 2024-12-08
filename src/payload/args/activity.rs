use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::payload::types::activity::ActivityRequest;

use super::macros::impl_request_args_type;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct SetActivityArgs {
    pid: u16,
    #[builder(with = |activity: impl Into<ActivityRequest>| {
        Box::new(activity.into())
    })]
    activity: Option<Box<ActivityRequest>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct SendActivityJoinInviteArgs {
    #[builder(into)]
    user_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct CloseActivityRequestArgs {
    #[builder(into)]
    user_id: Option<String>,
}

impl_request_args_type!(SetActivity);
impl_request_args_type!(SendActivityJoinInvite);
impl_request_args_type!(CloseActivityRequest);

#[cfg(test)]
mod tests {
    use crate::payload::types::activity::{Activity, ActivityType};

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
