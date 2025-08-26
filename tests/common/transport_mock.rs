use app_store_server_library::api_client::transport::{Transport, TransportError};
use http::StatusCode;

pub type RequestVerifier = Box<dyn Fn(&http::Request<Vec<u8>>, &Vec<u8>) -> () + Send + Sync>;

pub struct MockTransport {
    response_body: String,
    status_code: StatusCode,
    request_verifier: Option<RequestVerifier>,
}

impl MockTransport {
    pub fn new(body: String, status: StatusCode, verifier: Option<RequestVerifier>) -> Self {
        Self {
            response_body: body,
            status_code: status,
            request_verifier: verifier,
        }
    }
}

impl Transport for MockTransport {
    async fn send(&self, req: http::Request<Vec<u8>>) -> Result<http::Response<Vec<u8>>, TransportError> {
        let (parts, body) = req.into_parts();

        // Call the verifier if present
        if let Some(ref verifier) = self.request_verifier {
            verifier(
                &http::Request::from_parts(parts.clone(), body.clone()),
                &body,
            );
        }

        // Get the response body
        let body_str = self.response_body.clone();
        let response_body = body_str.into_bytes();

        // Build the response
        let response = http::Response::builder()
            .status(self.status_code)
            .header("Content-Type", "application/json")
            .body(response_body)
            .map_err(|e| TransportError::InvalidResponse(e.to_string()))?;

        Ok(response)
    }
}
