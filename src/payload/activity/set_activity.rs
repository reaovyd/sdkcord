use crate::payload::macros::make_command_reqres_payload;
use bon::Builder;
use serde::{
    Deserialize,
    Serialize,
};
use serde_repr::{
    Deserialize_repr,
    Serialize_repr,
};
use serde_with::skip_serializing_none;

make_command_reqres_payload!(SetActivity,
    (
        /// Used to update a user's Rich Presence
        /// When using SET_ACTIVITY, the activity object is limited to a type of Playing (0), Listening (2), Watching (3), or Competing (5)
        /// Read more from the docs [here][discorddocs]
        /// [discorddocs]: https://discord.com/developers/docs/topics/rpc#setactivity
    ),
    (pid, u32,
        (
        /// The application's process id
        ///
        /// This will bind to a process id on your computer. When your process is killed, the
        /// activity will end in Discord.
        )
    ),
    (activity, Option<Activity>,
        (#[doc = "The rich presence to assign to the user"]),
        (
            #[serde(skip_serializing_if = "Option::is_none")]
            #[builder(into)]
        )
    )
);

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[builder(derive(Debug))]
pub struct Activity {
    #[serde(rename = "type")]
    #[builder(into)]
    activity_type: ActivityType,
    #[builder(into)]
    timestamps: Option<Timestamps>,
    #[builder(into)]
    details: Option<String>,
    #[builder(into)]
    state: Option<String>,
    #[builder(into)]
    party: Option<Party>,
    #[builder(into)]
    assets: Option<Assets>,
    #[builder(into)]
    secrets: Option<Secrets>,
    #[builder(into)]
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

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[builder(derive(Debug))]
pub struct Timestamps {
    start: Option<u64>,
    end: Option<u64>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[builder(derive(Debug))]
pub struct Party {
    #[builder(into)]
    id: Option<String>,
    #[builder(into)]
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
#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[builder(derive(Debug))]
pub struct Assets {
    /// Large image displayed on a user's Rich Presence
    #[builder(into)]
    large_image: Option<String>,
    /// Text displayed when hovering over the large image of the activity
    #[builder(into)]
    large_text: Option<String>,
    /// Small image displayed on a user's Rich Presence
    #[builder(into)]
    small_image: Option<String>,
    /// Text displayed when hovering over the small image of the activity
    #[builder(into)]
    small_text: Option<String>,
}

/// Activity secrets
#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[builder(derive(Debug))]
pub struct Secrets {
    /// Secret for joining a party
    #[builder(into)]
    join: Option<String>,
    /// Secret for spectating a game
    #[builder(into)]
    spectate: Option<String>,
    /// Secret for a specific instanced match
    #[serde(rename = "match")]
    #[builder(into)]
    secrets_match: Option<String>,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::payload::{
        Activity,
        SetActivityArgs,
        Timestamps,
    };

    use super::{
        Party,
        SetActivity,
    };

    #[test]
    fn test_basic_build() {
        let activity = SetActivity::new(
            SetActivityArgs::builder()
                .pid(2333u32)
                .activity(Activity::builder().activity_type(super::ActivityType::Playing).build())
                .build(),
        );
        let serialized = serde_json::to_string(&activity).unwrap();
        assert!(serialized.contains(r#""activity":{"type":0"#));
    }

    #[test]
    fn test_party_build() {
        let party = Party::builder().size([3, 2]).build();
        assert_eq!(party.size, Some([3, 2]))
    }

    #[test]
    fn test_timestamps_build() {
        // NOTE: just wanted to check Into u64 condition
        let timestamps = Timestamps::builder().start(3).end(5).build();
        assert_eq!(timestamps.start, Some(3));
        assert_eq!(timestamps.end, Some(5));
    }
}
