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
    pub failed: Vec<u32>,
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, serde::Serialize)]
pub struct NotificationRequestResult {
    pub success: usize,
    pub failed: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::ApiResponse;

    #[test]
    fn api_response_serializes_failed_indexes() {
        let response = ApiResponse {
            success: true,
            message: "Sent 1 notification(s); 2 failed".to_string(),
            failed: vec![3, 8],
        };

        assert_eq!(
            serde_json::to_value(response).unwrap(),
            serde_json::json!({
                "success": true,
                "message": "Sent 1 notification(s); 2 failed",
                "failed": [3, 8]
            })
        );
    }
}
