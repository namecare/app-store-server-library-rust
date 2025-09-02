use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::primitives::advanced_commerce::offer::Offer;
use crate::primitives::advanced_commerce::refund::Refund;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde_with::serde_as]
#[serde(rename_all = "camelCase")]
/// [AdvancedCommerceTransactionItem](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercetransactionitem)
pub struct AdvancedCommerceTransactionItem {
    /// The SKU of the item.
    ///
    /// [SKU](https://developer.apple.com/documentation/advancedcommerceapi/sku)
    #[serde(rename = "SKU")]
    pub sku: String,

    /// The new description for the item.
    ///
    /// [Description](https://developer.apple.com/documentation/advancedcommerceapi/description)
    pub description: String,

    /// The display name for the item.
    ///
    /// [Display Name](https://developer.apple.com/documentation/advancedcommerceapi/displayname)
    pub display_name: String,

    /// An offer for the item.
    ///
    /// [Offer](https://developer.apple.com/documentation/advancedcommerceapi/offer)
    pub offer: Offer,

    /// The price in milliunits.
    ///
    /// [Price](https://developer.apple.com/documentation/advancedcommerceapi/price)
    pub price: i64,

    pub refunds: Vec<Refund>,

    #[serde_as(as = "TimestampMilliSeconds<String, Flexible>")]
    pub revocation_date: DateTime<Utc>,
}
