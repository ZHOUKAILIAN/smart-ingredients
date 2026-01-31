//! Authentication handlers

use axum::extract::State;
use axum::{Json, Router};
use redis::AsyncCommands;
use shared::{
    AuthResponse, LoginRequest, LogoutRequest, RefreshRequest, RegisterRequest, UserProfile,
};
use uuid::Uuid;

use crate::{db, errors::AppError, services::auth, state::AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", axum::routing::post(register))
        .route("/login", axum::routing::post(login))
        .route("/refresh", axum::routing::post(refresh_token))
        .route("/logout", axum::routing::post(logout))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let (username, normalized) = normalize_username(&payload.username)?;
    validate_password(&payload.password)?;

    if db::get_user_by_username(&state.pool, &normalized)
        .await?
        .is_some()
    {
        return Err(AppError::BadRequest("账号已存在".to_string()));
    }

    let password_hash = auth::hash_password(&payload.password)?;
    let user =
        db::create_user_with_password(&state.pool, &username, &normalized, &password_hash).await?;
    db::ensure_user_preferences(&state.pool, user.id).await?;

    let analysis_count = db::count_user_analyses(&state.pool, user.id).await?;
    let tokens = auth::issue_tokens(&state.config.auth, user.id)?;
    store_refresh_token(&state, user.id, &tokens.refresh_token).await?;

    Ok(Json(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_in: tokens.expires_in,
        user: UserProfile {
            id: user.id,
            login_id: user_login_id(&user),
            created_at: user.created_at.to_rfc3339(),
            analysis_count,
        },
    }))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let (_username, normalized) = normalize_username(&payload.username)?;
    validate_password(&payload.password)?;

    let login_hash = auth::hash_login_id(&normalized, &state.config.auth.login_hash_key)?;
    let lock_key = format!("auth:login:lock:{}", login_hash);
    let attempts_key = format!("auth:login:attempts:{}", login_hash);
    let mut redis = state.redis.clone();

    if redis.exists(&lock_key).await? {
        return Err(AppError::RateLimited(
            "登录错误次数过多，请稍后再试".to_string(),
        ));
    }

    let user = db::get_user_by_username(&state.pool, &normalized).await?;
    let Some(user) = user else {
        bump_login_attempts(&mut redis, &attempts_key, &lock_key, &state).await?;
        return Err(AppError::Unauthorized("账号或密码错误".to_string()));
    };

    let Some(password_hash) = user.password_hash.as_deref() else {
        return Err(AppError::Unauthorized("账号或密码错误".to_string()));
    };

    if !auth::verify_password(password_hash, &payload.password)? {
        bump_login_attempts(&mut redis, &attempts_key, &lock_key, &state).await?;
        return Err(AppError::Unauthorized("账号或密码错误".to_string()));
    }

    let _: i64 = redis.del(&attempts_key).await.unwrap_or(0);
    db::update_user_last_login(&state.pool, user.id).await?;

    let analysis_count = db::count_user_analyses(&state.pool, user.id).await?;
    let tokens = auth::issue_tokens(&state.config.auth, user.id)?;
    store_refresh_token(&state, user.id, &tokens.refresh_token).await?;

    Ok(Json(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_in: tokens.expires_in,
        user: UserProfile {
            id: user.id,
            login_id: user_login_id(&user),
            created_at: user.created_at.to_rfc3339(),
            analysis_count,
        },
    }))
}

