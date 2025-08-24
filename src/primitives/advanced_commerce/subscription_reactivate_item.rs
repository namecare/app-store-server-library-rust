use serde::{Deserialize, Serialize};

/// An item for reactivating Advanced Commerce subscriptions.
///
/// [SubscriptionReactivateItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionreactivateitem)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionReactivateItem {
    /// The SKU identifier for the item.
    ///
    /// [SKU](https://developer.apple.com/documentation/advancedcommerceapi/sku)
    #[serde(rename = "SKU")]
    pub sku: String,
}