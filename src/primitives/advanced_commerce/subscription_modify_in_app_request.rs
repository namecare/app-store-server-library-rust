use crate::primitives::advanced_commerce::subscription_modify_descriptors::SubscriptionModifyDescriptors;
use crate::primitives::advanced_commerce::request_info::RequestInfo;
use crate::primitives::advanced_commerce::subscription_modify_add_item::SubscriptionModifyAddItem;
use crate::primitives::advanced_commerce::subscription_modify_change_item::SubscriptionModifyChangeItem;
use crate::primitives::advanced_commerce::subscription_modify_period_change::SubscriptionModifyPeriodChange;
use crate::primitives::advanced_commerce::subscription_modify_remove_item::SubscriptionModifyRemoveItem;
use serde::{Deserialize, Serialize};
use crate::primitives::advanced_commerce::in_app_request::AdvancedCommerceInAppRequest;
use crate::primitives::advanced_commerce::in_app_request_operation::InAppRequestOperation;
use crate::primitives::advanced_commerce::in_app_request_version::InAppRequestVersion;

/// The metadata your app provides to modify an auto-renewable subscription.
///
/// [SubscriptionModifyInAppRequest](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmodifyinapprequest)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionModifyInAppRequest {
    /// The operation type for this request.
    pub operation: InAppRequestOperation,

    /// The version of this request.
    pub version: InAppRequestVersion,

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

impl AdvancedCommerceInAppRequest for SubscriptionModifyInAppRequest {}