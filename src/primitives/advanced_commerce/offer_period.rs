use serde::{Deserialize, Serialize};

/// The period of the offer.
///
/// [Offer](https://developer.apple.com/documentation/advancedcommerceapi/offer)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum OfferPeriod {
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
}

impl OfferPeriod {
    pub fn as_str(&self) -> &str {
        match self {
            OfferPeriod::P3d => "P3D",
            OfferPeriod::P1w => "P1W",
            OfferPeriod::P2w => "P2W",
            OfferPeriod::P1m => "P1M",
            OfferPeriod::P2m => "P2M",
            OfferPeriod::P3m => "P3M",
            OfferPeriod::P6m => "P6M",
            OfferPeriod::P9m => "P9M",
            OfferPeriod::P1y => "P1Y",
        }
    }
}