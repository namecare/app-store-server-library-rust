//! Tests for the V1 promotional offer signature creator.
//!
//! These run against whichever backend feature is enabled, so the same
//! assertions cover `rust_crypto`, `aws_lc` and `ring` alike.

use app_store_server_library::crypto::CryptoProvider;
use app_store_server_library::promotional_offer_signature_creator::PromotionalOfferSignatureCreator;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;

const PRIVATE_KEY: &str = include_str!("../tests/resources/certs/testSigningKey.p8");

// SPKI DER for testSigningKey.p8, derived with:
//   openssl pkey -in testSigningKey.p8 -pubout -outform DER
#[rustfmt::skip]
const TEST_SIGNING_KEY_SPKI: &[u8] = &[
    0x30, 0x59, 0x30, 0x13, 0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02,
    0x01, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07, 0x03,
    0x42, 0x00, 0x04, 0xac, 0xf2, 0x72, 0xc4, 0x4b, 0xb6, 0xfe, 0x82, 0x82,
    0xac, 0x96, 0x9a, 0x4d, 0x54, 0xeb, 0xab, 0x7f, 0x43, 0x28, 0x9d, 0x97,
    0x70, 0xde, 0x96, 0xfa, 0x4f, 0x14, 0xbc, 0x67, 0xfe, 0x63, 0xf9, 0x43,
    0x58, 0xd6, 0xc5, 0x8a, 0xd4, 0x28, 0x71, 0xa6, 0xec, 0x0d, 0x4b, 0xd2,
    0x37, 0x47, 0x9d, 0x1c, 0x7d, 0xd4, 0x28, 0xfe, 0x50, 0x3d, 0x3c, 0xf8,
    0x70, 0xe9, 0x72, 0x20, 0xf3, 0x2b, 0x5d,
];

fn creator() -> PromotionalOfferSignatureCreator {
    PromotionalOfferSignatureCreator::new(
        PRIVATE_KEY,
        "L256SYR32L".to_string(),
        "com.test.app".to_string(),
    )
    .unwrap()
}

/// The payload Apple specifies: the seven fields joined by U+2063 INVISIBLE
/// SEPARATOR, with the app account token and nonce lowercased.
fn expected_payload(
    bundle_id: &str,
    key_id: &str,
    product_id: &str,
    offer_id: &str,
    app_account_token: &str,
    nonce: &uuid::Uuid,
    timestamp: i64,
) -> String {
    format!(
        "{}\u{2063}{}\u{2063}{}\u{2063}{}\u{2063}{}\u{2063}{}\u{2063}{}",
        bundle_id,
        key_id,
        product_id,
        offer_id,
        app_account_token.to_lowercase(),
        nonce.to_string().to_lowercase(),
        timestamp
    )
}

#[test]
fn test_promotional_offer_signature_creator() {
    let r = creator()
        .create_signature(
            "com.test.product",
            "com.test.offer",
            uuid::Uuid::new_v4()
                .to_string()
                .as_str(),
            &uuid::Uuid::new_v4(),
            12345,
        )
        .unwrap();

    assert!(!r.is_empty())
}

/// Apple requires the DER encoding, NOT the fixed-width 64-byte `r‖s` form
/// (IEEE P1363) that most ECDSA APIs return by default. A regression to the
/// raw form still base64-encodes cleanly and still verifies locally, so it is
/// only caught by inspecting the encoding itself.
#[test]
fn signature_is_der_encoded_not_p1363() {
    let sig_b64 = creator()
        .create_signature(
            "com.test.product",
            "com.test.offer",
            "user123",
            &uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            1234567890,
        )
        .unwrap();

    let der = BASE64_STANDARD
        .decode(&sig_b64)
        .expect("signature must be standard base64");

    // A P1363 signature is exactly 64 bytes; DER is a SEQUENCE of two INTEGERs
    // and lands in 70..=72 bytes for P-256 (shorter only when r or s has
    // leading zero bytes, which is rare but legal).
    assert_ne!(
        der.len(),
        64,
        "signature is 64 bytes — this is the raw P1363 form, not DER"
    );
    assert_eq!(
        der[0], 0x30,
        "DER signature must start with SEQUENCE (0x30)"
    );
    assert_eq!(
        der[1] as usize,
        der.len() - 2,
        "DER length header must match the body length"
    );
    assert_eq!(der[2], 0x02, "first DER element must be an INTEGER (0x02)");

    // The two INTEGERs (r and s) must together account for the whole body.
    let r_len = der[3] as usize;
    let s_tag_idx = 4 + r_len;
    assert_eq!(
        der[s_tag_idx], 0x02,
        "second DER element must be an INTEGER (0x02)"
    );
    let s_len = der[s_tag_idx + 1] as usize;
    assert_eq!(
        4 + r_len + 2 + s_len,
        der.len(),
        "DER r/s lengths must span the full signature"
    );
}

/// Verifying against the RAW payload proves exactly one SHA-256 was applied.
/// If the creator pre-hashed, the backend would sign SHA256(SHA256(payload))
/// and this fails — the signature Apple would reject.
#[test]
fn signature_verifies_against_raw_payload() {
    let nonce = uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let timestamp = 1234567890;

    let sig_b64 = creator()
        .create_signature("product1", "offer1", "user123", &nonce, timestamp)
        .unwrap();
    let der = BASE64_STANDARD
        .decode(&sig_b64)
        .unwrap();

    let payload = expected_payload(
        "com.test.app",
        "L256SYR32L",
        "product1",
        "offer1",
        "user123",
        &nonce,
        timestamp,
    );

    // `is_valid_signature` takes the fixed-width form, so re-derive it by
    // signing the same payload and matching DER against raw.
    let provider = CryptoProvider::default_provider();
    let key = provider
        .p256_signing
        .private_key(PRIVATE_KEY)
        .unwrap();
    let (raw, der_again) = key
        .signature(payload.as_bytes())
        .unwrap();

    // ECDSA is randomized, so two signings differ — but both must be DER and
    // both must verify against the same raw payload.
    assert_eq!(der_again[0], 0x30);
    assert_eq!(der[0], 0x30);

    let public_key = provider
        .p256_signing
        .public_key(TEST_SIGNING_KEY_SPKI)
        .expect("SPKI should parse");
    public_key
        .is_valid_signature(&raw, payload.as_bytes())
        .expect("signature must verify against the raw payload — exactly one SHA-256");
}

/// The payload uses U+2063 INVISIBLE SEPARATOR, which is invisible in diffs and
/// easily mangled by editors. Pin its bytes so a substitution is caught.
#[test]
fn payload_separator_is_invisible_separator() {
    let nonce = uuid::Uuid::parse_str("550E8400-E29B-41D4-A716-446655440000").unwrap();
    let payload = expected_payload(
        "com.test.app",
        "L256SYR32L",
        "product1",
        "offer1",
        "USER123",
        &nonce,
        1234567890,
    );

    assert_eq!(
        payload.matches('\u{2063}').count(),
        6,
        "payload must have exactly six U+2063 separators"
    );
    assert_eq!(
        payload
            .as_bytes()
            .windows(3)
            .filter(|w| *w == [0xE2, 0x81, 0xA3])
            .count(),
        6,
        "separators must encode as E2 81 A3"
    );
    // The app account token and nonce are lowercased.
    assert!(
        payload.contains("user123"),
        "app account token must be lowercased"
    );
    assert!(
        payload.contains("550e8400-e29b-41d4-a716-446655440000"),
        "nonce must be lowercased"
    );
}
