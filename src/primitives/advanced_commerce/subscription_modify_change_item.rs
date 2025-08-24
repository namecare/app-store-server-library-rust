use serde::{Deserialize, Serialize};
use crate::primitives::advanced_commerce::offer::Offer;
use crate::primitives::advanced_commerce::effective::Effective;

/// The data your app provides to change an item of an auto-renewable subscription.
///
/// [SubscriptionModifyChangeItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmodifychangeitem)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionModifyChangeItem {
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
    /// [Effective](https://developer.apple.com/documentation/advancedcommerceapi/effective)
    pub effective: Effective,

    /// The price in milliunits.
    ///
    /// [Price](https://developer.apple.com/documentation/advancedcommerceapi/price)
    pub price: i64,

    /// The reason for the change.
    /// Possible Values: UPGRADE, DOWNGRADE, APPLY_OFFER
    ///
    /// [Reason](https://developer.apple.com/documentation/advancedcommerceapi/reason)
    pub reason: String,

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

impl SubscriptionModifyChangeItem {
    pub fn new(
        sku: String,
        current_sku: String,
        description: String,
        display_name: String,
        effective: Effective,
        price: i64,
        reason: String,
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

    pub fn with_offer(mut self, offer: Offer) -> Self {
        self.offer = Some(offer);
        self
    }

    pub fn with_prorated_price(mut self, prorated_price: i64) -> Self {
        self.prorated_price = Some(prorated_price);
        self
    }
}