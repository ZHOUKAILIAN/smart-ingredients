//! Storage service for image handling

use std::path::{Path, PathBuf};

use anyhow::Result;
use tokio::fs;
use uuid::Uuid;

/// Store uploaded image locally and return public URL path
pub async fn store_image(
    image_data: &[u8],
    upload_dir: &str,
    filename: Option<&str>,
    extension: Option<&str>,
) -> Result<String> {
    let dir = Path::new(upload_dir);
    fs::create_dir_all(dir).await?;

    let safe_ext = extension
        .or_else(|| filename.and_then(|name| Path::new(name).extension().and_then(|e| e.to_str())))
        .unwrap_or("jpg");

    let stored_name = format!("{}.{}", Uuid::new_v4(), safe_ext);
    let path = dir.join(&stored_name);
    fs::write(&path, image_data).await?;

    Ok(format!("/uploads/{}", stored_name))
}

pub fn resolve_image_path(upload_dir: &str, image_url: &str) -> Result<PathBuf> {
    let trimmed = image_url.trim_start_matches('/');
    let path = Path::new(trimmed);
    let filename = path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("invalid image url"))?;
    Ok(Path::new(upload_dir).join(filename))
}
