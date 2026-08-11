# Contributing to app-store-server-library

Thank you for your interest in contributing to the Rust App Store Server
Library! This document provides guidelines and instructions for contributing.

Everyone taking part is expected to follow the [Code of Conduct](CODE_OF_CONDUCT.md).

## Getting started

The published crate builds on stable Rust, edition 2021, and needs 1.88 or
newer. A nightly toolchain is used for one formatting check, but nothing in
the crate itself requires it.

```sh
git clone https://github.com/namecare/app-store-server-library-rust
cd app-store-server-library-rust
cargo test --package app-store-server-library \
  --no-default-features --features aws_lc,api-client-reqwest,receipt-utility --all-targets
```

There are no default features, and no default crypto backend: a build with no
backend compiles but verifies nothing, so nearly every command below names one
explicitly. See the [feature flags](README.md#feature-flags) section of the
README for what each one turns on.

The workspace also holds `bench/measure` and `bench/compare`, which are not
published. `examples/` is deliberately a *separate* workspace (it is
`exclude`d from the root one) and pins its own lockfile, so a heavyweight web
framework dependency there never influences feature resolution for the
library.

This crate is a port of Apple's
[Swift library](https://github.com/apple/app-store-server-library-swift),
which is the reference implementation. Changes to models or API surface should
follow it, and Apple's Java, Python, and Node.js libraries are useful
cross-checks.

## Reporting bugs

If you find a bug, please open an issue on
[GitHub Issues](https://github.com/namecare/app-store-server-library-rust/issues).
The bug report template asks for what you would expect us to need:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior vs actual behavior
- Version, platform, and which crypto backend feature you enabled
- A minimal code example if possible

Please redact signing keys, receipts, and JWS payloads from anything you
attach — they carry real transaction data. If a reproduction genuinely needs
one, say so in the issue and send it privately rather than posting it.

For anything security-relevant, follow [SECURITY.md](SECURITY.md) instead and
report it privately.

## Suggesting features

Feature requests are welcome! Please open an issue with:

- A clear description of the feature
- Use cases and motivation
- Any implementation ideas you may have

If the feature exists in the Swift reference library, linking to it there is
the most useful thing you can include.

## Pull request process

1. Fork the repository and create your branch from `master`
2. Make your changes and ensure tests pass
3. Add tests for any new functionality
4. Update documentation if needed
5. Submit a pull request with a clear description of your changes

The pull request template covers what the description should contain, and
prompts you about breaking changes.

### PR guidelines

- Keep changes focused and atomic
- Follow existing code patterns
- Ensure all tests pass before submitting

Titles use [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/)
— `feat:`, `fix:`, `refactor:`, `chore:`, `docs:`, `test:` — with a `!` for
anything breaking, as in `feat!:`. The history is a reasonable guide to the
style.

## Before you push

`admin/check` runs everything CI runs, in the order that fails cheapest first:

```sh
./admin/check
```

It needs a few tools that are not part of a default toolchain. One-time setup:

```sh
rustup toolchain install nightly          # import formatting only
cargo install taplo-cli typos-cli cargo-deny --locked
```

The individual steps, for when you only need one of them:

```sh
# Formatting. The nightly pass handles import grouping and merging, which
# rustfmt cannot yet do on stable. `examples/` is its own workspace, so
# `--all` never reaches it and it needs a second invocation.
cargo fmt --all
cargo fmt --all --manifest-path examples/Cargo.toml
cargo +nightly fmt --all -- --config-path .rustfmt.unstable.toml
taplo fmt

# Linting. Runs clippy once per package and once per crypto backend rather
# than over the workspace, because a workspace-wide build unifies features:
# one member enabling a backend would enable it for every other member, and
# the single-backend configuration that users actually get would never be
# linted at all.
./admin/clippy -- --deny warnings

# Tests. One run per backend, for the same reason: `--all-features` compiles
# all three and proves only that they coexist, not that any one of them works
# on its own.
for backend in aws_lc ring rust_crypto ; do
  cargo test --package app-store-server-library \
    --no-default-features --features $backend,api-client-reqwest,receipt-utility --all-targets
done
cargo test --package app-store-server-library \
  --no-default-features --features aws_lc,api-client-reqwest,receipt-utility --doc

# Advisories, licenses, and spelling.
cargo deny --workspace --all-features check
typos --config .github/typos.toml
```

Examples in the crate's own documentation are compiled as doctests, so they
are checked by the doctest run above and cannot drift from the API. The
`README.md` snippets are not, so they need a manual read when the public API
changes.

Coverage is measured by `admin/coverage`, which wraps `cargo llvm-cov`. CI
runs it on every pull request and posts the per-file breakdown as a comment,
so there is usually no need to run it locally.

## Testing requirements

All contributions should include appropriate tests.

Where the Swift library has a test for the same behavior, mirroring it — same
signed payloads, same certificates, same expectations — is preferred over
writing a new one from scratch, since that is what keeps the two
implementations demonstrably in agreement.

Anything that touches signature verification, certificate chains, or receipt
parsing should be tested against every crypto backend. The per-backend test
targets in `Cargo.toml` handle this: a test file declared once per backend is
compiled and run once per backend, and skipped entirely in the backend-free
build.

## Core contributors: publishing a release

Releases are cut from `master` and published to crates.io. Only
`app-store-server-library` is published; the `bench/` members and `examples/`
are workspace-internal and never shipped.

### Pre-publish checks

1. `master` is green and up to date locally.
2. `./admin/check` passes (see [Before you push](#before-you-push)).
3. Bump `version` in `Cargo.toml`, following [SemVer](https://semver.org/). A
   breaking change to the public API needs a major bump.
4. Update the version in the `[dependencies]` snippet in `README.md`, and the
   supported API version tables there and in `CHANGELOG.md` if any of the
   Apple API versions moved.
5. Update `CHANGELOG.md`: rename the top (unreleased) entry to the new
   version and today's date, grouped under `### Added` / `### Changed` /
   `### Fixed` / `### Removed` as applicable.
6. Run `cargo package --package app-store-server-library --no-default-features --features aws_lc`
   and check the file list it prints — this is the point to notice if a file
   the build needs has been dropped, or if test fixtures have started
   shipping.
7. Run `cargo publish --package app-store-server-library --dry-run --no-default-features --features aws_lc`
   (each backend feature is optional, so the dry run only needs one to
   compile).

### Publishing

```sh
git commit -am "chore: Release vX.Y.Z"
git tag vX.Y.Z
git push origin master --tags
cargo publish --package app-store-server-library
```

### After publishing

- Confirm the new version is live on
  [crates.io](https://crates.io/crates/app-store-server-library) and that
  [docs.rs](https://docs.rs/app-store-server-library) has built it.
- Open a new empty `### Unreleased` section at the top of `CHANGELOG.md` for
  the next round of changes.
- Create a GitHub release from the tag, using the changelog entry as the
  description.

Thank you for contributing!
