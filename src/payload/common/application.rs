use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Application {
    pub id: Option<String>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub description: Option<String>,
}
