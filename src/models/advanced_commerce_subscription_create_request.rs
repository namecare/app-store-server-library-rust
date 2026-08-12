use serde::{Deserialize, Serialize};

use crate::models::advanced_commerce_descriptors::AdvancedCommerceDescriptors;
use crate::models::advanced_commerce_in_app_request::AdvancedCommerceInAppRequest;
use crate::models::advanced_commerce_in_app_request_operation::AdvancedCommerceInAppRequestOperation;
use crate::models::advanced_commerce_in_app_request_version::AdvancedCommerceInAppRequestVersion;
use crate::models::advanced_commerce_period::AdvancedCommercePeriod;
use crate::models::advanced_commerce_request_info::AdvancedCommerceRequestInfo;
use crate::models::advanced_commerce_subscription_create_item::AdvancedCommerceSubscriptionCreateItem;
use crate::models::helper_validation_utils::{validate_items, ValidationError};

/// The metadata your app provides when a customer purchases an auto-renewable subscription.
///
/// [AdvancedCommerceSubscriptionCreateRequest](https://developer.apple.com/documentation/advancedcommerceapi/subscriptioncreaterequest)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceSubscriptionCreateRequest {
    /// The operation type for this request.
    /// Value: CREATE_SUBSCRIPTION
    #[serde(default = "default_operation")]
    pub operation: AdvancedCommerceInAppRequestOperation,

    /// The version of this request.
    #[serde(default = "default_version")]
    pub version: AdvancedCommerceInAppRequestVersion,

    /// The currency of the price of the product.
    ///
    /// [currency](https://developer.apple.com/documentation/advancedcommerceapi/currency)
    pub currency: String,

    /// The display name and description of a subscription product.
    ///
    /// [AdvancedCommerceDescriptors](https://developer.apple.com/documentation/advancedcommerceapi/descriptors)
    pub descriptors: AdvancedCommerceDescriptors,

    /// The details of the subscription product for purchase.
    ///
    /// [AdvancedCommerceSubscriptionCreateItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptioncreateitem)
    pub items: Vec<AdvancedCommerceSubscriptionCreateItem>,

    /// The duration of a single cycle of an auto-renewable subscription.
    ///
    /// [period](https://developer.apple.com/documentation/advancedcommerceapi/period)
    pub period: AdvancedCommercePeriod,

    /// The identifier of a previous transaction for the subscription.
    ///
    /// [transactionId](https://developer.apple.com/documentation/advancedcommerceapi/transactionid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_transaction_id: Option<String>,

    /// The metadata to include in server requests.
    ///
    /// [requestInfo](https://developer.apple.com/documentation/advancedcommerceapi/requestinfo)
    pub request_info: AdvancedCommerceRequestInfo,

    /// The storefront for the transaction.
    ///
    /// [storefront](https://developer.apple.com/documentation/advancedcommerceapi/onetimechargecreaterequest)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storefront: Option<String>,

    /// The tax code for this product.
    ///
    /// [taxCode](https://developer.apple.com/documentation/advancedcommerceapi/onetimechargecreaterequest)
    pub tax_code: String,
}

impl AdvancedCommerceSubscriptionCreateRequest {
    pub fn new(
        currency: String,
        descriptors: AdvancedCommerceDescriptors,
        items: Vec<AdvancedCommerceSubscriptionCreateItem>,
        period: AdvancedCommercePeriod,
        request_info: AdvancedCommerceRequestInfo,
        tax_code: String,
    ) -> Result<Self, ValidationError> {
        Ok(Self {
            operation: default_operation(),
            version: default_version(),
            currency,
            descriptors,
            items: validate_items(items)?,
            period,
            previous_transaction_id: None,
            request_info,
            storefront: None,
            tax_code,
        })
    }

    pub fn with_previous_transaction_id(mut self, previous_transaction_id: String) -> Self {
        self.previous_transaction_id = Some(previous_transaction_id);
        self
    }

    pub fn with_storefront(mut self, storefront: String) -> Self {
        self.storefront = Some(storefront);
        self
    }
}

impl AdvancedCommerceInAppRequest for AdvancedCommerceSubscriptionCreateRequest {}

/// Apple fixes this request's operation; it is emitted on encode and defaulted on decode,
/// matching the Swift library where `operation` is a computed constant that is never decoded.
fn default_operation() -> AdvancedCommerceInAppRequestOperation {
    AdvancedCommerceInAppRequestOperation::CreateSubscription
}

/// Apple fixes this request's version; see [`default_operation`].
fn default_version() -> AdvancedCommerceInAppRequestVersion {
    AdvancedCommerceInAppRequestVersion::V1
}
