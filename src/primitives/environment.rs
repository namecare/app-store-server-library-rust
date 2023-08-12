use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum Environment {
    #[serde(rename = "Sandbox")]
    Sandbox,
    #[serde(rename = "Production")]
    Production,
}
