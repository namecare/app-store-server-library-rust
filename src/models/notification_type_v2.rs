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
    AdvancedCommerceRefund,
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

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    #[serde(untagged)]
    NotSupported(String),
}

impl NotificationTypeV2 {
    /// The string Apple sends for this notification type, as in `notificationType`.
    pub fn raw_value(&self) -> &str {
        match self {
            NotificationTypeV2::Subscribed => "SUBSCRIBED",
            NotificationTypeV2::DidChangeRenewalPref => "DID_CHANGE_RENEWAL_PREF",
            NotificationTypeV2::DidChangeRenewalStatus => "DID_CHANGE_RENEWAL_STATUS",
            NotificationTypeV2::OfferRedeemed => "OFFER_REDEEMED",
            NotificationTypeV2::DidRenew => "DID_RENEW",
            NotificationTypeV2::Expired => "EXPIRED",
            NotificationTypeV2::DidFailToRenew => "DID_FAIL_TO_RENEW",
            NotificationTypeV2::GracePeriodExpired => "GRACE_PERIOD_EXPIRED",
            NotificationTypeV2::PriceIncrease => "PRICE_INCREASE",
            NotificationTypeV2::AdvancedCommerceRefund => "REFUND",
            NotificationTypeV2::RefundDeclined => "REFUND_DECLINED",
            NotificationTypeV2::ConsumptionRequest => "CONSUMPTION_REQUEST",
            NotificationTypeV2::RenewalExtended => "RENEWAL_EXTENDED",
            NotificationTypeV2::Revoke => "REVOKE",
            NotificationTypeV2::Test => "TEST",
            NotificationTypeV2::RenewalExtension => "RENEWAL_EXTENSION",
            NotificationTypeV2::RefundReversed => "REFUND_REVERSED",
            NotificationTypeV2::ExternalPurchaseToken => "EXTERNAL_PURCHASE_TOKEN",
            NotificationTypeV2::OneTimeCharge => "ONE_TIME_CHARGE",
            NotificationTypeV2::MetadataUpdate => "METADATA_UPDATE",
            NotificationTypeV2::Migration => "MIGRATION",
            NotificationTypeV2::PriceChange => "PRICE_CHANGE",
            NotificationTypeV2::RescindConsent => "RESCIND_CONSENT",
            NotificationTypeV2::NotSupported(value) => value.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value_matches_wire_format() {
        let all = [
            NotificationTypeV2::Subscribed,
            NotificationTypeV2::DidChangeRenewalPref,
            NotificationTypeV2::DidChangeRenewalStatus,
            NotificationTypeV2::OfferRedeemed,
            NotificationTypeV2::DidRenew,
            NotificationTypeV2::Expired,
            NotificationTypeV2::DidFailToRenew,
            NotificationTypeV2::GracePeriodExpired,
            NotificationTypeV2::PriceIncrease,
            NotificationTypeV2::AdvancedCommerceRefund,
            NotificationTypeV2::RefundDeclined,
            NotificationTypeV2::ConsumptionRequest,
            NotificationTypeV2::RenewalExtended,
            NotificationTypeV2::Revoke,
            NotificationTypeV2::Test,
            NotificationTypeV2::RenewalExtension,
            NotificationTypeV2::RefundReversed,
            NotificationTypeV2::ExternalPurchaseToken,
            NotificationTypeV2::OneTimeCharge,
            NotificationTypeV2::MetadataUpdate,
            NotificationTypeV2::Migration,
            NotificationTypeV2::PriceChange,
            NotificationTypeV2::RescindConsent,
            NotificationTypeV2::NotSupported("FUTURE_NOTIFICATION".to_string()),
        ];
        for notification_type in all {
            let json = serde_json::to_value(&notification_type).unwrap();
            assert_eq!(
                json.as_str(),
                Some(notification_type.raw_value()),
                "{:?}",
                notification_type
            );
        }
    }

    #[test]
    fn raw_value_of_decoded_type_is_what_apple_sent() {
        let parsed: NotificationTypeV2 = serde_json::from_str("\"DID_RENEW\"").unwrap();
        assert_eq!(parsed.raw_value(), "DID_RENEW");
    }
}
