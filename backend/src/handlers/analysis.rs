//! Analysis-related handlers

use axum::{
    extract::{Multipart, Path, Query, State},
    Json,
    Router,
};
use serde::Deserialize;
use shared::{
    AnalysisResponse, AnalysisResult, AnalysisStatus, ConfirmRequest, HistoryItem, HistoryResponse,
    LlmStatus, OcrStatus, TableRow, UploadResponse,
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
        .route("/:id/confirm", axum::routing::post(confirm_handler))
        .route("/:id/retry-ocr", axum::routing::post(retry_ocr_handler))
        .route("/:id/retry-llm", axum::routing::post(retry_llm_handler))
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
            let size_mb = bytes.len() as f64 / (1024.0 * 1024.0);
            return Err(AppError::PayloadTooLarge(format!(
                "图片文件过大（当前 {:.1}MB），请选择小于 10MB 的图片",
                size_mb
            )));
        }

        file_bytes = Some(bytes.to_vec());
        break;
    }

    let file_bytes = file_bytes.ok_or_else(|| {
        AppError::BadRequest("缺少文件字段".to_string())
    })?;

    // Basic validation of Content-Type
    validate_content_type(content_type.as_deref())?;

    // Store image (will auto-detect format and convert if needed)
    let image_url = storage::store_image(
        &file_bytes,
        content_type.as_deref(),
        &state.config.upload_dir,
        filename.as_deref(),
    )
    .await
    .map_err(|err| AppError::Storage(err.to_string()))?;

    let id = db::insert_analysis(&state.pool, &image_url).await?;

    let pool = state.pool.clone();
    let config = state.config.clone();
    let image_url_clone = image_url.clone();
    tokio::spawn(async move {
        run_ocr_task(pool, config, id, image_url_clone).await;
    });

    Ok(Json(UploadResponse {
        id,
        status: AnalysisStatus::OcrPending,
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

/// Confirm OCR text and start LLM analysis
async fn confirm_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ConfirmRequest>,
) -> Result<Json<AnalysisResponse>, AppError> {
    let row = db::get_analysis(&state.pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("analysis not found".to_string()))?;

    if row.ocr_status != "completed" {
        return Err(AppError::BadRequest(
            "OCR not completed yet".to_string(),
        ));
    }

    let confirmed_text = payload.confirmed_text.trim().to_string();
    if confirmed_text.is_empty() || confirmed_text.len() > MAX_TEXT_LENGTH {
        return Err(AppError::BadRequest(
            "confirmed text length must be 1-5000".to_string(),
        ));
    }

    db::update_confirmed_text(&state.pool, id, &confirmed_text, "llm_pending")
        .await?;

    let pool = state.pool.clone();
    let llm = state.llm.clone();
    tokio::spawn(async move {
        run_llm_task(pool, llm, id, confirmed_text).await;
    });

    let updated = db::get_analysis(&state.pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("analysis not found".to_string()))?;

    Ok(Json(to_analysis_response(&updated)))
}

/// Retry OCR step
async fn retry_ocr_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AnalysisResponse>, AppError> {
    let row = db::get_analysis(&state.pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("analysis not found".to_string()))?;

    let pool = state.pool.clone();
    let config = state.config.clone();
    tokio::spawn(async move {
        run_ocr_task(pool, config, id, row.image_url).await;
    });

    let updated = db::get_analysis(&state.pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("analysis not found".to_string()))?;

    Ok(Json(to_analysis_response(&updated)))
}

/// Retry LLM step
async fn retry_llm_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AnalysisResponse>, AppError> {
    let row = db::get_analysis(&state.pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("analysis not found".to_string()))?;

    let confirmed_text = row.confirmed_text.clone().ok_or_else(|| {
        AppError::BadRequest("missing confirmed text".to_string())
    })?;

    db::update_llm_status(&state.pool, id, "pending", "llm_pending", None)
        .await?;

    let pool = state.pool.clone();
    let llm = state.llm.clone();
    tokio::spawn(async move {
        run_llm_task(pool, llm, id, confirmed_text).await;
    });

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

fn validate_content_type(content_type: Option<&str>) -> Result<(), AppError> {
    // Only do basic validation, actual format is auto-detected by image crate
    match content_type {
        Some(ct) if ct.starts_with("image/") => Ok(()),
        Some(other) => Err(AppError::UnsupportedMediaType(format!(
            "不支持的文件类型: {}。请上传图片文件（支持 JPEG、PNG、HEIC、HEIF、BMP、WebP、GIF、TIFF、SVG 格式）",
            other
        ))),
        None => Ok(()), // Allow no Content-Type, will be auto-detected
    }
}

fn to_analysis_response(row: &db::AnalysisRow) -> AnalysisResponse {
    AnalysisResponse {
        id: row.id,
        status: parse_status(&row.status),
        ocr_status: parse_ocr_status(&row.ocr_status),
        llm_status: parse_llm_status(&row.llm_status),
        ocr_text: row.ocr_text.clone(),
        confirmed_text: row.confirmed_text.clone(),
        ocr_completed_at: row
            .ocr_completed_at
            .as_ref()
            .map(|ts| ts.to_rfc3339()),
        result: row
            .result
            .as_ref()
            .and_then(|value| serde_json::from_value::<AnalysisResult>(value.clone()).ok()),
        error_message: row.error_message.clone(),
        created_at: row.created_at.to_rfc3339(),
        updated_at: row.updated_at.to_rfc3339(),
    }
}

fn parse_status(status: &str) -> AnalysisStatus {
    match status {
        "ocr_pending" => AnalysisStatus::OcrPending,
        "ocr_processing" => AnalysisStatus::OcrProcessing,
        "ocr_completed" => AnalysisStatus::OcrCompleted,
        "ocr_failed" => AnalysisStatus::OcrFailed,
        "llm_pending" => AnalysisStatus::LlmPending,
        "llm_processing" => AnalysisStatus::LlmProcessing,
        "completed" => AnalysisStatus::Completed,
        "failed" => AnalysisStatus::Failed,
        _ => AnalysisStatus::OcrPending,
    }
}

fn parse_ocr_status(status: &str) -> OcrStatus {
    match status {
        "processing" => OcrStatus::Processing,
        "completed" => OcrStatus::Completed,
        "failed" => OcrStatus::Failed,
        _ => OcrStatus::Pending,
    }
}

fn parse_llm_status(status: &str) -> LlmStatus {
    match status {
        "processing" => LlmStatus::Processing,
        "completed" => LlmStatus::Completed,
        "failed" => LlmStatus::Failed,
        _ => LlmStatus::Pending,
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

async fn run_ocr_task(
    pool: sqlx::PgPool,
    config: crate::config::AppConfig,
    analysis_id: Uuid,
    image_url: String,
) {
    let _ = db::update_ocr_status(
        &pool,
        analysis_id,
        "processing",
        "ocr_processing",
        None,
    )
    .await;

    let image_path = match storage::resolve_image_path(&config.upload_dir, &image_url) {
        Ok(path) => path,
        Err(err) => {
            let _ = db::update_ocr_status(
                &pool,
                analysis_id,
                "failed",
                "ocr_failed",
                Some(err.to_string()),
            )
            .await;
            return;
        }
    };

    let ocr_text = match ocr::extract_text(&image_path, &config.ocr).await {
        Ok(text) => text.trim().to_string(),
        Err(err) => {
            let _ = db::update_ocr_status(
                &pool,
                analysis_id,
                "failed",
                "ocr_failed",
                Some(err.to_string()),
            )
            .await;
            return;
        }
    };

    if ocr_text.is_empty() || ocr_text.len() > MAX_TEXT_LENGTH {
        let _ = db::update_ocr_status(
            &pool,
            analysis_id,
            "failed",
            "ocr_failed",
            Some("OCR text length invalid".to_string()),
        )
        .await;
        return;
    }

    let _ = db::save_ocr_result(&pool, analysis_id, &ocr_text, "ocr_completed")
        .await;
}

async fn run_llm_task(
    pool: sqlx::PgPool,
    llm: std::sync::Arc<dyn crate::services::llm::LlmProviderClient>,
    analysis_id: Uuid,
    text: String,
) {
    let _ = db::update_llm_status(
        &pool,
        analysis_id,
        "processing",
        "llm_processing",
        None,
    )
    .await;

    let result = match llm.analyze_ingredients(&text).await {
        Ok(result) => result,
        Err(err) => {
            let _ = db::update_llm_status(
                &pool,
                analysis_id,
                "failed",
                "failed",
                Some(err.to_string()),
            )
            .await;
            return;
        }
    };

    let result = ensure_summary_table(result);
    let result_json = match serde_json::to_value(&result) {
        Ok(value) => value,
        Err(err) => {
            let _ = db::update_llm_status(
                &pool,
                analysis_id,
                "failed",
                "failed",
                Some(err.to_string()),
            )
            .await;
            return;
        }
    };

    let _ = db::update_analysis_result(
        &pool,
        analysis_id,
        "completed",
        Some(result.health_score),
        Some(result_json),
        None,
    )
    .await;
}
