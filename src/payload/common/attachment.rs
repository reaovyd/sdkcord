use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Attachment {
    pub content_scan_version: Option<u32>,
    pub content_type: Option<String>,
    pub duration_secs: Option<f32>,
    pub filename: Option<String>,
    pub height: Option<u32>,
    pub id: Option<String>,
    pub placeholder: Option<String>,
    pub placeholder_version: Option<u32>,
    pub proxy_url: Option<String>,
    pub size: Option<u32>,
    pub spoiler: Option<bool>,
    pub url: Option<String>,
    pub waveform: Option<String>,
    pub width: Option<u32>,
}
