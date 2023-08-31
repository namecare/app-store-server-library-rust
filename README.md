# Apple App Store Server Rust Library
The Rust server library for the [App Store Server API](https://developer.apple.com/documentation/appstoreserverapi) and [App Store Server Notifications](https://developer.apple.com/documentation/appstoreservernotifications)

## ⚠️ Beta ⚠️

This software is currently in Beta testing. Therefore, it should only be used for testing purposes, like for the Sandbox environment. API signatures may change between releases and signature verification may receive security updates.

## Installation

Specify `app-store-server-library` in your project's `Cargo.toml` file, under the `[dependencies]` section:

```rust
[dependencies]
app-store-server-library = "0.4.0"
```
Check
[crates.io](https://crates.io/crates/app-store-server-library) for the latest version number.

## Usage

### Verification Usage

```rust

let root_cert = "apple-root-cert-in-base-base64-format"; // https://www.apple.com/certificateauthority/AppleRootCA-G3.cer
let root_cert_der = STANDARD.decode(root_cert).expect("Expect bytes"); // Use `base64` crate to decode base64 string into bytes 

let verifier = SignedDataVerifier::new(
    vec![root_cert_der], // Vector of root certificates
    Environment::Sandbox, // Environment
    "app.superapp.apple".to_string(), // Bundle id
    Some(12345678), // App id
);

let payload = "signed-payload";
let decoded_payload = verifier.verify_and_decode_notification(payload).unwrap();

```
## Documentation

* The full documentation is available at [docs.rs](https://docs.rs/google_maps/)
* [WWDC Video](https://developer.apple.com/videos/play/wwdc2023/10143/)

## References

- [Apple App Store Server Python Library](https://github.com/apple/app-store-server-library-python)
- [Apple App Store Server Java Library](https://github.com/apple/app-store-server-library-java)
- [Apple App Store Server Node Library](https://github.com/apple/app-store-server-library-node)
- [Apple App Store Server Swift Library](https://github.com/apple/app-store-server-library-swift)
