//! The Rust arm of the cross-language comparison.
//!
//! Deliberately a plain `std::time::Instant` loop rather than a benchmark
//! harness. The Swift, Node and Python arms are hand-written loops of exactly
//! this shape — warm up 50, time 500 iterations individually, report the
//! median — and the value of this suite is that all four cells of a row are
//! the same measurement. A harness would give this one column its own sampling
//! strategy and its own output format to scrape, which is two ways for it to
//! quietly stop meaning what its neighbours mean.
//!
//! Statistical depth is `bench/measure`'s job; that suite uses criterion and is
//! the one that tracks change over time.

use std::time::Instant;

use app_store_server_library_bench_compare as compare;

const WARMUP: usize = 50;
const ITERATIONS: usize = 500;

/// Standard median: sort, and average the two middle samples when `n` is even.
///
/// The median rather than the mean because benchmark noise is one-directional —
/// an interrupt or a page fault can only make a sample slower, never faster —
/// so the distribution has a hard floor and a long right tail, and the mean is
/// the one statistic a single bad sample destroys. Concretely: aws-lc's
/// one-time lazy init costs ~22 ms on its first call, which drags
/// `sign_promotional_offer`'s mean to ~239 µs against a true per-call cost of
/// ~16 µs.
fn median(samples: &mut [u128]) -> f64 {
    samples.sort_unstable();
    let n = samples.len();
    if n.is_multiple_of(2) {
        (samples[n / 2 - 1] as f64 + samples[n / 2] as f64) / 2.0
    } else {
        samples[n / 2] as f64
    }
}

fn main() {
    // These numbers describe whichever crypto backend the library resolved, and
    // the suite is pinned to aws_lc. Fail loudly rather than silently reporting
    // a different backend's performance under Rust's name.
    compare::assert_pinned_backend();

    let verifier = compare::verifier();
    let inputs = compare::inputs();
    let signer = compare::signer();

    for case in compare::CASES {
        // Run once first and skip on failure. A failing call is fast, and a
        // fast wrong number is worse than a missing one — this is what lets an
        // unsupported case render honestly as "unsupported" rather than as an
        // impressive timing.
        if !compare::run_case(case, &verifier, &inputs, &signer) {
            eprintln!("case {case} failed; not reporting a figure for it");
            continue;
        }

        for _ in 0..WARMUP {
            compare::run_case(case, &verifier, &inputs, &signer);
        }

        let mut samples = Vec::with_capacity(ITERATIONS);
        for _ in 0..ITERATIONS {
            let start = Instant::now();
            std::hint::black_box(compare::run_case(case, &verifier, &inputs, &signer));
            samples.push(start.elapsed().as_nanos());
        }

        let ns_per_op = median(&mut samples);
        println!(r#"{{"lib":"rust","case":"{case}","iterations":{ITERATIONS},"ns_per_op":{ns_per_op:.1}}}"#);
    }
}
