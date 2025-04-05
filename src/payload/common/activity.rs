use bon::{Builder, bon, builder};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use strum_macros::{Display, EnumString};

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Activity {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub activity_type: Option<ActivityType>,
    pub url: Option<String>,
    pub created_at: Option<u64>,
    pub timestamps: Option<Timestamps>,
    pub application_id: Option<String>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<Emoji>,
    pub party: Option<Party>,
    pub assets: Option<Assets>,
    pub secrets: Option<Secrets>,
    pub instance: Option<bool>,
    pub flags: Option<u16>,
    pub buttons: Option<Vec<Button>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ActivityRequest(Activity);

#[bon]
impl Activity {
    #[builder]
    pub const fn request_builder(
        #[builder(into)] activity_type: ActivityType,
        #[builder(into)] timestamps: Option<Timestamps>,
        #[builder(into)] details: Option<String>,
        #[builder(into)] state: Option<String>,
        #[builder(into)] party: Option<Party>,
        #[builder(into)] assets: Option<Assets>,
        #[builder(into)] secrets: Option<Secrets>,
        #[builder(into)] instance: Option<bool>,
    ) -> ActivityRequest {
        ActivityRequest(Activity {
            name: None,
            activity_type: Some(activity_type),
            url: None,
            created_at: None,
            timestamps,
            application_id: None,
            details,
            state,
            emoji: None,
            party,
            assets,
            secrets,
            instance,
            flags: None,
            buttons: None,
        })
    }
}

#[derive(
    Debug, Copy, Clone, Deserialize_repr, Serialize_repr, PartialEq, Eq, Hash, EnumString, Display,
)]
#[repr(u8)]
pub enum ActivityType {
    Playing = 0,
    Streaming = 1,
    Listening = 2,
    Watching = 3,
    Custom = 4,
    Competing = 5,
}

#[skip_serializing_none]
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct Timestamps {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct Emoji {
    #[builder(into)]
    pub name: Option<String>,
    #[builder(into)]
    pub id: Option<String>,
    #[builder(into)]
    pub animated: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct Party {
    #[builder(into)]
    pub id: Option<String>,
    #[builder(into)]
    pub size: Option<[u32; 2]>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Assets {
    #[builder(into)]
    pub large_image: Option<String>,
    #[builder(into)]
    pub large_text: Option<String>,
    #[builder(into)]
    pub small_image: Option<String>,
    #[builder(into)]
    pub small_text: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Secrets {
    #[builder(into)]
    pub join: Option<String>,
    #[builder(into)]
    pub spectate: Option<String>,
    #[serde(rename = "match")]
    #[builder(into)]
    pub secrets_match: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Button {
    #[builder(into)]
    pub label: Option<String>,
    #[builder(into)]
    pub url: Option<String>,
}

impl Button {
    pub fn new(label: impl Into<String>, url: impl Into<String>) -> Self {
        Button {
            label: Some(label.into()),
            url: Some(url.into()),
        }
    }
}

impl From<(u64, u64)> for Timestamps {
    fn from(value: (u64, u64)) -> Self {
        Timestamps {
            start: Some(value.0),
            end: Some(value.1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Activity, ActivityType, Assets, Party, Secrets};

    #[test]
    fn construct_activity_request() {
        let activity_request = Activity::request_builder()
            .activity_type(ActivityType::Watching)
            .state("State1")
            .details("Details1")
            .timestamps((10, 20))
            .assets(
                Assets::builder()
                    .large_image("123")
                    .large_text("123")
                    .small_image("smallimage")
                    .small_text("smalltext")
                    .build(),
            )
            .party(Party::builder().id("123").size([1, 2]).build())
            .secrets(
                Secrets::builder()
                    .join("123")
                    .spectate("145")
                    .secrets_match("123")
                    .build(),
            )
            .call();
        let activity_request = serde_json::to_string(&activity_request).unwrap();
        assert!(activity_request.contains("\"details\":\"Details1\""));
    }
}
