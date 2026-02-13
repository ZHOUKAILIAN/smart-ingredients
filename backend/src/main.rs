//! Smart Ingredients Backend Server
//!
//! Axum-based web server for image analysis

mod config;
mod db;
mod errors;
mod handlers;
mod middleware;
mod routes;
mod services;
mod state;

use anyhow::Result;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".parse().unwrap());
    fmt()
        .with_env_filter(filter)
        .json()
        .with_timer(UtcTime::rfc_3339())
        .with_current_span(false)
        .with_span_list(false)
        .init();

    info!("Starting Smart Ingredients Backend");

    dotenvy::dotenv().ok();

    let config = config::AppConfig::from_env()?;
    let pool = db::create_pool(&config.database_url).await?;
    db::run_migrations(&pool).await?;
    let redis = redis::Client::open(config.auth.redis_url.as_str())?
        .get_connection_manager()
        .await?;

    let http = reqwest::Client::new();
    let llm = services::llm::build_llm_client(&config.llm, http.clone());
    let rules = services::rules::RuleEngine::load_from_path(&config.rules_path);
    let state = state::AppState {
        pool,
        redis,
        config,
        llm: std::sync::Arc::from(llm),
        rules: std::sync::Arc::new(rules),
    };

    // Build application
    let app = routes::create_routes(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!("Server listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
