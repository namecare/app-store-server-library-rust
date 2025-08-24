use serde::{Deserialize, Serialize};

/// The reason for the offer.
///
/// [Offer](https://developer.apple.com/documentation/advancedcommerceapi/offer)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OfferReason {
    Acquisition,
    WinBack,
    Retention,
}

impl OfferReason {
    pub fn as_str(&self) -> &str {
        match self {
            OfferReason::Acquisition => "ACQUISITION",
            OfferReason::WinBack => "WIN_BACK",
            OfferReason::Retention => "RETENTION",
        }
    }
}