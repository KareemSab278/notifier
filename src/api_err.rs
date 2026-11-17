use crate::structs::{self, ApiError};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::BadRequest(message) => (
                StatusCode::BAD_REQUEST,
                Json(structs::ErrorResponse { error: message }),
            )
                .into_response(),
            ApiError::Internal(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(structs::ErrorResponse { error: message }),
            )
                .into_response(),
        }
    }
}
