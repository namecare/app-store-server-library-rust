# Security Policy

## Report a security issue

The app-store-server-library project team welcomes security reports and is
committed to providing prompt attention to security issues.

**Please do not open a public issue for a vulnerability.** Report privately
through either channel:

- [GitHub Security Advisories](https://github.com/namecare/app-store-server-library-rust/security/advisories/new) — preferred, since the report, the discussion, and the eventual advisory stay in one place.
- Email [support@namecare.app](mailto:support@namecare.app).

There is no bug bounty.

A useful report includes the version, the crypto backend feature in use
(`aws_lc`, `ring`, or `rust_crypto`), the platform, and what an attacker
gains. Signature verification, certificate chain validation, and receipt
parsing are the parts of this crate that handle attacker-controlled input, so
a report touching those is especially welcome.

Minor issues with no exploitable consequence — a documentation error, a
confusing API — are fine to file on the public
[issue tracker](https://github.com/namecare/app-store-server-library-rust/issues).

Certificate chain verification is delegated to the
[x509-validator](https://github.com/namecare/x509-validator) crate. If the
issue is in that code rather than in this crate, report it under
[its security policy](https://github.com/namecare/x509-validator/security/policy)
instead; either address reaches the same team, so a misfiled report is
forwarded rather than dropped.

## Advisories

The project team is committed to transparency in the security issue disclosure
process. Fixes are announced in the
[release notes](https://github.com/namecare/app-store-server-library-rust/releases)
and the [CHANGELOG](CHANGELOG.md), published as a
[GitHub Security Advisory](https://github.com/namecare/app-store-server-library-rust/security/advisories),
and filed with the
[RustSec advisory database](https://github.com/RustSec/advisory-db) so that
`cargo audit` picks them up.
