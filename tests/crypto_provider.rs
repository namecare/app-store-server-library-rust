//! Tests for the crypto primitives exposed by `CryptoProvider`.
//!
//! These run against whichever backend feature is enabled, so the same
//! assertions cover both `rust_crypto` and `aws_lc`.

use app_store_server_library::crypto::CryptoProvider;

/// NIST FIPS 180-2 test vector: SHA-1("abc").
const SHA1_ABC: [u8; 20] = [
    0xa9, 0x99, 0x3e, 0x36, 0x47, 0x06, 0x81, 0x6a, 0xba, 0x3e, 0x25, 0x71, 0x78, 0x50, 0xc2,
    0x6c, 0x9c, 0xd0, 0xd8, 0x9d,
];

#[test]
fn sha1_matches_nist_vector() {
    let provider = CryptoProvider::default_provider();
    assert_eq!(provider.sha1_hasher.hash(b"abc"), SHA1_ABC);
}

#[test]
fn load_p256_key_accepts_apple_key() {
    let pem = include_str!("resources/certs/testSigningKey.p8");
    let provider = CryptoProvider::default_provider();
    assert!(provider.p256_signing.private_key(pem).is_ok());
}

#[test]
fn load_p256_key_rejects_garbage() {
    let provider = CryptoProvider::default_provider();
    assert!(provider
        .p256_signing
        .private_key("not a pem file")
        .is_err());
}

#[test]
fn signature_is_der_encoded() {
    let pem = include_str!("resources/certs/testSigningKey.p8");
    let provider = CryptoProvider::default_provider();
    let key = provider
        .p256_signing
        .private_key(pem)
        .expect("key should load");

    // Pass the RAW message: the backend applies SHA-256 internally. Pre-hashing
    // here would sign SHA256(SHA256(msg)) and produce signatures Apple rejects.
    let signature = key.signature(b"message to sign").expect("signing should succeed");
    let sig = signature
        .der_representation()
        .expect("DER conversion should succeed");

    // DER SEQUENCE of two INTEGERs. Both backends must agree on this framing,
    // which is what `signature.derRepresentation` produces in the Swift library.
    assert_eq!(sig[0], 0x30, "expected DER SEQUENCE tag");
    assert_eq!(
        sig[1] as usize,
        sig.len() - 2,
        "DER length must match the remaining bytes"
    );
    assert_eq!(sig[2], 0x02, "expected DER INTEGER tag for r");
    // P-256 DER signatures are 70-72 bytes depending on integer padding.
    assert!(
        (70..=72).contains(&sig.len()),
        "unexpected signature length: {}",
        sig.len()
    );
}

#[test]
fn signature_raw_form_is_64_bytes_and_verifies() {
    let pem = include_str!("resources/certs/testSigningKey.p8");
    let provider = CryptoProvider::default_provider();
    let key = provider
        .p256_signing
        .private_key(pem)
        .expect("key should load");

    let message = b"message to sign";
    let signature = key.signature(message).expect("signing should succeed");

    // JWS (RFC 7515 3.1) requires the fixed-width r|s form, always 64 bytes.
    let raw = signature.raw_representation();
    assert_eq!(raw.len(), 64);

    // A signature rebuilt from the raw form must verify against the original
    // message — this is the exact round trip the JWS verification path uses.
    let rebuilt = provider
        .p256_signing
        .signature_from_raw(&raw)
        .expect("raw signature should parse");

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

    let public_key = provider
        .p256_signing
        .public_key(TEST_SIGNING_KEY_SPKI)
        .expect("SPKI should parse");

    public_key
        .is_valid_signature(rebuilt.as_ref(), message)
        .expect("signature must verify against the raw round trip");
}