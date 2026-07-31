use crate::models::advanced_commerce_effective::AdvancedCommerceEffective;
use serde::{Deserialize, Serialize};

/// AdvancedCommerceDescriptors for the metadata changes of a subscription.
///
/// [AdvancedCommerceSubscriptionChangeMetadataDescriptors](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionchangemetadatadescriptors)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceSubscriptionChangeMetadataDescriptors {
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
    /// [AdvancedCommerceEffective](https://developer.apple.com/documentation/advancedcommerceapi/effective)
    pub effective: AdvancedCommerceEffective,
}

impl AdvancedCommerceSubscriptionChangeMetadataDescriptors {
    pub fn new(effective: AdvancedCommerceEffective) -> Self {
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
