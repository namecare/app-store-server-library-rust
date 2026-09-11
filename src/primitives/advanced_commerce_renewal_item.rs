use serde::{Deserialize, Serialize};
use crate::primitives::advanced_commerce::offer::Offer;

/// [AdvancedCommerceRenewalItem](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercerenewalitem)
///
/// Every property is optional in Apple's schema; `offer` is present only when an offer applies.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceRenewalItem {
    #[serde(rename = "SKU")]
    pub sku: Option<String>,

    pub description: Option<String>,

    pub display_name: Option<String>,

    pub offer: Option<Offer>,

    pub price: Option<i64>,
}
