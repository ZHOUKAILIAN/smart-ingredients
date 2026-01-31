//! Custom middleware

use axum::{
    async_trait,
    extract::{FromRequestParts, MatchedPath, Request},
    http::{header, request::Parts, HeaderValue},
    middleware::Next,
    response::Response,
};
use std::{sync::OnceLock, time::Instant};
use tracing::info;
use uuid::Uuid;

use crate::{
    errors::{AppError, ErrorMeta},
    services::auth,
    state::AppState,
};

#[derive(Debug, Clone)]
pub struct RequestId(pub String);

static SERVICE_NAME: OnceLock<String> = OnceLock::new();
static DEPLOY_ENV: OnceLock<String> = OnceLock::new();

fn service_name() -> &'static str {
    SERVICE_NAME
        .get_or_init(|| std::env::var("SERVICE_NAME").unwrap_or_else(|_| "backend".to_string()))
        .as_str()
}

fn deploy_env() -> &'static str {
    DEPLOY_ENV
        .get_or_init(|| std::env::var("DEPLOY_ENV").unwrap_or_else(|_| "local".to_string()))
        .as_str()
}

/// Add request ID to all requests for tracing
pub async fn request_id_middleware(mut req: Request, next: Next) -> Response {
    let request_id = Uuid::new_v4().to_string();
    let header_value = HeaderValue::from_str(&request_id).unwrap();
    req.headers_mut()
        .insert("x-request-id", header_value.clone());
    req.extensions_mut().insert(RequestId(request_id));
    let mut response = next.run(req).await;
    response.headers_mut().insert("x-request-id", header_value);
    response
}

/// Log structured request/response metrics.
pub async fn trace_middleware(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    let request_id = req
        .extensions()
        .get::<RequestId>()
        .map(|id| id.0.clone())
        .unwrap_or_else(|| "-".to_string());
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map(|matched| matched.as_str().to_string())
        .unwrap_or_else(|| uri.path().to_string());

    let response = next.run(req).await;

    let status = response.status().as_u16();
    let latency_ms = start.elapsed().as_millis() as u64;
    let success = status < 400;

    if let Some(meta) = response.extensions().get::<ErrorMeta>() {
        info!(
            service = service_name(),
            env = deploy_env(),
            request_id,
            route,
            method = %method,
            status,
            latency_ms,
            success,
            error_code = meta.code,
            error_type = meta.error_type,
            "request"
        );
    } else {
        info!(
            service = service_name(),
            env = deploy_env(),
            request_id,
            route,
            method = %method,
            status,
            latency_ms,
            success,
            "request"
        );
    }

    response
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
}

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("missing authorization header".to_string()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("invalid authorization header".to_string()))?;

        let claims = auth::decode_access_token(&state.config.auth, token)
            .map_err(|_| AppError::Unauthorized("invalid token".to_string()))?;

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("invalid token subject".to_string()))?;

        Ok(Self { user_id })
    }
}

#[derive(Debug, Clone)]
pub struct OptionalAuthUser {
    pub user_id: Option<Uuid>,
}

#[async_trait]
impl FromRequestParts<AppState> for OptionalAuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = match parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
        {
            Some(value) => value,
            None => return Ok(Self { user_id: None }),
        };

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("invalid authorization header".to_string()))?;

        let claims = auth::decode_access_token(&state.config.auth, token)
            .map_err(|_| AppError::Unauthorized("invalid token".to_string()))?;

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("invalid token subject".to_string()))?;

        Ok(Self {
            user_id: Some(user_id),
        })
    }
}
