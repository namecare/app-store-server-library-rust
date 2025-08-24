use crate::primitives::advanced_commerce::descriptors::Descriptors;
use crate::primitives::advanced_commerce::period::Period;
use crate::primitives::advanced_commerce::request_info::RequestInfo;
use crate::primitives::advanced_commerce::subscription_create_item::SubscriptionCreateItem;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::primitives::advanced_commerce::request_operation::RequestOperation;
use crate::primitives::advanced_commerce::request_version::RequestVersion;

/// The metadata your app provides when a customer purchases an auto-renewable subscription.
///
/// [SubscriptionCreateRequest](https://developer.apple.com/documentation/advancedcommerceapi/subscriptioncreaterequest)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionCreateRequest {
    /// The operation type for this request.
    /// Value: CREATE_SUBSCRIPTION
    pub operation: RequestOperation,

    /// The version of this request.
    pub version: RequestVersion,

    /// The currency of the price of the product.
    ///
    /// [currency](https://developer.apple.com/documentation/advancedcommerceapi/currency)
    pub currency: String,

    /// The display name and description of a subscription product.
    ///
    /// [Descriptors](https://developer.apple.com/documentation/advancedcommerceapi/descriptors)
    pub descriptors: Descriptors,

    /// The details of the subscription product for purchase.
    ///
    /// [SubscriptionCreateItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptioncreateitem)
    pub items: Vec<SubscriptionCreateItem>,

    /// The duration of a single cycle of an auto-renewable subscription.
    ///
    /// [period](https://developer.apple.com/documentation/advancedcommerceapi/period)
    pub period: Period,

    /// The identifier of a previous transaction for the subscription.
    ///
    /// [transactionId](https://developer.apple.com/documentation/advancedcommerceapi/transactionid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_transaction_id: Option<String>,

    /// The metadata to include in server requests.
    ///
    /// [requestInfo](https://developer.apple.com/documentation/advancedcommerceapi/requestinfo)
    pub request_info: RequestInfo,

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

impl SubscriptionCreateRequest {
    pub fn new(
        currency: String,
        descriptors: Descriptors,
        items: Vec<SubscriptionCreateItem>,
        period: Period,
        tax_code: String,
        request_reference_id: Uuid,
    ) -> Self {
        Self {
            operation: RequestOperation::CreateSubscription,
            version: RequestVersion::V1,
            currency,
            descriptors,
            items,
            period,
            previous_transaction_id: None,
            request_info: RequestInfo::new(request_reference_id),
            storefront: None,
            tax_code,
        }
    }

    pub fn with_previous_transaction_id(mut self, previous_transaction_id: String) -> Self {
        self.previous_transaction_id = Some(previous_transaction_id);
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