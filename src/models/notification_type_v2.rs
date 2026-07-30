use serde::{Deserialize, Serialize};

/// The type that describes the in-app purchase or external purchase event for which the App Store sends the version 2 notification.
///
/// [notificationType](https://developer.apple.com/documentation/appstoreservernotifications/notificationtype)
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
    #[serde(rename = "EXTERNAL_PURCHASE_TOKEN")]
    ExternalPurchaseToken,
    #[serde(rename = "ONE_TIME_CHARGE")]
    OneTimeCharge,
    /// A notification type that indicates you used the Change Subscription Metadata endpoint to change the metadata for a subscription.
    /// This notification only applies to apps that use the Advanced Commerce API.
    ///
    /// [METADATA_UPDATE](https://developer.apple.com/documentation/appstoreservernotifications/notificationtype)
    #[serde(rename = "METADATA_UPDATE")]
    MetadataUpdate,
    /// A notification type that indicates you used the Migrate a Subscription to Advanced Commerce API endpoint.
    /// This notification only applies to apps that use the Advanced Commerce API.
    ///
    /// [MIGRATION](https://developer.apple.com/documentation/appstoreservernotifications/notificationtype)
    #[serde(rename = "MIGRATION")]
    Migration,
    /// A notification type that indicates that you called the Change Subscription Price endpoint.
    /// This notification only applies to apps that use the Advanced Commerce API.
    ///
    /// [PRICE_CHANGE](https://developer.apple.com/documentation/appstoreservernotifications/notificationtype)
    #[serde(rename = "PRICE_CHANGE")]
    PriceChange,
    /// A notification type that indicates the parent or guardian has withdrawn consent for a child's app usage.
    ///
    /// [RESCIND_CONSENT](https://developer.apple.com/documentation/appstoreservernotifications/notificationtype)
    #[serde(rename = "RESCIND_CONSENT")]
    RescindConsent,
}
