//! Asserts that every benchmarked case produces the outcome its name claims.
//!
//! These live in `tests/` because the bench files are `harness = false`, where
//! an in-file `#[test]` would never run.

use app_store_server_library::chain_verifier::ChainVerifier;
use app_store_server_library::models::app_store_environment::Environment;
use app_store_server_library::receipt_utility::{
    extract_transaction_id_from_app_receipt, extract_transaction_id_from_transaction_receipt,
};
use app_store_server_library_bench_measure as bench;

#[test]
fn verify_cases_actually_verify() {
    let verifier = bench::sandbox_verifier();

    let notification = bench::fixture("mock_signed_data/testNotification");
    assert!(
        verifier.verify_and_decode_notification(&notification).is_ok(),
        "verify/notification must succeed, or it benchmarks the error path"
    );

    let transaction = bench::fixture("mock_signed_data/transactionInfo");
    assert!(
        verifier.verify_and_decode_signed_transaction(&transaction).is_ok(),
        "verify/transaction must succeed"
    );

    let renewal_info = bench::fixture("mock_signed_data/renewalInfo");
    assert!(
        verifier.verify_and_decode_renewal_info(&renewal_info).is_ok(),
        "verify/renewal_info must succeed"
    );
}

#[test]
fn decode_cases_succeed_under_xcode() {
    let verifier = bench::verifier(
        Environment::Xcode,
        "com.example.naturelab.backyardbirds.example",
        Some(531412),
    );

    let transaction = bench::fixture("xcode/xcode-signed-transaction");
    assert!(verifier.verify_and_decode_signed_transaction(&transaction).is_ok());

    let renewal_info = bench::fixture("xcode/xcode-signed-renewal-info");
    assert!(verifier.verify_and_decode_renewal_info(&renewal_info).is_ok());

    let app_transaction = bench::fixture("xcode/xcode-signed-app-transaction");
    assert!(verifier.verify_and_decode_app_transaction(&app_transaction).is_ok());
}

/// The `decode/` group must NOT be doing chain verification — that is the
/// whole reason it is named `decode/` and not `verify/`. An Xcode-environment
/// verifier built with a root CA that signed nothing still accepts the
/// fixture, which proves no chain validation is happening.
#[test]
fn decode_group_skips_verification() {
    let verifier = app_store_server_library::signed_data_verifier::SignedDataVerifier::new(
        vec![bench::root_ca_der()],
        Environment::Xcode,
        "com.example.naturelab.backyardbirds.example".to_string(),
        Some(531412),
        false,
    )
    .expect("valid configuration");

    let transaction = bench::fixture("xcode/xcode-signed-transaction");
    assert!(
        verifier.verify_and_decode_signed_transaction(&transaction).is_ok(),
        "Xcode environment must bypass chain verification; if this fails, the \
         decode/ group is doing real crypto and is misnamed"
    );
}

#[test]
fn chain_cases_validate() {
    let (leaf, intermediate, root) = bench::test_chain();
    let verifier = ChainVerifier::new(vec![root]);
    let t: u64 = 1700000000;
    assert!(
        verifier.verify_at(&leaf, &intermediate, Some(t), false, t).is_ok(),
        "chain/verify_test_chain must validate"
    );

    let (real_leaf, real_intermediate, real_root) = bench::real_apple_chain();
    let real = ChainVerifier::new(vec![real_root]);
    assert!(
        real.verify_at(
            &real_leaf,
            &real_intermediate,
            Some(bench::EFFECTIVE_DATE),
            false,
            bench::EFFECTIVE_DATE
        )
        .is_ok(),
        "chain/verify_real_apple must validate at the pinned EFFECTIVE_DATE"
    );
}

/// Pins hazard 1: with online checks on, a repeat verification is served from
/// the cache. If this ever stops holding, `chain/verify_cached` is measuring
/// something else and the other benchmarks' `false` is no longer meaningful.
#[test]
fn cache_engages_when_online_checks_enabled() {
    let (leaf, intermediate, root) = bench::test_chain();
    let verifier = ChainVerifier::new(vec![root]);
    let t: u64 = 1700000000;

    assert_eq!(verifier.cache_len(), 0, "cache starts empty");
    verifier
        .verify_at(&leaf, &intermediate, Some(t), true, t)
        .expect("first verification succeeds");
    assert_eq!(verifier.cache_len(), 1, "online checks must populate the cache");
}

/// The converse: with online checks off — the setting every other benchmark
/// uses — nothing is cached, so each iteration does the full work.
#[test]
fn cache_stays_empty_when_online_checks_disabled() {
    let (leaf, intermediate, root) = bench::test_chain();
    let verifier = ChainVerifier::new(vec![root]);
    let t: u64 = 1700000000;

    verifier
        .verify_at(&leaf, &intermediate, Some(t), false, t)
        .expect("verification succeeds");
    assert_eq!(
        verifier.cache_len(),
        0,
        "with online checks off the cache must stay empty, or the verify/ and \
         chain/ benchmarks are measuring cache hits"
    );
}

#[test]
fn receipt_cases_extract_expected_ids() {
    let with_transaction = bench::fixture("xcode/xcode-app-receipt-with-transaction");
    assert_eq!(
        extract_transaction_id_from_app_receipt(&with_transaction).expect("parses"),
        Some("0".to_string())
    );

    let legacy = bench::fixture("xcode/xcode-app-receipt-legacy");
    assert_eq!(
        extract_transaction_id_from_app_receipt(&legacy).expect("parses"),
        Some("2000000909538865".to_string())
    );

    let empty = bench::fixture("xcode/xcode-app-receipt-empty");
    assert_eq!(
        extract_transaction_id_from_app_receipt(&empty).expect("parses"),
        None,
        "the empty receipt has no in-app purchases"
    );

    let transaction = bench::fixture("mock_signed_data/legacyTransaction");
    assert_eq!(
        extract_transaction_id_from_transaction_receipt(&transaction).expect("parses"),
        Some("33993399".to_string())
    );
}

#[test]
fn sign_cases_produce_signatures() {
    use app_store_server_library::promotional_offer_signature_creator::PromotionalOfferSignatureCreator;

    let creator = PromotionalOfferSignatureCreator::new(
        bench::SIGNING_KEY_PEM,
        "L256SYR32L".to_string(),
        "com.test.app".to_string(),
    )
    .expect("valid signing key");
    let nonce = uuid::Uuid::parse_str("3db5c98d-8acf-4e29-831e-8e1f82f9f6e9").expect("valid uuid");
    let signature = creator
        .create_signature(
            "com.test.product",
            "com.test.offer",
            "6b9f1f4a-1a1e-4b0e-9b0e-1a1e4b0e9b0e",
            &nonce,
            12345,
        )
        .expect("signing succeeds");
    assert!(!signature.is_empty(), "sign/promotional_offer_v1 must produce output");
}

/// The whole suite's numbers describe whichever backend the library actually
/// resolved. The crate pins aws_lc, but a dependency enabling `rust_crypto`
/// would silently redirect every measurement — verified: such a build resolves
/// `CryptoProvider { p256_signing: RustCrypto }` with no other symptom.
#[test]
fn benchmarks_run_on_the_pinned_backend() {
    app_store_server_library_bench_measure::assert_pinned_backend();
}
