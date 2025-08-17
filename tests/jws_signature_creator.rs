use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use serde::Serialize;
use serde_json::Value;
use app_store_server_library::jws_signature_creator::{AdvancedCommerceInAppRequest, AdvancedCommerceInAppSignatureCreator, IntroductoryOfferEligibilitySignatureCreator, PromotionalOfferV2SignatureCreator};

#[test]
fn test_promotional_offer_v2_signature_creator() {
    let test_signing_key = include_str!("../tests/resources/certs/testSigningKey.p8");
    let creator = PromotionalOfferV2SignatureCreator::new(
        test_signing_key,
        "keyId".to_string(),
        "issuerId".to_string(),
        "bundleId".to_string(),
    )
        .unwrap();

    let signature = creator
        .create_signature(
            "productId",
            "offerIdentifier",
            Some("transactionId".to_string()),
        )
        .unwrap();

    let parts: Vec<&str> = signature.split('.').collect();
    assert_eq!(parts.len(), 3);

    // Decode and verify header
    let header_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(parts[0])
        .unwrap();
    let header: Value = serde_json::from_slice(&header_bytes).unwrap();

    assert_eq!(header["typ"], "JWT");
    assert_eq!(header["alg"], "ES256");
    assert_eq!(header["kid"], "keyId");

    // Decode and verify payload
    let payload_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(parts[1])
        .unwrap();
    let payload: Value = serde_json::from_slice(&payload_bytes).unwrap();

    assert_eq!(payload["iss"], "issuerId");
    assert!(payload["iat"].is_number());
    assert!(payload["exp"].is_null());
    assert_eq!(payload["aud"], "promotional-offer");
    assert_eq!(payload["bid"], "bundleId");
    assert!(payload["nonce"].is_string());
    assert_eq!(payload["productId"], "productId");
    assert_eq!(payload["offerIdentifier"], "offerIdentifier");
    assert_eq!(payload["transactionId"], "transactionId");
}

#[test]
fn test_promotional_offer_v2_signature_creator_without_transaction_id() {
    let test_signing_key = include_str!("../tests/resources/certs/testSigningKey.p8");
    let creator = PromotionalOfferV2SignatureCreator::new(
        test_signing_key,
        "keyId".to_string(),
        "issuerId".to_string(),
        "bundleId".to_string(),
    )
        .unwrap();

    let signature = creator
        .create_signature("productId", "offerIdentifier", None)
        .unwrap();

    let parts: Vec<&str> = signature.split('.').collect();
    let payload_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(parts[1])
        .unwrap();
    let payload: Value = serde_json::from_slice(&payload_bytes).unwrap();

    assert!(payload["transactionId"].is_null());
}

#[test]
fn test_introductory_offer_eligibility_signature_creator() {
    let test_signing_key = include_str!("../tests/resources/certs/testSigningKey.p8");
    let creator = IntroductoryOfferEligibilitySignatureCreator::new(
        test_signing_key,
        "keyId".to_string(),
        "issuerId".to_string(),
        "bundleId".to_string(),
    )
        .unwrap();

    let signature = creator
        .create_signature("productId", true, "transactionId")
        .unwrap();

    let parts: Vec<&str> = signature.split('.').collect();
    assert_eq!(parts.len(), 3);

    // Decode and verify header
    let header_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(parts[0])
        .unwrap();
    let header: Value = serde_json::from_slice(&header_bytes).unwrap();

    assert_eq!(header["typ"], "JWT");
    assert_eq!(header["alg"], "ES256");
    assert_eq!(header["kid"], "keyId");

    // Decode and verify payload
    let payload_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(parts[1])
        .unwrap();
    let payload: Value = serde_json::from_slice(&payload_bytes).unwrap();

    assert_eq!(payload["iss"], "issuerId");
    assert!(payload["iat"].is_number());
    assert!(payload["exp"].is_null());
    assert_eq!(payload["aud"], "introductory-offer-eligibility");
    assert_eq!(payload["bid"], "bundleId");
    assert!(payload["nonce"].is_string());
    assert_eq!(payload["productId"], "productId");
    assert_eq!(payload["allowIntroductoryOffer"], true);
    assert_eq!(payload["transactionId"], "transactionId");
}

#[derive(Debug, Serialize)]
struct TestInAppRequest {
    #[serde(rename = "testData")]
    test_data: String,
}

impl AdvancedCommerceInAppRequest for TestInAppRequest {}

#[test]
fn test_advanced_commerce_in_app_signature_creator() {
    let test_signing_key = include_str!("../tests/resources/certs/testSigningKey.p8");
    let creator = AdvancedCommerceInAppSignatureCreator::new(
        test_signing_key,
        "keyId".to_string(),
        "issuerId".to_string(),
        "bundleId".to_string(),
    )
        .unwrap();

    let in_app_request = TestInAppRequest {
        test_data: "testData".to_string(),
    };

    let signature = creator.create_signature(&in_app_request).unwrap();

    let parts: Vec<&str> = signature.split('.').collect();
    assert_eq!(parts.len(), 3);

    // Decode and verify header
    let header_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(parts[0])
        .unwrap();
    let header: Value = serde_json::from_slice(&header_bytes).unwrap();

    assert_eq!(header["typ"], "JWT");
    assert_eq!(header["alg"], "ES256");
    assert_eq!(header["kid"], "keyId");

    // Decode and verify payload
    let payload_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(parts[1])
        .unwrap();
    let payload: Value = serde_json::from_slice(&payload_bytes).unwrap();

    assert_eq!(payload["iss"], "issuerId");
    assert!(payload["iat"].is_number());
    assert!(payload["exp"].is_null());
    assert_eq!(payload["aud"], "advanced-commerce-api");
    assert_eq!(payload["bid"], "bundleId");
    assert!(payload["nonce"].is_string());

    // Verify the request field
    let base64_encoded_request = payload["request"].as_str().unwrap();
    let request_data = BASE64_STANDARD.decode(base64_encoded_request).unwrap();
    let decoded_request: Value = serde_json::from_slice(&request_data).unwrap();
    assert_eq!(decoded_request["testData"], "testData");
}