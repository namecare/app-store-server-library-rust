use serde::{Deserialize, Serialize};
use crate::primitives::advanced_commerce::offer::Offer;

/// The data your app provides to add items when it makes changes to an auto-renewable subscription.
///
/// [SubscriptionModifyAddItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmodifyadditem)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionModifyAddItem {
    /// The item's product identifier.
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

    /// The price in milliunits.
    ///
    /// [Price](https://developer.apple.com/documentation/advancedcommerceapi/price)
    pub price: i64,

    /// An offer for the item.
    ///
    /// [Offer](https://developer.apple.com/documentation/advancedcommerceapi/offer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer: Option<Offer>,

    /// The prorated price for the item.
    ///
    /// [ProratedPrice](https://developer.apple.com/documentation/advancedcommerceapi/proratedprice)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorated_price: Option<i64>,
}

impl SubscriptionModifyAddItem {
    pub fn new(
        sku: String,
        description: String,
        display_name: String,
        price: i64,
    ) -> Self {
        Self {
            sku,
            description,
            display_name,
            price,
            offer: None,
            prorated_price: None,
        }
    }

    pub fn with_offer(mut self, offer: Offer) -> Self {
        self.offer = Some(offer);
        self
    }

    pub fn with_prorated_price(mut self, prorated_price: i64) -> Self {
        self.prorated_price = Some(prorated_price);
        self
    }
}