//! Actix-web example: an App Store Server Notifications webhook plus a
//! promotional-offer signing endpoint.
//!
//! Run with:
//!
//! ```sh
//! cargo run --manifest-path examples/Cargo.toml --bin actix_server
//! ```

use std::sync::Arc;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use app_store_server_library_examples::common::config;
use app_store_server_library_examples::common::error::AppError;
use app_store_server_library_examples::common::handlers::{
    handle_notification, handle_promotional_offer, state, AppState, NotificationRequest, PromotionalOfferRequest,
};

// `std::io::Error::other` (the fix clippy::io_other_error wants) was
// stabilized in Rust 1.74, but this crate's MSRV is 1.65.0. The
// `ErrorKind::Other` form below is the MSRV-compatible equivalent and is
// intentional - do not "helpfully" swap it back to `Error::other`.
#[allow(clippy::io_other_error)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::from_env().map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?;
    let port = config.port;
    let app_state =
        state(&config).map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error.to_string()))?;

    println!("[actix] listening on http://0.0.0.0:{}", port);
    println!("[actix]   POST /notifications");
    println!("[actix]   POST /promotional-offer");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&app_state)))
            .route("/health", web::get().to(health))
            .route("/notifications", web::post().to(notifications))
            .route("/promotional-offer", web::post().to(promotional_offer))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

async fn health() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

async fn notifications(
    app_state: web::Data<Arc<AppState>>,
    body: web::Json<NotificationRequest>,
) -> Result<HttpResponse, AppError> {
    let response = handle_notification(&app_state, &body)?;
    Ok(HttpResponse::Ok().json(response))
}

async fn promotional_offer(
    app_state: web::Data<Arc<AppState>>,
    body: web::Json<PromotionalOfferRequest>,
) -> Result<HttpResponse, AppError> {
    let nonce = uuid::Uuid::new_v4();
    let timestamp = chrono::Utc::now().timestamp_millis();

    let response = handle_promotional_offer(&app_state, &body, nonce, timestamp)?;
    Ok(HttpResponse::Ok().json(response))
}
