# app-store-server-library-bench-measure

Regression benchmarks.

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

## What is covered

| Bench | Groups | Benchmarks | Notes |
|---|---|---|---|
| `verify` | `verify/`, `decode/` | 6 | full verify path vs. Xcode parse-only path |
| `chain` | `chain/` | 3 | `ChainVerifier` in isolation, including the caching hazard |
| `sign` | `sign/` | 4 | the four public signature creators |
| `receipt` | `receipt/` | 4 | both public receipt extractors |
| `jws` | `jws/` | 3 | the JWS helpers called on every verification |

## CI

`.github/workflows/benchmarks.yml` runs this suite and posts a delta table on
pull requests. It never fails the build — crypto dominates these numbers, and
a wall-clock gate would alert on machine noise more often than on real
regressions.
