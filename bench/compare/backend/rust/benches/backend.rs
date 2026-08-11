//! Run: COMPARE_BACKEND=aws_lc cargo bench --no-default-features --features aws_lc --bench backend

use app_store_server_library_bench_backend as backend;
use app_store_server_library::promotional_offer_signature_creator::PromotionalOfferSignatureCreator;
use app_store_server_library::signed_data_verifier::SignedDataVerifier;

const SAMPLE_COUNT: u32 = 500;
const SAMPLE_SIZE: u32 = 1;

/// The library picks its backend through a cfg cascade that prefers `rust_crypto`,
/// so a stray transitive feature can redirect a `ring` build to another backend with
/// no other symptom, mislabelling a whole column of the table.
fn assert_requested_backend() {
    let Ok(requested) = std::env::var("COMPARE_BACKEND") else {
        return;
    };
    let resolved = backend::resolved_backend();
    let expected = match requested.as_str() {
        "aws_lc" => "AwsLc",
        "rust_crypto" => "RustCrypto",
        "ring" => "Ring",
        other => panic!("unknown backend: {other}"),
    };
    assert!(
        resolved.contains(expected),
        "requested {requested} but the library resolved {resolved}"
    );
}

fn main() {
    assert_requested_backend();
    divan::main();
}

struct Fixture {
    verifier: SignedDataVerifier,
    inputs: backend::Inputs,
    signer: PromotionalOfferSignatureCreator,
}

impl Fixture {
    fn new() -> Self {
        Self {
            verifier: backend::verifier(),
            inputs: backend::inputs(),
            signer: backend::signer(),
        }
    }

    fn run(&self, case: &str) {
        let ok = backend::run_case(case, &self.verifier, &self.inputs, &self.signer);
        assert!(ok, "case {case} failed; refusing to report a figure for it");
    }
}

macro_rules! case_benches {
    ($($name:ident),* $(,)?) => {
        $(
            #[divan::bench(sample_count = SAMPLE_COUNT, sample_size = SAMPLE_SIZE)]
            fn $name(bencher: divan::Bencher) {
                let fixture = Fixture::new();
                bencher.bench_local(|| fixture.run(stringify!($name)));
            }
        )*
    };
}

case_benches! {
    verify_notification,
    verify_transaction,
    verify_renewal_info,
    receipt_app,
    receipt_app_legacy,
    sign_promotional_offer,
}
