use crate::primitives::advanced_commerce::offer_reason::OfferReason;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;

/// An offer for Advanced Commerce requests.
///
/// [Advanced Commerce API Documentation](https://developer.apple.com/documentation/advancedcommerceapi)
#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RequestOffer {
    /// The unique identifier for the offer.
    ///
    /// [Offer ID](https://developer.apple.com/documentation/advancedcommerceapi/offerid)
    pub offer_id: String,
    
    /// The reason for the offer.
    ///
    /// [Offer Reason](https://developer.apple.com/documentation/advancedcommerceapi/offerreason)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_reason: Option<OfferReason>,
    
    /// The discount percentage for the offer.
    ///
    /// [Discount Percentage](https://developer.apple.com/documentation/advancedcommerceapi/discountpercentage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_percentage: Option<i32>,
    
    /// The date until which the offer is valid.
    ///
    /// [Valid Until](https://developer.apple.com/documentation/advancedcommerceapi/validuntil)
    #[serde(rename = "validUntil")]
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<DateTime<Utc>>,
}

impl RequestOffer {
    pub fn new(offer_id: String) -> Self {
        Self {
            offer_id,
            offer_reason: None,
            discount_percentage: None,
            valid_until: None,
        }
    }
    
    pub fn with_offer_reason(mut self, reason: OfferReason) -> Self {
        self.offer_reason = Some(reason);
        self
    }
    
    pub fn with_discount_percentage(mut self, percentage: i32) -> Self {
        self.discount_percentage = Some(percentage);
        self
    }
    
    pub fn with_valid_until(mut self, valid_until: DateTime<Utc>) -> Self {
        self.valid_until = Some(valid_until);
        self
    }
}