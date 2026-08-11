# Cross-language comparison benchmarks

This suite runs the same six workloads through Apple's App Store Server Library in
four languages — this Rust crate, Apple's Swift, Node and Python libraries — on one
machine, and prints a single table.

## Requirements
- One machine
- Swift's arm needs jemalloc installed

```sh
brew install jemalloc              # macOS
apt-get install libjemalloc-dev    # Debian/Ubuntu
```

## How it is put together

The suite is four stages, and the split between the last two is the important
one:

| Stage | What it is | Where |
| --- | --- | --- |
| 0 | A native project per language, runnable standalone | `rust/`, `swift/`, `node/`, `python/` |
| 1 | Each buildable and runnable by its own toolchain | see [Running](#running) |
| 2 | Each arm uses **its language's own benchmark harness** | Divan, `package-benchmark`, tinybench, pytest-benchmark |
| 3 | Run every arm, capture its harness's **raw output verbatim** | `run.sh` → `results/raw/` |
| 4 | Parse all four formats and render one table | `render.py` → [`RESULTS.md`](RESULTS.md) |

## Running

Everything:

```sh
./run.sh
```

Every arm whose toolchain is present runs; each harness's native output is
captured to `results/raw/<lib>.{txt,json}`; `render.py` then parses all four and
writes and prints [`RESULTS.md`](RESULTS.md).

A missing toolchain **skips** that arm with a warning and the run still exits `0` — a
three-language table is useful, a hard failure is not. Skipped and failed arms are
named on stderr and in the **Coverage** section of `RESULTS.md`, so an absent column
can never be mistaken for a measured one. A failing arm keeps its stderr in
`results/raw/<lib>.err`.

One arm at a time:

```sh
# Rust — Divan
cd rust && cargo bench --bench compare

# Swift — package-benchmark (needs jemalloc; see caveat 5)
cd swift && swift package --disable-sandbox benchmark

# Swift — XCTest measure. Does NOT feed the table; a zero-setup sanity check
# that every case still runs. MUST be -c release; `swift test` defaults to debug.
cd swift && swift test -c release

# Node — tinybench
cd node && npm install && node runner.mjs

# Python — pytest-benchmark  (create the venv first if missing:
#   python3 -m venv .venv && .venv/bin/pip install -r requirements.txt)
cd python && .venv/bin/python -m pytest test_compare.py
```

Each prints its harness's own output, which is exactly what `run.sh` captures.

To re-render the table from raw output already on disk, without re-running
anything:

```sh
python3 render.py
```

## Each arm runs its own language's harness

Every column is produced by the benchmark tool a developer in that language would
actually reach for:

| Arm | Harness | Raw artifact | What Stage 4 reads |
| --- | --- | --- | --- |
| Rust | Divan 0.1.21 | `raw/rust.txt` | the `median` column of its console table |
| Swift | `package-benchmark` 1.36 (the `Bench` target) | `raw/swift/` | the p50 row of each case's HDR histogram |
| Node | tinybench 6.1.3 | `raw/node.json` | raw `samples_ns`, reduced here |
| Python | pytest-benchmark 5.2.3 | `raw/python.json` | raw `stats.data`, reduced here |

## Different crypto underneath

Each arm sits on a different cryptographic backend as shipped:

| Arm | Backend |
| --- | --- |
| Rust | `aws-lc-rs`  |
| Swift | `swift-crypto` |
| Node | Node's OpenSSL bindings |
| Python | the `cryptography` package |

>This compares **libraries as shipped, not algorithms**.

## The results

The current table lives in [`RESULTS.md`](RESULTS.md)

| Case | Rust | Swift | Node | Python |
| --- | ---: | ---: | ---: | ---: |
| `verify_notification` | 133.50 µs | 528.38 µs | 328.92 µs | 583.52 µs |
| `verify_transaction` | 133.00 µs | 519.93 µs | 312.58 µs | 579.94 µs |
| `verify_renewal_info` | 134.30 µs | 531.97 µs | 318.56 µs | 579.69 µs |
| `receipt_app` | 2.58 µs | 18.88 µs | *unsupported* | 611.31 µs |
| `receipt_app_legacy` | 5.04 µs | 46.85 µs | 292.15 µs | 1.39 ms |
| `sign_promotional_offer` | 15.91 µs | 112.13 µs | 21.37 µs | 25.50 µs |

## Swift stacktrace 

| Stage | Median | Share of ~525 µs |
| --- | ---: | ---: |
| `Certificate(derEncoded:)` ×2 | 25.6 µs | 5% |
| `Verifier` construction + `validate` (X.509 chain) | 245.5 µs | 47% |
| JWT signature verification, as the library does it | 210.0 µs | 40% |
| ├─ raw `P256.isValidSignature` inside that | *116.8 µs* | *22%* |
| └─ `JWTKeyCollection` construction + `add` | *21.4 µs* | *4%* |

## Benchmark Targets

| Arm | Source | Declared in |
|---|---|---|
| Rust | this repo, by path | `rust/Cargo.toml` |
| Swift | `github.com/apple/app-store-server-library-swift`, branch `main` | `swift/Package.swift` |
| Node | npm `@apple/app-store-server-library` | `node/package.json` |
| Python | PyPI `app-store-server-library` | `python/requirements.txt` |

