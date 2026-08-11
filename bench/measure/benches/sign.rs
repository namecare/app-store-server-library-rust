//! Signature creation — the four public creators.
//!
//! Variance note: `PromotionalOfferSignatureCreator` takes an explicit nonce
//! and timestamp, so it is fully deterministic. The three JWS creators
//! generate a fresh `Uuid::new_v4()` and read `Utc::now()` internally on every
//! call; that variance cannot be removed through the public API, so it is
//! documented rather than hidden.

use app_store_server_library::jws_signature_creator::{
    AdvancedCommerceInAppSignatureCreator, IntroductoryOfferEligibilitySignatureCreator,
    PromotionalOfferV2SignatureCreator,
};
use app_store_server_library::models::advanced_commerce_in_app_request::AdvancedCommerceInAppRequest;
use app_store_server_library::promotional_offer_signature_creator::PromotionalOfferSignatureCreator;
use app_store_server_library_bench_measure as bench;
use criterion::{criterion_group, criterion_main, Criterion};
use serde::Serialize;
use std::hint::black_box;

#[derive(Serialize)]
struct BenchInAppRequest {
    #[serde(rename = "testData")]
    test_data: String,
}

impl AdvancedCommerceInAppRequest for BenchInAppRequest {}

fn sign(c: &mut Criterion) {
    app_store_server_library_bench_measure::assert_pinned_backend();

    let v1 = PromotionalOfferSignatureCreator::new(
        bench::SIGNING_KEY_PEM,
        "L256SYR32L".to_string(),
        "com.test.app".to_string(),
    )
    .expect("valid signing key");
    // Fixed nonce and timestamp: this creator is deterministic given them,
    // which removes a source of run-to-run variance the JWS creators have.
    let nonce = uuid::Uuid::parse_str("3db5c98d-8acf-4e29-831e-8e1f82f9f6e9").expect("valid uuid");
    c.bench_function("sign/promotional_offer_v1", |b| {
        b.iter(|| {
            black_box(v1.create_signature(
                black_box("com.test.product"),
                black_box("com.test.offer"),
                black_box("6b9f1f4a-1a1e-4b0e-9b0e-1a1e4b0e9b0e"),
                black_box(&nonce),
                black_box(12345),
            ))
            .is_ok()
        })
    });

    let v2 = PromotionalOfferV2SignatureCreator::new(
        bench::SIGNING_KEY_PEM,
        "keyId".to_string(),
        "issuerId".to_string(),
        "bundleId".to_string(),
    )
    .expect("valid signing key");
    c.bench_function("sign/promotional_offer_v2", |b| {
        b.iter(|| {
            black_box(v2.create_signature(
                black_box("productId"),
                black_box("offerIdentifier"),
                Some("transactionId".to_string()),
            ))
            .is_ok()
        })
    });

    let intro = IntroductoryOfferEligibilitySignatureCreator::new(
        bench::SIGNING_KEY_PEM,
        "keyId".to_string(),
        "issuerId".to_string(),
        "bundleId".to_string(),
    )
    .expect("valid signing key");
    c.bench_function("sign/introductory_offer_eligibility", |b| {
        b.iter(|| {
            black_box(intro.create_signature(black_box("productId"), black_box(true), black_box("transactionId")))
                .is_ok()
        })
    });

    let advanced = AdvancedCommerceInAppSignatureCreator::new(
        bench::SIGNING_KEY_PEM,
        "keyId".to_string(),
        "issuerId".to_string(),
        "bundleId".to_string(),
    )
    .expect("valid signing key");
    let request = BenchInAppRequest {
        test_data: "testData".to_string(),
    };
    c.bench_function("sign/advanced_commerce_in_app", |b| {
        b.iter(|| black_box(advanced.create_signature(black_box(&request))).is_ok())
    });
}

criterion_group!(benches, sign);
criterion_main!(benches);
