pub use channel::*;
pub use guild::*;
pub use message::*;
pub use notification::*;
pub use speaking::*;
pub use voice::*;

use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(tag = "evt", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscribeRequest {
    GuildStatus(GuildStatusSubscriptionEvent),
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(tag = "evt", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UnsubscribeRequest {
    GuildStatus(GuildStatusUnsubscriptionEvent),
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash, Default)]
struct EmptyArgs {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    inner: Option<()>,
}

mod channel;
mod guild;
mod macros;
mod message;
mod notification;
mod speaking;
mod voice;
