use serde::{Deserialize, Serialize};

/// The reason for the Advanced Commerce request.
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdvancedCommerceReason {
    Upgrade,
    Downgrade,
    ApplyOffer,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    #[serde(untagged)]
    NotSupported(String),
}
