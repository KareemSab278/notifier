#[derive(serde::Deserialize)]
pub struct Notification {
    pub title: String,
    pub body: String,
    pub push_token: String,
}

#[derive(Debug, serde::Serialize)]
pub enum ApiError {
    BadRequest(String),
    Internal(String),
}

#[derive(serde::Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, serde::Serialize)]
pub struct NotificationRequestResult {
    pub success: usize,
    pub failed: usize,
}
