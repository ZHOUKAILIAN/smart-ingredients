//! OCR service for text extraction

use std::path::Path;

use anyhow::Result;

use crate::config::OcrConfig;

/// Extract text from image using PaddleOCR
pub async fn extract_text(image_path: &Path, config: &OcrConfig) -> Result<String> {
    let bytes = tokio::fs::read(image_path).await?;
    let part = reqwest::multipart::Part::bytes(bytes)
        .file_name("image.jpg")
        .mime_str("image/jpeg")?;
    let form = reqwest::multipart::Form::new().part("file", part);

    let client = reqwest::Client::builder()
        .timeout(config.timeout)
        .build()?;

    let response = client
        .post(&config.paddle_url)
        .multipart(form)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
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
