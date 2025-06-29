use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{attachment::Attachment, embed::Embed, user::User};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Message {
    pub attachments: Option<Vec<Attachment>>,
    pub author: Option<User>,
    pub author_color: Option<String>,
    pub blocked: Option<bool>,
    pub bot: Option<bool>,
    pub content: Option<String>,
    pub edited_timestamp: Option<DateTime<Utc>>,
    pub embeds: Option<Vec<Embed>>,
    pub id: Option<String>,
    pub mention_everyone: Option<bool>,
}
