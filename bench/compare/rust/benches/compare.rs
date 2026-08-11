//! The Rust arm of the cross-language comparison, under Divan.
//!
//! Every arm of this suite runs its own language's idiomatic benchmark harness
//! — Divan here, XCTest `measure` in Swift, tinybench in Node, pytest-benchmark
//! in Python. Stage 3 captures each harness's native output verbatim; Stage 4
//! (`render.py`) owns all knowledge of the four formats and normalises them
//! into one table.
//!
//! ## Sample counts are pinned, not adaptive
//!
//! Every harness in this suite is adaptive by default, and each needs a
//! different explicit opt-out. Divan's is `sample_count` × `sample_size`, set
//! here to 500 × 1 so the measured window matches the other three arms.
//!
//! `--sample-count`/`--sample-size` on the command line override these
//! attributes, so `run.sh` pins the contract centrally and this file records
//! the intent.
//!
//! ## What Stage 4 reads
//!
//! Divan has no JSON export: `Stats` and its per-sample data are `pub(crate)`,
//! and the CLI offers no structured-output or precision flag (verified against
//! divan 0.1.21's source and its full `--help`). So Rust's raw artifact is
//! Divan's console table, and the `median` column is what `render.py` parses.
//!
//! That makes Rust the one arm whose artifact carries a harness-reduced median
//! (4 significant figures) rather than raw samples; Swift, Node and Python all
//! export their full sample sets. It does not affect the table — the ratios are
//! ~4×, and the rendered figures are rounded to 2 decimals anyway — but tail
//! statistics for Rust would need a re-run rather than a re-render.

use app_store_server_library_bench_compare as compare;
use app_store_server_library::promotional_offer_signature_creator::PromotionalOfferSignatureCreator;
use app_store_server_library::signed_data_verifier::SignedDataVerifier;

/// Matches the other three arms: 500 timed samples of one iteration each.
const SAMPLE_COUNT: u32 = 500;
const SAMPLE_SIZE: u32 = 1;

fn main() {
    // These numbers describe whichever crypto backend the library resolved, and
    // the suite is pinned to aws_lc. Fail loudly rather than silently reporting
    // a different backend's performance under Rust's name.
    compare::assert_pinned_backend();

    divan::main();
}

/// Built once per benchmark rather than per iteration, so the measured work is
/// the verification itself and not verifier construction — the same setup split
/// every other arm makes.
struct Fixture {
    verifier: SignedDataVerifier,
    inputs: compare::Inputs,
    signer: PromotionalOfferSignatureCreator,
}

impl Fixture {
    fn new() -> Self {
        Self {
            verifier: compare::verifier(),
            inputs: compare::inputs(),
            signer: compare::signer(),
        }
    }

    /// Runs one case and asserts it succeeded.
    ///
    /// A failing call is fast, and a fast wrong number is worse than a missing
    /// one — so a case that stops working fails the benchmark loudly instead of
    /// quietly reporting an impressive timing for work that did not happen.
    fn run(&self, case: &str) {
        let ok = compare::run_case(case, &self.verifier, &self.inputs, &self.signer);
        assert!(ok, "case {case} failed; refusing to report a figure for it");
    }
}

/// One `#[divan::bench]` per case, named exactly as the shared case list, so
/// the tree Divan prints uses the same names every other arm reports.
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
