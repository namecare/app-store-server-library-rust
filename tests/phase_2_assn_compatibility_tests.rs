// Task 2.1: Verify App Store Server Notifications v2 payloads still deserialize
// correctly after the Phase 1 commitment-info changes.
//
// NOTE ON BRIEF DEVIATION: the brief for this task assumed `ResponseBodyV2` could
// be built directly via `serde_json::from_str` on the fixture JSON, and that it
// exposed a `signed_payload` field to assert on directly. Neither holds:
//
//   * `ResponseBodyV2` (src/primitives/response_body_v2.rs) is just
//     `{ signed_payload: Option<String> }` — a JWS wrapper, not the notification
//     body. The actual notification fields (notificationType, subtype, data,
//     summary, etc.) live on `ResponseBodyV2DecodedPayload`, which is produced by
//     verifying and decoding a JWS, not by direct JSON deserialization.
//   * The fixtures under tests/resources/models/*.json are plain (unsigned) JSON
//     documents representing the decoded payload, exactly like the existing
//     tests in tests/signed_data_verifier.rs. They are not JWS strings, so a
//     naive `serde_json::from_str::<ResponseBodyV2>` on them would just produce
//     `signed_payload: None` and not exercise any decoding at all.
//
// This file instead mirrors the established pattern from
// tests/signed_data_verifier.rs: sign the fixture JSON into a JWS using the
// test signing key, then run it through `SignedDataVerifier::verify_and_decode_notification`,
// asserting on the resulting `ResponseBodyV2DecodedPayload`.

use app_store_server_library::primitives::environment::Environment;
use app_store_server_library::primitives::notification_type_v2::NotificationTypeV2;
use app_store_server_library::primitives::status::Status;
use app_store_server_library::primitives::subtype::Subtype;
use app_store_server_library::signed_data_verifier::SignedDataVerifier;
use app_store_server_library::utils::StringExt;
use jsonwebtoken::Algorithm;
use serde_json::{Map, Value};
use std::fs;

const ROOT_CA_BASE64_ENCODED: &str = "MIIBgjCCASmgAwIBAgIJALUc5ALiH5pbMAoGCCqGSM49BAMDMDYxCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApDYWxpZm9ybmlhMRIwEAYDVQQHDAlDdXBlcnRpbm8wHhcNMjMwMTA1MjEzMDIyWhcNMzMwMTAyMjEzMDIyWjA2MQswCQYDVQQGEwJVUzETMBEGA1UECAwKQ2FsaWZvcm5pYTESMBAGA1UEBwwJQ3VwZXJ0aW5vMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEc+/Bl+gospo6tf9Z7io5tdKdrlN1YdVnqEhEDXDShzdAJPQijamXIMHf8xWWTa1zgoYTxOKpbuJtDplz1XriTaMgMB4wDAYDVR0TBAUwAwEB/zAOBgNVHQ8BAf8EBAMCAQYwCgYIKoZIzj0EAwMDRwAwRAIgemWQXnMAdTad2JDJWng9U4uBBL5mA7WI05H7oH7c6iQCIHiRqMjNfzUAyiu9h6rOU/K+iTR0I/3Y/NSWsXHX+acc";

fn get_default_signed_data_verifier() -> SignedDataVerifier {
    let verifier = SignedDataVerifier::new(
        vec![ROOT_CA_BASE64_ENCODED
            .as_der_bytes()
            .unwrap()],
        Environment::LocalTesting,
        "com.example".to_string(),
        Some(1234),
    );

    verifier
}

fn create_signed_data_from_json(path: &str) -> String {
    let json_payload = fs::read_to_string(path).expect("Failed to read JSON file");
    let json: Map<String, Value> = serde_json::from_str(json_payload.as_str()).expect("Expect JSON");

    let header = jsonwebtoken::Header::new(Algorithm::ES256);
    let private_key_pem = include_str!("resources/certs/testSigningKey.p8");
    let key = jsonwebtoken::EncodingKey::from_ec_pem(private_key_pem.as_bytes()).expect("Failed to load test key");
    jsonwebtoken::encode(&header, &json, &key).expect("Failed to encode JWT")
}

