use serde::{Deserialize, Serialize};

use crate::models::advanced_commerce_offer::AdvancedCommerceOffer;

/// The data that describes a subscription item.
///
/// [AdvancedCommerceSubscriptionCreateItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptioncreateitem)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceSubscriptionCreateItem {
    /// The item's product identifier, which you define.
    ///
    /// [SKU](https://developer.apple.com/documentation/advancedcommerceapi/sku)
    #[serde(rename = "SKU")]
    pub sku: String,

    /// A description of the product.
    ///
    /// [Description](https://developer.apple.com/documentation/advancedcommerceapi/description)
    pub description: String,

    /// The product name, suitable for display to customers.
    ///
    /// [Display Name](https://developer.apple.com/documentation/advancedcommerceapi/displayname)
    pub display_name: String,

    /// The price in milliunits.
    ///
    /// [Price](https://developer.apple.com/documentation/advancedcommerceapi/price)
    pub price: i64,

    /// An offer for the subscription.
    ///
    /// [AdvancedCommerceOffer](https://developer.apple.com/documentation/advancedcommerceapi/offer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer: Option<AdvancedCommerceOffer>,
}

impl AdvancedCommerceSubscriptionCreateItem {
    pub fn new(sku: String, description: String, display_name: String, price: i64) -> Self {
        Self {
            sku,
            description,
            display_name,
            price,
            offer: None,
        }
    }

    pub fn with_offer(mut self, offer: AdvancedCommerceOffer) -> Self {
        self.offer = Some(offer);
        self
    }
}
