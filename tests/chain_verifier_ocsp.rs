mod common;

use app_store_server_library::chain_verifier::{ChainVerifier, ChainVerifierError};
use app_store_server_library::utils::StringExt;
use common::*;

extern crate base64;

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
    
    let verifier = ChainVerifier::new(vec![root]);
    let _public_key = verifier.verify(&leaf, &intermediate, Some(EFFECTIVE_DATE)).unwrap();
    Ok(())
}
