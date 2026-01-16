//! Smart Ingredients Backend Server
//!
//! Axum-based web server for image analysis

mod db;
mod errors;
mod handlers;
mod middleware;
mod routes;
mod services;

use anyhow::Result;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info".parse().unwrap());
    fmt().with_env_filter(filter).init();

    info!("Starting Smart Ingredients Backend");

    // Build application
    let app = routes::create_routes();

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!("Server listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
