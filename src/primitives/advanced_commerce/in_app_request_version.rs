use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum InAppRequestVersion {
    #[serde(rename = "1")]
    V1,
}