async fn refresh_token(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    if payload.refresh_token.trim().is_empty() {
        return Err(AppError::BadRequest("refresh token required".to_string()));
    }

    let mut redis = state.redis.clone();
    let refresh_key = format!("auth:refresh:{}", payload.refresh_token);
    let user_id: Option<String> = redis.get(&refresh_key).await?;
    let user_id =
        user_id.ok_or_else(|| AppError::Unauthorized("invalid refresh token".to_string()))?;
    let user_id = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::Unauthorized("invalid refresh token".to_string()))?;

    let user = db::get_user_by_id(&state.pool, user_id)
        .await?
        .ok_or_else(|| AppError::Unauthorized("invalid refresh token".to_string()))?;

    let analysis_count = db::count_user_analyses(&state.pool, user.id).await?;
    let tokens = auth::issue_tokens(&state.config.auth, user.id)?;
    let new_refresh_key = format!("auth:refresh:{}", tokens.refresh_token);
    let user_tokens_key = format!("auth:user:{}:tokens", user.id);
    let _: i64 = redis.del(&refresh_key).await?;
    let _: i64 = redis.srem(&user_tokens_key, &payload.refresh_token).await?;
    let refresh_ttl_seconds = (state.config.auth.refresh_ttl_days * 24 * 3600) as u64;
    let refresh_ttl_seconds_i64 = refresh_ttl_seconds as i64;
    redis
        .set_ex::<_, _, ()>(new_refresh_key, user.id.to_string(), refresh_ttl_seconds)
        .await?;
    let _: i64 = redis.sadd(&user_tokens_key, &tokens.refresh_token).await?;
    let _: () = redis
        .expire::<_, ()>(&user_tokens_key, refresh_ttl_seconds_i64)
        .await?;
    Ok(Json(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_in: tokens.expires_in,
        user: UserProfile {
            id: user.id,
            login_id: user_login_id(&user),
            created_at: user.created_at.to_rfc3339(),
            analysis_count,
        },
    }))
}

async fn logout(
    State(state): State<AppState>,
    Json(payload): Json<LogoutRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if payload.refresh_token.trim().is_empty() {
        return Err(AppError::BadRequest("refresh token required".to_string()));
    }

    let mut redis = state.redis.clone();
    let refresh_key = format!("auth:refresh:{}", payload.refresh_token);
    if let Ok(user_id) = redis.get::<_, String>(&refresh_key).await {
        let user_tokens_key = format!("auth:user:{}:tokens", user_id);
        let _: i64 = redis.srem(&user_tokens_key, &payload.refresh_token).await?;
    }
    let _: i64 = redis.del(&refresh_key).await?;

    Ok(Json(serde_json::json!({ "success": true })))
}

fn normalize_username(username: &str) -> Result<(String, String), AppError> {
    let trimmed = username.trim();
    let length = trimmed.chars().count();
    if length < 4 || length > 20 {
        return Err(AppError::BadRequest("账号长度需为 4-20 位".to_string()));
    }
    if !trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err(AppError::BadRequest(
            "账号仅允许字母、数字和下划线".to_string(),
        ));
    }
    Ok((trimmed.to_string(), trimmed.to_lowercase()))
}

fn validate_password(password: &str) -> Result<(), AppError> {
    if password.trim().len() < 6 {
        return Err(AppError::BadRequest("密码长度至少 6 位".to_string()));
    }
    Ok(())
}

fn user_login_id(user: &db::UserRow) -> String {
    user.username.clone().unwrap_or_else(|| "用户".to_string())
}

async fn store_refresh_token(
    state: &AppState,
    user_id: Uuid,
    refresh_token: &str,
) -> Result<(), AppError> {
    let refresh_key = format!("auth:refresh:{}", refresh_token);
    let user_tokens_key = format!("auth:user:{}:tokens", user_id);
    let mut redis = state.redis.clone();
    let refresh_ttl_seconds = (state.config.auth.refresh_ttl_days * 24 * 3600) as u64;
    let refresh_ttl_seconds_i64 = refresh_ttl_seconds as i64;
    redis
        .set_ex::<_, _, ()>(refresh_key, user_id.to_string(), refresh_ttl_seconds)
        .await?;
    let _: i64 = redis.sadd(&user_tokens_key, refresh_token).await?;
    let _: () = redis
        .expire::<_, ()>(&user_tokens_key, refresh_ttl_seconds_i64)
        .await?;
    Ok(())
}

async fn bump_login_attempts(
    redis: &mut redis::aio::ConnectionManager,
    attempts_key: &str,
    lock_key: &str,
    state: &AppState,
) -> Result<(), AppError> {
    let attempts: i64 = redis.incr(attempts_key, 1).await?;
    if attempts == 1 {
        let _ = redis
            .expire::<_, ()>(attempts_key, state.config.auth.login_lock_seconds as i64)
            .await;
    }
    if attempts >= state.config.auth.login_max_attempts as i64 {
        redis
            .set_ex::<_, _, ()>(lock_key, 1, state.config.auth.login_lock_seconds)
            .await?;
    }
    Ok(())
}
