use serde::{Deserialize, Serialize};

/// The relationship of the user with the family-shared purchase to which they have access.
///
/// [inAppOwnershipType](https://developer.apple.com/documentation/appstoreserverapi/inappownershiptype)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum InAppOwnershipType {
    #[serde(rename = "FAMILY_SHARED")]
    FamilyShared,
    #[serde(rename = "PURCHASED")]
    Purchased,
}
