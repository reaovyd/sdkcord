use crate::payload::request::macros::make_request_payload;
use paste::paste;
use derive_builder::Builder;
use uuid::Uuid;
use serde::{
    Deserialize,
    Serialize,
};
use serde_repr::{
    Deserialize_repr,
    Serialize_repr,
};
use serde_with::skip_serializing_none;

make_request_payload!(SetActivity,
    #[doc = "Used to update a user's Rich Presence."],
    #[doc = "When using SET_ACTIVITY, the activity object is limited to a type of Playing (0), Listening (2), Watching (3), or Competing (5)."],
    /// Read more from the docs [here][discorddocs]
    ,
    /// [discorddocs]: https://discord.com/developers/docs/topics/rpc#setactivity
    ,
    (pid, u32, (#[doc = "The application's process id"])),
    (activity, Option<Activity>,
        (#[doc = "The rich presence to assign to the user"]),
        (#[serde(skip_serializing_if = "Option::is_none")], #[builder(setter(strip_option), default)])
    )
);

#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[builder(setter(into))]
#[skip_serializing_none]
pub struct Activity {
    #[serde(rename = "type")]
    activity_type: ActivityType,
    #[builder(default)]
    timestamps: Option<Timestamps>,
    #[builder(default)]
    details: Option<String>,
    #[builder(default)]
    state: Option<String>,
    #[builder(default)]
    party: Option<Party>,
    #[builder(default)]
    assets: Option<Assets>,
    #[builder(default)]
    secrets: Option<Secrets>,
    #[builder(default)]
    instance: Option<bool>,
}

/// Type of Activity user is doing
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ActivityType {
    /// Playing {name}
    ///
    /// # Note
    /// This would be the name of your application. Oftentimes, you cannot set
    /// the name and we do not provide a way to add that name as well.
    Playing = 0,
    /// Listening To {name}
    ///
    /// # Note
    /// This would be the name of your application. Oftentimes, you cannot set
    /// the name and we do not provide a way to add that name as well.
    Listening = 2,
    /// Watching {name}
    ///
    /// # Note
    /// This would be the name of your application. Oftentimes, you cannot set
    /// the name and we do not provide a way to add that name as well.
    Watching = 3,
    /// Competing {name}
    ///
    /// # Note
    /// This would be the name of your application. Oftentimes, you cannot set
    /// the name and we do not provide a way to add that name as well.
    Competing = 5,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[skip_serializing_none]
pub struct Timestamps {
    #[builder(default)]
    start: Option<u64>,
    #[builder(default)]
    end: Option<u64>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[skip_serializing_none]
pub struct Party {
    #[builder(default)]
    id: Option<String>,
    #[builder(default)]
    size: Option<[u32; 2]>,
}

/// The Assets Type
///
/// When you click on a Discord user who has an activity set, there is typically
/// a picture and a maybe a small picture on that activity set. This is where
/// you indicate the image and text you want to appear when users see your Rich
/// Presence.
///
/// Look at the [Discord docs][discorddocs] for more info on what image names
/// are supported. [discorddocs]: https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-asset-image
#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[skip_serializing_none]
pub struct Assets {
    /// Large image displayed on a user's Rich Presence
    #[builder(default)]
    large_image: Option<String>,
    /// Text displayed when hovering over the large image of the activity
    #[builder(default)]
    large_text: Option<String>,
    /// Small image displayed on a user's Rich Presence
    #[builder(default)]
    small_image: Option<String>,
    /// Text displayed when hovering over the small image of the activity
    #[builder(default)]
    small_text: Option<String>,
}

/// Activity secrets
#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[skip_serializing_none]
pub struct Secrets {
    /// Secret for joining a party
    #[builder(default)]
    join: Option<String>,
    /// Secret for spectating a game
    #[builder(default)]
    spectate: Option<String>,
    /// Secret for a specific instanced match
    #[serde(rename = "match")]
    #[builder(default)]
    secrets_match: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{
        ActivityBuilder,
        SetActivity,
        SetActivityArgsBuilder,
    };

    #[test]
    fn test_basic_build() {
        let activity = SetActivity::new(
            SetActivityArgsBuilder::create_empty()
                .pid(2333u32)
                .activity(
                    ActivityBuilder::create_empty()
                        .activity_type(super::ActivityType::Playing)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        );
        let serialized = serde_json::to_string(&activity).unwrap();
        assert!(serialized.contains(r#""activity":{"type":0"#));
    }
}
