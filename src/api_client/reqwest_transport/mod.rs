// Without a TLS backend, reqwest cannot do HTTPS and every API call fails at
// connect time (regressed in 4.3.0). Fail at build time instead of runtime.
#[cfg(not(any(feature = "reqwest-tls-rustls", feature = "reqwest-tls-native")))]
compile_error!(
    "reqwest transport enabled without a TLS backend; use `api-client-reqwest` \
     (rustls) or `api-client-reqwest-native-tls` (native-tls)."
);

pub mod reqwest_http_transport;
