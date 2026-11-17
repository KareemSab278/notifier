mod api;
mod api_err;
mod notify;
mod structs;
use axum::{Router, http::Method, routing::post};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/send_notifications", post(api::notify))
        .route("/send_notifications/", post(api::notify));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::POST])
        .allow_headers(Any);

    api::api(app.layer(cors)).await?;

    Ok(())
}
