use serde::{Deserialize, Serialize};

/// The payment mode for a discount offer on an In-App Purchase.
///
/// [offerDiscountType](https://developer.apple.com/documentation/appstoreserverapi/offerdiscounttype)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OfferDiscountType {
    #[serde(rename = "FREE_TRIAL")]
    FreeTrial,
    #[serde(rename = "PAY_AS_YOU_GO")]
    PayAsYouGo,
    #[serde(rename = "PAY_UP_FRONT")]
    PayUpFront,
    #[serde(rename = "ONE_TIME")]
    OneTime,
}
