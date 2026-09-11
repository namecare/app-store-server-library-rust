use serde::{Deserialize, Serialize};
use crate::primitives::advanced_commerce::descriptors::Descriptors;
use crate::primitives::advanced_commerce::period::Period;
use crate::primitives::advanced_commerce_renewal_item::AdvancedCommerceRenewalItem;

/// [AdvancedCommerceRenewalInfo](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercerenewalinfo)
///
/// Every property is optional in Apple's schema.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceRenewalInfo {
    /// advancedCommerceConsistencyToken
    pub consistency_token: Option<String>,

    /// advancedCommerceDescriptors
    pub descriptors: Option<Descriptors>,

    /// advancedCommerceRenewalItems
    pub items: Option<Vec<AdvancedCommerceRenewalItem>>,

    /// advancedCommercePeriod
    pub period: Option<Period>,

    /// advancedCommerceRequestReferenceId
    pub request_reference_id: Option<String>,

    /// advancedCommerceTaxCode
    pub tax_code: Option<String>,
}
