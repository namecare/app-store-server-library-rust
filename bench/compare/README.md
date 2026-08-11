# Cross-language comparison benchmarks

## What question this answers

**"How do we rank?"**

This suite runs the same six workloads through Apple's App Store Server Library in
four languages — this Rust crate, Apple's Swift, Node and Python libraries — on one
machine, and prints a single table.

That is a different question from the one [`bench/measure`](../measure) answers.
`measure` asks **"did our own code get slower?"**: it tracks this crate against
itself over time, it is what CI watches, and a regression there is a bug. `compare`
asks where this crate sits against the other implementations of the same API at one
moment. It is not run in CI, its numbers do not port between machines, and a change
in the ranking is usually a fact about someone else's library rather than about ours.

Use `measure` to catch regressions. Use `compare` to answer "is this fast, relative
to the alternatives?" once, and to document *why* the answer is what it is.

## Running

Everything:

```sh
./run.sh
```

Every arm whose toolchain is present runs; each writes JSONL to
`results/<lib>.jsonl`; `render.py` then writes and prints [`RESULTS.md`](RESULTS.md).

A missing toolchain **skips** that arm with a warning and the run still exits `0` — a
three-language table is useful, a hard failure is not. Skipped and failed arms are
named on stderr and in the **Coverage** section of `RESULTS.md`, so an absent column
can never be mistaken for a measured one. A failing arm keeps its stderr in
`results/<lib>.err`.

One arm at a time:

```sh
# Rust
cargo run -p app-store-server-library-bench-compare --bin runner --release

# Swift
cd swift && swift build -c release && .build/release/runner

# Node
cd node && node runner.mjs

# Python  (create the venv first if missing:
#   python3 -m venv .venv && .venv/bin/pip install -r requirements.txt)
cd python && .venv/bin/python runner.py
```

Each arm produces one JSON object per line:

```json
{"lib":"rust","case":"verify_notification","iterations":500,"ns_per_op":155330.0}
```

`ns_per_op` is the **median** sample and `iterations` is the number of samples it
was taken from.

**All four arms are the same hand-written loop**, and each prints these lines
directly: warm up 50 iterations, time 500 iterations individually, sort, take the
middle sample. No arm uses a benchmark harness here, and there is no output to
parse.

That is deliberate. A harness would give one column its own sampling strategy and
its own text format to scrape — two ways for it to quietly stop meaning what its
neighbours mean. When a table's whole purpose is that a ratio between two cells is
meaningful, the cells have to be measured identically, and the cheapest way to
guarantee that is for all four to run the same twenty lines. Statistical depth is
[`bench/measure`](../measure)'s job; that suite uses criterion, keeps full
distributions and HTML reports, and is the one that tracks change over time.

To re-render the table from results already on disk, without re-running anything:

```sh
python3 render.py
```

## The results

The current table lives in [`RESULTS.md`](RESULTS.md), which carries a hand-filled
machine/date line. Representative figures, ns/op converted to the unit that reads
best per row:

| Case | Rust | Swift | Node | Python |
| --- | ---: | ---: | ---: | ---: |
| `verify_notification` | 155.33 µs | 534.54 µs | 323.44 µs | 585.44 µs |
| `verify_transaction` | 133.12 µs | 524.00 µs | 314.79 µs | 580.75 µs |
| `verify_renewal_info` | 134.33 µs | 525.88 µs | 312.17 µs | 580.12 µs |
| `receipt_app` | 2.58 µs | 18.21 µs | *unsupported* | 608.31 µs |
| `receipt_app_legacy` | 5.04 µs | 42.71 µs | 276.27 µs | 1.39 ms |
| `sign_promotional_offer` | 15.79 µs | 120.58 µs | 22.08 µs | 25.54 µs |

**These are wall-clock numbers from a single machine.** They establish a ranking;
they are not portable figures. Re-run `./run.sh` on your own hardware before quoting
any absolute number, and read the caveats below before quoting any ratio.

## Why median

**Every one of the four columns is a median**, and every arm takes it the same
way: 50 warmup iterations, then 500 iterations timed individually, sorted, middle
sample. Same warmup, same sample count, same statistic — so the ratios in the
table compare like with like by construction, not by careful documentation.

