//! LLM service for ingredient analysis

use async_trait::async_trait;

use crate::config::{LlmConfig, LlmProvider};
use crate::services::llm_deepseek::DeepSeekClient;

#[async_trait]
pub trait LlmProviderClient: Send + Sync {
    async fn analyze_ingredients(&self, text: &str) -> anyhow::Result<shared::AnalysisResult>;
}

pub fn build_llm_client(config: &LlmConfig, http: reqwest::Client) -> Box<dyn LlmProviderClient> {
    match config.provider {
        LlmProvider::DeepSeek => Box::new(DeepSeekClient::new(config, http)),
    }
}
