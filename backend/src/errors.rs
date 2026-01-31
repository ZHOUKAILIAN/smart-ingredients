//! Error handling

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Clone)]
pub struct ErrorMeta {
    pub code: &'static str,
    pub error_type: &'static str,
}

/// Application error type
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unsupported media type: {0}")]
    UnsupportedMediaType(String),

    #[error("Payload too large: {0}")]
    PayloadTooLarge(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[allow(dead_code)]
    #[error("OCR error: {0}")]
    Ocr(String),

    #[allow(dead_code)]
    #[error("LLM error: {0}")]
    Llm(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[allow(dead_code)]
    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[allow(dead_code)]
    #[error("Rate limit exceeded: {0}")]
    RateLimited(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, error_type, message) = match &self {
            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, "BAD_REQUEST", "validation", msg)
            }
            AppError::UnsupportedMediaType(msg) => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "UNSUPPORTED_MEDIA_TYPE",
                "validation",
                msg,
            ),
            AppError::PayloadTooLarge(msg) => (
                StatusCode::PAYLOAD_TOO_LARGE,
                "PAYLOAD_TOO_LARGE",
                "validation",
                msg,
            ),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", "validation", msg),
            AppError::Ocr(msg) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "OCR_ERROR",
                "dependency",
                msg,
            ),
            AppError::Llm(msg) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "LLM_ERROR",
                "dependency",
                msg,
            ),
            AppError::Storage(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "STORAGE_ERROR",
                "dependency",
                msg,
            ),
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "internal",
                msg,
            ),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", "auth", msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, "FORBIDDEN", "auth", msg),
            AppError::RateLimited(msg) => (
                StatusCode::TOO_MANY_REQUESTS,
                "RATE_LIMIT_EXCEEDED",
                "rate_limit",
                msg,
            ),
        };

        error!(error_code = code, error_type, "App error: {}", self);

        let body = shared::ApiError::new(code, message);

        let mut response = (status, Json(body)).into_response();
        response
            .extensions_mut()
            .insert(ErrorMeta { code, error_type });
        response
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        AppError::Internal(err.to_string())
    }
}
