//! `ChainVerifier` in isolation — the certificate-path cost that dominates a full verification.

use std::hint::black_box;

use app_store_server_library::chain_verifier::ChainVerifier;
use app_store_server_library_bench_measure as bench;
use criterion::{criterion_group, criterion_main, Criterion};

fn chain(c: &mut Criterion) {
    app_store_server_library_bench_measure::assert_pinned_backend();

    let (leaf, intermediate, root) = bench::test_chain();
    let verifier = ChainVerifier::new(vec![root]);
    // The synthetic chain's own validity window; these certs are valid
    // 2023-01-05 to 2033-01-01, so any time inside that range works.
    let test_chain_time: u64 = 1700000000;
    c.bench_function("chain/verify_test_chain", |b| {
        b.iter(|| {
            black_box(verifier.verify_at(
                black_box(&leaf),
                black_box(&intermediate),
                Some(test_chain_time),
                false,
                test_chain_time,
            ))
            .is_ok()
        })
    });

    let (real_leaf, real_intermediate, real_root) = bench::real_apple_chain();
    let real_verifier = ChainVerifier::new(vec![real_root]);
    c.bench_function("chain/verify_real_apple", |b| {
        b.iter(|| {
            black_box(real_verifier.verify_at(
                black_box(&real_leaf),
                black_box(&real_intermediate),
                Some(bench::EFFECTIVE_DATE),
                false,
                bench::EFFECTIVE_DATE,
            ))
            .is_ok()
        })
    });

    // The one benchmark that enables caching. Named so it cannot be mistaken
    // for cold verification: after the first iteration this is a HashMap
    // lookup keyed on the certificate bytes, which is exactly the point —
    // it measures what a caching caller actually pays on a repeat call.
    let cached_verifier = ChainVerifier::new(vec![bench::root_ca_der()]);
    let _ = cached_verifier.verify_at(
        &leaf,
        &intermediate,
        Some(test_chain_time),
        true,
        test_chain_time,
    );
    c.bench_function("chain/verify_cached", |b| {
        b.iter(|| {
            black_box(cached_verifier.verify_at(
                black_box(&leaf),
                black_box(&intermediate),
                Some(test_chain_time),
                true,
                test_chain_time,
            ))
            .is_ok()
        })
    });
}

criterion_group!(benches, chain);
criterion_main!(benches);
