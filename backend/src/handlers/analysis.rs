//! Analysis-related handlers

use axum::{
    extract::{Multipart, Path, Query, State},
    Json,
    Router,
};
use serde::Deserialize;
use shared::{
    AnalysisResponse, AnalysisResult, AnalysisStatus, HistoryItem, HistoryResponse, TableRow,
    UploadResponse,
};
use uuid::Uuid;

use crate::{
    db,
    errors::AppError,
    services::{ocr, storage},
    state::AppState,
};

/// Create analysis routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/upload", axum::routing::post(upload_handler))
        .route("/:id", axum::routing::get(get_handler))
        .route("/:id/analyze", axum::routing::post(analyze_handler))
        .route("/history", axum::routing::get(history_handler))
}

/// Upload image for analysis
async fn upload_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
    let mut file_bytes = None;
    let mut filename = None;
    let mut content_type = None;

    while let Some(field) = multipart.next_field().await.map_err(|err| {
        AppError::BadRequest(format!("invalid multipart field: {}", err))
    })? {
        if field.name() != Some("file") {
            continue;
        }

        filename = field.file_name().map(|name| name.to_string());
        content_type = field.content_type().map(|ct| ct.to_string());

        let bytes = field.bytes().await.map_err(|err| {
            AppError::BadRequest(format!("failed to read file: {}", err))
        })?;

        if bytes.len() > MAX_UPLOAD_BYTES {
            return Err(AppError::PayloadTooLarge("file exceeds 10MB".to_string()));
        }

        file_bytes = Some(bytes.to_vec());
        break;
    }

    let file_bytes = file_bytes.ok_or_else(|| {
        AppError::BadRequest("missing file field".to_string())
    })?;

    let extension = validate_content_type(content_type.as_deref())?;
    let image_url = storage::store_image(
        &file_bytes,
        &state.config.upload_dir,
        filename.as_deref(),
        extension,
    )
    .await
    .map_err(|err| AppError::Storage(err.to_string()))?;

    let id = db::insert_analysis(&state.pool, &image_url).await?;

    Ok(Json(UploadResponse {
        id,
        status: AnalysisStatus::Pending,
        image_url,
    }))
}

/// Get analysis status and result
async fn get_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AnalysisResponse>, AppError> {
    let row = db::get_analysis(&state.pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("analysis not found".to_string()))?;

    Ok(Json(to_analysis_response(&row)))
}

/// Run OCR + LLM analysis
async fn analyze_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AnalysisResponse>, AppError> {
    let row = db::get_analysis(&state.pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("analysis not found".to_string()))?;

    db::update_analysis_status(&state.pool, id, "processing").await?;

    let image_path =
        storage::resolve_image_path(&state.config.upload_dir, &row.image_url)?;
    let ocr_text = ocr::extract_text(&image_path, &state.config.ocr)
        .await
        .map_err(|err| AppError::Ocr(err.to_string()))?;

    let ocr_text = ocr_text.trim().to_string();
    if ocr_text.is_empty() || ocr_text.len() > MAX_TEXT_LENGTH {
        db::update_analysis_result(
            &state.pool,
            id,
            "failed",
            None,
            None,
            Some("OCR text length invalid".to_string()),
        )
        .await?;
        return Err(AppError::BadRequest(
            "OCR text length must be 1-5000".to_string(),
        ));
    }

    db::update_analysis_text(&state.pool, id, &ocr_text, "processing").await?;

    let result = match state.llm.analyze_ingredients(&ocr_text).await {
        Ok(result) => result,
        Err(err) => {
            db::update_analysis_result(
                &state.pool,
                id,
                "failed",
                None,
                None,
                Some(err.to_string()),
            )
            .await?;
            return Err(AppError::Llm(err.to_string()));
        }
    };

    let result = ensure_summary_table(result);
    let result_json = serde_json::to_value(&result)
        .map_err(|err| AppError::Internal(err.to_string()))?;

    db::update_analysis_result(
        &state.pool,
        id,
        "completed",
        Some(result.health_score),
        Some(result_json),
        None,
    )
    .await?;

    let updated = db::get_analysis(&state.pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("analysis not found".to_string()))?;

    Ok(Json(to_analysis_response(&updated)))
}

/// Get analysis history
async fn history_handler(
    State(state): State<AppState>,
    Query(params): Query<HistoryQuery>,
) -> Result<Json<HistoryResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    let (total, rows) = db::list_history(&state.pool, limit, offset).await?;
    let items = rows
        .into_iter()
        .map(|row| HistoryItem {
            id: row.id,
            image_url: row.image_url,
            health_score: row.health_score,
            created_at: row.created_at.to_rfc3339(),
            is_favorite: false,
        })
        .collect();

    Ok(Json(HistoryResponse {
        total,
        page,
        limit,
        items,
    }))
}

#[derive(Debug, Deserialize)]
struct HistoryQuery {
    page: Option<i64>,
    limit: Option<i64>,
}

const MAX_UPLOAD_BYTES: usize = 10 * 1024 * 1024;
const MAX_TEXT_LENGTH: usize = 5000;

fn validate_content_type(content_type: Option<&str>) -> Result<Option<&'static str>, AppError> {
    match content_type {
        Some("image/jpeg") => Ok(Some("jpg")),
        Some("image/png") => Ok(Some("png")),
        Some("image/webp") => Ok(Some("webp")),
        Some(other) => Err(AppError::UnsupportedMediaType(format!(
            "unsupported content type: {}",
            other
        ))),
        None => Ok(None),
    }
}

fn to_analysis_response(row: &db::AnalysisRow) -> AnalysisResponse {
    AnalysisResponse {
        id: row.id,
        status: parse_status(&row.status),
        ocr_text: row.text.clone(),
        result: row
            .result
            .as_ref()
            .and_then(|value| serde_json::from_value::<AnalysisResult>(value.clone()).ok()),
        error_message: row.error_message.clone(),
        created_at: row.created_at.to_rfc3339(),
    }
}

fn parse_status(status: &str) -> AnalysisStatus {
    match status {
        "processing" => AnalysisStatus::Processing,
        "completed" => AnalysisStatus::Completed,
        "failed" => AnalysisStatus::Failed,
        _ => AnalysisStatus::Pending,
    }
}

fn ensure_summary_table(mut result: AnalysisResult) -> AnalysisResult {
    if result.summary.trim().is_empty() {
        let count = result.ingredients.len();
        result.summary = if count == 0 {
            "未识别到配料信息，请尝试上传更清晰的图片。".to_string()
        } else {
            format!("识别到 {} 项配料，可查看表格详情。", count)
        };
    }

    if result.table.is_empty() {
        result.table = result
            .ingredients
            .iter()
            .map(|item| TableRow {
                name: item.name.clone(),
                category: item.category.clone(),
                function: item.description.clone().unwrap_or_default(),
                risk_level: item.risk_level.clone(),
                note: String::new(),
            })
            .collect();
    }

    result
}
