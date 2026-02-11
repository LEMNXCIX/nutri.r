use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use nutri_core::utils::error::AppError;
use serde_json::json;

pub struct ApiError(pub AppError);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self.0 {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Configuration(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::Ollama(msg) => (StatusCode::BAD_GATEWAY, msg.clone()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::Network(msg) => (StatusCode::BAD_GATEWAY, msg.clone()),
            AppError::Serialization(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        let body = Json(json!({
            "error": {
                "message": message,
                "type": format!("{:?}", self.0) // This might output e.g. "NotFound(\"foo\")"
            }
        }));

        (status, body).into_response()
    }
}

impl From<AppError> for ApiError {
    fn from(err: AppError) -> Self {
        ApiError(err)
    }
}
