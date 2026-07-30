use crate::models::advanced_commerce_descriptors::Descriptors;
use crate::models::advanced_commerce_period::Period;
use crate::models::advanced_commerce_renewal_item::AdvancedCommerceRenewalItem;
use serde::{Deserialize, Serialize};

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
