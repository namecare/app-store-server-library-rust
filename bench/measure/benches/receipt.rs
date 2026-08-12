//! Receipt parsing — both public extractors.

use std::hint::black_box;

use app_store_server_library::receipt_utility::{
    extract_transaction_id_from_app_receipt, extract_transaction_id_from_transaction_receipt,
};
use app_store_server_library_bench_measure as bench;
use criterion::{criterion_group, criterion_main, Criterion};

fn receipt(c: &mut Criterion) {
    app_store_server_library_bench_measure::assert_pinned_backend();

    // PKCS#7 BER walk over a receipt containing one in-app transaction.
    let with_transaction = bench::fixture("xcode/xcode-app-receipt-with-transaction");
    c.bench_function("receipt/app_receipt", |b| {
        b.iter(|| {
            black_box(extract_transaction_id_from_app_receipt(black_box(
                &with_transaction,
            )))
            .is_ok()
        })
    });

    // The largest committed receipt (7 KB) — the BER-parsing worst case.
    let legacy = bench::fixture("xcode/xcode-app-receipt-legacy");
    c.bench_function("receipt/app_receipt_legacy", |b| {
        b.iter(|| black_box(extract_transaction_id_from_app_receipt(black_box(&legacy))).is_ok())
    });

    // No in-app purchases: the full structure is walked and nothing is found,
    // so this is the search cost without the extraction.
    let empty = bench::fixture("xcode/xcode-app-receipt-empty");
    c.bench_function("receipt/app_receipt_empty", |b| {
        b.iter(|| black_box(extract_transaction_id_from_app_receipt(black_box(&empty))).is_ok())
    });

    // Regex over a decoded plist, including two Regex compilations per call.
    let transaction = bench::fixture("mock_signed_data/legacyTransaction");
    c.bench_function("receipt/transaction_receipt", |b| {
        b.iter(|| {
            black_box(extract_transaction_id_from_transaction_receipt(black_box(
                &transaction,
            )))
            .is_ok()
        })
    });
}

criterion_group!(benches, receipt);
criterion_main!(benches);
