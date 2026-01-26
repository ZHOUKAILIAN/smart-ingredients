//! User-related handlers

use axum::{
    extract::{Path, Query, State},
    Json, Router,
};
use serde::Deserialize;
use redis::AsyncCommands;
use shared::{
    BatchDeleteRequest, HistoryItem, HistoryPruneRequest, HistoryPruneResponse, HistoryResponse,
    LocalHistoryMigrateRequest, LocalHistoryMigrateResponse, UpdatePreferencesRequest,
    UserPreferences, UserProfile,
};
use uuid::Uuid;

use crate::{
    db,
    errors::AppError,
    middleware::AuthUser,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/me", axum::routing::get(get_profile))
        .route("/me", axum::routing::delete(delete_account))
        .route("/preferences", axum::routing::get(get_preferences))
        .route("/preferences", axum::routing::put(update_preferences))
        .route("/history", axum::routing::get(list_history))
        .route("/history", axum::routing::delete(delete_history_batch))
        .route("/history/batch", axum::routing::post(migrate_history_batch))
        .route("/history/prune", axum::routing::post(prune_history))
        .route("/history/:id", axum::routing::delete(delete_history))
}

async fn get_profile(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
) -> Result<Json<UserProfile>, AppError> {
    let user = db::get_user_by_id(&state.pool, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("user not found".to_string()))?;
    let phone = crate::services::auth::decrypt_phone(
        &user.phone_encrypted,
        &state.config.auth.phone_enc_key,
    )
    .unwrap_or_else(|_| "".to_string());
    let analysis_count = db::count_user_analyses(&state.pool, user.id).await?;

    Ok(Json(UserProfile {
        id: user.id,
        phone_masked: crate::services::auth::mask_phone(&phone),
        created_at: user.created_at.to_rfc3339(),
        analysis_count,
    }))
}

async fn delete_account(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut redis = state.redis.clone();
    let tokens_key = format!("auth:user:{}:tokens", user_id);
    let tokens: Vec<String> = redis.smembers(&tokens_key).await.unwrap_or_default();
    for token in tokens {
        let refresh_key = format!("auth:refresh:{}", token);
        let _: i64 = redis.del(&refresh_key).await?;
    }
    let _: i64 = redis.del(&tokens_key).await?;

    db::delete_user(&state.pool, user_id).await?;

    Ok(Json(serde_json::json!({ "success": true })))
}

async fn get_preferences(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
) -> Result<Json<UserPreferences>, AppError> {
    let prefs = db::get_user_preferences(&state.pool, user_id).await?;
    let prefs = match prefs {
        Some(prefs) => prefs,
        None => {
            db::ensure_user_preferences(&state.pool, user_id).await?;
            db::get_user_preferences(&state.pool, user_id)
                .await?
                .ok_or_else(|| AppError::Internal("failed to load preferences".to_string()))?
        }
    };

    Ok(Json(UserPreferences {
        preferences: prefs.preferences,
        updated_at: prefs.updated_at.to_rfc3339(),
    }))
}

async fn update_preferences(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Json(payload): Json<UpdatePreferencesRequest>,
) -> Result<Json<UserPreferences>, AppError> {
    let prefs = db::upsert_user_preferences(&state.pool, user_id, &payload.preferences).await?;

    Ok(Json(UserPreferences {
        preferences: prefs.preferences,
        updated_at: prefs.updated_at.to_rfc3339(),
    }))
}

async fn list_history(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Query(params): Query<HistoryQuery>,
) -> Result<Json<HistoryResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    let (total, rows) = db::list_user_history(&state.pool, user_id, limit, offset).await?;
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

async fn delete_history(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let deleted = db::delete_user_history(&state.pool, user_id, id).await?;
    if deleted == 0 {
        return Err(AppError::NotFound("history item not found".to_string()));
    }
    Ok(Json(serde_json::json!({ "success": true })))
}

async fn delete_history_batch(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Json(payload): Json<BatchDeleteRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if payload.ids.is_empty() {
        return Err(AppError::BadRequest("ids cannot be empty".to_string()));
    }
    db::delete_user_histories(&state.pool, user_id, &payload.ids).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

async fn migrate_history_batch(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Json(payload): Json<LocalHistoryMigrateRequest>,
) -> Result<Json<LocalHistoryMigrateResponse>, AppError> {
    if payload.ids.is_empty() {
        return Err(AppError::BadRequest("ids cannot be empty".to_string()));
    }

    let migrated = db::migrate_user_histories(&state.pool, user_id, &payload.ids).await?;
    let total_after = db::count_user_analyses(&state.pool, user_id).await?;
    let skipped = payload.ids.len() as i64 - migrated as i64;

    Ok(Json(LocalHistoryMigrateResponse {
        migrated: migrated as i64,
        skipped,
        total_after,
    }))
}

async fn prune_history(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Json(payload): Json<HistoryPruneRequest>,
) -> Result<Json<HistoryPruneResponse>, AppError> {
    if payload.delete_count <= 0 {
        return Err(AppError::BadRequest("delete_count must be positive".to_string()));
    }

    let deleted = db::prune_user_history(&state.pool, user_id, payload.delete_count).await?;
    let total_after = db::count_user_analyses(&state.pool, user_id).await?;

    Ok(Json(HistoryPruneResponse {
        deleted: deleted as i64,
        total_after,
    }))
}

#[derive(Debug, Deserialize)]
struct HistoryQuery {
    page: Option<i64>,
    limit: Option<i64>,
}
