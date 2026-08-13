//! Decoding must survive enumeration values Apple adds after this release.
//!
//! Apple introduces new enumeration values without notice. Before this
//! behavior existed, one unrecognized value aborted the whole payload and the
//! caller lost every other field. Each Apple enumeration now carries a
//! `NotSupported` variant holding the original wire value, mirroring the
//! `raw*` properties the Swift, Java, Python, and Node ports expose.

use app_store_server_library::models::auto_renew_status::AutoRenewStatus;
use app_store_server_library::models::jws_renewal_info_decoded_payload::JWSRenewalInfoDecodedPayload;
use app_store_server_library::models::jws_transaction_decoded_payload::JWSTransactionDecodedPayload;
use app_store_server_library::models::notification_type_v2::NotificationTypeV2;
use app_store_server_library::models::offer_type::OfferType;
use app_store_server_library::models::product_type::ProductType;
use app_store_server_library::models::response_body_v2_decoded_payload::ResponseBodyV2DecodedPayload;
use app_store_server_library::models::status::Status;
use app_store_server_library::models::subtype::Subtype;

// ---------------------------------------------------------------------------
// String-backed enumerations
// ---------------------------------------------------------------------------

#[test]
fn unknown_string_value_decodes_into_not_supported() {
    let parsed: ProductType = serde_json::from_str("\"Holographic Subscription\"").expect("must decode leniently");
    assert_eq!(
        parsed,
        ProductType::NotSupported("Holographic Subscription".to_string())
    );
}

#[test]
fn known_string_value_still_decodes_into_its_variant() {
    let parsed: ProductType = serde_json::from_str("\"Consumable\"").expect("must decode");
    assert_eq!(parsed, ProductType::Consumable);
}

#[test]
fn unknown_string_value_round_trips_unchanged() {
    let parsed: NotificationTypeV2 = serde_json::from_str("\"FUTURE_NOTIFICATION\"").expect("must decode leniently");
    assert_eq!(
        parsed,
        NotificationTypeV2::NotSupported("FUTURE_NOTIFICATION".to_string())
    );

    // Forwarding a payload we did not fully understand must not corrupt it.
    assert_eq!(
        serde_json::to_string(&parsed).expect("must serialize"),
        "\"FUTURE_NOTIFICATION\""
    );
}

// ---------------------------------------------------------------------------
// Integer-backed enumerations
// ---------------------------------------------------------------------------

#[test]
fn unknown_integer_value_decodes_into_not_supported() {
    let parsed: Status = serde_json::from_str("99").expect("must decode leniently");
    assert_eq!(parsed, Status::NotSupported(99));
}

#[test]
fn known_integer_value_still_decodes_into_its_variant() {
    let parsed: Status = serde_json::from_str("1").expect("must decode");
    assert_eq!(parsed, Status::Active);
}

#[test]
fn unknown_integer_value_round_trips_unchanged() {
    let parsed: OfferType = serde_json::from_str("42").expect("must decode leniently");
    assert_eq!(parsed, OfferType::NotSupported(42));
    assert_eq!(
        serde_json::to_string(&parsed).expect("must serialize"),
        "42"
    );
}

#[test]
fn known_integer_value_round_trips_to_its_wire_value() {
    let parsed: AutoRenewStatus = serde_json::from_str("1").expect("must decode");
    assert_eq!(parsed, AutoRenewStatus::On);
    assert_eq!(serde_json::to_string(&parsed).expect("must serialize"), "1");
}

// ---------------------------------------------------------------------------
// The behavior that motivated all of the above
// ---------------------------------------------------------------------------

#[test]
fn unknown_value_does_not_discard_the_rest_of_the_transaction() {
    let payload = r#"{
        "transactionId": "12345",
        "originalTransactionId": "54321",
        "bundleId": "com.example",
        "type": "Holographic Subscription",
        "inAppOwnershipType": "TELEPATHICALLY_SHARED",
        "revocationReason": 77,
        "price": 10990,
        "currency": "USD"
    }"#;

    let transaction: JWSTransactionDecodedPayload =
        serde_json::from_str(payload).expect("unknown values must not fail the payload");

    // The unrecognized values are preserved rather than dropped.
    assert_eq!(
        transaction.r#type,
        Some(ProductType::NotSupported(
            "Holographic Subscription".to_string()
        ))
    );

    // Every surrounding field still decodes, which is the point of the change.
    assert_eq!(transaction.transaction_id.as_deref(), Some("12345"));
    assert_eq!(
        transaction
            .original_transaction_id
            .as_deref(),
        Some("54321")
    );
    assert_eq!(transaction.bundle_id.as_deref(), Some("com.example"));
    assert_eq!(transaction.price, Some(10990));
    assert_eq!(transaction.currency.as_deref(), Some("USD"));
}

#[test]
fn unknown_value_does_not_discard_the_rest_of_the_notification() {
    let payload = r#"{
        "notificationType": "FUTURE_NOTIFICATION",
        "subtype": "FUTURE_SUBTYPE",
        "notificationUUID": "002e14d5-51f5-4503-b5a8-c3a1af68eb20",
        "version": "2.0"
    }"#;

    let notification: ResponseBodyV2DecodedPayload =
        serde_json::from_str(payload).expect("unknown values must not fail the payload");

    assert_eq!(
        notification.notification_type,
        Some(NotificationTypeV2::NotSupported(
            "FUTURE_NOTIFICATION".to_string()
        ))
    );
    assert_eq!(
        notification.subtype,
        Some(Subtype::NotSupported("FUTURE_SUBTYPE".to_string()))
    );
    assert_eq!(
        notification
            .notification_uuid
            .as_deref(),
        Some("002e14d5-51f5-4503-b5a8-c3a1af68eb20")
    );
    assert_eq!(notification.version.as_deref(), Some("2.0"));
}

#[test]
fn unknown_integer_value_does_not_discard_the_rest_of_the_renewal_info() {
    let payload = r#"{
        "originalTransactionId": "12345",
        "autoRenewProductId": "com.example.product",
        "autoRenewStatus": 7,
        "expirationIntent": 99,
        "currency": "USD"
    }"#;

    let renewal: JWSRenewalInfoDecodedPayload =
        serde_json::from_str(payload).expect("unknown values must not fail the payload");

    assert_eq!(
        renewal.auto_renew_status,
        Some(AutoRenewStatus::NotSupported(7))
    );
    assert_eq!(
        renewal
            .original_transaction_id
            .as_deref(),
        Some("12345")
    );
    assert_eq!(
        renewal.auto_renew_product_id.as_deref(),
        Some("com.example.product")
    );
    assert_eq!(renewal.currency.as_deref(), Some("USD"));
}
