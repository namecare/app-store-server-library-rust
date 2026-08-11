use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;

use crate::models::advanced_commerce_offer::AdvancedCommerceOffer;
use crate::models::advanced_commerce_refund::AdvancedCommerceRefund;

#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
/// [AdvancedCommerceTransactionItem](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercetransactionitem)
pub struct AdvancedCommerceTransactionItem {
    /// The SKU of the item.
    ///
    /// [SKU](https://developer.apple.com/documentation/advancedcommerceapi/sku)
    #[serde(rename = "SKU")]
    pub sku: Option<String>,

    /// The new description for the item.
    ///
    /// [Description](https://developer.apple.com/documentation/advancedcommerceapi/description)
    pub description: Option<String>,

    /// The display name for the item.
    ///
    /// [Display Name](https://developer.apple.com/documentation/advancedcommerceapi/displayname)
    pub display_name: Option<String>,

    /// An offer for the item.
    ///
    /// [AdvancedCommerceOffer](https://developer.apple.com/documentation/advancedcommerceapi/offer)
    pub offer: Option<AdvancedCommerceOffer>,

    /// The price in milliunits.
    ///
    /// [Price](https://developer.apple.com/documentation/advancedcommerceapi/price)
    pub price: Option<i64>,

    pub refunds: Option<Vec<AdvancedCommerceRefund>>,

    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub revocation_date: Option<DateTime<Utc>>,
}
