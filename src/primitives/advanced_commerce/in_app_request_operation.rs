use serde::{Deserialize, Serialize};

/// The type of request operation.
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InAppRequestOperation {
    CreateSubscription,
    CreateOneTimeCharge,
    ModifySubscription,
    ReactivateSubscription,
    RevokeSubscription,
}