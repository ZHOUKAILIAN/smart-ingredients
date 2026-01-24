//! Storage service for image handling

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tokio::fs;
use tracing::info;
use uuid::Uuid;

use crate::services::image_converter;

/// Store uploaded image locally and return public URL path
pub async fn store_image(
    bytes: &[u8],
    content_type: Option<&str>,
    upload_dir: &str,
    original_filename: Option<&str>,
) -> Result<String> {
    // 1. Detect image format
    let format = image_converter::detect_format(bytes, content_type, original_filename)
        .context("图片格式检测失败")?;

    // 2. Convert format if needed
    let (final_bytes, extension) = if format.needs_conversion() {
        info!(
            "转换图片格式: {:?} -> {:?}",
            format,
            format.target_format()
        );

        // Handle GIF specially (extract first frame)
        if matches!(format, image_converter::SupportedFormat::Gif) {
            image_converter::extract_gif_first_frame(bytes)
                .context("GIF 第一帧提取失败")?
        } else if matches!(format, image_converter::SupportedFormat::Svg) {
            image_converter::convert_svg_to_png(bytes)
                .context("SVG 光栅化失败")?
        } else {
            image_converter::convert_image(bytes, format)
                .context("图片格式转换失败")?
        }
    } else {
        let ext = match format {
            image_converter::SupportedFormat::Jpeg => "jpg",
            image_converter::SupportedFormat::Png => "png",
            _ => unreachable!(),
        };
        (bytes.to_vec(), ext)
    };

    // 3. Generate filename
    let filename = generate_filename(original_filename, extension);
    let dir = Path::new(upload_dir);

    // 4. Ensure directory exists
    fs::create_dir_all(dir).await?;

    // 5. Write file
    let path = dir.join(&filename);
    fs::write(&path, &final_bytes).await?;

    // 6. Return relative path
    Ok(format!("/uploads/{}", filename))
}

fn generate_filename(original: Option<&str>, extension: &str) -> String {
    let uuid = Uuid::new_v4();
    let timestamp = chrono::Utc::now().timestamp();

    if let Some(original) = original {
        let stem = Path::new(original)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("image");
        format!("{}_{}_{}.{}", stem, timestamp, uuid, extension)
    } else {
        format!("image_{}_{}.{}", timestamp, uuid, extension)
    }
}

pub fn resolve_image_path(upload_dir: &str, image_url: &str) -> Result<PathBuf> {
    let trimmed = image_url.trim_start_matches('/');
    let path = Path::new(trimmed);
    let filename = path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("invalid image url"))?;
    Ok(Path::new(upload_dir).join(filename))
}
