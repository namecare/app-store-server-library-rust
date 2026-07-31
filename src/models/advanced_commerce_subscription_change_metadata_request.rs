use crate::models::advanced_commerce_request_info::AdvancedCommerceRequestInfo;
use crate::models::advanced_commerce_subscription_change_metadata_descriptors::AdvancedCommerceSubscriptionChangeMetadataDescriptors;
use crate::models::advanced_commerce_subscription_change_metadata_item::AdvancedCommerceSubscriptionChangeMetadataItem;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The request data your app provides to change the metadata of an auto-renewable subscription.
///
/// [AdvancedCommerceSubscriptionChangeMetadataRequest](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionchangemetadatarequest)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceSubscriptionChangeMetadataRequest {
    /// The metadata to include in server requests.
    ///
    /// [requestInfo](https://developer.apple.com/documentation/advancedcommerceapi/requestinfo)
    pub request_info: AdvancedCommerceRequestInfo,

    /// The data your app provides to change the descriptors of an auto-renewable subscription.
    ///
    /// [AdvancedCommerceSubscriptionChangeMetadataDescriptors](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionchangemetadatadescriptors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<AdvancedCommerceSubscriptionChangeMetadataDescriptors>,

    /// The list of items to change metadata for in the subscription.
    ///
    /// [AdvancedCommerceSubscriptionChangeMetadataItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionchangemetadatitem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AdvancedCommerceSubscriptionChangeMetadataItem>>,

    /// The storefront for the transaction.
    ///
    /// [storefront](https://developer.apple.com/documentation/advancedcommerceapi/onetimechargecreaterequest)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storefront: Option<String>,

    /// The tax code for this product.
    ///
    /// [TaxCode](https://developer.apple.com/documentation/advancedcommerceapi/taxcode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
}

impl AdvancedCommerceSubscriptionChangeMetadataRequest {
    pub fn new(request_reference_id: Uuid) -> Self {
        Self {
            request_info: AdvancedCommerceRequestInfo::new(request_reference_id),
            descriptors: None,
            items: None,
            storefront: None,
            tax_code: None,
        }
    }

    pub fn with_request_info(mut self, request_info: AdvancedCommerceRequestInfo) -> Self {
        self.request_info = request_info;
        self
    }

    pub fn with_descriptors(mut self, descriptors: AdvancedCommerceSubscriptionChangeMetadataDescriptors) -> Self {
        self.descriptors = Some(descriptors);
        self
    }

    pub fn with_items(mut self, items: Vec<AdvancedCommerceSubscriptionChangeMetadataItem>) -> Self {
        self.items = Some(items);
        self
    }

    pub fn add_item(mut self, item: AdvancedCommerceSubscriptionChangeMetadataItem) -> Self {
        if self.items.is_none() {
            self.items = Some(Vec::new());
        }
        if let Some(ref mut items) = self.items {
            items.push(item);
        }
        self
    }

    pub fn with_storefront(mut self, storefront: String) -> Self {
        self.storefront = Some(storefront);
        self
    }

    pub fn with_tax_code(mut self, tax_code: String) -> Self {
        self.tax_code = Some(tax_code);
        self
    }
}
