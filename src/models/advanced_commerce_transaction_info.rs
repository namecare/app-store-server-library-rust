use serde::{Deserialize, Serialize};

use crate::models::advanced_commerce_descriptors::AdvancedCommerceDescriptors;
use crate::models::advanced_commerce_period::AdvancedCommercePeriod;
use crate::models::advanced_commerce_transaction_item::AdvancedCommerceTransactionItem;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
/// [AdvancedCommerceTransactionInfo](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercetransactioninfo)
pub struct AdvancedCommerceTransactionInfo {
    /// [descriptors](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercedescriptors)
    pub descriptors: Option<AdvancedCommerceDescriptors>,

    /// [estimatedTax](https://developer.apple.com/documentation/appstoreserverapi/advancedcommerceestimatedtax)
    pub estimated_tax: Option<i64>,

    /// [items](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercetransactionitems)
    pub items: Option<Vec<AdvancedCommerceTransactionItem>>,

    /// [period](https://developer.apple.com/documentation/appstoreserverapi/advancedcommerceperiod)
    pub period: Option<AdvancedCommercePeriod>,

    /// [requestReferenceId](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercerequestreferenceid)
    pub request_reference_id: Option<String>,

    /// [taxCode](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercetaxcode)
    pub tax_code: Option<String>,

    /// [taxExclusivePrice](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercetaxexclusiveprice)
    pub tax_exclusive_price: Option<i64>,

    /// [taxRate](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercetaxrate)
    pub tax_rate: Option<String>,
}
