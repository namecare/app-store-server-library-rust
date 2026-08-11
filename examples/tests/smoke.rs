//! End-to-end smoke tests: boot each example server and drive it over HTTP.
//!
//! These run in DEMO_MODE, which trusts the bundled self-signed test CA. That
//! is what makes the positive path testable offline — the `testNotification`
//! fixture chains to that CA. No Apple-signed payload exists in this
//! repository, so real production verification cannot be exercised here.

use std::process::{Child, Command};
use std::time::Duration;

struct Server(Child);

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// Starts `bin` on `port` in demo mode and waits for it to answer /health.
async fn start(bin: &str, port: u16) -> Server {
    let child = Command::new(env!("CARGO"))
        .args(["run", "--quiet", "--bin", bin])
        .env("CARGO_MANIFEST_DIR", env!("CARGO_MANIFEST_DIR"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("DEMO_MODE", "1")
        .env("PORT", port.to_string())
        .env("BUNDLE_ID", "com.example")
        .env("APP_APPLE_ID", "1234")
        .env("ENVIRONMENT", "sandbox")
        .spawn()
        .expect("failed to spawn example server");

    let server = Server(child);
    let client = reqwest::Client::new();
    let health = format!("http://127.0.0.1:{}/health", port);

    // Building the binary can take a while on a cold CI cache.
    for _ in 0..120 {
        if client.get(&health).send().await.is_ok() {
            return server;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    panic!("{} did not become healthy within 60s", bin);
}

fn fixture() -> String {
    include_str!("../assets/testNotification").trim().to_string()
}

async fn assert_endpoints(port: u16) {
    let client = reqwest::Client::new();
    let notifications = format!("http://127.0.0.1:{}/notifications", port);

    // A valid, correctly-chained notification is accepted.
    let response = client
        .post(&notifications)
        .json(&serde_json::json!({ "signedPayload": fixture() }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    let decoded: serde_json::Value = response.json().await.unwrap();
    assert_eq!(decoded["notificationType"], "TEST");

    // A payload that is not a valid JWS is rejected.
    let response = client
        .post(&notifications)
        .json(&serde_json::json!({ "signedPayload": "not-a-jws" }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 401);

    // A body missing signedPayload is a client error.
    let response = client
        .post(&notifications)
        .json(&serde_json::json!({ "wrong": "field" }))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_client_error());

    // The signing endpoint returns a base64 signature.
    let response = client
        .post(format!("http://127.0.0.1:{}/promotional-offer", port))
        .json(&serde_json::json!({
            "productId": "com.example.pro",
            "offerId": "welcome",
            "applicationUsername": "user-1",
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);

    let body: serde_json::Value = response.json().await.unwrap();
    let signature = body["signature"].as_str().unwrap();
    assert!(!signature.is_empty());
    assert_eq!(body["keyIdentifier"], "DEMOKEYID");
}

#[tokio::test]
async fn axum_server_handles_both_endpoints() {
    let _server = start("axum_server", 18081).await;
    assert_endpoints(18081).await;
}

#[tokio::test]
async fn actix_server_handles_both_endpoints() {
    let _server = start("actix_server", 18082).await;
    assert_endpoints(18082).await;
}
