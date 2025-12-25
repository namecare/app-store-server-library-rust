use serde::{Deserialize, Serialize};

/// The status of the Advanced Commerce price increase.
///
/// [advancedCommercePriceIncreaseInfoStatus](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercepriceincreasestatus)
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
}

/// Information about the Advanced Commerce price increase.
///
/// [advancedCommercePriceIncreaseInfo](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercepriceincrease)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommercePriceIncreaseInfo {
    /// The dependent SKUs for the price increase.
    ///
    /// [dependentSKUs](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercepriceincreasedependentskus)
    pub dependent_skus: Option<Vec<String>>,

    /// The new price for the subscription.
    ///
    /// [price](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercepriceincreaseprice)
    pub price: Option<i64>,

    /// The status of the price increase.
    ///
    /// [status](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercepriceincreasestatus)
    pub status: Option<AdvancedCommercePriceIncreaseInfoStatus>,
}