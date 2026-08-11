# Crypto backend comparison

This suite runs the same six workloads through this crate's public API, once per
crypto backend, and prints a single table. Unlike `../rust-vs-others/`, there is
only one language and one library here — the variable is the crypto backend
underneath.

## What it answers

How this crate's public API performs on the same six cases across its three
supported crypto backends: `aws-lc-rs`, `RustCrypto`, and `ring`. All three arms
call the same Rust code through the same API; nothing else varies.

## Requirements

Cargo only. No jemalloc, no other language toolchains — every arm is this same
crate rebuilt with a different feature.

## Running

Everything:

```sh
./run.sh
```

Each backend's arm runs, its Divan console table is captured verbatim to
`results/raw/<backend>.{txt,err}`, and `render.py` then parses all three and
writes and prints [`RESULTS.md`](RESULTS.md).

A missing or failing backend **skips** that arm with a warning and the run
still exits `0`. Skipped and failed backends are named on stderr and in the
**Coverage** section of `RESULTS.md`, so an absent column can never be mistaken
for a measured one. A failing arm keeps its stderr in `results/raw/<backend>.err`.

One backend at a time:

```sh
cd rust && COMPARE_BACKEND=aws_lc cargo bench --no-default-features --features aws_lc --bench backend
```

Substitute `rust_crypto` or `ring` for the other two arms.

To re-render the table from raw output already on disk, without re-running
anything:

```sh
python3 render.py
```

## Three rebuilds

Switching Cargo features invalidates the build cache, so a full `./run.sh` run
rebuilds the library three times — once per backend. There is no filter flag to
avoid this; it is the cost of comparing backends selected at compile time rather
than at runtime.

## Runtime-verified backend

Each bench run asserts that the backend it actually linked matches the
`COMPARE_BACKEND` environment variable it was started with, before any case
runs. This exists because the library selects its backend through a cfg cascade
that prefers `rust_crypto`: if a stray transitive feature pulled in the wrong
backend, the build would still succeed, and without this check a `ring` arm
could silently report `rust_crypto` numbers under the `ring` column. The guard
turns that failure mode into an immediate panic instead of a mislabeled table.

## No-crypto rows

`receipt_app` and `receipt_app_legacy` parse an unsigned receipt and touch no
crypto backend at all. Their rows are expected to agree — modulo noise — across
every column; a large gap there points at something other than the crypto
backend.

## Results

The current table lives in [`RESULTS.md`](RESULTS.md).

See also the sibling [`../rust-vs-others/`](../rust-vs-others/) suite, which
compares this crate against Apple's Swift, Node, and Python libraries.
