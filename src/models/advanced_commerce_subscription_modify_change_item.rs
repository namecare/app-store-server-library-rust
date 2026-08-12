use serde::{Deserialize, Serialize};

use crate::models::advanced_commerce_effective::AdvancedCommerceEffective;
use crate::models::advanced_commerce_offer::AdvancedCommerceOffer;
use crate::models::advanced_commerce_reason::AdvancedCommerceReason;

/// The data your app provides to change an item of an auto-renewable subscription.
///
/// [AdvancedCommerceSubscriptionModifyChangeItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmodifychangeitem)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceSubscriptionModifyChangeItem {
    /// The new SKU identifier for the item.
    ///
    /// [SKU](https://developer.apple.com/documentation/advancedcommerceapi/sku)
    #[serde(rename = "SKU")]
    pub sku: String,

    /// The original SKU of the item.
    ///
    /// [currentSKU](https://developer.apple.com/documentation/advancedcommerceapi/sku)
    #[serde(rename = "currentSKU")]
    pub current_sku: String,

    /// The description of the item.
    ///
    /// [Description](https://developer.apple.com/documentation/advancedcommerceapi/description)
    pub description: String,

    /// The display name of the item.
    ///
    /// [Display Name](https://developer.apple.com/documentation/advancedcommerceapi/displayname)
    pub display_name: String,

    /// When the change takes effect.
    ///
    /// [AdvancedCommerceEffective](https://developer.apple.com/documentation/advancedcommerceapi/effective)
    pub effective: AdvancedCommerceEffective,

    /// The price in milliunits.
    ///
    /// [Price](https://developer.apple.com/documentation/advancedcommerceapi/price)
    pub price: i64,

    /// The reason for the change.
    ///
    /// [AdvancedCommerceReason](https://developer.apple.com/documentation/advancedcommerceapi/reason)
    pub reason: AdvancedCommerceReason,

    /// An offer for the item.
    ///
    /// [AdvancedCommerceOffer](https://developer.apple.com/documentation/advancedcommerceapi/offer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer: Option<AdvancedCommerceOffer>,

    /// The prorated price for the item.
    ///
    /// [ProratedPrice](https://developer.apple.com/documentation/advancedcommerceapi/proratedprice)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorated_price: Option<i64>,
}

impl AdvancedCommerceSubscriptionModifyChangeItem {
    pub fn new(
        sku: String,
        current_sku: String,
        description: String,
        display_name: String,
        effective: AdvancedCommerceEffective,
        price: i64,
        reason: AdvancedCommerceReason,
    ) -> Self {
        Self {
            sku,
            current_sku,
            description,
            display_name,
            effective,
            price,
            reason,
            offer: None,
            prorated_price: None,
        }
    }

    pub fn with_offer(mut self, offer: AdvancedCommerceOffer) -> Self {
        self.offer = Some(offer);
        self
    }

    pub fn with_prorated_price(mut self, prorated_price: i64) -> Self {
        self.prorated_price = Some(prorated_price);
        self
    }
}
