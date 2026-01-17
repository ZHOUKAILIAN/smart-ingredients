//! OCR service for text extraction

use std::path::Path;

use anyhow::Result;
use tokio::process::Command;

use crate::config::OcrConfig;

/// Extract text from image using local tesseract
pub async fn extract_text(image_path: &Path, config: &OcrConfig) -> Result<String> {
    let output = Command::new("tesseract")
        .arg(image_path)
        .arg("stdout")
        .arg("-l")
        .arg(&config.lang)
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("tesseract failed: {}", stderr.trim()));
    }

    let text = String::from_utf8(output.stdout)?;
    Ok(text.trim().to_string())
}
