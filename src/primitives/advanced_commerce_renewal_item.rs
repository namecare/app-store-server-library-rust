use serde::{Deserialize, Serialize};
use crate::primitives::advanced_commerce::offer::Offer;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceRenewalItem {
    #[serde(rename = "SKU")]
    pub sku: String,

    pub description: String,

    pub display_name: String,

    pub offer: Offer,

    pub price: i64,
}