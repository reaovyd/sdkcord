use serde::{Deserialize, Serialize};

use super::Payload;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PayloadResponse(pub Payload);
