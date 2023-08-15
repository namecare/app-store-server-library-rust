use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum AutoRenewStatus {
    Off = 0,
    On = 1,
}