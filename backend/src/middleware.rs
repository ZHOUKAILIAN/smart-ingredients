//! Custom middleware

use axum::{
    async_trait,
    extract::{FromRequestParts, Request},
    http::{header, request::Parts},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{errors::AppError, services::auth, state::AppState};

/// Add request ID to all requests for tracing
pub async fn request_id_middleware(
    mut req: Request,
    next: Next,
) -> Response {
    let request_id = Uuid::new_v4().to_string();
    req.headers_mut().insert(
        "x-request-id",
        request_id.parse().unwrap(),
    );
    next.run(req).await
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
