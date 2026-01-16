//! Shared error types

use serde::{Deserialize, Serialize};

/// API error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// Error code
    pub code: String,
    /// Human-readable message
    pub message: String,
    /// Additional details
    pub details: Option<serde_json::Value>,
    /// Request ID for tracing
    pub request_id: Option<String>,
}

/// Error codes
pub mod error_codes {
    pub const BAD_REQUEST: &str = "BAD_REQUEST";
    pub const UNAUTHORIZED: &str = "UNAUTHORIZED";
    pub const NOT_FOUND: &str = "NOT_FOUND";
    pub const CONFLICT: &str = "CONFLICT";
    pub const PAYLOAD_TOO_LARGE: &str = "PAYLOAD_TOO_LARGE";
    pub const UNSUPPORTED_MEDIA_TYPE: &str = "UNSUPPORTED_MEDIA_TYPE";
    pub const RATE_LIMIT_EXCEEDED: &str = "RATE_LIMIT_EXCEEDED";
    pub const INTERNAL_ERROR: &str = "INTERNAL_ERROR";
    pub const SERVICE_UNAVAILABLE: &str = "SERVICE_UNAVAILABLE";
    pub const OCR_ERROR: &str = "OCR_ERROR";
    pub const OCR_TIMEOUT: &str = "OCR_TIMEOUT";
    pub const LLM_ERROR: &str = "LLM_ERROR";
    pub const LLM_TIMEOUT: &str = "LLM_TIMEOUT";
    pub const STORAGE_ERROR: &str = "STORAGE_ERROR";
}

impl ApiError {
    /// Create a new API error
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
            request_id: None,
        }
    }

    /// Add details to the error
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    /// Add request ID to the error
    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }
}