#[test]
fn test_signed_notification_v2_deserialization() {
    let signed_notification = create_signed_data_from_json("tests/resources/models/signedNotification.json");
    let verifier = get_default_signed_data_verifier();

    let notification = verifier
        .verify_and_decode_notification(&signed_notification)
        .expect("Expected notification to verify and decode successfully");

    assert_eq!(NotificationTypeV2::Subscribed, notification.notification_type);
    assert_eq!(Subtype::InitialBuy, notification.subtype.expect("Expect subtype"));
    assert_eq!(
        "002e14d5-51f5-4503-b5a8-c3a1af68eb20",
        notification.notification_uuid
    );
    assert!(notification.data.is_some());
}

#[test]
fn test_signed_summary_notification_deserialization() {
    let signed_notification = create_signed_data_from_json("tests/resources/models/signedSummaryNotification.json");
    let verifier = get_default_signed_data_verifier();

    let notification = verifier
        .verify_and_decode_notification(&signed_notification)
        .expect("Expected summary notification to verify and decode successfully");

    assert_eq!(NotificationTypeV2::RenewalExtension, notification.notification_type);
    assert_eq!(Subtype::Summary, notification.subtype.expect("Expect subtype"));
    assert!(notification.summary.is_some());

    let summary = notification.summary.unwrap();
    assert_eq!(5, summary.succeeded_count);
    assert_eq!(2, summary.failed_count);
}

#[test]
fn test_signed_consumption_request_notification_deserialization() {
    let signed_notification =
        create_signed_data_from_json("tests/resources/models/signedConsumptionRequestNotification.json");
    let verifier = get_default_signed_data_verifier();

    let notification = verifier
        .verify_and_decode_notification(&signed_notification)
        .expect("Expected consumption request notification to verify and decode successfully");

    assert_eq!(NotificationTypeV2::ConsumptionRequest, notification.notification_type);
    assert!(notification.data.is_some());

    let data = notification.data.unwrap();
    assert_eq!(Status::Active, data.status.expect("Expect status"));
    assert_eq!(
        "signed_transaction_info_value",
        data.signed_transaction_info
            .as_deref()
            .expect("Expect signed_transaction_info")
    );
    assert_eq!(
        "signed_renewal_info_value",
        data.signed_renewal_info
            .as_deref()
            .expect("Expect signed_renewal_info")
    );
}

#[test]
fn test_backward_compatibility_notification_without_commitment_fields() {
    // The ASSN v2 outer envelope (notificationType/subtype/data/summary) carries no
    // commitment fields of its own -- those live one level down, inside the inner
    // signedTransactionInfo / signedRenewalInfo JWS strings referenced by `data`,
    // which are covered separately by the Phase 1 tests
    // (test_jws_transaction_decoded_payload_with_commitment_info and
    // test_jws_renewal_info_decoded_payload_with_commitment_info). This test just
    // confirms that a notification payload with no commitment-related content
    // anywhere still verifies and decodes cleanly, i.e. the new optional fields on
    // JWSTransactionDecodedPayload/JWSRenewalInfoDecodedPayload did not break
    // ASSN v2 notification decoding.
    let signed_notification = create_signed_data_from_json("tests/resources/models/signedNotification.json");
    let verifier = get_default_signed_data_verifier();

    let notification = verifier
        .verify_and_decode_notification(&signed_notification)
        .expect("Expected notification to verify and decode successfully");

    assert_eq!(NotificationTypeV2::Subscribed, notification.notification_type);
    assert_eq!(
        "002e14d5-51f5-4503-b5a8-c3a1af68eb20",
        notification.notification_uuid
    );
}