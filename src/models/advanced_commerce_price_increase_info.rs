use serde::{Deserialize, Serialize};

use crate::models::advanced_commerce_price_increase_info_status::AdvancedCommercePriceIncreaseInfoStatus;

/// Information about the Advanced Commerce price increase.
///
/// [advancedCommercePriceIncreaseInfo](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercepriceincrease)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommercePriceIncreaseInfo {
    /// The dependent SKUs for the price increase.
    ///
    /// [dependentSKUs](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercepriceincreasedependentskus)
    #[serde(rename = "dependentSKUs")]
    pub dependent_skus: Option<Vec<String>>,

    /// The new price for the subscription.
    ///
    /// [price](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercepriceincreaseprice)
    pub price: Option<i64>,

    /// The status of the price increase.
    ///
    /// [status](https://developer.apple.com/documentation/appstoreservernotifications/advancedcommercepriceincreaseinfostatus)
    pub status: Option<AdvancedCommercePriceIncreaseInfoStatus>,
}
