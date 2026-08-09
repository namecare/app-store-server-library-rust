use crate::api_client::error::{ApiClientError, ConfigurationError};
use crate::api_client::transport::{Transport, TransportError};
use crate::crypto::jws;
use crate::crypto::CryptoProvider;
use crate::models::app_store_environment::Environment;

use chrono::Utc;
use http::Method;
use http::{Request, Response};
use serde::{Deserialize, Serialize};

pub struct ApiClient<T: Transport> {
    base_url: String,
    signing_key: Vec<u8>,
    key_id: String,
    issuer_id: String,
    bundle_id: String,
    transport: T,
}

impl<T: Transport> ApiClient<T> {
    /// Creates a new App Store Server API client.
    ///
    /// # Arguments
    ///
    /// * `signing_key` - The private key used for signing JWT tokens
    /// * `key_id` - The key identifier from App Store Connect
    /// * `issuer_id` - The issuer ID from App Store Connect
    /// * `bundle_id` - The app's bundle identifier
    /// * `environment` - The environment to use (Production or Sandbox). Xcode environment is not supported for API calls.
    /// * `transport` - The HTTP transport implementation
    ///
    /// # Errors
    ///
    /// Returns an error if the Xcode environment is provided, as it's only for local receipt validation.
    pub fn new(
        signing_key: Vec<u8>,
        key_id: &str,
        issuer_id: &str,
        bundle_id: &str,
        environment: Environment,
        transport: T,
    ) -> Result<Self, ConfigurationError> {
        // Xcode environment is only for local receipt validation and cannot be used with the API
        if matches!(environment, Environment::Xcode) {
            return Err(ConfigurationError::InvalidEnvironment(
                "Xcode environment is not supported for App Store Server API calls. Use Sandbox or Production instead."
                    .to_string(),
            ));
        }

        let base_url = environment.base_url();
        Ok(Self {
            base_url,
            signing_key,
            key_id: key_id.to_string(),
            issuer_id: issuer_id.to_string(),
            bundle_id: bundle_id.to_string(),
            transport,
        })
    }

    pub(crate) fn generate_token(&self) -> Result<String, ApiClientError> {
        let now = Utc::now();
        let future_time = now + chrono::Duration::minutes(5);

        let header = serde_json::json!({
            "alg": "ES256",
            "kid": self.key_id,
            "typ": "JWT",
        });

        let claims = Claims {
            bid: &self.bundle_id,
            iss: &self.issuer_id,
            aud: "appstoreconnect-v1",
            iat: now.timestamp(),
            exp: future_time.timestamp(),
        };

        let signing_error = || {
            ApiClientError::new(500, None, Some("Failed to sign request token".to_string()))
        };

        let pem = std::str::from_utf8(self.signing_key.as_slice())
            .map_err(|_| signing_error())?;
        let key = CryptoProvider::default_provider()
            .p256_signing
            .private_key(pem)
            .map_err(|_| signing_error())?;

        let encoded_header = jws::b64url_encode(
            &serde_json::to_vec(&header).map_err(|_| signing_error())?,
        );
        let encoded_payload = jws::b64url_encode(
            &serde_json::to_vec(&claims).map_err(|_| signing_error())?,
        );
        let signing_input = format!("{encoded_header}.{encoded_payload}");

        let (raw, _) = key
            .signature(signing_input.as_bytes())
            .map_err(|_| signing_error())?;

        Ok(jws::encode_compact(
            &encoded_header,
            &encoded_payload,
            &raw,
        ))
    }

    pub(crate) fn build_request<B: serde::Serialize>(
        &self,
        path: &str,
        method: Method,
        body: Option<&B>,
    ) -> Result<Request<Vec<u8>>, ApiClientError> {
        let (body_bytes, content_type) = if let Some(body_data) = body {
            let serialized = serde_json::to_vec(body_data)
                .map_err(|_| ApiClientError::new(400, None, Some("Failed to serialize request body".to_string())))?;
            (serialized, Some("application/json"))
        } else {
            (Vec::new(), None)
        };

        self.build_request_base(path, method, body_bytes, content_type)
    }

    pub(crate) fn build_request_with_custom_content(
        &self,
        path: &str,
        method: Method,
        body: Vec<u8>,
        content_type: &str,
    ) -> Result<Request<Vec<u8>>, ApiClientError> {
        self.build_request_base(path, method, body, Some(content_type))
    }

    fn build_request_base(
        &self,
        path: &str,
        method: Method,
        body: Vec<u8>,
        content_type: Option<&str>,
    ) -> Result<Request<Vec<u8>>, ApiClientError> {
        let url = format!("{}{}", self.base_url, path);

        let mut request_builder = Request::builder()
            .method(method)
            .uri(url)
            .header("User-Agent", "app-store-server-library/rust/4.3.0")
            .header("Authorization", format!("Bearer {}", self.generate_token()?))
            .header("Accept", "application/json");

        if let Some(ct) = content_type {
            request_builder = request_builder.header("Content-Type", ct);
        }

        request_builder.body(body).map_err(|e| e.into())
    }

    pub(crate) async fn make_request_with_response_body<Res>(
        &self,
        request: Request<Vec<u8>>,
    ) -> Result<Res, ApiClientError>
    where
        Res: for<'de> Deserialize<'de>,
    {
        let response = self.make_request(request).await?;
        self.extract_response(&response)
    }

    pub(crate) async fn make_request_without_response_body(
        &self,
        request: Request<Vec<u8>>,
    ) -> Result<(), ApiClientError> {
        let response = self.make_request(request).await?;

        let status_code = response.status().as_u16();
        if !(200..300).contains(&status_code) {
            return Err(self.extract_error(&response));
        }

        Ok(())
    }

    pub(crate) async fn make_request(&self, request: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, TransportError> {
        self.transport.send(request).await
    }

    pub(crate) fn extract_response<Res>(&self, response: &Response<Vec<u8>>) -> Result<Res, ApiClientError>  where
        Res: for<'de> Deserialize<'de>
    {
        let status_code = response.status().as_u16();

        if !(200..300).contains(&status_code) {
            return Err(self.extract_error(&response))
        }

        let body = response.body();
        let json_result = serde_json::from_slice::<Res>(body)
            .map_err(|_| ApiClientError::new(500, None, Some("Failed to deserialize response JSON".to_string())))?;

        Ok(json_result)
    }

    pub(crate) fn extract_error(&self, response: &Response<Vec<u8>>) -> ApiClientError {
        let status_code = response.status().as_u16();

        #[derive(Deserialize)]
        struct ErrorPayload {
            #[serde(rename = "errorCode")]
            error_code: Option<i64>,
            #[serde(rename = "errorMessage")]
            error_message: Option<String>,
        }

        serde_json::from_slice::<ErrorPayload>(response.body())
            .ok()
            .and_then(|payload| match (payload.error_code, payload.error_message) {
                (Some(error_code), Some(error_message)) => {
                    Some(ApiClientError::new(status_code, Some(error_code), Some(error_message)))
                }
                _ => None,
            })
            .unwrap_or_else(|| ApiClientError::new(status_code, None, None))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims<'a> {
    bid: &'a str,
    iss: &'a str,
    aud: &'a str,
    iat: i64,
    exp: i64,
}