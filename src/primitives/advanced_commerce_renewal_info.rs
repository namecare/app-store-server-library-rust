use serde::{Deserialize, Serialize};
use crate::primitives::advanced_commerce::descriptors::Descriptors;
use crate::primitives::advanced_commerce::period::Period;
use crate::primitives::advanced_commerce_renewal_item::AdvancedCommerceRenewalItem;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceRenewalInfo {
    /// advancedCommerceConsistencyToken
    pub consistency_token: String,

    /// advancedCommerceDescriptors
    pub descriptors: Descriptors,

    /// advancedCommerceRenewalItems
    pub items: Vec<AdvancedCommerceRenewalItem>,

    /// advancedCommercePeriod
    pub period: Period,

    /// advancedCommerceRequestReferenceId
    pub request_reference_id: String,

    /// advancedCommerceTaxCode
    pub tax_code: String,
}