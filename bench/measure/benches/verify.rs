//! The five `verify_and_decode_*` entry points, plus the parse-only path.

use std::hint::black_box;

use app_store_server_library::models::app_store_environment::Environment;
use app_store_server_library_bench_measure as bench;
use criterion::{criterion_group, criterion_main, Criterion};

fn verify(c: &mut Criterion) {
    app_store_server_library_bench_measure::assert_pinned_backend();

    // Verifier construction decodes root certificates. That is setup, not
    // verification, so it happens once outside the timed region.
    let verifier = bench::sandbox_verifier();

    let notification = bench::fixture("mock_signed_data/testNotification");
    c.bench_function("verify/notification", |b| {
        b.iter(|| black_box(verifier.verify_and_decode_notification(black_box(&notification))).is_ok())
    });

    let transaction = bench::fixture("mock_signed_data/transactionInfo");
    c.bench_function("verify/transaction", |b| {
        b.iter(|| black_box(verifier.verify_and_decode_signed_transaction(black_box(&transaction))).is_ok())
    });

    let renewal_info = bench::fixture("mock_signed_data/renewalInfo");
    c.bench_function("verify/renewal_info", |b| {
        b.iter(|| black_box(verifier.verify_and_decode_renewal_info(black_box(&renewal_info))).is_ok())
    });
}

fn decode(c: &mut Criterion) {
    app_store_server_library_bench_measure::assert_pinned_backend();

    // Xcode environment: verification is skipped, so these measure header and
    // payload decoding only.
    let verifier = bench::verifier(
        Environment::Xcode,
        "com.example.naturelab.backyardbirds.example",
        Some(531412),
    );

    let transaction = bench::fixture("xcode/xcode-signed-transaction");
    c.bench_function("decode/xcode_transaction", |b| {
        b.iter(|| black_box(verifier.verify_and_decode_signed_transaction(black_box(&transaction))).is_ok())
    });

    let renewal_info = bench::fixture("xcode/xcode-signed-renewal-info");
    c.bench_function("decode/xcode_renewal_info", |b| {
        b.iter(|| black_box(verifier.verify_and_decode_renewal_info(black_box(&renewal_info))).is_ok())
    });

    let app_transaction = bench::fixture("xcode/xcode-signed-app-transaction");
    c.bench_function("decode/xcode_app_transaction", |b| {
        b.iter(|| black_box(verifier.verify_and_decode_app_transaction(black_box(&app_transaction))).is_ok())
    });
}

criterion_group!(benches, verify, decode);
criterion_main!(benches);
