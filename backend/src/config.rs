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
    pub provider: OcrProvider,
    pub lang: String,
    pub timeout: Duration,
    pub paddle_url: String,
    pub psm: Option<u8>,
    pub oem: Option<u8>,
    pub preprocess: OcrPreprocessConfig,
}

#[derive(Debug, Clone)]
pub struct OcrPreprocessConfig {
    pub enabled: bool,
    pub min_width: i32,
    pub max_width: i32,
    pub deskew: bool,
    pub binary: bool,
    pub denoise: bool,
    pub clahe: bool,
    pub sharpen: bool,
    pub morph_close: bool,
}

#[derive(Debug, Clone)]
pub enum LlmProvider {
    DeepSeek,
}

#[derive(Debug, Clone)]
pub enum OcrProvider {
    Tesseract,
    Paddle,
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
            provider: parse_ocr_provider(
                env::var("OCR_PROVIDER").unwrap_or_else(|_| "tesseract".to_string()),
            )?,
            lang: env::var("OCR_LANG").unwrap_or_else(|_| "chi_sim+eng".to_string()),
            timeout: Duration::from_secs(
                env::var("OCR_TIMEOUT")
                    .ok()
                    .and_then(|value| value.parse::<u64>().ok())
                    .unwrap_or(30),
            ),
            paddle_url: env::var("OCR_PADDLE_URL")
                .unwrap_or_else(|_| "http://ocr:8000/ocr".to_string()),
            psm: parse_optional_u8("OCR_PSM")?,
            oem: parse_optional_u8("OCR_OEM")?,
            preprocess: OcrPreprocessConfig {
                enabled: parse_bool(
                    env::var("OCR_PREPROCESS_ENABLE").unwrap_or_else(|_| "true".to_string()),
                ),
                min_width: env::var("OCR_PREPROCESS_MIN_WIDTH")
                    .ok()
                    .and_then(|value| value.parse::<i32>().ok())
                    .unwrap_or(1600),
                max_width: env::var("OCR_PREPROCESS_MAX_WIDTH")
                    .ok()
                    .and_then(|value| value.parse::<i32>().ok())
                    .unwrap_or(2000),
                deskew: parse_bool(
                    env::var("OCR_PREPROCESS_DESKEW").unwrap_or_else(|_| "true".to_string()),
                ),
                binary: parse_bool(
                    env::var("OCR_PREPROCESS_BINARY").unwrap_or_else(|_| "true".to_string()),
                ),
                denoise: parse_bool(
                    env::var("OCR_PREPROCESS_DENOISE").unwrap_or_else(|_| "true".to_string()),
                ),
                clahe: parse_bool(
                    env::var("OCR_PREPROCESS_CLAHE").unwrap_or_else(|_| "true".to_string()),
                ),
                sharpen: parse_bool(
                    env::var("OCR_PREPROCESS_SHARPEN").unwrap_or_else(|_| "true".to_string()),
                ),
                morph_close: parse_bool(
                    env::var("OCR_PREPROCESS_MORPH_CLOSE")
                        .unwrap_or_else(|_| "false".to_string()),
                ),
            },
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

fn parse_ocr_provider(value: String) -> anyhow::Result<OcrProvider> {
    match value.trim().to_lowercase().as_str() {
        "tesseract" => Ok(OcrProvider::Tesseract),
        "paddle" => Ok(OcrProvider::Paddle),
        other => Err(anyhow::anyhow!("unsupported OCR_PROVIDER: {}", other)),
    }
}

fn parse_optional_u8(key: &str) -> anyhow::Result<Option<u8>> {
    let value = match env::var(key) {
        Ok(value) => value,
        Err(_) => return Ok(None),
    };
    let parsed = value
        .trim()
        .parse::<u8>()
        .map_err(|_| anyhow::anyhow!("{} must be a number", key))?;
    Ok(Some(parsed))
}

fn parse_bool(value: String) -> bool {
    matches!(value.trim().to_lowercase().as_str(), "1" | "true" | "yes" | "on")
}
