use crate::primitives::environment::Environment;
use crate::api_client::transport::Transport;
use crate::api_client::error::{ApiServiceError, APIServiceErrorCode, ConfigurationError, ErrorPayload};

use chrono::Utc;
use http::Method;
use http::{Request, Response};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use serde::de::DeserializeOwned;

pub struct ApiClient<T: Transport, API, E: APIServiceErrorCode + DeserializeOwned> {
    base_url: String,
    signing_key: Vec<u8>,
    key_id: String,
    issuer_id: String,
    bundle_id: String,
    transport: T,
    _api: PhantomData<API>,
    _api_error: PhantomData<E>,
}

unsafe impl<T: Transport, API, E: APIServiceErrorCode + DeserializeOwned> Send for ApiClient<T, API, E> {}
unsafe impl<T: Transport, API, E: APIServiceErrorCode + DeserializeOwned> Sync for ApiClient<T, API, E> {}

impl<T: Transport, API, E: APIServiceErrorCode + DeserializeOwned> ApiClient<T, API, E> {
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
            _api: PhantomData,
            _api_error: PhantomData,
        })
    }

    pub(super) fn generate_token(&self) -> String {
        let future_time = Utc::now() + chrono::Duration::minutes(5);
        let key_id = (&self.key_id).to_string();

        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(key_id);

        let claims = Claims {
            bid: &self.bundle_id,
            iss: &self.issuer_id,
            aud: "appstoreconnect-v1",
            exp: future_time.timestamp(),
        };

        encode(
            &header,
            &claims,
            &EncodingKey::from_ec_pem(self.signing_key.as_slice()).unwrap(),
        )
        .unwrap()
    }

    pub(super) fn build_request<B: serde::Serialize>(
        &self,
        path: &str,
        method: Method,
        body: Option<&B>,
    ) -> Result<Request<Vec<u8>>, ApiServiceError<E>> {
        let (body_bytes, content_type) = if let Some(body_data) = body {
            let serialized = serde_json::to_vec(body_data).map_err(|_| ApiServiceError {
                http_status_code: 400,
                api_error: None,
                error_code: None,
                error_message: Some("Failed to serialize request body".to_string()),
            })?;
            (serialized, Some("application/json"))
        } else {
            (Vec::new(), None)
        };

        self.build_request_base(path, method, body_bytes, content_type)
    }

    pub(super) fn build_request_with_custom_content(
        &self,
        path: &str,
        method: Method,
        body: Vec<u8>,
        content_type: &str,
    ) -> Result<Request<Vec<u8>>, ApiServiceError<E>> {
        self.build_request_base(path, method, body, Some(content_type))
    }

    fn build_request_base(
        &self,
        path: &str,
        method: Method,
        body: Vec<u8>,
        content_type: Option<&str>,
    ) -> Result<Request<Vec<u8>>, ApiServiceError<E>> {
        let url = format!("{}{}", self.base_url, path);

        let mut request_builder = Request::builder()
            .method(method)
            .uri(url)
            .header("User-Agent", "app-store-server-library/rust/4.2.0")
            .header("Authorization", format!("Bearer {}", self.generate_token()))
            .header("Accept", "application/json");

        if let Some(ct) = content_type {
            request_builder = request_builder.header("Content-Type", ct);
        }

        request_builder
            .body(body)
            .map_err(|e| e.into())
    }

    pub(super) async fn make_request_with_response_body<Res>(&self, request: Request<Vec<u8>>) -> Result<Res, ApiServiceError<E>>
    where
        Res: for<'de> Deserialize<'de>,
    {
        let response = self.make_request(request).await?;
        let body = response.into_body();
        let json_result = serde_json::from_slice::<Res>(&body).map_err(|_| ApiServiceError {
            http_status_code: 500,
            api_error: None,
            error_code: None,
            error_message: Some("Failed to deserialize response JSON".to_string()),
        })?;
        Ok(json_result)
    }

    pub(super) async fn make_request_without_response_body(&self, request: Request<Vec<u8>>) -> Result<(), ApiServiceError<E>> {
        let _ = self.make_request(request).await?;
        Ok(())
    }

    pub(super) async fn make_request(&self, request: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, ApiServiceError<E>> {
        let response = self
            .transport
            .send(request).await?;

        let status_code = response.status().as_u16();

        if status_code >= 200 && status_code < 300 {
            Ok(response)
        } else {
            Err(self.extract_error(&response))
        }
    }

    pub(super) fn extract_error(&self, response: &Response<Vec<u8>>) -> ApiServiceError<E> {
        let status_code = response.status().as_u16();

        serde_json::from_slice::<ErrorPayload<E>>(response.body())
            .map(|payload| {
                ApiServiceError {
                    http_status_code: status_code,
                    api_error: Some(payload.error_code),
                    error_code: payload.raw_error_code,
                    error_message: payload.error_message,
                }
            })
            .unwrap_or_else(|_| ApiServiceError {
                http_status_code: status_code,
                api_error: None,
                error_code: None,
                error_message: Some("Failed to deserialize error JSON".to_string()),
            })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims<'a> {
    bid: &'a str,
    iss: &'a str,
    aud: &'a str,
    exp: i64,
}
