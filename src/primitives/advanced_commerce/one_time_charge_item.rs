use serde::{Deserialize, Serialize};

/// The details of a one-time charge product, including its display name, price, SKU, and metadata.
///
/// [OneTimeChargeItem](https://developer.apple.com/documentation/advancedcommerceapi/onetimechargeitem)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OneTimeChargeItem {
    /// The stock keeping unit (SKU) for the product.
    #[serde(rename = "SKU")]
    pub sku: String,
    
    /// The description of the product.
    pub description: String,
    
    /// The display name for the product.
    pub display_name: String,
    
    /// The price, in milliunits of the currency, of the one-time charge product.
    ///
    /// [Price](https://developer.apple.com/documentation/advancedcommerceapi/price)
    pub price: i64,
}

impl OneTimeChargeItem {
    pub fn new(sku: String, description: String, display_name: String, price: i64) -> Self {
        Self {
            sku,
            description,
            display_name,
            price,
        }
    }
}