use crate::{notify as notification_sender, structs};
use axum::{Json, Router, extract::rejection::JsonRejection};
use tokio::net::TcpListener;

pub async fn api(app: Router) -> Result<(), std::io::Error> {
    // https://i.scdn.co/image/ab67616d00001e02136d6b463eb235abc20a9c4a
    let addr = "0.0.0.0:6767";
    let listener = TcpListener::bind(addr).await?;

    println!("Server running at http://{}", addr);
    axum::serve(listener, app).await
}

pub async fn notify(
    payload: Result<Json<Vec<structs::Notification>>, JsonRejection>,
) -> Result<Json<structs::ApiResponse>, structs::ApiError> {
    let Json(payload) =
        payload.map_err(|error| structs::ApiError::BadRequest(error.body_text()))?;

    notification_sender::validate_notifications_payload(&payload)
        .map_err(structs::ApiError::BadRequest)?;

    let result = notification_sender::send_notifications(payload)
        .await
        .map_err(|error| {
            eprintln!("Failed to send notification: {error}");
            structs::ApiError::Internal("Failed to send notification".to_string())
        })?;

    Ok(Json(structs::ApiResponse {
        success: true,
        message: format!(
            "Sent {} notification(s); {} failed",
            result.success, result.failed
        ),
    }))
}
