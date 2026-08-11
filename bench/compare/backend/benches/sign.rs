//! ECDSA P-256 signing.
//!
//! One backend is live per build. Run the whole comparison with `./run.sh`, or
//! a single backend with:
//!
//! ```sh
//! COMPARE_BACKEND=aws_lc cargo bench --no-default-features --features aws_lc --bench sign
//! ```

use app_store_server_library_bench_backend::{assert_requested_backend, signer};
use divan::Bencher;
use uuid::Uuid;

fn main() {
    assert_requested_backend();
    divan::main();
}

#[divan::bench]
fn promo_offer(bencher: Bencher) {
    let signer = signer();
    let nonce = Uuid::parse_str("3db5c98d-8acf-4e29-831e-8e1f82f9f6e9").expect("valid uuid");

    bencher.bench_local(|| {
        signer
            .create_signature(
                "com.test.product",
                "com.test.offer",
                "6b9f1f4a-1a1e-4b0e-9b0e-1a1e4b0e9b0e",
                &nonce,
                12345,
            )
            .expect("signature is created")
    });
}