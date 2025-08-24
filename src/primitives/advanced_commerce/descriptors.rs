use serde::{Deserialize, Serialize};

/// The description and display name of the subscription to migrate to that you manage.
///
/// [Descriptors](https://developer.apple.com/documentation/advancedcommerceapi/descriptors)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Descriptors {
    /// A string you provide that describes a SKU.
    ///
    /// [Description](https://developer.apple.com/documentation/appstoreserverapi/description)
    pub description: String,

    /// A string with a product name that you can localize and is suitable for display to customers.
    ///
    /// [DisplayName](https://developer.apple.com/documentation/appstoreserverapi/displayname)
    pub display_name: String,
}

impl Descriptors {
    pub fn new(description: String, display_name: String) -> Self {
        Self {
            description,
            display_name,
        }
    }
}