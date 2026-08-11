# Apple App Store Server Rust Library Examples

Two equivalent servers — one [Axum](https://github.com/tokio-rs/axum), one
[Actix-web](https://actix.rs) — showing how to consume this library from a real
HTTP service. Both expose the same endpoints, so they double as a side-by-side
framework comparison on an identical task.

## Running

```sh
cargo run --manifest-path examples/Cargo.toml --bin axum_server
cargo run --manifest-path examples/Cargo.toml --bin actix_server
```

## Endpoints

### `POST /notifications`

An [App Store Server Notifications V2](https://developer.apple.com/documentation/appstoreservernotifications)
webhook. Point App Store Connect at this URL.

```json
{ "signedPayload": "eyJhbGciOi..." }
```

- `200` — the payload verified and was handled. Apple retries anything else, so
  a real integration must persist the notification durably before returning
  `200`.
- `401` — verification failed: bad signature, wrong bundle id, or wrong
  environment.
- `400` — the body was malformed or `signedPayload` was empty.

### `POST /promotional-offer`

Signs a [subscription promotional offer](https://developer.apple.com/documentation/storekit/generating-a-signature-for-promotional-offers)
for a StoreKit client.

```json
{ "productId": "com.example.pro", "offerId": "welcome", "applicationUsername": "user-1" }
```

Returns `signature`, `nonce`, `timestamp`, and `keyIdentifier`.

> **This endpoint signs with your private key.** The example does not
> authenticate it, because authentication is application-specific. Put your own
> auth in front of it before deploying anything like this.

## Configuration

| Variable | Default | Meaning |
|---|---|---|
| `PORT` | `8080` | Listen port |
| `BUNDLE_ID` | `com.example` | Expected bundle id |
| `APP_APPLE_ID` | `1234` | Expected app Apple id |
| `ENVIRONMENT` | `sandbox` | `sandbox` or `production` |
| `APPLE_ROOT_CERTS_DIR` | unset | Directory of `.cer`/`.der` roots |
| `DEMO_MODE` | unset | Trust the bundled test CA instead |
| `PROMO_KEY_PATH` | unset | Path to your `.p8` signing key |
| `PROMO_KEY_ID` | `DEMOKEYID` | Key identifier |

## Root certificates

By default the servers trust **Apple's four public root CAs**, embedded as
base64 constants. That is the correct setting for real traffic — no setup
needed. `APPLE_ROOT_CERTS_DIR` overrides them if you want to manage roots
yourself.

`DEMO_MODE=1` instead trusts `assets/testCA.der`, the self-signed CA from this
repository's test suite. It exists so `assets/testNotification` verifies
offline, and it accepts **nothing else**. Never run a real deployment with it.

## Demo key

With `PROMO_KEY_PATH` unset, offers are signed with `assets/testSigningKey.p8`,
a test key from this repository. The signatures are structurally valid but the
App Store will reject them. Supply your own key from App Store Connect.

>**Never commit a real `.p8` signing key to source control.** In production,
load it from a secret store or an environment-provided path (`PROMO_KEY_PATH`
already supports this) — never bundle it into the repository the way this
example's test-only key is bundled.

## Tests

```sh
cargo test --manifest-path examples/Cargo.toml
```

The smoke tests boot each binary in `DEMO_MODE` and drive it over real HTTP.

