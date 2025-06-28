use serde::{Deserialize, Serialize};

use super::Payload;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[repr(transparent)]
pub struct PayloadResponse(pub Payload);
