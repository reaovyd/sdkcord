use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Embed {
    pub author: Option<Author>,
    pub color: Option<String>,
    pub content_scan_version: Option<u32>,
    pub fields: Option<Vec<Field>>,
    pub footer: Option<Footer>,
    pub id: Option<String>,
    pub image: Option<Image>,
    pub provider: Option<Provider>,
    pub raw_description: Option<String>,
    pub raw_title: Option<String>,
    pub thumbnail: Option<Thumbnail>,
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    pub embed_type: EmbedType,
    pub url: Option<Url>,
    pub video: Option<Video>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Author {
    pub name: Option<String>,
    pub url: Option<String>,
    pub icon_url: Option<Url>,
    pub proxy_icon_url: Option<Url>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Field {
    pub name: Option<String>,
    pub value: Option<String>,
    pub inline: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Footer {
    pub icon_proxy_url: Option<Url>,
    pub icon_url: Option<Url>,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Image(pub EmbedContent);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Thumbnail(pub EmbedContent);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Video(pub EmbedContent);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct EmbedContent {
    pub content_type: Option<String>,
    pub flags: Option<u32>,
    pub height: Option<u32>,
    pub placeholder: Option<String>,
    pub placeholder_version: Option<u32>,
    pub proxy_url: Option<Url>,
    pub src_is_animated: Option<bool>,
    pub url: Option<Url>,
    pub width: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Provider {
    pub name: Option<String>,
    pub url: Option<Url>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum EmbedType {
    Rich,
    Image,
    Video,
    Gifv,
    Article,
    Link,
    PollResult,
}
