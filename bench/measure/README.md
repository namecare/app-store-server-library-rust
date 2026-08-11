# app-store-server-library-bench-measure

Regression benchmarks: **did our own code get slower?** Not published.

Where the `compare` crate ranks this library against other implementations,
this one holds every axis it can still — one backend, one fixed effective
date, one set of committed fixtures — so that a change in a number means a
change in our code and not in the environment. Uses
[criterion](https://github.com/bheisler/criterion.rs), which tracks history
between runs and is what CI benchmark services ingest.

## Running

    cargo bench -p app-store-server-library-bench-measure              # everything
    cargo bench -p app-store-server-library-bench-measure --bench chain
    cargo bench -p app-store-server-library-bench-measure -- --test    # run once, don't measure

Criterion saves each run under `target/criterion/` and reports the delta
against the previous one, so a local before/after is just two runs with the
change in between.

## HTML report

Every run writes plots and an index page. Open it with:

    open target/criterion/report/index.html      # macOS
    xdg-open target/criterion/report/index.html  # Linux

The index links every benchmark; each one gets a page with its probability
density, iteration-time scatter, and — once there is a previous run to
compare against — a before/after plot. Comparison plots need two runs, so the
first run after `cargo clean` shows none.

## What is covered

| Bench | Groups | Benchmarks | Notes |
|---|---|---|---|
| `verify` | `verify/`, `decode/` | 6 | full verify path vs. Xcode parse-only path |
| `chain` | `chain/` | 3 | `ChainVerifier` in isolation, including the caching hazard |
| `sign` | `sign/` | 4 | the four public signature creators |
| `receipt` | `receipt/` | 4 | both public receipt extractors |
| `jws` | `jws/` | 3 | the JWS helpers called on every verification |

20 benchmarks in total, all `harness = false` under criterion.

Measured on this machine (Darwin arm64). These figures rank against each
other and against the same machine's next run — they do not port to another
machine or another backend:

| Benchmark | Time |
|---|---|
| `verify/notification` | 139.07 µs |
| `verify/transaction` | 138.92 µs |
| `verify/renewal_info` | 138.50 µs |
| `decode/xcode_transaction` | 2.32 µs |
| `decode/xcode_renewal_info` | 1.73 µs |
| `decode/xcode_app_transaction` | 1.70 µs |
| `chain/verify_test_chain` | 92.24 µs |
| `chain/verify_real_apple` | 315.57 µs |
| `chain/verify_cached` | 402.41 ns |
| `sign/promotional_offer_v1` | 15.97 µs |
| `sign/promotional_offer_v2` | 17.83 µs |
| `sign/introductory_offer_eligibility` | 18.12 µs |
| `sign/advanced_commerce_in_app` | 17.73 µs |
| `receipt/app_receipt` | 2.64 µs |
| `receipt/app_receipt_legacy` | 5.16 µs |
| `receipt/app_receipt_empty` | 1.94 µs |
| `receipt/transaction_receipt` | 82.27 µs |
| `jws/decode_header` | 1.22 µs |
| `jws/signing_input` | 292.79 ns |
| `jws/decode_payload_bytes` | 243.09 ns |

## Three hazards

These are the reasons the suite is structured the way it is. Each one is also
pinned by a test in `tests/scenarios.rs`, because a hazard that only lives in
a doc comment eventually stops being true and nothing complains.

### 1. The public-key cache

`ChainVerifier` caches the verified leaf SPKI keyed on `(leaf, intermediate)`
when `enable_online_checks = true`. Every benchmark in this suite pins that
flag `false` — except one, `chain/verify_cached`, which exists specifically
to measure the cache path and is named so it cannot be mistaken for cold
verification.

The consequence of getting this wrong: `chain/verify_cached` measures 402.41
ns against `chain/verify_test_chain`'s 92.24 µs — a **229x** difference. A
benchmark loop that left online checks on by accident would, after its first
iteration, be reporting a `HashMap` lookup as if it were certificate chain
verification. `tests/scenarios.rs::cache_engages_when_online_checks_enabled`
and `::cache_stays_empty_when_online_checks_disabled` pin both sides of this.

### 2. Xcode/LocalTesting skip verification entirely

`decode_signed_object` returns after header and payload decode for the Xcode
and LocalTesting environments — no chain verification, no signature check.
That is why the parse-only benchmarks in `verify.rs` are grouped `decode/`
and never `verify/`: the group name is a claim about what actually ran, and
these do not run crypto.

The consequence: `verify/*` costs ~139 µs while `decode/*` costs ~1.7–2.3 µs
— a 60–80x gap. `tests/scenarios.rs::decode_group_skips_verification` proves
this the hard way, by building an Xcode-environment verifier from a root CA
that signed nothing at all and confirming it still accepts the fixture.

### 3. The backend is a process-wide `OnceLock` with a cfg cascade preferring `rust_crypto`

The crate's `Cargo.toml` pins `app-store-server-library` with
`default-features = false, features = ["aws_lc", "receipt-utility"]`.
Enabling `rust_crypto` alongside it would silently resolve the wrong backend
— verified: such a build resolves `CryptoProvider { p256_signing: RustCrypto
}`, with nothing else in the output to say so.

This is guarded two ways:

- **Compile time**, for the declared path: `#[cfg(feature = "rust_crypto")]`
  / `#[cfg(feature = "ring")]` in `src/lib.rs` hit a `compile_error!` if this
  crate's own `rust_crypto` or `ring` forwarding feature is enabled.
- **Runtime**, for the paths a `cfg` cannot see (a dependency enabling the
  feature, or feature unification pulling it in transitively):
  `assert_pinned_backend()` is called at the top of every bench group and is
  exercised by `tests/scenarios.rs::benchmarks_run_on_the_pinned_backend`.

## Rules

**Benchmark ids are the tracked metric names.** Renaming one starts a fresh
metric with no history — the one way a regression suite quietly stops
working. Treat the strings passed to `bench_function` as fixed.

**One backend.** `aws_lc` is pinned in `Cargo.toml` with
`default-features = false`; see hazard 3 above. Crypto is the dominant cost
of most of these numbers, so this choice sets their absolute scale —
switching it restarts the history.

**Verifier and creator construction is not timed.** `SignedDataVerifier::new`
decodes root certificates and the signature creators parse a PEM key; both
happen once, outside the `b.iter` closure. That is setup, not the thing being
measured.

## Variance

`sign/promotional_offer_v1` (`PromotionalOfferSignatureCreator`) takes an
explicit nonce and timestamp as arguments, so it is fully deterministic
given fixed inputs. The other three signing benchmarks —
`sign/promotional_offer_v2`, `sign/introductory_offer_eligibility`, and
`sign/advanced_commerce_in_app` — go through JWS creators that generate a
fresh `Uuid::new_v4()` and read `Utc::now()` internally on every call. That
variance cannot be removed through the public API, so it is documented here
rather than hidden or worked around with an internal seam.

## A notable finding

`receipt/transaction_receipt` costs 82.27 µs — 16 to 43x more than the other
three receipt benchmarks (1.94–5.16 µs) — despite operating on the smallest
input of the group, a ~100 byte transaction receipt. The other three walk a
PKCS#7 BER structure, one of them over a 7 KB legacy receipt, and are still
faster.

The cause is `extract_transaction_id_from_transaction_receipt`
(`src/receipt_utility.rs`), which compiles two `Regex` objects on every call
instead of building them once. This is a real, fixable inefficiency in the
library, not a quirk of the benchmark — which is exactly the kind of thing
this suite exists to surface rather than average away.

## Correctness

    cargo test -p app-store-server-library-bench-measure --release --test scenarios

Runs 9 tests. They live in `tests/` rather than as `#[test]` fns inside the
bench files, because `harness = false` means an in-file test would never run.

These exist because a criterion benchmark only measures wall-clock time — it
has no opinion on whether the call it just timed did the right thing. A
benchmark whose call returns `Err` still measures *something*, and often
looks impressively fast doing it, because failing fast is cheap. Nothing in
criterion's summary output would say so. `tests/scenarios.rs` asserts that
every benchmarked case actually produces the outcome its name claims —
`verify/notification` really verifies, `decode/xcode_transaction` really
skips verification, `chain/verify_cached` really hits the cache — so a
benchmark can't quietly start measuring its own error path.

## CI

`.github/workflows/benchmarks.yml` runs this suite and posts a delta table on
pull requests. It never fails the build — crypto dominates these numbers, and
a wall-clock gate would alert on machine noise more often than on real
regressions.
