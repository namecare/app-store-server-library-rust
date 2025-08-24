use serde::{Deserialize, Serialize};

/// An item for migrating Advanced Commerce subscriptions.
///
/// [SubscriptionMigrateItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmigrateitem)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionMigrateItem {
    /// The SKU identifier for the item.
    ///
    /// [SKU](https://developer.apple.com/documentation/advancedcommerceapi/sku)
    #[serde(rename = "SKU")]
    pub sku: String,

    /// The description of the item.
    ///
    /// [Description](https://developer.apple.com/documentation/advancedcommerceapi/description)
    pub description: String,

    /// The display name of the item.
    ///
    /// [Display Name](https://developer.apple.com/documentation/advancedcommerceapi/displayname)
    pub display_name: String,
}

impl SubscriptionMigrateItem {
    pub fn new(
        sku: String,
        description: String,
        display_name: String,
    ) -> Self {
        Self {
            sku,
            description,
            display_name,
        }
    }
}