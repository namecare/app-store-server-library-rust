use serde::{Deserialize, Serialize};

/// An item for removing from Advanced Commerce subscription modifications.
///
/// [AdvancedCommerceSubscriptionModifyRemoveItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmodifyremoveitem)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceSubscriptionModifyRemoveItem {
    /// The SKU identifier for the item.
    ///
    /// [SKU](https://developer.apple.com/documentation/advancedcommerceapi/sku)
    #[serde(rename = "SKU")]
    pub sku: String,
}
