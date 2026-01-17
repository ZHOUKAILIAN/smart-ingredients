//! Application configuration

use std::{env, time::Duration};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub upload_dir: String,
    pub llm: LlmConfig,
    pub ocr: OcrConfig,
}

#[derive(Debug, Clone)]
pub struct OcrConfig {
    pub lang: String,
    pub timeout: Duration,
}

#[derive(Debug, Clone)]
pub enum LlmProvider {
    DeepSeek,
}

#[derive(Debug, Clone)]
pub struct LlmConfig {
    pub provider: LlmProvider,
    pub api_key: String,
    pub api_url: String,
    pub model: String,
    pub timeout: Duration,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL is required"))?;
        let upload_dir = env::var("UPLOAD_DIR").unwrap_or_else(|_| "uploads".to_string());

        let provider = parse_llm_provider(
            env::var("LLM_PROVIDER").unwrap_or_else(|_| "deepseek".to_string()),
        )?;

        let api_key = env::var("DEEPSEEK_API_KEY")
            .map_err(|_| anyhow::anyhow!("DEEPSEEK_API_KEY is required"))?;
        let api_url = env::var("DEEPSEEK_API_URL")
            .unwrap_or_else(|_| "https://api.deepseek.com/v1/chat/completions".to_string());
        let model = env::var("DEEPSEEK_MODEL").unwrap_or_else(|_| "deepseek-chat".to_string());

        let llm = LlmConfig {
            provider,
            api_key,
            api_url,
            model,
            timeout: Duration::from_secs(30),
        };

        let ocr = OcrConfig {
            lang: env::var("OCR_LANG").unwrap_or_else(|_| "chi_sim+eng".to_string()),
            timeout: Duration::from_secs(10),
        };

        Ok(Self {
            database_url,
            upload_dir,
            llm,
            ocr,
        })
    }
}

fn parse_llm_provider(value: String) -> anyhow::Result<LlmProvider> {
    match value.trim().to_lowercase().as_str() {
        "deepseek" => Ok(LlmProvider::DeepSeek),
        other => Err(anyhow::anyhow!("unsupported LLM_PROVIDER: {}", other)),
    }
}
