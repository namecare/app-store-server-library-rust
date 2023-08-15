use serde::{Deserialize, Serialize};

/// A notification type value that App Store Server Notifications V2 uses.
///
/// [notificationType](https://developer.apple.com/documentation/appstoreserverapi/notificationtype)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum NotificationTypeV2 {
    #[serde(rename = "SUBSCRIBED")]
    Subscribed,
    #[serde(rename = "DID_CHANGE_RENEWAL_PREF")]
    DidChangeRenewalPref,
    #[serde(rename = "DID_CHANGE_RENEWAL_STATUS")]
    DidChangeRenewalStatus,
    #[serde(rename = "OFFER_REDEEMED")]
    OfferRedeemed,
    #[serde(rename = "DID_RENEW")]
    DidRenew,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(rename = "DID_FAIL_TO_RENEW")]
    DidFailToRenew,
    #[serde(rename = "GRACE_PERIOD_EXPIRED")]
    GracePeriodExpired,
    #[serde(rename = "PRICE_INCREASE")]
    PriceIncrease,
    #[serde(rename = "REFUND")]
    Refund,
    #[serde(rename = "REFUND_DECLINED")]
    RefundDeclined,
    #[serde(rename = "CONSUMPTION_REQUEST")]
    ConsumptionRequest,
    #[serde(rename = "RENEWAL_EXTENDED")]
    RenewalExtended,
    #[serde(rename = "REVOKE")]
    Revoke,
    #[serde(rename = "TEST")]
    Test,
    #[serde(rename = "RENEWAL_EXTENSION")]
    RenewalExtension,
    #[serde(rename = "REFUND_REVERSED")]
    RefundReversed,
}
