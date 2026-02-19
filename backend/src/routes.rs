//! Route definitions

use axum::{extract::DefaultBodyLimit, Router};
use tower_http::{cors::CorsLayer, limit::RequestBodyLimitLayer, services::ServeDir};

use crate::state::AppState;
use crate::{
    handlers::{analysis, auth, community, users},
    middleware,
};

/// Create all application routes
pub fn create_routes(state: AppState) -> Router {
    let upload_dir = state.config.upload_dir.clone();
    Router::new()
        .nest("/api/v1/analysis", analysis::routes())
        .nest("/api/v1/auth", auth::routes())
        .nest("/api/v1/users", users::routes())
        .nest("/api/v1/community", community::routes())
        .nest_service("/uploads", ServeDir::new(upload_dir))
        .layer(CorsLayer::permissive())
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))
        .route_layer(axum::middleware::from_fn(middleware::trace_middleware))
        .layer(axum::middleware::from_fn(middleware::request_id_middleware))
        .route("/health", axum::routing::get(health))
        .with_state(state)
}

/// Health check endpoint
async fn health() -> &'static str {
    "ok"
}
