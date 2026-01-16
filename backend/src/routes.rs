//! Route definitions

use axum::Router;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};

use crate::handlers::analysis;

/// Create all application routes
pub fn create_routes() -> Router {
    Router::new()
        .nest("/api/v1/analysis", analysis::routes())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .route("/health", axum::routing::get(health))
}

/// Health check endpoint
async fn health() -> &'static str {
    "ok"
}
