use serde::{Deserialize, Serialize};

/// The reason for the offer.
///
/// [AdvancedCommerceOffer](https://developer.apple.com/documentation/advancedcommerceapi/offer)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdvancedCommerceOfferReason {
    Acquisition,
    WinBack,
    Retention,
}

impl AdvancedCommerceOfferReason {
    pub fn as_str(&self) -> &str {
        match self {
            AdvancedCommerceOfferReason::Acquisition => "ACQUISITION",
            AdvancedCommerceOfferReason::WinBack => "WIN_BACK",
            AdvancedCommerceOfferReason::Retention => "RETENTION",
        }
    }
}
