use crate::primitives::advanced_commerce::effective::Effective;
use serde::{Deserialize, Serialize};

/// Descriptors for the metadata changes of a subscription.
///
/// [SubscriptionChangeMetadataDescriptors](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionchangemetadatadescriptors)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionChangeMetadataDescriptors {
    /// The new description for the subscription.
    ///
    /// [Description](https://developer.apple.com/documentation/advancedcommerceapi/description)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// The new display name for the subscription.
    ///
    /// [Display Name](https://developer.apple.com/documentation/advancedcommerceapi/displayname)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    
    /// The string that determines when the metadata change goes into effect.
    ///
    /// [Effective](https://developer.apple.com/documentation/advancedcommerceapi/effective)
    pub effective: Effective,
}

impl SubscriptionChangeMetadataDescriptors {
    pub fn new(effective: Effective) -> Self {
        Self {
            description: None,
            display_name: None,
            effective,
        }
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