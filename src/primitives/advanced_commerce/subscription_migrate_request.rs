use crate::primitives::advanced_commerce::subscription_migrate_descriptors::SubscriptionMigrateDescriptors;
use crate::primitives::advanced_commerce::request_info::RequestInfo;
use crate::primitives::advanced_commerce::subscription_migrate_item::SubscriptionMigrateItem;
use crate::primitives::advanced_commerce::subscription_migrate_renewal_item::SubscriptionMigrateRenewalItem;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The subscription details you provide to migrate a subscription from In-App Purchase to the Advanced Commerce API, such as descriptors, items, storefront, and more.
///
/// [SubscriptionMigrateRequest](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmigraterequest)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionMigrateRequest {
    /// The metadata to include in server requests.
    ///
    /// [requestInfo](https://developer.apple.com/documentation/advancedcommerceapi/requestinfo)
    pub request_info: RequestInfo,

    /// The descriptors for the subscription migration request
    ///
    /// [SubscriptionMigrateDescriptors](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmigratedescriptors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<SubscriptionMigrateDescriptors>,

    /// An array of one or more SKUs, along with descriptions and display names, that are included in the subscription.
    ///
    /// [SubscriptionMigrateItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmigrateitem)
    pub items: Vec<SubscriptionMigrateItem>,

    /// The renewal items for the subscription migration request
    ///
    /// [SubscriptionMigrateRenewalItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmigraterenewalitem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_items: Option<Vec<SubscriptionMigrateRenewalItem>>,

    /// The storefront for the subscription migration request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storefront: Option<String>,

    /// Your generic product ID for an auto-renewable subscription. You configure this product ID in App Store Connect during setup.
    pub target_product_id: String,

    /// The tax code for the subscription migration request
    pub tax_code: String,
}

impl SubscriptionMigrateRequest {
    pub fn new(request_reference_id: Uuid, items: Vec<SubscriptionMigrateItem>, target_product_id: String, tax_code: String) -> Self {
        Self {
            request_info: RequestInfo::new(request_reference_id),
            descriptors: None,
            items,
            renewal_items: None,
            storefront: None,
            target_product_id,
            tax_code,
        }
    }

    pub fn with_descriptors(mut self, descriptors: SubscriptionMigrateDescriptors) -> Self {
        self.descriptors = Some(descriptors);
        self
    }

    pub fn with_renewal_items(mut self, renewal_items: Vec<SubscriptionMigrateRenewalItem>) -> Self {
        self.renewal_items = Some(renewal_items);
        self
    }

    pub fn with_storefront(mut self, storefront: String) -> Self {
        self.storefront = Some(storefront);
        self
    }

    pub fn with_request_info(mut self, request_info: RequestInfo) -> Self {
        self.request_info = request_info;
        self
    }
}