use crate::primitives::advanced_commerce::validation_utils::{validate_sku, ValidationError};
use serde::{Deserialize, Serialize};

/// An item for Advanced Commerce subscription price changes.
///
/// [SubscriptionPriceChangeItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionpricechangeitem)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionPriceChangeItem {
    /// The SKU identifier for the item.
    ///
    /// [SKU](https://developer.apple.com/documentation/advancedcommerceapi/sku)
    #[serde(rename = "SKU")]
    pub sku: String,

    /// The product identifiers of dependent SKUs in a subscription price change.
    /// Each dependentSKU value is a string with a maximum length of 128 characters.
    ///
    /// [dependentSKU](https://developer.apple.com/documentation/advancedcommerceapi/dependentsku)
    #[serde(rename = "dependentSKUs", skip_serializing_if = "Option::is_none")]
    pub dependent_skus: Option<Vec<String>>,

    /// The new price in milliunits.
    ///
    /// [Price](https://developer.apple.com/documentation/advancedcommerceapi/price)
    pub price: i64,
}

impl SubscriptionPriceChangeItem {
    /// Creates a new `SubscriptionPriceChangeItem`, validating the SKU and each dependent SKU.
    pub fn new(
        sku: String,
        price: i64,
        dependent_skus: Option<Vec<String>>,
    ) -> Result<Self, ValidationError> {
        let sku = validate_sku(&sku)?;
        if let Some(ref skus) = dependent_skus {
            for dependent_sku in skus {
                validate_sku(dependent_sku)?;
            }
        }
        Ok(Self {
            sku,
            dependent_skus,
            price,
        })
    }
}