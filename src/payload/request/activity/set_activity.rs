use crate::payload::request::macros::make_request_payload;
use derive_builder::Builder;
use paste::paste;
use serde::{
    Deserialize,
    Serialize,
};
use serde_repr::{
    Deserialize_repr,
    Serialize_repr,
};
use uuid::Uuid;

make_request_payload!(SetActivity,
    #[doc = "Used to update a user's Rich Presence."],
    #[doc = "When using SET_ACTIVITY, the activity object is limited to a type of Playing (0), Listening (2), Watching (3), or Competing (5)."],
    /// Read more from the docs [here][discorddocs]
    ,
    /// [discorddocs]: https://discord.com/developers/docs/topics/rpc#setactivity
    ,
    (pid, u32, "The application's process id"),
    (activity, Option<Activity>, "The rich presence to assign to the user", #[serde(skip_serializing_if = "Option::is_none")])
);

#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Activity {
    #[serde(rename = "type")]
    activity_type: ActivityType,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamps: Option<Timestamps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    party: Option<Party>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assets: Option<Assets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets: Option<Secrets>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
pub struct Timestamps {
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<u64>,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Party {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
pub struct Assets {
    /// Large image displayed on a user's Rich Presence
    #[serde(skip_serializing_if = "Option::is_none")]
    large_image: Option<String>,
    /// Text displayed when hovering over the large image of the activity
    #[serde(skip_serializing_if = "Option::is_none")]
    large_text: Option<String>,
    /// Small image displayed on a user's Rich Presence
    #[serde(skip_serializing_if = "Option::is_none")]
    small_image: Option<String>,
    /// Text displayed when hovering over the small image of the activity
    #[serde(skip_serializing_if = "Option::is_none")]
    small_text: Option<String>,
}

/// Activity secrets
#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Secrets {
    /// Secret for joining a party
    #[serde(skip_serializing_if = "Option::is_none")]
    join: Option<String>,
    /// Secret for spectating a game
    #[serde(skip_serializing_if = "Option::is_none")]
    spectate: Option<String>,
    /// Secret for a specific instanced match
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "match")]
    secrets_match: Option<String>,
}