The reason is that benchmark noise is **one-directional**. An interrupt, a
scheduling decision, a GC pause or a page fault can only ever make a sample
*slower* — nothing makes the work finish faster than the machine can do it. So the
distribution has a hard floor and a long right tail, and the mean is precisely the
statistic that a single bad sample destroys.

This is not hypothetical here. `aws-lc`'s one-time lazy initialisation costs about
**22 ms** on the very first call. That single sample is enough to drag
`sign_promotional_offer`'s **mean to ~239 µs** while its **median stays at
~15.8 µs** — the mean overstates the real per-call cost by **15×**. A reader would
have no way to tell from the number alone; it just looks like signing is expensive.

The median is also stable run to run: three consecutive runs gave **15.83 / 15.87 /
15.89 µs**. A number that moves by 0.4% between runs can support a ranking claim;
a mean that swings with whatever else the machine was doing cannot.

The median is not the *whole* truth — it deliberately discards the tail, so it
answers "what does a typical call cost?" and not "what is my worst case?". For
ranking four libraries against each other, typical cost is the right question. If
you need tail behaviour, `bench/measure` keeps full distributions via criterion.

## Fairness caveats

A cross-language table invites over-reading, so these belong right next to it rather
than in a footnote. All four are real, and all four are in the numbers above.

### 1. Async-ness differs by language, and it is in the numbers

Swift's verify methods are `async` and its `ChainVerifier` is an `actor`; Node's
return promises; Rust's and Python's are plain synchronous calls.

The Swift and Node figures therefore include executor scheduling, promise
scheduling and actor-hop overhead that the Rust and Python figures never pay. That
overhead is a real cost those libraries' users pay on every call, so it belongs in
the measurement — the benchmark measures the library as you would actually call it.

**But for Swift it is not where the time goes.** See "Why is Swift ~4× slower?"
below: the gap is dominated by cryptography and by per-call object construction,
not by concurrency machinery. It would be wrong to read this caveat as an
explanation of the Swift column.

### 2. Different crypto underneath

Each arm sits on a different cryptographic backend as shipped:

