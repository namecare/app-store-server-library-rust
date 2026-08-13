use serde::{Deserialize, Serialize};

/// The duration of a single cycle of an auto-renewable subscription.
///
/// [period](https://developer.apple.com/documentation/advancedcommerceapi/period)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum AdvancedCommercePeriod {
    /// One week period
    #[serde(rename = "P1W")]
    P1W,
    /// One months period
    #[serde(rename = "P1M")]
    P1M,
    /// Two months period
    #[serde(rename = "P2M")]
    P2M,
    /// Three months period
    #[serde(rename = "P3M")]
    P3M,
    /// Six months period
    #[serde(rename = "P6M")]
    P6M,
    /// One year period
    #[serde(rename = "P1Y")]
    P1Y,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    #[serde(untagged)]
    NotSupported(String),
}
