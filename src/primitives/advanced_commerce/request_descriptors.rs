use serde::{Deserialize, Serialize};

/// The display name and description of a subscription product.
///
/// [RequestDescriptors](https://developer.apple.com/documentation/advancedcommerceapi)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RequestDescriptors {
    /// A string you provide that describes the SKU.
    ///
    /// [description](https://developer.apple.com/documentation/advancedcommerceapi/description)
    pub description: String,
    
    /// A string with a product name that you can localize and is suitable for display to customers.
    ///
    /// [displayName](https://developer.apple.com/documentation/advancedcommerceapi/displayname)
    pub display_name: String,
}

impl RequestDescriptors {
    pub fn new(description: String, display_name: String) -> Self {
        Self {
            description,
            display_name,
        }
    }
}