use crate::primitives::advanced_commerce::subscription_modify_descriptors::SubscriptionModifyDescriptors;
use crate::primitives::advanced_commerce::request_info::RequestInfo;
use crate::primitives::advanced_commerce::subscription_modify_add_item::SubscriptionModifyAddItem;
use crate::primitives::advanced_commerce::subscription_modify_change_item::SubscriptionModifyChangeItem;
use crate::primitives::advanced_commerce::subscription_modify_period_change::SubscriptionModifyPeriodChange;
use crate::primitives::advanced_commerce::subscription_modify_remove_item::SubscriptionModifyRemoveItem;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::primitives::advanced_commerce::request_operation::RequestOperation;
use crate::primitives::advanced_commerce::request_version::RequestVersion;

/// The metadata your app provides to modify an auto-renewable subscription.
///
/// [SubscriptionModifyInAppRequest](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmodifyinapprequest)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionModifyInAppRequest {
    /// The operation type for this request.
    pub operation: RequestOperation,

    /// The version of this request.
    pub version: RequestVersion,

    /// The currency of the price of the product.
    ///
    /// [currency](https://developer.apple.com/documentation/advancedcommerceapi/currency)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// The display name and description of a subscription product.
    ///
    /// [Descriptors](https://developer.apple.com/documentation/advancedcommerceapi/descriptors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<SubscriptionModifyDescriptors>,

    /// Items to add to the subscription.
    ///
    /// [AddItems](https://developer.apple.com/documentation/advancedcommerceapi/additems)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_items: Option<Vec<SubscriptionModifyAddItem>>,

    /// Items to change in the subscription.
    ///
    /// [ChangeItems](https://developer.apple.com/documentation/advancedcommerceapi/changeitems)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_items: Option<Vec<SubscriptionModifyChangeItem>>,

    /// Items to remove from the subscription.
    ///
    /// [RemoveItems](https://developer.apple.com/documentation/advancedcommerceapi/removeitems)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_items: Option<Vec<SubscriptionModifyRemoveItem>>,

    /// Period change for the subscription.
    ///
    /// [PeriodChange](https://developer.apple.com/documentation/advancedcommerceapi/periodchange)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_change: Option<SubscriptionModifyPeriodChange>,

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
    /// [taxCode](https://developer.apple.com/documentation/advancedcommerceapi/taxcode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    /// The original transaction ID of the subscription.
    ///
    /// [transactionId](https://developer.apple.com/documentation/advancedcommerceapi/transactionid)
    pub transaction_id: String,

    /// Whether to retain the billing cycle.
    ///
    /// [retainBillingCycle](https://developer.apple.com/documentation/advancedcommerceapi/retainbillingcycle)
    pub retain_billing_cycle: bool,
}

impl SubscriptionModifyInAppRequest {
    pub fn new(request_reference_id: Uuid, transaction_id: String, retain_billing_cycle: bool) -> Self {
        Self {
            operation: RequestOperation::ModifySubscription,
            version: RequestVersion::V1,
            currency: None,
            descriptors: None,
            add_items: None,
            change_items: None,
            remove_items: None,
            period_change: None,
            request_info: RequestInfo::new(request_reference_id),
            storefront: None,
            tax_code: None,
            transaction_id,
            retain_billing_cycle,
        }
    }

    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn with_descriptors(mut self, descriptors: SubscriptionModifyDescriptors) -> Self {
        self.descriptors = Some(descriptors);
        self
    }

    pub fn with_add_items(mut self, add_items: Vec<SubscriptionModifyAddItem>) -> Self {
        self.add_items = Some(add_items);
        self
    }

    pub fn with_change_items(mut self, change_items: Vec<SubscriptionModifyChangeItem>) -> Self {
        self.change_items = Some(change_items);
        self
    }

    pub fn with_remove_items(mut self, remove_items: Vec<SubscriptionModifyRemoveItem>) -> Self {
        self.remove_items = Some(remove_items);
        self
    }

    pub fn with_period_change(mut self, period_change: SubscriptionModifyPeriodChange) -> Self {
        self.period_change = Some(period_change);
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

    pub fn with_request_info(mut self, request_info: RequestInfo) -> Self {
        self.request_info = request_info;
        self
    }
}