| Arm | Backend |
| --- | --- |
| Rust | `aws-lc-rs` (this crate's `aws_lc` feature, the same pin `measure` uses) |
| Swift | `swift-crypto` |
| Node | Node's OpenSSL bindings |
| Python | the `cryptography` package |

This compares **libraries as shipped, not algorithms**. The Rust column would move
if this crate were built on a different backend (`rust_crypto` or `ring`), and the
Rust arm is deliberately pinned to `aws_lc` so the number here is the same Rust that
CI tracks in `measure`.

### 3. Warmup differs between a JIT, an interpreter and AOT code

Node's JIT and Python's interpreter have warmup behaviour that Rust and Swift do
not. All four arms therefore warm up **50 iterations** before measuring, and every
arm reports its sample count on every JSONL line, so the measured window is
visible and the warmup is never timed. This narrows the gap; it does not erase the
difference in how each runtime reaches steady state.

### 4. One machine

Wall-clock, one box, whatever else that box was doing. **These rank, they do not
port.**

## Why is Swift ~4× slower?

The table shows Swift at ~525 µs against Rust's ~135 µs for the same verification,
and it would be easy to attribute that to `async`. It is worth knowing where the
time actually goes, so the stages were measured individually against the same
fixture (median of 200, same machine):

| Stage | Median | Share of ~525 µs |
| --- | ---: | ---: |
| `Certificate(derEncoded:)` ×2 | 25.6 µs | 5% |
| `Verifier` construction + `validate` (X.509 chain) | 245.5 µs | 47% |
| JWT signature verification, as the library does it | 210.0 µs | 40% |
| ├─ raw `P256.isValidSignature` inside that | *116.8 µs* | *22%* |
| └─ `JWTKeyCollection` construction + `add` | *21.4 µs* | *4%* |

Three things stand out, and **none of them is concurrency**:

**1. The cryptography itself is the largest single cost.** Swift's raw P-256
signature check takes **116.8 µs**. Rust's *entire* `verify_notification` — chain
building, JSON decoding, signature check, everything — takes **155.3 µs**. So one
`isValidSignature` call in swift-crypto costs about 75% of Rust's whole pipeline.

This is `aws-lc-rs` — a BoringSSL derivative with hand-written assembly — against
swift-crypto, which under SwiftPM on Darwin sets `CRYPTO_IN_SWIFTPM` and therefore
`@_exported import CryptoKit` (verified in the resolved checkout). So this is not
an unoptimised library losing to an optimised one: it is Apple's own CryptoKit,
and aws-lc-rs is simply faster at P-256 verification on this hardware. No amount
of restructuring the Swift library would recover that difference.

**2. Chain verification rebuilds its verifier on every call.**
`verifyChainWithoutCaching` constructs a fresh `Verifier`, a fresh policy set and a
fresh `CertificateStore` for the intermediate on each invocation — 245 µs. Rust's
`ChainVerifier` holds its roots and rebuilds less per call.

**3. A `JWTKeyCollection` actor is allocated per verification.** The library builds
a new `JWTKeyCollection`, adds the leaf key to it, and verifies — 210 µs total, of
which the raw crypto is 117 µs. The remaining ~93 µs is JWTKit overhead: actor
creation, key registration, re-parsing the token, and re-decoding the payload that
`verify` had *already* decoded a few lines earlier.

Points 2 and 3 are per-call construction that a caching or reuse strategy could
reduce; point 1 is the backend and is not addressable from this library. The
practical reading: **Swift's gap here is mostly cryptographic throughput, with a
meaningful secondary cost in objects rebuilt per call — not async overhead.**

The same caveat as everywhere else applies: this is one machine, and swift-crypto's
P-256 path differs across platforms (on Linux it uses BoringSSL rather than
CryptoKit, which may well change this picture).

## Two findings from building this

Both of these came out of getting the arms running, and both are properties of
Apple's own libraries rather than of this benchmark.

### Node cannot parse the BER app receipt

The `receipt_app` case uses `xcode-app-receipt-with-transaction`, which is
**BER indefinite-length** encoded — its header is `30 80`.

Node's `@apple/app-store-server-library` 3.1.0 ships a **DER-only** ASN.1 parser and
fails on it with `too short ASN.1 value`. Rust (2.68 µs), Swift (19.42 µs) and
Python (647.41 µs) all parse the identical fixture without complaint.

Indefinite-length BER is what real Apple app receipts actually use, so this is a
**genuine capability gap between Apple's own libraries**, not an artifact of a badly
chosen fixture. This is exactly why the table renders that cell as `unsupported`
with a footnote rather than as a blank or a zero: a blank cell reads as "slow", and
the truth is "this library cannot do this at all".

### The Python arm needs a private-attribute workaround

Apple's Python library is the only arm that sets OpenSSL's `X509_STRICT` flag. Apple's
own shared test fixtures carry no `authorityKeyIdentifier`, so OpenSSL 3.x rejects
the chain **before any signature check happens**. There is no public API to turn the
flag off, so `python/runner.py` reaches into a private attribute:

```python
verifier._chain_verifier.enable_strict_checks = False
```

A benchmark that quietly disables validation is measuring nothing, so this was
verified not to hollow out the workload. With the flag off:

- a **tampered signature** is still rejected,
- an **empty root list** is still rejected,
- an **unrelated real CA** (Apple Root CA G3) is still rejected.

The trust anchor is genuinely enforced; only the strict-extension policy is relaxed,
and the Python column is still measuring real chain verification and signature
checking.

## Where each library comes from

Nothing is vendored into this repository. Every arm resolves its library from
upstream, so a run measures what those projects actually ship:

| Arm | Source | Declared in |
|---|---|---|
| Rust | this repo, by path | `Cargo.toml` |
| Swift | `github.com/apple/app-store-server-library-swift`, branch `main` | `swift/Package.swift` |
| Node | npm `@apple/app-store-server-library` | `node/package.json` |
| Python | PyPI `app-store-server-library` | `python/requirements.txt` |

Swift tracks a **branch** rather than a version range, because this suite compares
the current state of each official library — pinning Swift to a release while the
others float would quietly compare different points in their histories.
`swift/Package.resolved` records the exact commit a run resolved, so any published
figure stays reproducible after the fact.

Node and Python pin exact versions (`3.1.0`, `3.1.2`) because their registries do not
offer a branch equivalent; bump them deliberately when refreshing the table.

## What is ignored

`node/node_modules/`, `python/.venv/`, `swift/.build/` and `results/` are all
git-ignored — they are fetched or generated, never committed. **`RESULTS.md` is
not** — the rendered table is the deliverable and belongs in the repo.
