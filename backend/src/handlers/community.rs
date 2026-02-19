//! Community post handlers

use axum::{
    extract::{Multipart, Path, Query, State},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    db,
    errors::AppError,
    middleware::OptionalAuthUser,
    services::{community, storage},
    state::AppState,
};
use shared::{
    CommunityCreatePayload, CommunityDeleteRequest, CommunityPostCreated, CommunityPostDetail,
    CommunityPostListItem, CommunityPostListResponse,
};

const MAX_COMMUNITY_IMAGE_BYTES: usize = 10 * 1024 * 1024;

#[derive(Deserialize)]
struct CommunityListQuery {
    page: Option<i64>,
    limit: Option<i64>,
}

/// Create community routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/posts", axum::routing::post(create_post).get(list_posts))
        .route(
            "/posts/:id",
            axum::routing::get(get_post).delete(delete_post),
        )
}

async fn create_post(
    State(state): State<AppState>,
    OptionalAuthUser { user_id: auth_user }: OptionalAuthUser,
    mut multipart: Multipart,
) -> Result<Json<CommunityPostCreated>, AppError> {
    let mut payload: Option<CommunityCreatePayload> = None;
    let mut image_bytes: Option<Vec<u8>> = None;
    let mut image_content_type: Option<String> = None;
    let mut image_filename: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|err| AppError::BadRequest(format!("解析上传字段失败: {}", err)))?
    {
        match field.name() {
            Some("payload") => {
                let text = field
                    .text()
                    .await
                    .map_err(|err| AppError::BadRequest(format!("读取 payload 失败: {}", err)))?;
                let parsed: CommunityCreatePayload = serde_json::from_str(&text)
                    .map_err(|_| AppError::BadRequest("payload 格式不正确".to_string()))?;
                payload = Some(parsed);
            }
            Some("card_image") => {
                let content_type = field.content_type().map(|ct| ct.to_string());
                let filename = field.file_name().map(|name| name.to_string());
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|err| AppError::BadRequest(format!("读取图片失败: {}", err)))?;
                if bytes.len() > MAX_COMMUNITY_IMAGE_BYTES {
                    let size_mb = bytes.len() as f64 / (1024.0 * 1024.0);
                    return Err(AppError::PayloadTooLarge(format!(
                        "图片文件过大（当前 {:.1}MB），请选择小于 10MB 的图片",
                        size_mb
                    )));
                }
                image_bytes = Some(bytes.to_vec());
                image_content_type = content_type;
                image_filename = filename;
            }
            _ => {}
        }
    }

    let payload = payload.ok_or_else(|| AppError::BadRequest("缺少 payload".to_string()))?;

    let validated = community::validate_create_payload(
        &payload,
        auth_user,
        &state.config.auth.login_hash_key,
    )?;

    let card_image_url = if let Some(bytes) = image_bytes {
        Some(
            storage::store_image(
                &bytes,
                image_content_type.as_deref(),
                &state.config.upload_dir,
                image_filename.as_deref(),
            )
            .await
            .map_err(|err| AppError::Storage(err.to_string()))?,
        )
    } else {
        None
    };

    let card_payload =
        serde_json::to_value(&payload.card_payload).map_err(|_| {
            AppError::BadRequest("卡片数据格式不正确".to_string())
        })?;

    let author_type = match validated.author_type {
        shared::CommunityAuthorType::Anonymous => "anonymous",
        shared::CommunityAuthorType::User => "user",
    };

    let (id, created_at, stored_image_url) = db::insert_community_post(
        &state.pool,
        author_type,
        validated.user_id,
        validated.share_token_hash.as_deref(),
        &payload.summary_text,
        payload.health_score,
        &payload.ingredients_raw,
        card_payload,
        card_image_url.as_deref(),
        payload.source_analysis_id,
    )
    .await?;

    Ok(Json(CommunityPostCreated {
        id,
        created_at: created_at.to_rfc3339(),
        card_image_url: stored_image_url,
    }))
}

async fn list_posts(
    State(state): State<AppState>,
    Query(params): Query<CommunityListQuery>,
) -> Result<Json<CommunityPostListResponse>, AppError> {
    let limit = params.limit.unwrap_or(20).clamp(1, 100);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * limit;

    let (total, rows) = db::list_community_posts(&state.pool, limit, offset).await?;
    let items = rows
        .into_iter()
        .map(|row| CommunityPostListItem {
            id: row.id,
            summary_text: row.summary_text,
            health_score: row.health_score,
            card_image_url: row.card_image_url,
            author_label: row.author_label,
            created_at: row.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(CommunityPostListResponse {
        total,
        page,
        limit,
        items,
    }))
}

async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<CommunityPostDetail>, AppError> {
    let row = db::get_community_post(&state.pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("帖子不存在".to_string()))?;

    let card_payload = serde_json::from_value(row.card_payload)
        .map_err(|_| AppError::Internal("卡片数据解析失败".to_string()))?;

    Ok(Json(CommunityPostDetail {
        id: row.id,
        summary_text: row.summary_text,
        health_score: row.health_score,
        ingredients_raw: row.ingredients_raw,
        card_payload,
        card_image_url: row.card_image_url,
        author_label: row.author_label,
        created_at: row.created_at.to_rfc3339(),
    }))
}

async fn delete_post(
    State(state): State<AppState>,
    OptionalAuthUser { user_id: auth_user }: OptionalAuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<CommunityDeleteRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let deleted = if let Some(user_id) = auth_user {
        db::delete_community_post_by_user(&state.pool, id, user_id).await?
    } else {
        let token = payload
            .share_token
            .filter(|t| !t.trim().is_empty())
            .ok_or_else(|| AppError::Unauthorized("缺少分享令牌".to_string()))?;
        let hash = community::hash_share_token(&token, &state.config.auth.login_hash_key)?;
        db::delete_community_post_by_token(&state.pool, id, &hash).await?
    };

    if deleted == 0 {
        return Err(AppError::NotFound("帖子不存在或无权限".to_string()));
    }

    Ok(Json(serde_json::json!({ "deleted": true })))
}
