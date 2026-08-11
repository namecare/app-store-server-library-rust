use serde::{Deserialize, Serialize};

use crate::models::advanced_commerce_offer::AdvancedCommerceOffer;
use crate::models::advanced_commerce_price_increase_info::AdvancedCommercePriceIncreaseInfo;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceRenewalItem {
    #[serde(rename = "SKU")]
    pub sku: Option<String>,

    pub description: Option<String>,

    pub display_name: Option<String>,

    pub offer: Option<AdvancedCommerceOffer>,

    pub price: Option<i64>,

    /// Information about a price increase for the item.
    ///
    /// [advancedCommercePriceIncreaseInfo](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercepriceincreaseinfo)
    pub price_increase_info: Option<AdvancedCommercePriceIncreaseInfo>,
}
