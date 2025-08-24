use serde::{Deserialize, Serialize};

/// The display name and description of a subscription to migrate to.
///
/// [SubscriptionMigrateDescriptors](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmigratedescriptors)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionMigrateDescriptors {
    /// The description of the subscription to migrate to. This string doesn't display to customers.
    ///
    /// [Description](https://developer.apple.com/documentation/advancedcommerceapi/description)
    pub description: String,
    
    /// The display name of the subscription to migrate to. This string displays to customers.
    ///
    /// [Display Name](https://developer.apple.com/documentation/advancedcommerceapi/displayname)
    pub display_name: String,
}

impl SubscriptionMigrateDescriptors {
    pub fn new(description: String, display_name: String) -> Self {
        Self {
            description,
            display_name,
        }
    }
}