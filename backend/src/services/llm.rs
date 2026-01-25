//! LLM service for ingredient analysis

use async_trait::async_trait;

use crate::config::{LlmConfig, LlmProvider};
use crate::services::llm_deepseek::DeepSeekClient;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreferenceType {
    None,
    WeightLoss,
    Health,
    Fitness,
    Allergy,
    Kids,
}

impl PreferenceType {
    pub fn from_str(value: Option<&str>) -> Self {
        match value.unwrap_or("none").trim().to_lowercase().as_str() {
            "weight_loss" | "weight-loss" | "weightloss" => Self::WeightLoss,
            "health" => Self::Health,
            "fitness" => Self::Fitness,
            "allergy" => Self::Allergy,
            "kids" | "children" => Self::Kids,
            _ => Self::None,
        }
    }

    pub fn as_key(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::WeightLoss => "weight_loss",
            Self::Health => "health",
            Self::Fitness => "fitness",
            Self::Allergy => "allergy",
            Self::Kids => "kids",
        }
    }
}

#[async_trait]
pub trait LlmProviderClient: Send + Sync {
    async fn analyze_ingredients(
        &self,
        text: &str,
        preference: PreferenceType,
    ) -> anyhow::Result<shared::AnalysisResult>;
}

pub fn build_llm_client(config: &LlmConfig, http: reqwest::Client) -> Box<dyn LlmProviderClient> {
    match config.provider {
        LlmProvider::DeepSeek => Box::new(DeepSeekClient::new(config, http)),
    }
}
