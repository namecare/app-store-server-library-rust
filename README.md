# Apple App Store Server Rust Library
The Rust server library for the [App Store Server API](https://developer.apple.com/documentation/appstoreserverapi) and [App Store Server Notifications](https://developer.apple.com/documentation/appstoreservernotifications)

## Installation

Specify `app-store-server-library` in your project's `Cargo.toml` file, under the `[dependencies]` section:

```rust
[dependencies]
app-store-server-library = { version = "0.9.1", features = ["receipt_utility"] }
```
Check
[crates.io](https://crates.io/crates/app-store-server-library) for the latest version number.

## Usage

### Verification Usage

```rust
// .unwrap() used for example purposes only
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

### Receipt Usage
```rust
let receipt = "MI..";
let transaction_id = extract_transaction_id_from_app_receipt(receipt);
```
> Note: To extract transaction id from app/tx receipt, `receipt_utility` feature must be enabled.

### Promotional Offer Signature Creation
```rust
// .unwrap() used for example purposes only
let private_key = include_str!("../assets/SubscriptionKey_L256SYR32L.p8");
let creator = PromotionalOfferSignatureCreator::new(private_key, "L256SYR32L".to_string(), "com.test.app".to_string()).unwrap();
let signature: String = creator.create_signature("com.test.product", "com.test.offer", uuid::Uuid::new_v4().to_string().as_str(), &uuid::Uuid::new_v4(), i64::try_from(system_timestamp()).unwrap()).unwrap();
```

## Documentation

* The full documentation is available at [docs.rs](https://docs.rs/app-store-server-library/)
* [WWDC Video](https://developer.apple.com/videos/play/wwdc2023/10143/)

## References

- [Apple App Store Server Python Library](https://github.com/apple/app-store-server-library-python)
- [Apple App Store Server Java Library](https://github.com/apple/app-store-server-library-java)
- [Apple App Store Server Node Library](https://github.com/apple/app-store-server-library-node)
- [Apple App Store Server Swift Library](https://github.com/apple/app-store-server-library-swift)
