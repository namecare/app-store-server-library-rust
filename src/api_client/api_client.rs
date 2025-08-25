use crate::primitives::environment::Environment;
use crate::api_client::transport::Transport;
use crate::api_client::error::{APIServiceError, APIServiceErrorCode, ConfigurationError, ErrorPayload};

use chrono::Utc;
use http::Method;
use http::{Request, Response};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use serde::de::DeserializeOwned;

pub struct APIClient<T: Transport, API, E: APIServiceErrorCode + DeserializeOwned> {
    base_url: String,
    signing_key: Vec<u8>,
    key_id: String,
    issuer_id: String,
    bundle_id: String,
    transport: T,
    _api: PhantomData<API>,
    _api_error: PhantomData<E>,
}

unsafe impl<T: Transport, API, E: APIServiceErrorCode + DeserializeOwned> Send for APIClient<T, API, E> {}
unsafe impl<T: Transport, API, E: APIServiceErrorCode + DeserializeOwned> Sync for APIClient<T, API, E> {}

impl<T: Transport, API, E: APIServiceErrorCode + DeserializeOwned> APIClient<T, API, E> {
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
    ) -> Result<Request<Vec<u8>>, APIServiceError<E>> {
        let url = format!("{}{}", self.base_url, path);

        let mut request_builder = Request::builder()
            .method(method)
            .uri(url)
            .header("User-Agent", "app-store-server-library/rust/4.0.0")
            .header("Authorization", format!("Bearer {}", self.generate_token()))
            .header("Accept", "application/json");

        let body_bytes = if let Some(body_data) = body {
            request_builder = request_builder.header("Content-Type", "application/json");
            serde_json::to_vec(body_data).map_err(|_| APIServiceError {
                http_status_code: 400,
                api_error: None,
                error_message: Some("Failed to serialize request body".to_string()),
            })?
        } else {
            Vec::new()
        };

        request_builder
            .body(body_bytes)
            .map_err(|e| APIServiceError {
                http_status_code: 500,
                api_error: None,
                error_message: Some(format!("Failed to build request: {}", e)),
            })
    }

    pub(super) async fn make_request_with_response_body<Res>(&self, request: Request<Vec<u8>>) -> Result<Res, APIServiceError<E>>
    where
        Res: for<'de> Deserialize<'de>,
    {
        let response = self.make_request(request).await?;
        let body = response.into_body();
        let json_result = serde_json::from_slice::<Res>(&body).map_err(|_| APIServiceError {
            http_status_code: 500,
            api_error: None,
            error_message: Some("Failed to deserialize response JSON".to_string()),
        })?;
        Ok(json_result)
    }

    pub(super) async fn make_request_without_response_body(&self, request: Request<Vec<u8>>) -> Result<(), APIServiceError<E>> {
        let _ = self.make_request(request).await?;
        Ok(())
    }

    pub(super) async fn make_request(&self, request: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, APIServiceError<E>> {
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

    pub(super) fn extract_error(&self, response: &Response<Vec<u8>>) -> APIServiceError<E> {
        let status_code = response.status().as_u16();

        serde_json::from_slice::<ErrorPayload<E>>(response.body())
            .map(|payload| {
                APIServiceError {
                    http_status_code: status_code,
                    api_error: payload.error_code,
                    error_message: payload.error_message,
                }
            })
            .unwrap_or_else(|_| APIServiceError {
                http_status_code: status_code,
                api_error: None,
                error_message: None,
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
