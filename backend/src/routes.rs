//! Route definitions

use axum::Router;
use tower_http::{
    cors::CorsLayer,
    limit::RequestBodyLimitLayer,
    trace::TraceLayer,
};

use crate::{handlers::analysis, middleware};
use crate::state::AppState;

/// Create all application routes
pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1/analysis", analysis::routes())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))
        .layer(axum::middleware::from_fn(middleware::request_id_middleware))
        .route("/health", axum::routing::get(health))
        .with_state(state)
}

/// Health check endpoint
async fn health() -> &'static str {
    "ok"
}
