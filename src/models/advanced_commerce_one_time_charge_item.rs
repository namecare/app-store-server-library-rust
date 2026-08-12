use serde::{Deserialize, Serialize};

/// The details of a one-time charge product, including its display name, price, SKU, and metadata.
///
/// [AdvancedCommerceOneTimeChargeItem](https://developer.apple.com/documentation/advancedcommerceapi/onetimechargeitem)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceOneTimeChargeItem {
    /// The stock keeping unit (SKU) for the product.
    #[serde(rename = "SKU")]
    pub sku: String,

    ///A description of the product that doesn’t display to customers.
    ///
    ///[description](https://developer.apple.com/documentation/advancedcommerceapi/description)
    pub description: String,

    ///The product name, suitable for display to customers.
    ///
    ///[displayName](https://developer.apple.com/documentation/advancedcommerceapi/displayname)
    pub display_name: String,

    /// The price, in milliunits of the currency, of the one-time charge product.
    ///
    /// [Price](https://developer.apple.com/documentation/advancedcommerceapi/price)
    pub price: i64,
}

impl AdvancedCommerceOneTimeChargeItem {
    pub fn new(sku: String, description: String, display_name: String, price: i64) -> Self {
        Self {
            sku,
            description,
            display_name,
            price,
        }
    }
}
