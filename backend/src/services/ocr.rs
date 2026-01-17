//! OCR service for text extraction

use std::path::Path;

use anyhow::Result;
use tokio::process::Command;

use crate::config::{OcrConfig, OcrProvider};
use crate::services::ocr_preprocess;

/// Extract text from image using local tesseract
pub async fn extract_text(image_path: &Path, config: &OcrConfig) -> Result<String> {
    match config.provider {
        OcrProvider::Tesseract => extract_text_tesseract(image_path, config).await,
        OcrProvider::Paddle => extract_text_paddle(image_path, config).await,
    }
}

async fn extract_text_tesseract(image_path: &Path, config: &OcrConfig) -> Result<String> {
    let preprocess_config = config.preprocess.clone();
    let preprocess_path = tokio::task::spawn_blocking({
        let image_path = image_path.to_path_buf();
        move || ocr_preprocess::preprocess_image(&image_path, &preprocess_config)
    })
    .await??;

    let tesseract_input = preprocess_path
        .as_ref()
        .map(|path| path.as_path())
        .unwrap_or(image_path);

    let output = Command::new("tesseract")
        .arg(tesseract_input)
        .arg("stdout")
        .arg("-l")
        .arg(&config.lang)
        .args(build_tesseract_args(config))
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("tesseract failed: {}", stderr.trim()));
    }

    if let Some(path) = preprocess_path {
        let _ = tokio::fs::remove_file(path).await;
    }

    let text = String::from_utf8(output.stdout)?;
    Ok(text.trim().to_string())
}

async fn extract_text_paddle(image_path: &Path, config: &OcrConfig) -> Result<String> {
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

fn build_tesseract_args(config: &OcrConfig) -> Vec<String> {
    let mut args = Vec::new();
    if let Some(psm) = config.psm {
        args.push("--psm".to_string());
        args.push(psm.to_string());
    }
    if let Some(oem) = config.oem {
        args.push("--oem".to_string());
        args.push(oem.to_string());
    }
    args
}

#[derive(serde::Deserialize)]
struct PaddleOcrResponse {
    text: String,
}
