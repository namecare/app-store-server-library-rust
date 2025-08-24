use serde::{Deserialize, Serialize};
use crate::primitives::advanced_commerce::effective::Effective;

/// The metadata to change for an item, specifically its SKU, description, and display name.
///
/// [SubscriptionChangeMetadataItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionchangemetadataitem)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionChangeMetadataItem {
    /// The new SKU of the item.
    ///
    /// [SKU](https://developer.apple.com/documentation/advancedcommerceapi/sku)
    #[serde(skip_serializing_if = "Option::is_none", rename = "SKU")]
    pub sku: Option<String>,
    
    /// The original SKU of the item.
    ///
    /// [currentSKU](https://developer.apple.com/documentation/advancedcommerceapi/sku)
    #[serde(rename = "currentSKU")]
    pub current_sku: String,
    
    /// The new description for the item.
    ///
    /// [Description](https://developer.apple.com/documentation/advancedcommerceapi/description)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// The new display name for the item.
    ///
    /// [Display Name](https://developer.apple.com/documentation/advancedcommerceapi/displayname)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    
    /// The string that determines when the metadata change goes into effect.
    ///
    /// [Effective](https://developer.apple.com/documentation/advancedcommerceapi/effective)
    pub effective: Effective,
}

impl SubscriptionChangeMetadataItem {
    pub fn new(
        current_sku: String,
        effective: Effective,
    ) -> Self {
        Self {
            sku: None,
            current_sku,
            description: None,
            display_name: None,
            effective,
        }
    }
    
    pub fn with_sku(mut self, sku: String) -> Self {
        self.sku = Some(sku);
        self
    }
    
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    
    pub fn with_display_name(mut self, display_name: String) -> Self {
        self.display_name = Some(display_name);
        self
    }
}