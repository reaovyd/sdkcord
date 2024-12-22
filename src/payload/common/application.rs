use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Application {
    id: Option<String>,
    name: Option<String>,
    icon: Option<String>,
    description: Option<String>,
}
