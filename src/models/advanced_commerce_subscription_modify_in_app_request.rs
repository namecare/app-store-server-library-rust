use serde::{Deserialize, Serialize};

use crate::models::advanced_commerce_in_app_request::AdvancedCommerceInAppRequest;
use crate::models::advanced_commerce_in_app_request_operation::AdvancedCommerceInAppRequestOperation;
use crate::models::advanced_commerce_in_app_request_version::AdvancedCommerceInAppRequestVersion;
use crate::models::advanced_commerce_request_info::AdvancedCommerceRequestInfo;
use crate::models::advanced_commerce_subscription_modify_add_item::AdvancedCommerceSubscriptionModifyAddItem;
use crate::models::advanced_commerce_subscription_modify_change_item::AdvancedCommerceSubscriptionModifyChangeItem;
use crate::models::advanced_commerce_subscription_modify_descriptors::AdvancedCommerceSubscriptionModifyDescriptors;
use crate::models::advanced_commerce_subscription_modify_period_change::AdvancedCommerceSubscriptionModifyPeriodChange;
use crate::models::advanced_commerce_subscription_modify_remove_item::AdvancedCommerceSubscriptionModifyRemoveItem;
use crate::models::helper_validation_utils::{validate_items, ValidationError};

/// The metadata your app provides to modify an auto-renewable subscription.
///
/// [AdvancedCommerceSubscriptionModifyInAppRequest](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmodifyinapprequest)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceSubscriptionModifyInAppRequest {
    /// The operation type for this request.
    #[serde(default = "default_operation")]
    pub operation: AdvancedCommerceInAppRequestOperation,

    /// The version of this request.
    #[serde(default = "default_version")]
    pub version: AdvancedCommerceInAppRequestVersion,

    /// The currency of the price of the product.
    ///
    /// [currency](https://developer.apple.com/documentation/advancedcommerceapi/currency)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// The display name and description of a subscription product.
    ///
    /// [AdvancedCommerceDescriptors](https://developer.apple.com/documentation/advancedcommerceapi/descriptors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<AdvancedCommerceSubscriptionModifyDescriptors>,

    /// Items to add to the subscription.
    ///
    /// [AddItems](https://developer.apple.com/documentation/advancedcommerceapi/additems)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_items: Option<Vec<AdvancedCommerceSubscriptionModifyAddItem>>,

    /// Items to change in the subscription.
    ///
    /// [ChangeItems](https://developer.apple.com/documentation/advancedcommerceapi/changeitems)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_items: Option<Vec<AdvancedCommerceSubscriptionModifyChangeItem>>,

    /// Items to remove from the subscription.
    ///
    /// [RemoveItems](https://developer.apple.com/documentation/advancedcommerceapi/removeitems)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_items: Option<Vec<AdvancedCommerceSubscriptionModifyRemoveItem>>,

    /// AdvancedCommercePeriod change for the subscription.
    ///
    /// [PeriodChange](https://developer.apple.com/documentation/advancedcommerceapi/periodchange)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_change: Option<AdvancedCommerceSubscriptionModifyPeriodChange>,

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

impl AdvancedCommerceSubscriptionModifyInAppRequest {
    pub fn new(request_info: AdvancedCommerceRequestInfo, transaction_id: String, retain_billing_cycle: bool) -> Self {
        Self {
            operation: default_operation(),
            version: default_version(),
            currency: None,
            descriptors: None,
            add_items: None,
            change_items: None,
            remove_items: None,
            period_change: None,
            request_info,
            storefront: None,
            tax_code: None,
            transaction_id,
            retain_billing_cycle,
        }
    }

    pub fn with_add_items(
        mut self,
        add_items: Vec<AdvancedCommerceSubscriptionModifyAddItem>,
    ) -> Result<Self, ValidationError> {
        self.add_items = Some(validate_items(add_items)?);
        Ok(self)
    }

    pub fn with_change_items(
        mut self,
        change_items: Vec<AdvancedCommerceSubscriptionModifyChangeItem>,
    ) -> Result<Self, ValidationError> {
        self.change_items = Some(validate_items(change_items)?);
        Ok(self)
    }

    pub fn with_remove_items(
        mut self,
        remove_items: Vec<AdvancedCommerceSubscriptionModifyRemoveItem>,
    ) -> Result<Self, ValidationError> {
        self.remove_items = Some(validate_items(remove_items)?);
        Ok(self)
    }

    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn with_descriptors(mut self, descriptors: AdvancedCommerceSubscriptionModifyDescriptors) -> Self {
        self.descriptors = Some(descriptors);
        self
    }

    pub fn with_period_change(mut self, period_change: AdvancedCommerceSubscriptionModifyPeriodChange) -> Self {
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
}

impl AdvancedCommerceInAppRequest for AdvancedCommerceSubscriptionModifyInAppRequest {}

/// Apple fixes this request's operation; it is emitted on encode and defaulted on decode,
/// matching the Swift library where `operation` is a computed constant that is never decoded.
fn default_operation() -> AdvancedCommerceInAppRequestOperation {
    AdvancedCommerceInAppRequestOperation::ModifySubscription
}

/// Apple fixes this request's version; see [`default_operation`].
fn default_version() -> AdvancedCommerceInAppRequestVersion {
    AdvancedCommerceInAppRequestVersion::V1
}
