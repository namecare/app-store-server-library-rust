# Benchmark comparison suites

This folder holds two independent comparison suites, each built on the same
shared fixture set.

- **`resources/`** — the shared fixtures used by both suites. Every arm in
  every suite runs against byte-identical inputs; that is what makes any ratio
  between two cells in either table meaningful.

- **[`rust-vs-others/`](rust-vs-others/)** — this crate against Apple's Swift,
  Node, and Python libraries. Needs four language toolchains plus jemalloc
  (for the Swift arm). → [`rust-vs-others/RESULTS.md`](rust-vs-others/RESULTS.md)

- **[`backend/`](backend/)** — the same six cases across this crate's three
  crypto backends: `aws_lc`, `rust_crypto`, and `ring`. Needs cargo only.
  → [`backend/RESULTS.md`](backend/RESULTS.md)

## Shared cases

Both suites run the same six cases:

- `verify_notification`
- `verify_transaction`
- `verify_renewal_info`
- `receipt_app`
- `receipt_app_legacy`
- `sign_promotional_offer`

## Independence

The two suites do not depend on each other. Neither's missing toolchain
affects the other's report: `backend/` runs and renders its table with cargo
alone even if no Swift, Node, or Python toolchain is installed, and
`rust-vs-others/` is unaffected by which crypto backend `backend/` last
measured. See each suite's own README for how to run it.
