use serde::{Deserialize, Serialize};

/// The renewal billing plan type for a monthly subscription with a 12-month commitment.
///
/// [RenewalBillingPlanType](https://developer.apple.com/documentation/appstoreserverapi/renewalbillingplantype)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RenewalBillingPlanType {
    #[serde(rename = "BILLED_UPFRONT")]
    BilledUpfront,
    #[serde(rename = "MONTHLY")]
    Monthly,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    #[serde(untagged)]
    NotSupported(String),
}
