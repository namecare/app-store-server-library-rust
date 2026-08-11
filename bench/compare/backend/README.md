# Crypto backend comparison

This suite runs the same four workloads through this crate's public API, once per
crypto backend, and prints a single table.

Every case here exercises the crypto backend. Workloads that touch no crypto —
receipt parsing, for one — are deliberately absent: they would report the same
figure in every column and say nothing about the backend.

## Layout

The benches are grouped by what they exercise, one file each:

```
benches/verify.rs    notification, transaction, renewal_info
benches/sign.rs      promo_offer
src/lib.rs           fixtures only - test data, verifier, signer
```

Each bench block calls the library directly, so what is being measured is
readable from the block itself. `src/` holds no benchmark logic — only the
setup the blocks share.

The crypto backend is a **compile-time** Cargo feature, so only one can be
linked into a binary at a time. That makes the backend the outer axis: each
bench file is run once per backend, as a separate process, rather than having
one bench function per backend inside a file.

## Requirements

Cargo only. No jemalloc, no other language toolchains — every arm is this same
crate rebuilt with a different feature.

## Running

Everything:

```sh
./run.sh
```

Every backend/bench pair runs, its Divan console table is captured verbatim to
`.output/<backend>.<bench>.{txt,err}`, and `render.py` then parses them all
and writes and prints [`RESULTS.md`](RESULTS.md).

A missing or failing arm **skips** with a warning and the run still exits `0`.
Skipped and failed arms are named on stderr, and their cells render as `n/a`,
so an absent figure can never be mistaken for a measured one. A failing arm
keeps its stderr in `.output/<backend>.<bench>.err`.

The output directory is set by `OUTPUT_DIR` at the top of `run.sh`, defaulting
to `.output` (gitignored). Both scripts read the same variable, so overriding it
keeps them in step:

```sh
OUTPUT_DIR=/tmp/bench-run ./run.sh
```

One backend and one bench at a time:

```sh
COMPARE_BACKEND=aws_lc cargo bench --no-default-features --features aws_lc --bench verify
```

Substitute `rust_crypto` or `ring` for the other backends, and `sign` for the
other bench file.

To re-render the table from raw output already on disk, without re-running
anything:

```sh
python3 render.py
```

## Three rebuilds

Switching Cargo features invalidates the build cache, so a full `./run.sh` run
rebuilds the library three times — once per backend, not once per arm. The bench
targets within a single backend share that build. There is no filter flag
to avoid the three rebuilds; it is the cost of comparing backends selected at
compile time rather than at runtime.

## Runtime-verified backend

Each bench run asserts that the backend it actually linked matches the
`COMPARE_BACKEND` environment variable it was started with, before any case
runs. This exists because the library selects its backend through a cfg cascade
that prefers `rust_crypto`: if a stray transitive feature pulled in the wrong
backend, the build would still succeed, and without this check a `ring` arm
could silently report `rust_crypto` numbers under the `ring` column. The guard
turns that failure mode into an immediate panic instead of a mislabeled table.

## Results

The current table lives in [`RESULTS.md`](RESULTS.md).

See also the sibling [`../rust-vs-others/`](../rust-vs-others/) suite, which
compares this crate against Apple's Swift, Node, and Python libraries.
