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

    /// The product identifier of a dependent SKU in a subscription price change.
    /// The dependentSKU value is a string with a maximum length of 128 characters.
    ///
    /// [dependentSKU](https://developer.apple.com/documentation/advancedcommerceapi/dependentsku)
    #[serde(rename = "dependentSKUs")]
    pub dependent_skus: String,
    /// The new price in milliunits.
    ///
    /// [Price](https://developer.apple.com/documentation/advancedcommerceapi/price)
    pub price: i64,
}
