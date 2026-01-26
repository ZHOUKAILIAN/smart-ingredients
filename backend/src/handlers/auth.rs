//! Authentication handlers

use axum::{Json, Router};
use axum::extract::State;
use redis::AsyncCommands;
use shared::{AuthResponse, LogoutRequest, RefreshRequest, SendSmsRequest, SendSmsResponse, UserProfile, VerifySmsRequest};
use uuid::Uuid;

use crate::{
    config::SmsProvider,
    db,
    errors::AppError,
    services::auth,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/sms/send", axum::routing::post(send_sms))
        .route("/sms/verify", axum::routing::post(verify_sms))
        .route("/refresh", axum::routing::post(refresh_token))
        .route("/logout", axum::routing::post(logout))
}

async fn send_sms(
    State(state): State<AppState>,
    Json(payload): Json<SendSmsRequest>,
) -> Result<Json<SendSmsResponse>, AppError> {
    let phone = normalize_phone(&payload.phone)?;
    let phone_hash = auth::hash_phone(&phone, &state.config.auth.phone_hash_key)?;
    let mut redis = state.redis.clone();

    let lock_key = format!("sms:lock:{}", phone_hash);
    if redis.exists(&lock_key).await? {
        return Err(AppError::SmsLocked("验证码错误次数过多，请稍后再试".to_string()));
    }

    let cooldown_key = format!("sms:cooldown:{}", phone_hash);
    if redis.exists(&cooldown_key).await? {
        return Err(AppError::SmsCooldown("请稍后再试".to_string()));
    }

    let code = auth::generate_sms_code();
    let code_key = format!("sms:code:{}", phone_hash);
    let attempts_key = format!("sms:attempts:{}", phone_hash);

    let mut pipe = redis::pipe();
    pipe.atomic()
        .set_ex(&code_key, &code, state.config.auth.sms_code_ttl_seconds)
        .set_ex(&cooldown_key, 1, state.config.auth.sms_cooldown_seconds)
        .set_ex(&attempts_key, 0, state.config.auth.sms_lock_seconds);
    pipe.query_async::<()>(&mut redis).await?;

    match state.config.auth.sms_provider {
        SmsProvider::Mock => {
            tracing::info!("Mock SMS code sent to {}: {}", phone, code);
        }
    }

    let debug_code = if state.config.auth.sms_mock_return {
        Some(code)
    } else {
        None
    };

    Ok(Json(SendSmsResponse {
        success: true,
        cooldown_seconds: state.config.auth.sms_cooldown_seconds as i64,
        debug_code,
    }))
}

async fn verify_sms(
    State(state): State<AppState>,
    Json(payload): Json<VerifySmsRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let phone = normalize_phone(&payload.phone)?;
    let phone_hash = auth::hash_phone(&phone, &state.config.auth.phone_hash_key)?;
    let mut redis = state.redis.clone();

    let lock_key = format!("sms:lock:{}", phone_hash);
    if redis.exists(&lock_key).await? {
        return Err(AppError::SmsLocked("验证码错误次数过多，请稍后再试".to_string()));
    }

    let code_key = format!("sms:code:{}", phone_hash);
    let attempts_key = format!("sms:attempts:{}", phone_hash);
    let stored_code: Option<String> = redis.get(&code_key).await?;
    let stored_code = stored_code.ok_or_else(|| {
        AppError::SmsCodeExpired("验证码已过期，请重新获取".to_string())
    })?;

    if payload.code.trim() != stored_code {
        let attempts: i64 = redis.incr(&attempts_key, 1).await?;
        if attempts >= state.config.auth.sms_max_attempts as i64 {
            redis
                .set_ex::<_, _, ()>(&lock_key, 1, state.config.auth.sms_lock_seconds)
                .await?;
            redis.del::<_, ()>(&code_key).await?;
            return Err(AppError::SmsLocked("验证码错误次数过多，请稍后再试".to_string()));
        }
        return Err(AppError::SmsCodeInvalid("验证码错误".to_string()));
    }

    let _: i64 = redis.del(&code_key).await?;
    let _: i64 = redis.del(&attempts_key).await?;

    let user = match db::get_user_by_phone_hash(&state.pool, &phone_hash).await? {
        Some(user) => {
            db::update_user_last_login(&state.pool, user.id).await?;
            user
        }
        None => {
            let encrypted_phone = auth::encrypt_phone(&phone, &state.config.auth.phone_enc_key)?;
            let user = db::create_user(&state.pool, &encrypted_phone, &phone_hash).await?;
            db::ensure_user_preferences(&state.pool, user.id).await?;
            user
        }
    };

    let analysis_count = db::count_user_analyses(&state.pool, user.id).await?;
    let tokens = auth::issue_tokens(&state.config.auth, user.id)?;
    let refresh_key = format!("auth:refresh:{}", tokens.refresh_token);
    let user_tokens_key = format!("auth:user:{}:tokens", user.id);
    let mut redis = state.redis.clone();
    let refresh_ttl_seconds = (state.config.auth.refresh_ttl_days * 24 * 3600) as u64;
    let refresh_ttl_seconds_i64 = refresh_ttl_seconds as i64;
    redis
        .set_ex::<_, _, ()>(refresh_key, user.id.to_string(), refresh_ttl_seconds)
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
            phone_masked: auth::mask_phone(&phone),
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
    let user_id = user_id.ok_or_else(|| AppError::Unauthorized("invalid refresh token".to_string()))?;
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
    let phone = auth::decrypt_phone(&user.phone_encrypted, &state.config.auth.phone_enc_key)
        .unwrap_or_else(|_| "".to_string());

    Ok(Json(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_in: tokens.expires_in,
        user: UserProfile {
            id: user.id,
            phone_masked: auth::mask_phone(&phone),
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

fn normalize_phone(phone: &str) -> Result<String, AppError> {
    let trimmed = phone.trim();
    let digits: String = trimmed.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() < 8 || digits.len() > 15 {
        return Err(AppError::BadRequest("手机号格式错误".to_string()));
    }
    Ok(digits)
}
