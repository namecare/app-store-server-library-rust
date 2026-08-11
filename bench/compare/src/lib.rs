//! The cases every language arm runs, over byte-identical inputs.
//!
//! Each case is a single function, so `src/bin/runner.rs` measures exactly one
//! definition of the work rather than a hand-copied variant of it.

use app_store_server_library::models::app_store_environment::Environment;
use app_store_server_library::promotional_offer_signature_creator::PromotionalOfferSignatureCreator;
use app_store_server_library::receipt_utility::extract_transaction_id_from_app_receipt;
use app_store_server_library::signed_data_verifier::SignedDataVerifier;

/// Fails loudly if the pinned crypto backend is not the one actually in effect.
///
/// The library selects its backend through a cfg cascade that prefers
/// `rust_crypto` (see `src/crypto/mod.rs`), so any dependency enabling that
/// feature would silently redirect every Rust figure in the comparison table to
/// a different backend — verified: such a build resolves
/// `CryptoProvider { p256_signing: RustCrypto }` with no other symptom.
///
/// This is a runtime check rather than `#[cfg(feature = "rust_crypto")]` plus
/// `compile_error!`: a cargo-feature cfg written here tests for a feature on
/// *this* crate, which declares none, so it is always false and never fires.
/// Checking the provider the library actually resolved is what enforces the pin.
pub fn assert_pinned_backend() {
    let provider = format!(
        "{:?}",
        app_store_server_library::crypto::CryptoProvider::default_provider()
    );
    assert!(
        provider.contains("AwsLc"),
        "compare is pinned to aws_lc but the library resolved {provider}; \
         the cross-language table would describe the wrong Rust backend"
    );
}

/// The canonical case names, shared with every other language arm.
pub const CASES: &[&str] = &[
    "verify_notification",
    "verify_transaction",
    "verify_renewal_info",
    "receipt_app",
    "receipt_app_legacy",
    "sign_promotional_offer",
];

fn data(relative: &str) -> String {
    let path = format!("{}/data/{}", env!("CARGO_MANIFEST_DIR"), relative);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"))
}

fn root_ca() -> Vec<u8> {
    let path = format!("{}/data/certs/testCA.der", env!("CARGO_MANIFEST_DIR"));
    std::fs::read(&path).unwrap_or_else(|e| panic!("read {path}: {e}"))
}

pub fn verifier() -> SignedDataVerifier {
    SignedDataVerifier::new(
        vec![root_ca()],
        Environment::Sandbox,
        "com.example".to_string(),
        Some(1234),
        false,
    )
    .expect("valid verifier configuration")
}

pub struct Inputs {
    pub notification: String,
    pub transaction: String,
    pub renewal_info: String,
    pub receipt: String,
    pub receipt_legacy: String,
}

pub fn inputs() -> Inputs {
    Inputs {
        notification: data("signed/testNotification"),
        transaction: data("signed/transactionInfo"),
        renewal_info: data("signed/renewalInfo"),
        receipt: data("receipts/xcode-app-receipt-with-transaction"),
        receipt_legacy: data("receipts/xcode-app-receipt-legacy"),
    }
}

pub fn signing_key() -> String {
    data("certs/testSigningKey.p8")
}

pub fn signer() -> PromotionalOfferSignatureCreator {
    PromotionalOfferSignatureCreator::new(&signing_key(), "L256SYR32L".to_string(), "com.test.app".to_string())
        .expect("valid signing key")
}

pub fn run_case(
    case: &str,
    verifier: &SignedDataVerifier,
    inputs: &Inputs,
    signer: &PromotionalOfferSignatureCreator,
) -> bool {
    match case {
        "verify_notification" => verifier.verify_and_decode_notification(&inputs.notification).is_ok(),
        "verify_transaction" => verifier
            .verify_and_decode_signed_transaction(&inputs.transaction)
            .is_ok(),
        "verify_renewal_info" => verifier.verify_and_decode_renewal_info(&inputs.renewal_info).is_ok(),
        "receipt_app" => extract_transaction_id_from_app_receipt(&inputs.receipt).is_ok(),
        "receipt_app_legacy" => extract_transaction_id_from_app_receipt(&inputs.receipt_legacy).is_ok(),
        "sign_promotional_offer" => {
            let nonce = uuid::Uuid::parse_str("3db5c98d-8acf-4e29-831e-8e1f82f9f6e9").expect("valid uuid");
            signer
                .create_signature(
                    "com.test.product",
                    "com.test.offer",
                    "6b9f1f4a-1a1e-4b0e-9b0e-1a1e4b0e9b0e",
                    &nonce,
                    12345,
                )
                .is_ok()
        }
        other => panic!("unknown case: {other}"),
    }
}
