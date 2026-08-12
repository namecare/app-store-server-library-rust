# Shared benchmark inputs

Every language arm of the comparison — Rust, Swift, Node, Python — reads these
exact bytes. They are duplicated here rather than referenced out of
`tests/resources/` so that the comparison suite does not reach into the
library's test tree, and so a change to a test fixture cannot silently move the
published numbers.

**These are synthetic test fixtures, not real customer data.** The signed blobs
were generated for this library's test suite and chain to `certs/testCA.der`, a
throwaway CA. Nothing here is a real receipt, a real transaction, or a real
Apple-issued certificate.

## Contents

| File | Source | Used by |
|---|---|---|
| `signed/testNotification` | `tests/resources/mock_signed_data/testNotification` | `verify_notification` |
| `signed/transactionInfo` | `tests/resources/mock_signed_data/transactionInfo` | `verify_transaction` |
| `signed/renewalInfo` | `tests/resources/mock_signed_data/renewalInfo` | `verify_renewal_info` |
| `receipts/xcode-app-receipt-with-transaction` | `tests/resources/xcode/…` | `receipt_app` |
| `receipts/xcode-app-receipt-legacy` | `tests/resources/xcode/…` | `receipt_app_legacy` |
| `certs/testCA.der` | `tests/resources/certs/testCA.der` | the trusted root for every arm |
| `certs/testSigningKey.p8` | `tests/resources/certs/testSigningKey.p8` | `sign_promotional_offer` |

`certs/testCA.der` (390 bytes) is byte-identical to the
`ROOT_CA_BASE64_ENCODED` constant in `tests/common/mod.rs`, decoded — verified
by SHA-256, both `48aa70550eab2cd7…`.

## Why the signed blobs, and not the Xcode ones

The three files under `signed/` are the only committed fixtures that exercise a
**full** verification: certificate chain building plus ECDSA signature checking.
They are verified under `Environment::Sandbox`.

The library short-circuits `Environment::Xcode` and `Environment::LocalTesting`
— `decode_signed_object` returns after header and payload decoding, with no
chain verification and no signature check. A benchmark built on those fixtures
would measure JSON parsing while claiming to measure verification. Measured in
this repo: ~139 µs for a real verification against ~1.7–2.3 µs for the
parse-only path.

The two files under `receipts/` are Xcode receipts, which is correct for their
purpose: the receipt extractors perform no signature validation in any of the
four libraries.
