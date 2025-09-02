use serde::{Deserialize, Serialize};
use crate::primitives::advanced_commerce::descriptors::Descriptors;
use crate::primitives::advanced_commerce::period::Period;
use crate::primitives::advanced_commerce_transaction_item::AdvancedCommerceTransactionItem;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
/// [AdvancedCommerceTransactionInfo](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercetransactioninfo)
pub struct AdvancedCommerceTransactionInfo {

    /// [descriptors](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercedescriptors)
    pub descriptors: Descriptors,

    /// [estimatedTax](https://developer.apple.com/documentation/appstoreserverapi/advancedcommerceestimatedtax)
    pub estimated_tax: i64,

    /// [items](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercetransactionitems)
    pub items: Vec<AdvancedCommerceTransactionItem>,

    /// [period](https://developer.apple.com/documentation/appstoreserverapi/advancedcommerceperiod)
    pub period: Period,

    /// [requestReferenceId](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercerequestreferenceid)
    pub request_reference_id: String,

    /// [taxCode](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercetaxcode)
    pub tax_code: String,

    /// [taxExclusivePrice](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercetaxexclusiveprice)
    pub tax_exclusive_price: i64,

    /// [taxRate](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercetaxrate)
    pub tax_rate: String,
}