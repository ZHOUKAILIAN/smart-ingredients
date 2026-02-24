//! OCR service for text extraction

use std::path::Path;

use anyhow::Result;
use reqwest::StatusCode;

use crate::config::OcrConfig;

/// Extract text from image using PaddleOCR
pub async fn extract_text(image_path: &Path, config: &OcrConfig) -> Result<String> {
    let bytes = tokio::fs::read(image_path).await?;
    let part = reqwest::multipart::Part::bytes(bytes)
        .file_name("image.jpg")
        .mime_str("image/jpeg")?;
    let form = reqwest::multipart::Form::new().part("file", part);

    let client = reqwest::Client::builder().timeout(config.timeout).build()?;

    let response = client
        .post(&config.paddle_url)
        .multipart(form)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        if status == StatusCode::UNPROCESSABLE_ENTITY {
            let fallback = "未识别到文字，请重新拍摄或上传更清晰的图片".to_string();
            let message = parse_ocr_error_message(&body).unwrap_or(fallback);
            return Err(anyhow::anyhow!(message));
        }
        return Err(anyhow::anyhow!(
            "paddle OCR failed: status {} body {}",
            status,
            body
        ));
    }

    let result: PaddleOcrResponse = response.json().await?;
    Ok(result.text.trim().to_string())
}

#[derive(serde::Deserialize)]
struct PaddleOcrResponse {
    text: String,
}

#[derive(serde::Deserialize)]
struct OcrErrorBody {
    message: Option<String>,
    detail: Option<String>,
}

fn parse_ocr_error_message(body: &str) -> Option<String> {
    let parsed: OcrErrorBody = serde_json::from_str(body).ok()?;
    parsed.message.or(parsed.detail)
}

#[cfg(test)]
mod tests {
    use super::parse_ocr_error_message;

    #[test]
    fn parse_message_field() {
        let body = r#"{\"message\":\"empty\"}"#;
        assert_eq!(parse_ocr_error_message(body), Some("empty".to_string()));
    }

    #[test]
    fn parse_detail_field() {
        let body = r#"{\"detail\":\"empty\"}"#;
        assert_eq!(parse_ocr_error_message(body), Some("empty".to_string()));
    }

    #[test]
    fn returns_none_on_invalid_json() {
        let body = "not-json";
        assert_eq!(parse_ocr_error_message(body), None);
    }
}
