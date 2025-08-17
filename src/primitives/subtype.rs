use serde::{Deserialize, Serialize};

/// A string that provides details about select notification types in version 2.
///
/// [subtype](https://developer.apple.com/documentation/appstoreservernotifications/subtype)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum Subtype {
    #[serde(rename = "INITIAL_BUY")]
    InitialBuy,
    #[serde(rename = "RESUBSCRIBE")]
    Resubscribe,
    #[serde(rename = "DOWNGRADE")]
    Downgrade,
    #[serde(rename = "UPGRADE")]
    Upgrade,
    #[serde(rename = "AUTO_RENEW_ENABLED")]
    AutoRenewEnabled,
    #[serde(rename = "AUTO_RENEW_DISABLED")]
    AutoRenewDisabled,
    #[serde(rename = "VOLUNTARY")]
    Voluntary,
    #[serde(rename = "BILLING_RETRY")]
    BillingRetry,
    #[serde(rename = "PRICE_INCREASE")]
    PriceIncrease,
    #[serde(rename = "GRACE_PERIOD")]
    GracePeriod,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "ACCEPTED")]
    Accepted,
    #[serde(rename = "BILLING_RECOVERY")]
    BillingRecovery,
    #[serde(rename = "PRODUCT_NOT_FOR_SALE")]
    ProductNotForSale,
    #[serde(rename = "SUMMARY")]
    Summary,
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(rename = "UNREPORTED")]
    Unreported,
    #[serde(rename = "ACTIVE_TOKEN_REMINDER")]
    ActiveTokenReminder,
    #[serde(rename = "CREATED")]
    Created,
}
