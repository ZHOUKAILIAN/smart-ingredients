//! Analysis request and response types

use crate::{AnalysisStatus, LlmStatus, OcrStatus};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request to upload and analyze an image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadRequest {
    /// Image data (base64 encoded or multipart)
    pub image_data: Vec<u8>,
    /// Optional filename
    pub filename: Option<String>,
}

/// Response after image upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResponse {
    /// Unique analysis ID
    pub id: Uuid,
    /// Current status
    pub status: AnalysisStatus,
    /// Image URL
    pub image_url: String,
}

/// Request to confirm OCR text and trigger LLM analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmRequest {
    /// User confirmed/edited text
    pub confirmed_text: String,
}

/// Analysis result from LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Health score (0-100)
    pub health_score: i32,
    /// Summary of the ingredient list
    #[serde(default)]
    pub summary: String,
    /// Table-friendly ingredient rows
    #[serde(default)]
    pub table: Vec<TableRow>,
    /// List of detected ingredients
    #[serde(default)]
    pub ingredients: Vec<IngredientInfo>,
    /// Warnings about harmful ingredients
    #[serde(default)]
    pub warnings: Vec<Warning>,
    /// Overall recommendation
    pub recommendation: String,
}

/// Information about a single ingredient
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngredientInfo {
    /// Ingredient name
    pub name: String,
    /// Category (additive, allergen, nutrition, etc.)
    pub category: String,
    /// Health risk level (low, medium, high)
    pub risk_level: String,
    /// Description of the ingredient
    pub description: Option<String>,
}

/// Ingredient row for table rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRow {
    /// Ingredient name
    pub name: String,
    /// Category (additive, allergen, nutrition, etc.)
    #[serde(default)]
    pub category: String,
    /// Ingredient function or role
    #[serde(default)]
    pub function: String,
    /// Health risk level (low, medium, high, unknown)
    #[serde(default)]
    pub risk_level: String,
    /// Additional note
    #[serde(default)]
    pub note: String,
}

/// Warning about harmful ingredients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warning {
    /// Type of warning
    pub warning_type: String,
    /// Affected ingredients
    pub ingredients: Vec<String>,
    /// Warning message
    pub message: String,
}

/// Full analysis response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResponse {
    /// Analysis ID
    pub id: Uuid,
    /// Current status
    pub status: AnalysisStatus,
    /// OCR status
    pub ocr_status: OcrStatus,
    /// LLM status
    pub llm_status: LlmStatus,
    /// Extracted OCR text
    pub ocr_text: Option<String>,
    /// User confirmed/edited text
    pub confirmed_text: Option<String>,
    /// OCR completion timestamp
    pub ocr_completed_at: Option<String>,
    /// Analysis result (available when completed)
    pub result: Option<AnalysisResult>,
    /// Error message (if failed)
    pub error_message: Option<String>,
    /// Creation timestamp
    pub created_at: String,
    /// Update timestamp
    pub updated_at: String,
}

/// History list item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryItem {
    /// Analysis ID
    pub id: Uuid,
    /// Image URL
    pub image_url: String,
    /// Health score
    pub health_score: Option<i32>,
    /// Creation timestamp
    pub created_at: String,
    /// Whether marked as favorite
    pub is_favorite: bool,
}

/// History list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryResponse {
    /// Total number of items
    pub total: i64,
    /// Current page number
    pub page: i64,
    /// Items per page
    pub limit: i64,
    /// History items
    pub items: Vec<HistoryItem>,
}
