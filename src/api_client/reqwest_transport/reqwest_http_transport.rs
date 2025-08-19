use crate::api_client::transport::{Transport, TransportError};
use reqwest::Client;

impl From<reqwest::Error> for TransportError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            TransportError::Timeout
        } else if err.is_connect() {
            TransportError::NetworkError(format!("Connection failed: {}", err))
        } else if err.is_request() {
            TransportError::RequestFailed(format!("Request error: {}", err))
        } else {
            TransportError::Other(err.to_string())
        }
    }
}

#[derive(Clone)]
pub struct ReqwestHttpTransport {
    client: Client,
}

impl ReqwestHttpTransport {
    pub fn new() -> ReqwestHttpTransport {
        Self {
            client: Client::new()
        }
    }
}

impl Transport for ReqwestHttpTransport {
    async fn send(&self, req: http::Request<Vec<u8>>) -> Result<http::Response<Vec<u8>>, TransportError> {
        let (parts, body_bytes) = req.into_parts();

        let mut reqwest_request = self
            .client
            .request(parts.method, parts.uri.to_string());

        for (name, value) in parts.headers.iter() {
            reqwest_request = reqwest_request.header(name.as_str(), value.as_bytes());
        }

        reqwest_request = reqwest_request.body(body_bytes);

        let response = reqwest_request.send().await?;

        let status = http::StatusCode::from_u16(response.status().as_u16())?;

        let mut http_response_builder = http::Response::builder().status(status);

        for (name, value) in response.headers().iter() {
            http_response_builder = http_response_builder.header(name.as_str(), value.as_bytes());
        }

        let body_bytes = response.bytes().await?.to_vec();

        http_response_builder
            .body(body_bytes)
            .map_err(|e| TransportError::InvalidResponse(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_client::transport::Transport;

    #[tokio::test]
    async fn test_reqwest_http_transport_basic_request() {
        let transport = ReqwestHttpTransport::new();

        // Create a simple GET request to example.com
        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri("https://example.com")
            .header("User-Agent", "test-agent")
            .body(Vec::new())
            .expect("Failed to build request");

        // Send the request
        let response = transport.send(request).await;

        // Assert the request was successful
        assert!(response.is_ok(), "Request failed: {:?}", response.err());

        let response = response.unwrap();

        // Check status code (example.com should return 200)
        assert_eq!(
            response.status(),
            http::StatusCode::OK,
            "Unexpected status code"
        );

        // Check that we got a body
        let body = response.body();
        assert!(!body.is_empty(), "Response body should not be empty");

        // Check that the body contains expected content
        let body_str = String::from_utf8_lossy(body);
        assert!(
            body_str.contains("Example Domain"),
            "Response should contain 'Example Domain'"
        );
    }

    #[tokio::test]
    async fn test_reqwest_http_transport_with_body() {
        let transport = ReqwestHttpTransport::new();

        // Create a POST request with JSON body to httpbin.org
        let body = r#"{"test": "data"}"#.as_bytes().to_vec();
        let request = http::Request::builder()
            .method(http::Method::POST)
            .uri("https://httpbin.org/post")
            .header("Content-Type", "application/json")
            .header("User-Agent", "test-agent")
            .body(body)
            .expect("Failed to build request");

        // Send the request
        let response = transport.send(request).await;

        // Assert the request was successful
        assert!(response.is_ok(), "Request failed: {:?}", response.err());

        let response = response.unwrap();

        // Check status code
        assert_eq!(
            response.status(),
            http::StatusCode::OK,
            "Unexpected status code"
        );

        // Check that we got a body
        let body = response.body();
        assert!(!body.is_empty(), "Response body should not be empty");

        // httpbin.org echoes back the data we sent
        let body_str = String::from_utf8_lossy(body);
        assert!(
            body_str.contains(r#""test": "data""#),
            "Response should contain our test data"
        );
    }
}
