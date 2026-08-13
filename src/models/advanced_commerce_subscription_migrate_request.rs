use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::advanced_commerce_request_info::AdvancedCommerceRequestInfo;
use crate::models::advanced_commerce_subscription_migrate_descriptors::AdvancedCommerceSubscriptionMigrateDescriptors;
use crate::models::advanced_commerce_subscription_migrate_item::AdvancedCommerceSubscriptionMigrateItem;
use crate::models::advanced_commerce_subscription_migrate_renewal_item::AdvancedCommerceSubscriptionMigrateRenewalItem;
use crate::models::helper_validation_utils::{validate_items, ValidationError};

/// The subscription details you provide to migrate a subscription from In-App Purchase to the Advanced Commerce API, such as descriptors, items, storefront, and more.
///
/// [AdvancedCommerceSubscriptionMigrateRequest](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmigraterequest)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceSubscriptionMigrateRequest {
    /// The metadata to include in server requests.
    ///
    /// [requestInfo](https://developer.apple.com/documentation/advancedcommerceapi/requestinfo)
    pub request_info: AdvancedCommerceRequestInfo,

    /// The descriptors for the subscription migration request
    ///
    /// [AdvancedCommerceSubscriptionMigrateDescriptors](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmigratedescriptors)
    pub descriptors: AdvancedCommerceSubscriptionMigrateDescriptors,

    /// An array of one or more SKUs, along with descriptions and display names, that are included in the subscription.
    ///
    /// [AdvancedCommerceSubscriptionMigrateItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmigrateitem)
    pub items: Vec<AdvancedCommerceSubscriptionMigrateItem>,

    /// The renewal items for the subscription migration request
    ///
    /// [AdvancedCommerceSubscriptionMigrateRenewalItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmigraterenewalitem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_items: Option<Vec<AdvancedCommerceSubscriptionMigrateRenewalItem>>,

    /// The storefront for the subscription migration request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storefront: Option<String>,

    /// Your generic product ID for an auto-renewable subscription. You configure this product ID in App Store Connect during setup.
    pub target_product_id: String,

    /// The tax code for the subscription migration request
    pub tax_code: String,
}

impl AdvancedCommerceSubscriptionMigrateRequest {
    pub fn new(
        request_reference_id: Uuid,
        descriptors: AdvancedCommerceSubscriptionMigrateDescriptors,
        items: Vec<AdvancedCommerceSubscriptionMigrateItem>,
        target_product_id: String,
        tax_code: String,
    ) -> Result<Self, ValidationError> {
        Ok(Self {
            request_info: AdvancedCommerceRequestInfo::new(request_reference_id),
            descriptors,
            items: validate_items(items)?,
            renewal_items: None,
            storefront: None,
            target_product_id,
            tax_code,
        })
    }

    pub fn with_descriptors(mut self, descriptors: AdvancedCommerceSubscriptionMigrateDescriptors) -> Self {
        self.descriptors = descriptors;
        self
    }

    pub fn with_renewal_items(
        mut self,
        renewal_items: Vec<AdvancedCommerceSubscriptionMigrateRenewalItem>,
    ) -> Result<Self, ValidationError> {
        self.renewal_items = Some(validate_items(renewal_items)?);
        Ok(self)
    }

    pub fn with_storefront(mut self, storefront: String) -> Self {
        self.storefront = Some(storefront);
        self
    }

    pub fn with_request_info(mut self, request_info: AdvancedCommerceRequestInfo) -> Self {
        self.request_info = request_info;
        self
    }
}
