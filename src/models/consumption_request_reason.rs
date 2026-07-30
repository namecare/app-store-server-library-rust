use serde::{Deserialize, Serialize};

/// The customer-provided reason for a refund request.
///
/// [consumptionRequestReason](https://developer.apple.com/documentation/appstoreservernotifications/consumptionrequestreason)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsumptionRequestReason {
    #[serde(rename = "UNINTENDED_PURCHASE")]
    UnintendedPurchase,
    #[serde(rename = "FULFILLMENT_ISSUE")]
    FulfillmentIssue,
    #[serde(rename = "UNSATISFIED_WITH_PURCHASE")]
    UnsatisfiedWithPurchase,
    #[serde(rename = "LEGAL")]
    Legal,
    #[serde(rename = "OTHER")]
    Other,
}
