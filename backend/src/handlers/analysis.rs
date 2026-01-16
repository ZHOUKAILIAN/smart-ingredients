//! Analysis-related handlers

use axum::{
    extract::Path,
    Json,
    Router,
};
use shared::{AnalysisStatus, UploadResponse};
use uuid::Uuid;

/// Create analysis routes
pub fn routes() -> Router {
    Router::new()
        .route("/upload", axum::routing::post(upload_handler))
        .route("/:id", axum::routing::get(get_handler))
        .route("/history", axum::routing::get(history_handler))
}

/// Upload image for analysis
async fn upload_handler() -> Result<Json<UploadResponse>, crate::errors::AppError> {
    // TODO: Implement image upload and analysis
    let id = Uuid::new_v4();
    Ok(Json(UploadResponse {
        id,
        status: AnalysisStatus::Pending,
        image_url: "https://example.com/image.jpg".to_string(),
    }))
}

/// Get analysis status and result
async fn get_handler(
    Path(id): Path<Uuid>,
) -> Result<Json<shared::AnalysisResponse>, crate::errors::AppError> {
    // TODO: Implement analysis retrieval
    Ok(Json(shared::AnalysisResponse {
        id,
        status: AnalysisStatus::Pending,
        ocr_text: None,
        result: None,
        error_message: None,
        created_at: chrono::Utc::now().to_rfc3339(),
    }))
}

/// Get analysis history
async fn history_handler() -> Result<Json<shared::HistoryResponse>, crate::errors::AppError> {
    // TODO: Implement history retrieval
    Ok(Json(shared::HistoryResponse {
        total: 0,
        page: 1,
        limit: 20,
        items: vec![],
    }))
}
