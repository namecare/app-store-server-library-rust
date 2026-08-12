//! JWS verification: signature check plus certificate chain validation.
//!
//! One backend is live per build. Run the whole comparison with `./run.sh`, or
//! a single backend with:
//!
//! ```sh
//! COMPARE_BACKEND=aws_lc cargo bench --no-default-features --features aws_lc --bench verify
//! ```

use app_store_server_library_bench_backend::{assert_requested_backend, resource, verifier};
use divan::Bencher;

fn main() {
    assert_requested_backend();
    divan::main();
}

#[divan::bench]
fn notification(bencher: Bencher) {
    let verifier = verifier();
    let signed = resource("signed/testNotification");

    bencher.bench_local(|| {
        verifier
            .verify_and_decode_notification(&signed)
            .expect("notification verifies")
    });
}

#[divan::bench]
fn transaction(bencher: Bencher) {
    let verifier = verifier();
    let signed = resource("signed/transactionInfo");

    bencher.bench_local(|| {
        verifier
            .verify_and_decode_signed_transaction(&signed)
            .expect("transaction verifies")
    });
}

#[divan::bench]
fn renewal_info(bencher: Bencher) {
    let verifier = verifier();
    let signed = resource("signed/renewalInfo");

    bencher.bench_local(|| {
        verifier
            .verify_and_decode_renewal_info(&signed)
            .expect("renewal info verifies")
    });
}
