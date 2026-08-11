//! The JWS helpers called on every single verification.

use std::hint::black_box;

use app_store_server_library::crypto::jws;
use app_store_server_library_bench_measure as bench;
use criterion::{criterion_group, criterion_main, Criterion};

fn jws_helpers(c: &mut Criterion) {
    app_store_server_library_bench_measure::assert_pinned_backend();

    let token = bench::fixture("mock_signed_data/testNotification");

    c.bench_function("jws/decode_header", |b| {
        b.iter(|| black_box(jws::decode_header(black_box(&token))).is_ok())
    });

    c.bench_function("jws/signing_input", |b| {
        b.iter(|| black_box(jws::signing_input(black_box(&token))).is_ok())
    });

    c.bench_function("jws/decode_payload_bytes", |b| {
        b.iter(|| black_box(jws::decode_payload_bytes(black_box(&token))).is_ok())
    });
}

criterion_group!(benches, jws_helpers);
criterion_main!(benches);
