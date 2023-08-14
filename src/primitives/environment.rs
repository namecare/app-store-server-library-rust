use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum Environment {
    #[serde(rename = "Sandbox")]
    Sandbox,
    #[serde(rename = "Production")]
    Production,
}
