use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ErrorData {
    pub code: Option<u32>,
    pub message: Option<String>,
}
