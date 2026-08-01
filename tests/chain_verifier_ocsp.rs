#![cfg(all(feature = "rust_crypto", feature = "ocsp"))]

mod common;

use app_store_server_library::chain_verifier::{ChainVerifier, ChainVerifierError};
use app_store_server_library::crypto::CryptoProvider;
use app_store_server_library::utils::StringExt;
use common::*;

extern crate base64;

fn create_verifier() -> Box<dyn ChainVerifier> {
    let provider = CryptoProvider::default_provider();
    (provider.chain_verifier)()
}

#[test]
fn test_apple_chain_is_valid_with_ocsp() -> Result<(), ChainVerifierError> {
    let root = REAL_APPLE_ROOT_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();
    let leaf = REAL_APPLE_SIGNING_CERTIFICATE_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();
    let intermediate = REAL_APPLE_INTERMEDIATE_BASE64_ENCODED
        .as_der_bytes()
        .unwrap();

    let verifier = create_verifier();
    let _public_key = verifier.verify(&leaf, &intermediate, &[root], Some(EFFECTIVE_DATE), true)?;

    // OCSP check would be called explicitly if needed:
    // check_ocsp_status(&leaf_cert, &intermediate_cert)?;

    Ok(())
}
