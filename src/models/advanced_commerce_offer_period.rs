use serde::{Deserialize, Serialize};

/// The period of the offer.
///
/// [AdvancedCommerceOffer](https://developer.apple.com/documentation/advancedcommerceapi/offer)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum AdvancedCommerceOfferPeriod {
    #[serde(rename = "P3D")]
    P3d,
    #[serde(rename = "P1W")]
    P1w,
    #[serde(rename = "P2W")]
    P2w,
    #[serde(rename = "P1M")]
    P1m,
    #[serde(rename = "P2M")]
    P2m,
    #[serde(rename = "P3M")]
    P3m,
    #[serde(rename = "P6M")]
    P6m,
    #[serde(rename = "P9M")]
    P9m,
    #[serde(rename = "P1Y")]
    P1y,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    #[serde(untagged)]
    NotSupported(String),
}

impl AdvancedCommerceOfferPeriod {
    pub fn as_str(&self) -> &str {
        match self {
            AdvancedCommerceOfferPeriod::P3d => "P3D",
            AdvancedCommerceOfferPeriod::P1w => "P1W",
            AdvancedCommerceOfferPeriod::P2w => "P2W",
            AdvancedCommerceOfferPeriod::P1m => "P1M",
            AdvancedCommerceOfferPeriod::P2m => "P2M",
            AdvancedCommerceOfferPeriod::P3m => "P3M",
            AdvancedCommerceOfferPeriod::P6m => "P6M",
            AdvancedCommerceOfferPeriod::P9m => "P9M",
            AdvancedCommerceOfferPeriod::P1y => "P1Y",
            AdvancedCommerceOfferPeriod::NotSupported(value) => value.as_str(),
        }
    }
}
