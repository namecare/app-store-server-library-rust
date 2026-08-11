use serde::{Deserialize, Serialize};

use crate::models::advanced_commerce_descriptors::AdvancedCommerceDescriptors;
use crate::models::advanced_commerce_period::AdvancedCommercePeriod;
use crate::models::advanced_commerce_renewal_item::AdvancedCommerceRenewalItem;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceRenewalInfo {
    /// advancedCommerceConsistencyToken
    pub consistency_token: Option<String>,

    /// advancedCommerceDescriptors
    pub descriptors: Option<AdvancedCommerceDescriptors>,

    /// advancedCommerceRenewalItems
    pub items: Option<Vec<AdvancedCommerceRenewalItem>>,

    /// advancedCommercePeriod
    pub period: Option<AdvancedCommercePeriod>,

    /// advancedCommerceRequestReferenceId
    pub request_reference_id: Option<String>,

    /// advancedCommerceTaxCode
    pub tax_code: Option<String>,
}
