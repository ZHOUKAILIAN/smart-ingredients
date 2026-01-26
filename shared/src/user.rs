//! Shared user/profile request/response types

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub preferences: serde_json::Value,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePreferencesRequest {
    pub preferences: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteRequest {
    pub ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalHistoryMigrateRequest {
    pub ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalHistoryMigrateResponse {
    pub migrated: i64,
    pub skipped: i64,
    pub total_after: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryPruneRequest {
    pub delete_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryPruneResponse {
    pub deleted: i64,
    pub total_after: i64,
}
