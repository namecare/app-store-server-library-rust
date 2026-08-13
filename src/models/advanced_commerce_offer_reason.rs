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

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    #[serde(untagged)]
    NotSupported(String),
}

impl AdvancedCommerceOfferReason {
    pub fn as_str(&self) -> &str {
        match self {
            AdvancedCommerceOfferReason::Acquisition => "ACQUISITION",
            AdvancedCommerceOfferReason::WinBack => "WIN_BACK",
            AdvancedCommerceOfferReason::Retention => "RETENTION",
            AdvancedCommerceOfferReason::NotSupported(value) => value.as_str(),
        }
    }
}
