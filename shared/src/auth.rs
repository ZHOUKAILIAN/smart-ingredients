//! Shared auth request/response types

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendSmsRequest {
    pub phone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendSmsResponse {
    pub success: bool,
    pub cooldown_seconds: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifySmsRequest {
    pub phone: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user: UserProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub phone_masked: String,
    pub created_at: String,
    pub analysis_count: i64,
}
