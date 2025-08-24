use serde::{Deserialize, Serialize};

/// An item for Advanced Commerce subscription price changes.
///
/// [SubscriptionPriceChangeItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionpricechangeitem)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionPriceChangeItem {
    /// The SKU identifier for the item.
    ///
    /// [SKU](https://developer.apple.com/documentation/advancedcommerceapi/sku)
    #[serde(rename = "SKU")]
    pub sku: String,

    /// The new price in milliunits.
    ///
    /// [Price](https://developer.apple.com/documentation/advancedcommerceapi/price)
    pub price: i64,
}
