#[cfg(any(feature = "api-client-reqwest", feature = "api-client-reqwest-native-tls"))]
pub mod reqwest_transport;

// `api_client::api_client::ApiClient` is the published path downstream code
// imports; renaming the inner module to satisfy the lint would be a breaking
// change for no benefit to callers.
#[allow(clippy::module_inception)]
pub mod api_client;
pub mod error;
pub mod transport;
