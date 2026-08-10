# Changelog

All notable changes to this project are documented in this file.

## [6.0.0]

A major release that restructures the crate to mirror the Swift reference library, makes the crypto backend pluggable, and updates the models to the latest Apple API versions.

### Supported API versions

| API | 4.3.0 | 6.0.0 |
|-----|-------|-------|
| App Store Server API | 1.19 | 1.21 |
| Retention Messaging API | 1.3 | 1.5 |
| Advanced Commerce API | 1.2 | 1.2 |

### Added

- **Pluggable crypto backends.** Every cryptographic operation now goes through a `CryptoProvider`, backed by one of three mutually interchangeable implementations selected by crate feature:
  - `aws_lc` — [aws-lc-rs](https://github.com/aws/aws-lc-rs) (fastest)
  - `ring` — [ring](https://github.com/briansmith/ring)
  - `rust_crypto` — pure Rust 

- `IntroductoryOfferEligibilitySignatureCreator` for signing introductory offer eligibility JWS.
- Explicit reqwest TLS backend features, `reqwest-tls-rustls` and `reqwest-tls-native`. Enabling the reqwest transport without a TLS backend is now a compile error rather than a runtime connection failure.

### Changed

Breaking changes are listed in the migration table below.

- Certificate chain verification is delegated to the [`x509-validator`](https://github.com/namecare/x509-validator) crate.
- Module layout, API client method names, and model naming now follow the Swift library.
- API responses with no body are handled correctly by status code.

### Removed

- **The `ocsp` feature.** Online certificate revocation checking is no longer part of this crate; chain verification moved to `x509-validator`.
- The `primitives` module (renamed to `models`) and the `api_client::api` module tree.

## [4.3.0] and earlier

See the [release history](https://github.com/namecare/app-store-server-library-rust/releases).