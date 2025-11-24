# Apple App Store Server Rust Library
The Rust server library for the [App Store Server API](https://developer.apple.com/documentation/appstoreserverapi), [App Store Server Notifications](https://developer.apple.com/documentation/appstoreservernotifications), the [Retention Messaging API](https://developer.apple.com/documentation/retentionmessaging), and [Advanced Commerce API](https://developer.apple.com/documentation/AdvancedCommerceAPI).

## Requirements

- Rust 1.65.0 or later

## Installation

Specify `app-store-server-library` in your project's `Cargo.toml` file, under the `[dependencies]` section:

```toml
[dependencies]
app-store-server-library = { version = "4.1.2", features = ["receipt-utility", "api-client", "ocsp"] }
```

### Feature Flags

- `api-client` - Enables the App Store Server/Advanced Commerce API client functionality
- `receipt-utility` - Enables receipt processing and transaction ID extraction
- `ocsp` - Enables OCSP (Online Certificate Status Protocol) verification

Check [crates.io](https://crates.io/crates/app-store-server-library) for the latest version number.

## Obtaining an In-App Purchase key from App Store Connect

To use the App Store Server API or create promotional offer signatures, a signing key downloaded from App Store Connect is required. To obtain this key, you must have the Admin role. Go to Users and Access > Integrations > In-App Purchase. Here you can create and manage keys, as well as find your Issuer ID. When using a key, you'll need the Key ID and the Issuer ID as well.

## Obtaining Apple Root Certificates  

Download and store the root certificates found in the Apple Root Certificates section of the [Apple PKI](https://www.apple.com/certificateauthority/) site. Provide these certificates as an array to a SignedDataVerifier to allow verifying the signed data comes from Apple.

## Usage

### API Usage

#### App Store Server API
```rust
use app_store_server_library::{AppStoreServerApiClient, Environment, AppStoreApiResponse, APIError};

#[tokio::main]
async fn main() {
    let issuer_id = "99b16628-15e4-4668-972b-eeff55eeff55";
    let key_id = "ABCDEFGHIJ";
    let bundle_id = "com.example";
    let encoded_key = std::fs::read_to_string("/path/to/key/SubscriptionKey_ABCDEFGHIJ.p8").unwrap(); // Adjust the path accordingly
    let environment = Environment::Sandbox;
    let transport = ReqwestHttpTransport::new(); // You can use any http client, but you must implement `Transport` trait for it.
    let client = AppStoreServerApiClient::new(encoded_key, key_id, issuer_id, bundle_id, environment, transport);
    match client.request_test_notification().await {
        Ok(response) => {
            println!("{}", response.test_notification_token);
        }
        Err(err) => {
            println!("{}", err.http_status_code);
            println!("{:?}", err.error_code);
            println!("{:?}", err.api_error);
            println!("{}", err.error_message);
        }
    }
}
```

#### Advanced Commerce Server API
```rust
// NOTE: .unwrap() used for example purposes only

use app_store_server_library::{AppStoreServerApiClient, Environment, AppStoreApiResponse, APIError};

#[tokio::main]
async fn main() {
    let issuer_id = "99b16628-15e4-4668-972b-eeff55eeff55";
    let key_id = "ABCDEFGHIJ";
    let bundle_id = "com.example";
    let encoded_key = std::fs::read_to_string("/path/to/key/SubscriptionKey_ABCDEFGHIJ.p8").unwrap(); // Adjust the path accordingly
    let environment = Environment::Sandbox;
    let transport = ReqwestHttpTransport::new(); // You can use any http client, but you must implement `Transport` trait for it.
    let client = AdvancedCommerceApiClient::new(encoded_key, key_id, issuer_id, bundle_id, environment, transport);
    
    let transaction_id = "txId";
    let subscription_cancel_request = SubscriptionCancelRequest(...);
    match client.cancel_subscription(transaction_id, &subscription_cancel_request).await {
        Ok(response) => {
            println!("{}", response.signed_renewal_info);
            println!("{}", response.signed_transaction_info);
        }
        Err(err) => {
            println!("{}", err.http_status_code);
            println!("{:?}", err.error_code);
            println!("{:?}", err.api_error);
            println!("{}", err.error_message);
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
    Some(12345678), // App id
);

let payload = "signed-payload";
let decoded_payload = verifier.verify_and_decode_notification(payload).unwrap();
```

#### OCSP Verification

When the `ocsp` feature is enabled, the library will automatically perform OCSP (Online Certificate Status Protocol) checks to verify that certificates haven't been revoked. This provides an additional layer of security by checking certificate validity in real-time with Apple's OCSP responders.

To enable OCSP verification:

```toml
[dependencies]
app-store-server-library = { version = "4.1.2", features = ["ocsp"] }
```

OCSP verification is performed automatically when verifying signed data.

> Note: OCSP request is blocking, not async.   
> Async signed data verification is coming soon.

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

use app_store_server_library::promotional_offer::PromotionalOfferSignatureCreator;

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

use app_store_server_library::promotional_offer_v2::PromotionalOfferV2SignatureCreator;
 
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

### Advanced Commerce Signature Creation

#### Prepare request object:
- Receive request object from the client. 
- Or create request from the server side.

Supported request objects: `OneTimeChargeCreateRequest`, `SubscriptionCreateRequest`, `SubscriptionModifyInAppRequest` or `SubscriptionReactivateInAppRequest`.

```rust
// NOTE: .unwrap() used for example purposes only

use app_store_server_library::promotional_offer_v2::PromotionalOfferV2SignatureCreator;

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
