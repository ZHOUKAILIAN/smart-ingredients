//! Shared application state

use std::sync::Arc;

use redis::aio::ConnectionManager;
use sqlx::PgPool;

use crate::{config::AppConfig, services::llm::LlmProviderClient, services::rules::RuleEngine};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub redis: ConnectionManager,
    pub config: AppConfig,
    pub llm: Arc<dyn LlmProviderClient>,
    pub rules: Arc<RwLock<RuleEngine>>,
}
