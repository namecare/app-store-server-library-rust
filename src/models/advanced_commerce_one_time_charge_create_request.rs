use crate::models::advanced_commerce_in_app_request::AdvancedCommerceInAppRequest;
use crate::models::advanced_commerce_in_app_request_operation::AdvancedCommerceInAppRequestOperation;
use crate::models::advanced_commerce_one_time_charge_item::AdvancedCommerceOneTimeChargeItem;
use crate::models::advanced_commerce_request_info::AdvancedCommerceRequestInfo;
use serde::{Deserialize, Serialize};
use crate::models::advanced_commerce_in_app_request_version::AdvancedCommerceInAppRequestVersion;

/// The request data your app provides when a customer purchases a one-time-charge product.
///
/// [AdvancedCommerceOneTimeChargeCreateRequest](https://developer.apple.com/documentation/advancedcommerceapi/onetimechargecreaterequest)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceOneTimeChargeCreateRequest {
    /// The operation type for this request.
    /// Value: CREATE_ONE_TIME_CHARGE
    #[serde(default = "default_operation")]
    pub operation: AdvancedCommerceInAppRequestOperation,

    /// The version of this request.
    #[serde(default = "default_version")]
    pub version: AdvancedCommerceInAppRequestVersion,

    /// The metadata to include in server requests.
    ///
    /// [requestInfo](https://developer.apple.com/documentation/advancedcommerceapi/requestinfo)
    pub request_info: AdvancedCommerceRequestInfo,

    /// The currency of the price of the product.
    ///
    /// [currency](https://developer.apple.com/documentation/advancedcommerceapi/currency)
    pub currency: String,

    /// The details of the product for purchase.
    ///
    /// [AdvancedCommerceOneTimeChargeItem](https://developer.apple.com/documentation/advancedcommerceapi/onetimechargeitem)
    pub item: AdvancedCommerceOneTimeChargeItem,

    /// The storefront for the transaction.
    ///
    /// [storefront](https://developer.apple.com/documentation/advancedcommerceapi/storefront)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storefront: Option<String>,

    /// The tax code for this product.
    ///
    /// [taxCode](https://developer.apple.com/documentation/advancedcommerceapi/version)
    pub tax_code: String,
}

impl AdvancedCommerceInAppRequest for AdvancedCommerceOneTimeChargeCreateRequest {}

/// Apple fixes this request's operation; it is emitted on encode and defaulted on decode,
/// matching the Swift library where `operation` is a computed constant that is never decoded.
fn default_operation() -> AdvancedCommerceInAppRequestOperation {
    AdvancedCommerceInAppRequestOperation::CreateOneTimeCharge
}

/// Apple fixes this request's version; see [`default_operation`].
fn default_version() -> AdvancedCommerceInAppRequestVersion {
    AdvancedCommerceInAppRequestVersion::V1
}
