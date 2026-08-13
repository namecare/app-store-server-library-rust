use serde::{Deserialize, Serialize};

/// The status of the Advanced Commerce price increase.
///
/// [advancedCommercePriceIncreaseInfoStatus](https://developer.apple.com/documentation/appstoreservernotifications/advancedcommercepriceincreaseinfostatus)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum AdvancedCommercePriceIncreaseInfoStatus {
    /// The price increase is scheduled.
    #[serde(rename = "SCHEDULED")]
    Scheduled,
    /// The price increase is pending.
    #[serde(rename = "PENDING")]
    Pending,
    /// The price increase has been accepted.
    #[serde(rename = "ACCEPTED")]
    Accepted,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    #[serde(untagged)]
    NotSupported(String),
}
