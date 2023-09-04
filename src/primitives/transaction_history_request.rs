use crate::primitives::in_app_ownership_type::InAppOwnershipType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct TransactionHistoryRequest {
    /// An optional start date of the timespan for the transaction history records you’re requesting.
    #[serde(rename = "startDate")]
    pub start_date: Option<DateTime<Utc>>,

    /// An optional end date of the timespan for the transaction history records you’re requesting.
    #[serde(rename = "endDate")]
    pub end_date: Option<DateTime<Utc>>,

    /// An optional filter that indicates the product identifier to include in the transaction history.
    #[serde(rename = "productIds")]
    pub product_ids: Option<Vec<String>>,

    /// An optional filter that indicates the product type to include in the transaction history.
    #[serde(rename = "productTypes")]
    pub product_types: Option<Vec<ProductType>>,

    /// An optional sort order for the transaction history records.
    pub sort: Option<Order>,

    /// An optional filter that indicates the subscription group identifier to include in the transaction history.
    #[serde(rename = "subscriptionGroupIdentifiers")]
    pub subscription_group_identifiers: Option<Vec<String>>,

    /// An optional filter that limits the transaction history by the in-app ownership type.
    #[serde(rename = "inAppOwnershipType")]
    pub in_app_ownership_type: Option<InAppOwnershipType>,

    /// An optional Boolean value that indicates whether the response includes only revoked transactions.
    pub revoked: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum ProductType {
    #[serde(rename = "AUTO_RENEWABLE")]
    AutoRenewable,
    #[serde(rename = "NON_RENEWABLE")]
    NonRenewable,
    #[serde(rename = "CONSUMABLE")]
    Consumable,
    #[serde(rename = "NON_CONSUMABLE")]
    NonConsumable,
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum Order {
    #[serde(rename = "ASCENDING")]
    Ascending,
    #[serde(rename = "DESCENDING")]
    Descending,
}
