mod api;
mod notify;
mod structs;
use api::notify;
use axum::Router;
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let app = Router::new().route(
        "/send_notifications/",
        axum::routing::post(notify),
    );

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::POST])
        .allow_headers(Any);

    let app = app.layer(cors);

    api::api(app).await;
    Ok(())
}
