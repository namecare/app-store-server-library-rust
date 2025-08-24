#![cfg(not(feature = "ocsp"))]

mod common;

use app_store_server_library::chain_verifier::ChainVerificationFailureReason::{
    CertificateExpired, InvalidCertificate
};
use app_store_server_library::chain_verifier::{ChainVerifier, ChainVerifierError};
use app_store_server_library::utils::StringExt;
use base64::engine::general_purpose::STANDARD;
use base64::{DecodeError, Engine};
use common::*;

extern crate base64;

#[test]
fn test_valid_chain_without_ocsp() -> Result<(), ChainVerifierError> {
    let root = ROOT_CA_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();
    let leaf = LEAF_CERT_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();
    let intermediate = INTERMEDIATE_CA_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();

    let verifier = ChainVerifier::new(vec![root]);
    let public_key = verifier.verify(&leaf, &intermediate, Some(EFFECTIVE_DATE))?;
    assert_eq!(
        LEAF_CERT_PUBLIC_KEY_BASE64_ENCODED
            .as_der_bytes()
            .unwrap(),
        public_key
    );
    Ok(())
}

#[test]
fn test_valid_chain_invalid_intermediate_oid_without_ocsp() -> Result<(), ChainVerifierError> {
    let root = ROOT_CA_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();
    let leaf = LEAF_CERT_FOR_INTERMEDIATE_CA_INVALID_OID_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();
    let intermediate = INTERMEDIATE_CA_INVALID_OID_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();

    let verifier = ChainVerifier::new(vec![root]);
    let public_key = verifier.verify(&leaf, &intermediate, Some(EFFECTIVE_DATE));

    assert_eq!(
        public_key.expect_err("Expect error"),
        ChainVerifierError::VerificationFailure(InvalidCertificate)
    );
    Ok(())
}

#[test]
fn test_valid_chain_invalid_leaf_oid_without_ocsp() -> Result<(), ChainVerifierError> {
    let root = ROOT_CA_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();
    let leaf = LEAF_CERT_INVALID_OID_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();
    let intermediate = INTERMEDIATE_CA_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();

    let verifier = ChainVerifier::new(vec![root]);
    let public_key = verifier.verify(&leaf, &intermediate, Some(EFFECTIVE_DATE));

    assert_eq!(
        public_key.expect_err("Expect error"),
        ChainVerifierError::VerificationFailure(InvalidCertificate)
    );

    Ok(())
}

#[test]
fn test_invalid_base64_in_certificate_list() -> Result<(), ChainVerifierError> {
    assert_eq!(
        "abc"
            .as_der_bytes()
            .expect_err("Expect Error"),
        DecodeError::InvalidPadding
    );
    Ok(())
}

#[test]
fn test_invalid_data_in_certificate_list() -> Result<(), ChainVerifierError> {
    let root = ROOT_CA_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();
    let leaf = STANDARD
        .encode("abc")
        .as_der_bytes()
        .unwrap();
    let intermediate = INTERMEDIATE_CA_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();

    let verifier = ChainVerifier::new(vec![root]);
    let public_key = verifier.verify(&leaf, &intermediate, Some(EFFECTIVE_DATE));

    assert_eq!(
        public_key.expect_err("Expect error"),
        ChainVerifierError::VerificationFailure(InvalidCertificate)
    );
    Ok(())
}

#[test]
fn test_malformed_root_cert() -> Result<(), ChainVerifierError> {
    let malformed_root = STANDARD
        .encode("abc")
        .as_der_bytes()
        .unwrap();
    let leaf = LEAF_CERT_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();
    let intermediate = INTERMEDIATE_CA_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();

    let verifier = ChainVerifier::new(vec![malformed_root]);
    let public_key = verifier.verify(&leaf, &intermediate, Some(EFFECTIVE_DATE));
    assert_eq!(
        public_key.expect_err("Expect error"),
        ChainVerifierError::VerificationFailure(InvalidCertificate)
    );
    Ok(())
}

#[test]
fn test_chain_different_than_root_certificate() -> Result<(), ChainVerifierError> {
    let real_root = REAL_APPLE_ROOT_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();
    let leaf = LEAF_CERT_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();
    let intermediate = INTERMEDIATE_CA_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();

    let verifier = ChainVerifier::new(vec![real_root]);
    let public_key = verifier.verify(&leaf, &intermediate, Some(EFFECTIVE_DATE));

    assert_eq!(
        public_key.expect_err("Expect error"),
        ChainVerifierError::VerificationFailure(InvalidCertificate)
    );
    Ok(())
}

#[test]
fn test_valid_expired_chain() -> Result<(), ChainVerifierError> {
    let root = ROOT_CA_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();
    let leaf = LEAF_CERT_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();
    let intermediate = INTERMEDIATE_CA_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();

    let verifier = ChainVerifier::new(vec![root]);
    let public_key = verifier.verify(&leaf, &intermediate, Some(2280946846));

    assert_eq!(
        public_key.expect_err("Expect error"),
        ChainVerifierError::VerificationFailure(CertificateExpired)
    );
    Ok(())
}

#[test]
fn test_apple_chain_is_valid() -> Result<(), ChainVerifierError> {
    let root = REAL_APPLE_ROOT_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();
    let leaf = REAL_APPLE_SIGNING_CERTIFICATE_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();
    let intermediate = REAL_APPLE_INTERMEDIATE_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();

    let verifier = ChainVerifier::new(vec![root]);
    let _public_key = verifier.verify(&leaf, &intermediate, Some(EFFECTIVE_DATE))?;
    Ok(())
}

#[test]
fn test_apple_chain_is_valid_multi_root() -> Result<(), ChainVerifierError> {
    let leaf = REAL_APPLE_SIGNING_CERTIFICATE_BASE64_ENCODED.as_der_bytes()?;
    let intermediate = REAL_APPLE_INTERMEDIATE_BASE64_ENCODED.as_der_bytes()?;

    let multi_root: Vec<_> = REAL_APPLE_MULTI_ROOT_BASE64_ENCODED
        .into_iter()
        .map(|str| str.as_der_bytes().unwrap())
        .collect();

    let verifier = ChainVerifier::new(multi_root);
    let _public_key = verifier.verify(&leaf, &intermediate, Some(EFFECTIVE_DATE))?;
    Ok(())
}
