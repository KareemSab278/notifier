/*
    this is where the api layer will go. should accept cors headers from any origin in network

    should handle one request cleanly: /send_notifications/: POST request with notification payload in JSON format


    i am going to handle the /send_email/:email POST request with email payload in JSON format later but for now just use the /send_notifications/ endpoint
    should use the err thing i impl in api_err.rs. return either ApiResponse or ApiError

*/

use crate::structs;
use axum::{Json, Router, http::StatusCode};
use tokio::net::TcpListener;

pub async fn api(app: Router) -> () {
    
    let addr = "0.0.0.0:6767"; // https://i.scdn.co/image/ab67616d00001e02136d6b463eb235abc20a9c4a
    let listener = TcpListener::bind(addr).await.unwrap_or_else(|err| {
        eprintln!("Failed to bind to {}: {}", addr, err);
        kill_server();
    });

    println!("Server running at http://{}", addr);

    // listen and serve
    axum::serve(listener, app).await.unwrap_or_else(|err| {
        eprintln!("Server error: {}", err);
        kill_server();
    });
}

pub async fn notify(
    Json(payload): Json<structs::Notification>,
) -> Result<Json<structs::ApiResponse>, (StatusCode, Json<structs::ApiError>)> {
    let status: Result<crate::structs::NotificationRequestResult, Box<dyn std::error::Error>> =
        crate::notify::send_notifications(payload).await;

    if let Err(error) = status {
        eprintln!("Failed to send notification: {}", error);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(structs::ApiError::Internal(error.to_string())),
        ));
    }

    Ok(Json(structs::ApiResponse {
        success: true,
        message: "Notification sent successfully".to_string(),
    }))
}

fn kill_server() -> ! {
    std::process::exit(1);
}
