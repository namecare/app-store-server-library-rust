use crate::primitives::advanced_commerce::effective::Effective;
use serde::{Deserialize, Serialize};

/// The data your app provides to change the description and display name of an auto-renewable subscription.
///
/// [SubscriptionModifyDescriptors](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmodifydescriptors)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionModifyDescriptors {
    /// The description of the subscription.
    ///
    /// [Description](https://developer.apple.com/documentation/advancedcommerceapi/description)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// The display name of the subscription.
    ///
    /// [Display Name](https://developer.apple.com/documentation/advancedcommerceapi/displayname)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    
    /// When the modification takes effect.
    ///
    /// [Effective](https://developer.apple.com/documentation/advancedcommerceapi/effective)
    pub effective: Effective,
}

impl SubscriptionModifyDescriptors {
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