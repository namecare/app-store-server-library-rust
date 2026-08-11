//! Fixtures shared by the `verify` and `sign` benches.
//!
//! Everything here is setup: test data off disk and the two configured objects
//! the library needs. The benches themselves do the work, in the bench blocks.
//!
//! The crypto backend is a compile-time Cargo feature, so exactly one is live
//! per build. `run.sh` rebuilds and reruns the suite once per backend.

use app_store_server_library::models::app_store_environment::Environment;
use app_store_server_library::promotional_offer_signature_creator::PromotionalOfferSignatureCreator;
use app_store_server_library::signed_data_verifier::SignedDataVerifier;

/// Reads a file from `bench/compare/resources/`.
pub fn resource(relative: &str) -> String {
    let path = resource_path(relative);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"))
}

fn resource_path(relative: &str) -> String {
    format!("{}/../resources/{}", env!("CARGO_MANIFEST_DIR"), relative)
}

/// A verifier trusting the test CA, in the sandbox environment.
pub fn verifier() -> SignedDataVerifier {
    let path = resource_path("certs/testCA.der");
    let root_ca = std::fs::read(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));

    SignedDataVerifier::new(
        vec![root_ca],
        Environment::Sandbox,
        "com.example".to_string(),
        Some(1234),
        false,
    )
    .expect("valid verifier configuration")
}

/// A signature creator holding the test signing key.
pub fn signer() -> PromotionalOfferSignatureCreator {
    PromotionalOfferSignatureCreator::new(
        &resource("certs/testSigningKey.p8"),
        "L256SYR32L".to_string(),
        "com.test.app".to_string(),
    )
    .expect("valid signing key")
}

/// Panics unless the linked crypto backend is the one `COMPARE_BACKEND` asked for.
///
/// The library picks its backend through a cfg cascade that prefers `rust_crypto`,
/// so a stray transitive feature can redirect a `ring` build to another backend with
/// no other symptom, mislabelling a whole column of the table.
pub fn assert_requested_backend() {
    let Ok(requested) = std::env::var("COMPARE_BACKEND") else {
        return;
    };
    let expected = match requested.as_str() {
        "aws_lc" => "AwsLc",
        "rust_crypto" => "RustCrypto",
        "ring" => "Ring",
        other => panic!("unknown backend: {other}"),
    };
    let resolved = format!(
        "{:?}",
        app_store_server_library::crypto::CryptoProvider::default_provider()
    );
    assert!(
        resolved.contains(expected),
        "requested {requested} but the library resolved {resolved}"
    );
}
