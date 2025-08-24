use crate::primitives::advanced_commerce::one_time_charge_item::OneTimeChargeItem;
use crate::primitives::advanced_commerce::request_info::RequestInfo;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::primitives::advanced_commerce::request_operation::RequestOperation;
use crate::primitives::advanced_commerce::request_version::RequestVersion;

/// The request data your app provides when a customer purchases a one-time-charge product.
///
/// [OneTimeChargeCreateRequest](https://developer.apple.com/documentation/advancedcommerceapi/onetimechargecreaterequest)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OneTimeChargeCreateRequest {
    /// The operation type for this request.
    /// Value: CREATE_ONE_TIME_CHARGE
    pub operation: RequestOperation,
    
    /// The version of this request.
    pub version: RequestVersion,
    
    /// The metadata to include in server requests.
    ///
    /// [requestInfo](https://developer.apple.com/documentation/advancedcommerceapi/requestinfo)
    pub request_info: RequestInfo,
    
    /// The currency of the price of the product.
    ///
    /// [currency](https://developer.apple.com/documentation/advancedcommerceapi/currency)
    pub currency: String,
    
    /// The details of the product for purchase.
    ///
    /// [OneTimeChargeItem](https://developer.apple.com/documentation/advancedcommerceapi/onetimechargeitem)
    pub item: OneTimeChargeItem,
    
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

impl OneTimeChargeCreateRequest {
    pub fn new(
        currency: String,
        item: OneTimeChargeItem,
        tax_code: String,
        request_reference_id: Uuid,
    ) -> Self {
        Self {
            operation: RequestOperation::CreateOneTimeCharge,
            version: RequestVersion::V1,
            request_info: RequestInfo::new(request_reference_id),
            currency,
            item,
            storefront: None,
            tax_code,
        }
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