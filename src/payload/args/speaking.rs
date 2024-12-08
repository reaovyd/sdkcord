use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct SpeakingStartArgs(pub Speaking);

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct SpeakingStopArgs(pub Speaking);

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct Speaking {
    #[builder(into)]
    channel_id: Option<String>,
}

impl From<Speaking> for SpeakingStartArgs {
    fn from(value: Speaking) -> Self {
        Self(value)
    }
}

impl From<Speaking> for SpeakingStopArgs {
    fn from(value: Speaking) -> Self {
        Self(value)
    }
}
