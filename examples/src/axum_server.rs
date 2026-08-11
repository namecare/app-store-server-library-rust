//! Axum example: an App Store Server Notifications webhook plus a
//! promotional-offer signing endpoint.
//!
//! Run with:
//!
//! ```sh
//! cargo run --manifest-path examples/Cargo.toml --bin axum_server
//! ```

use app_store_server_library_examples::common::error::AppError;
use app_store_server_library_examples::common::handlers::{
    handle_notification, handle_promotional_offer, state, AppState, NotificationRequest,
    NotificationResponse, PromotionalOfferRequest, PromotionalOfferResponse,
};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use std::sync::Arc;
use app_store_server_library_examples::common::config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::from_env()?;
    let port = config.port;
    let app_state = state(&config)?;

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await?;
    println!("[axum] listening on http://0.0.0.0:{}", port);
    println!("[axum]   POST /notifications");
    println!("[axum]   POST /promotional-offer");

    axum::serve(listener, router(app_state)).await?;
    Ok(())
}

/// Builds the router. Separated from `main` so tests can mount it directly.
pub fn router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/notifications", post(notifications))
        .route("/promotional-offer", post(promotional_offer))
        .with_state(app_state)
}

async fn notifications(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<NotificationRequest>,
) -> Result<Json<NotificationResponse>, AppError> {
    handle_notification(&app_state, &body).map(Json)
}

async fn promotional_offer(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<PromotionalOfferRequest>,
) -> Result<Json<PromotionalOfferResponse>, AppError> {
    let nonce = uuid::Uuid::new_v4();
    let timestamp = chrono::Utc::now().timestamp_millis();

    handle_promotional_offer(&app_state, &body, nonce, timestamp).map(Json)
}
