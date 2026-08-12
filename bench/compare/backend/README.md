# Crypto backend comparison

This suite runs the same four workloads through this crate's public API, once per
crypto backend, and prints a single table.

## Layout

The benches are grouped by what they exercise, one file each:

```
benches/verify.rs    notification, transaction, renewal_info
benches/sign.rs      promo_offer
src/lib.rs           fixtures only - test data, verifier, signer
```

## Running

Everything:

```sh
./run.sh
```

Every backend/bench pair runs, its Divan console table is captured verbatim to
`.output/<backend>.<bench>.{txt,err}`, and `render.py` then parses them all
and writes and prints [`RESULTS.md`](RESULTS.md).

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

## Results

The current table lives in [`RESULTS.md`](RESULTS.md).
