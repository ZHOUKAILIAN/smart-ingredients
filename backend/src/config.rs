//! Application configuration

use std::{env, time::Duration};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub upload_dir: String,
    pub llm: LlmConfig,
    pub ocr: OcrConfig,
    pub auth: AuthConfig,
    pub rules_path: String,
    pub rules_refresh_seconds: u64,
}

#[derive(Debug, Clone)]
pub struct OcrConfig {
    pub lang: String,
    pub timeout: Duration,
    pub paddle_url: String,
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

#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub redis_url: String,
    pub jwt_secret: String,
    pub jwt_issuer: String,
    pub access_ttl_days: i64,
    pub refresh_ttl_days: i64,
    pub login_hash_key: String,
    pub login_max_attempts: u32,
    pub login_lock_seconds: u64,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url =
            env::var("DATABASE_URL").map_err(|_| anyhow::anyhow!("DATABASE_URL is required"))?;
        let upload_dir = env::var("UPLOAD_DIR").unwrap_or_else(|_| {
            let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("uploads");
            base_dir.to_string_lossy().to_string()
        });

        let provider = parse_llm_provider(
            env::var("LLM_PROVIDER").unwrap_or_else(|_| "deepseek".to_string()),
        )?;

        let api_key = env::var("DEEPSEEK_API_KEY")
            .map_err(|_| anyhow::anyhow!("DEEPSEEK_API_KEY is required"))?;
        let api_url = env::var("DEEPSEEK_API_URL")
            .unwrap_or_else(|_| "https://api.deepseek.com/v1/chat/completions".to_string());
        let model = env::var("DEEPSEEK_MODEL").unwrap_or_else(|_| "deepseek-chat".to_string());

        let llm_timeout = env::var("LLM_TIMEOUT")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(60);

        let llm = LlmConfig {
            provider,
            api_key,
            api_url,
            model,
            timeout: Duration::from_secs(llm_timeout),
        };

        let ocr = OcrConfig {
            lang: env::var("OCR_LANG").unwrap_or_else(|_| "chi_sim+eng".to_string()),
            timeout: Duration::from_secs(
                env::var("OCR_TIMEOUT")
                    .ok()
                    .and_then(|value| value.parse::<u64>().ok())
                    .unwrap_or(30),
            ),
            paddle_url: env::var("OCR_PADDLE_URL")
                .unwrap_or_else(|_| "http://ocr:8000/ocr".to_string()),
        };

        let auth = AuthConfig {
            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://redis:6379".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .map_err(|_| anyhow::anyhow!("JWT_SECRET is required"))?,
            jwt_issuer: env::var("JWT_ISSUER").unwrap_or_else(|_| "smart-ingredients".to_string()),
            access_ttl_days: env::var("JWT_ACCESS_TTL_DAYS")
                .ok()
                .and_then(|value| value.parse::<i64>().ok())
                .unwrap_or(30),
            refresh_ttl_days: env::var("JWT_REFRESH_TTL_DAYS")
                .ok()
                .and_then(|value| value.parse::<i64>().ok())
                .unwrap_or(60),
            login_hash_key: env::var("LOGIN_HASH_KEY")
                .map_err(|_| anyhow::anyhow!("LOGIN_HASH_KEY is required"))?,
            login_max_attempts: env::var("AUTH_LOGIN_MAX_ATTEMPTS")
                .ok()
                .and_then(|value| value.parse::<u32>().ok())
                .unwrap_or(5),
            login_lock_seconds: env::var("AUTH_LOGIN_LOCK_SECONDS")
                .ok()
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or(900),
        };

        let rules_path = env::var("RULES_PATH").unwrap_or_else(|_| {
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("rules.json")
                .to_string_lossy()
                .to_string()
        });

        let rules_refresh_seconds = env::var("RULES_REFRESH_SECONDS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(300);

        Ok(Self {
            database_url,
            upload_dir,
            llm,
            ocr,
            auth,
            rules_path,
            rules_refresh_seconds,
        })
    }
}

fn parse_llm_provider(value: String) -> anyhow::Result<LlmProvider> {
    match value.trim().to_lowercase().as_str() {
        "deepseek" => Ok(LlmProvider::DeepSeek),
        other => Err(anyhow::anyhow!("unsupported LLM_PROVIDER: {}", other)),
    }
}
