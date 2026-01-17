//! Shared application state

use std::sync::Arc;

use sqlx::PgPool;

use crate::{config::AppConfig, services::llm::LlmProviderClient};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: AppConfig,
    pub llm: Arc<dyn LlmProviderClient>,
}
