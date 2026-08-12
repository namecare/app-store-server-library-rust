# Apple App Store Server Rust Library

[![Build+test](https://github.com/namecare/app-store-server-library-rust/actions/workflows/build_test.yml/badge.svg?branch=master)](https://github.com/namecare/app-store-server-library-rust/actions/workflows/build_test.yml?query=branch%3Amaster)
[![Documentation](https://docs.rs/app-store-server-library/badge.svg)](https://docs.rs/app-store-server-library/)
[![Crates.io](https://img.shields.io/crates/v/app-store-server-library.svg)](https://crates.io/crates/app-store-server-library)
[![Coverage](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fnamecare%2Fapp-store-server-library-rust%2Fmaster%2F.local%2Fcoverage.json)](https://github.com/namecare/app-store-server-library-rust/actions/workflows/build_test.yml?query=branch%3Amaster)

The Rust server library for the [App Store Server API](https://developer.apple.com/documentation/appstoreserverapi), [App Store Server Notifications](https://developer.apple.com/documentation/appstoreservernotifications), the [Retention Messaging API](https://developer.apple.com/documentation/retentionmessaging), and [Advanced Commerce API](https://developer.apple.com/documentation/AdvancedCommerceAPI).

## Requirements

- Rust 1.88.0 or later

## Installation

Specify `app-store-server-library` in your project's `Cargo.toml` file, under the `[dependencies]` section:

```toml
[dependencies]
app-store-server-library = { version = "6.0.0", features = ["rust_crypto", "receipt-utility", "api-client-reqwest"] }
```

### Feature Flags

There are no default features.

#### Crypto backends (pick one)

- `rust_crypto` - Pure-Rust backend (`p256`, `p384`, `rsa`, `sha2`). No C toolchain required.
- `aws_lc` - [aws-lc-rs](https://github.com/aws/aws-lc-rs) backend. FIPS-friendly, needs a C build toolchain.
- `ring` - [ring](https://github.com/briansmith/ring) backend.

The backend is also forwarded to the `x509-validator` dependency used for certificate chain verification, so the whole crate ends up on a single crypto stack.

#### API client

- `api-client` - Enables the App Store Server / Advanced Commerce / Retention Messaging API clients. Bring your own HTTP client by implementing the `Transport` trait.
- `api-client-reqwest` - `api-client` plus the `reqwest` transport, using **rustls** for TLS.
- `api-client-reqwest-native-tls` - Same, but using the platform's **native-tls**.

> The reqwest transport requires a TLS backend; enabling `reqwest` without one is a compile error rather than a runtime connection failure.

#### Tools

- `receipt-utility` - Enables receipt processing and transaction ID extraction

Check [crates.io](https://crates.io/crates/app-store-server-library) for the latest version number.

### Supported API Versions

| API | Version |
|-----|---------|
| App Store Server API | 1.21 |
| Retention Messaging API | 1.5 |
| Advanced Commerce API | 1.2 |

## Obtaining an In-App Purchase key from App Store Connect

To use the App Store Server API or create promotional offer signatures, a signing key downloaded from App Store Connect is required. To obtain this key, you must have the Admin role. Go to Users and Access > Integrations > In-App Purchase. Here you can create and manage keys, as well as find your Issuer ID. When using a key, you'll need the Key ID and the Issuer ID as well.

## Obtaining Apple Root Certificates  

Download and store the root certificates found in the Apple Root Certificates section of the [Apple PKI](https://www.apple.com/certificateauthority/) site. Provide these certificates as an array to a SignedDataVerifier to allow verifying the signed data comes from Apple.

## Usage

### API Usage

The library ships three clients, matching the three APIs. All of them take a signing key as raw PEM bytes, return a `Result` from `new`, and are generic over the `Transport` trait — `ReqwestHttpTransport` is provided when the `api-client-reqwest` feature is on, but any HTTP client works.

#### App Store Server API
```rust
// NOTE: .unwrap() used for example purposes only

use app_store_server_library::app_store_server_api_client::AppStoreServerApiClient;
use app_store_server_library::api_client::reqwest_transport::reqwest_http_transport::ReqwestHttpTransport;
use app_store_server_library::models::app_store_environment::Environment;

#[tokio::main]
async fn main() {
    let issuer_id = "99b16628-15e4-4668-972b-eeff55eeff55";
    let key_id = "ABCDEFGHIJ";
    let bundle_id = "com.example";
    let signing_key = std::fs::read("/path/to/key/SubscriptionKey_ABCDEFGHIJ.p8").unwrap(); // Adjust the path accordingly
    let environment = Environment::Sandbox;
    let transport = ReqwestHttpTransport::new(); // You can use any http client, but you must implement `Transport` trait for it.
    let client = AppStoreServerApiClient::new(signing_key, key_id, issuer_id, bundle_id, environment, transport).unwrap();

    match client.request_test_notification().await {
        Ok(response) => {
            println!("{}", response.test_notification_token);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
```

The Retention Messaging API methods (message list, image upload, default configuration, performance tests, realtime URL) live on this same `AppStoreServerApiClient`, matching the Swift library's layout.

#### Advanced Commerce Server API
```rust
// NOTE: .unwrap() used for example purposes only

use app_store_server_library::advanced_commerce_api_client::AdvancedCommerceApiClient;
use app_store_server_library::api_client::reqwest_transport::reqwest_http_transport::ReqwestHttpTransport;
use app_store_server_library::models::app_store_environment::Environment;

#[tokio::main]
async fn main() {
    let issuer_id = "99b16628-15e4-4668-972b-eeff55eeff55";
    let key_id = "ABCDEFGHIJ";
    let bundle_id = "com.example";
    let signing_key = std::fs::read("/path/to/key/SubscriptionKey_ABCDEFGHIJ.p8").unwrap(); // Adjust the path accordingly
    let environment = Environment::Sandbox;
    let transport = ReqwestHttpTransport::new(); // You can use any http client, but you must implement `Transport` trait for it.
    let client = AdvancedCommerceApiClient::new(signing_key, key_id, issuer_id, bundle_id, environment, transport).unwrap();

    let transaction_id = "txId";
    let subscription_cancel_request = AdvancedCommerceSubscriptionCancelRequest { /* .. */ };
    match client.cancel_subscription(transaction_id, &subscription_cancel_request).await {
        Ok(response) => {
            println!("{}", response.signed_renewal_info);
            println!("{}", response.signed_transaction_info);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
```

### Verification Usage

```rust
// NOTE: .unwrap() used for example purposes only

let root_cert = "apple-root-cert-in-base-base64-format"; // https://www.apple.com/certificateauthority/AppleRootCA-G3.cer
let root_cert_der = root_cert.as_der_bytes().unwrap(); // Use `base64` crate to decode base64 string into bytes 

let verifier = SignedDataVerifier::new(
    vec![root_cert_der], // Vector of root certificates
    Environment::Sandbox, // Environment
    "app.superapp.apple".to_string(), // Bundle id
    Some(12345678), // App id — required in Production
    false, // enable_online_checks
).unwrap();

let payload = "signed-payload";
let decoded_payload = verifier.verify_and_decode_notification(payload).unwrap();
```

Certificate chain verification is delegated to the [`x509-validator`](https://github.com/namecare/x509-validator) crate, which runs on the same crypto backend you selected above.

### Receipt Usage
```rust
let receipt = "MI..";
let transaction_id = extract_transaction_id_from_app_receipt(receipt);
```
> Note: To extract transaction id from app/tx receipt, `receipt-utility` feature must be enabled.

### Promotional Offer Signature Creation

#### V1 Signature Creation
```rust
// NOTE: .unwrap() used for example purposes only

use app_store_server_library::promotional_offer_signature_creator::PromotionalOfferSignatureCreator;

let private_key = include_str!("../assets/SubscriptionKey_L256SYR32L.p8");
let creator = PromotionalOfferSignatureCreator::new(private_key, "L256SYR32L".to_string(), "com.test.app".to_string()).unwrap();

let nonce = uuid::Uuid::new_v4();
let timestamp = chrono::Utc::now().timestamp_millis();
let signature: String = creator.create_signature(
    "com.test.product",
    "com.test.offer", 
    uuid::Uuid::new_v4().to_string().as_str(), // app account token
    &nonce,
    timestamp
).unwrap();
```

#### V2 Signature Creation  
```rust
// NOTE: .unwrap() used for example purposes only

use app_store_server_library::jws_signature_creator::PromotionalOfferV2SignatureCreator;

let private_key = include_str!("../assets/SubscriptionKey_L256SYR32L.p8");
let creator = PromotionalOfferV2SignatureCreator::new(
    private_key, 
    "L256SYR32L".to_string(),     // Key ID
    "issuer_id".to_string(),       // Issuer ID
    "com.test.app".to_string()     // Bundle ID
).unwrap();

let signature: String = creator.create_signature(
    "com.test.product",             // Product ID
    "com.test.offer",               // Offer identifier
    Some("transaction_id".to_string()) // Optional transaction ID
).unwrap();
```

#### Introductory Offer Eligibility Signature Creation
```rust
// NOTE: .unwrap() used for example purposes only

use app_store_server_library::jws_signature_creator::IntroductoryOfferEligibilitySignatureCreator;

let private_key = include_str!("../assets/SubscriptionKey_L256SYR32L.p8");
let creator = IntroductoryOfferEligibilitySignatureCreator::new(
    private_key,
    "L256SYR32L".to_string(),      // Key ID
    "issuer_id".to_string(),       // Issuer ID
    "com.test.app".to_string()     // Bundle ID
).unwrap();

let signature: String = creator.create_signature(
    "com.test.product",            // Product ID
    true,                          // Allow introductory offer
    "transaction_id"               // Transaction ID
).unwrap();
```

### Advanced Commerce Signature Creation

#### Prepare request object:
- Receive request object from the client. 
- Or create request from the server side.

Supported request objects (any type implementing `AdvancedCommerceInAppRequest`): `AdvancedCommerceOneTimeChargeCreateRequest`, `AdvancedCommerceSubscriptionCreateRequest`, `AdvancedCommerceSubscriptionModifyInAppRequest` or `AdvancedCommerceSubscriptionReactivateInAppRequest`.

```rust
// NOTE: .unwrap() used for example purposes only

use app_store_server_library::jws_signature_creator::AdvancedCommerceInAppSignatureCreator;

let request_object = ... // Receive from client side or create on server side 
let private_key = include_str!("../assets/SubscriptionKey_L256SYR32L.p8");
let creator = AdvancedCommerceInAppSignatureCreator::new(
    private_key, 
    "L256SYR32L".to_string(),     // Key ID
    "issuer_id".to_string(),       // Issuer ID
    "com.test.app".to_string()     // Bundle ID
).unwrap();

let signature: String = creator.create_signature(
    advanced_commerce_in_app_request: &request_object
).unwrap();
```

## Documentation

* Upgrading from 4.x? See the [CHANGELOG](CHANGELOG.md) for breaking changes and a migration table.
* The full documentation is available at [docs.rs](https://docs.rs/app-store-server-library/)
* [App Store Server API Documentation](https://developer.apple.com/documentation/appstoreserverapi)
* [App Store Server Notifications Documentation](https://developer.apple.com/documentation/appstoreservernotifications)
* [Retention Messaging API Documentation](https://developer.apple.com/documentation/retentionmessaging)
* [Advanced Commerce API Documentation](https://developer.apple.com/documentation/advancedcommerceapi)
* [WWDC Video](https://developer.apple.com/videos/play/wwdc2023/10143/)

## References

- [Apple App Store Server Python Library](https://github.com/apple/app-store-server-library-python)
- [Apple App Store Server Java Library](https://github.com/apple/app-store-server-library-java)
- [Apple App Store Server Node Library](https://github.com/apple/app-store-server-library-node)
- [Apple App Store Server Swift Library](https://github.com/apple/app-store-server-library-swift)

## Benchmarks

Two crates, in [x509-validator-bench]:

- [`measure`][bench-measure] — Regression benchmarks.
- [`compare`][bench-compare] — Compare backends and parsers ([results][bench-results]).


## License

app-store-server-library is distributed under the following two licenses:

- Apache License version 2.0.
- MIT license.

These are included as LICENSE-APACHE and LICENSE-MIT respectively.  
You may use this software under the terms of any of these licenses, at your option.
