//! Image format conversion service

use anyhow::{Context, Result};
use image::{DynamicImage, ImageEncoder, ImageFormat};

/// Supported image formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedFormat {
    Jpeg,
    Png,
    WebP,
    Bmp,
    Gif,
    Tiff,
    #[cfg(feature = "heic")]
    Heic,
    Svg,
}

impl SupportedFormat {
    /// Check if format needs conversion to standard format
    pub fn needs_conversion(&self) -> bool {
        match self {
            SupportedFormat::Jpeg | SupportedFormat::Png => false,
            SupportedFormat::Bmp
            | SupportedFormat::Gif
            | SupportedFormat::Tiff
            | SupportedFormat::WebP
            | SupportedFormat::Svg => true,
            #[cfg(feature = "heic")]
            SupportedFormat::Heic => true,
        }
    }

    /// Get target format for conversion
    pub fn target_format(&self) -> ImageFormat {
        match self {
            SupportedFormat::Png => ImageFormat::Png,
            SupportedFormat::Svg => ImageFormat::Png,
            _ => ImageFormat::Jpeg, // Convert all others to JPEG
        }
    }

    /// Get file extension
    pub fn extension(&self) -> &'static str {
        match self {
            SupportedFormat::Jpeg => "jpg",
            SupportedFormat::Png => "png",
            SupportedFormat::WebP => "webp",
            SupportedFormat::Bmp => "bmp",
            SupportedFormat::Gif => "gif",
            SupportedFormat::Tiff => "tiff",
            #[cfg(feature = "heic")]
            SupportedFormat::Heic => "heic",
            SupportedFormat::Svg => "svg",
        }
    }
}

/// Automatically detect image format
pub fn detect_format(
    bytes: &[u8],
    content_type: Option<&str>,
    filename: Option<&str>,
) -> Result<SupportedFormat> {
    if is_svg_content_type(content_type)
        || matches_extension(filename, &["svg"])
        || is_svg_payload(bytes)
    {
        return Ok(SupportedFormat::Svg);
    }

    #[cfg(feature = "heic")]
    {
        if is_heic_content_type(content_type)
            || matches_extension(filename, &["heic", "heif"])
            || is_heic_payload(bytes)
        {
            return Ok(SupportedFormat::Heic);
        }
    }

    let format = image::guess_format(bytes)
        .context("无法识别图片格式，请确保上传的是有效的图片文件")?;

    match format {
        ImageFormat::Jpeg => Ok(SupportedFormat::Jpeg),
        ImageFormat::Png => Ok(SupportedFormat::Png),
        ImageFormat::WebP => Ok(SupportedFormat::WebP),
        ImageFormat::Bmp => Ok(SupportedFormat::Bmp),
        ImageFormat::Gif => Ok(SupportedFormat::Gif),
        ImageFormat::Tiff => Ok(SupportedFormat::Tiff),
        other => Err(anyhow::anyhow!(
            "不支持的图片格式: {:?}。支持的格式: JPEG, PNG, WebP, BMP, GIF, TIFF, HEIC/HEIF, SVG",
            other
        )),
    }
}

/// Validate that the image is valid
pub fn validate_image(bytes: &[u8]) -> Result<DynamicImage> {
    image::load_from_memory(bytes).context("图片文件损坏或格式不正确，无法解析")
}

/// Convert image to standard format
/// Returns (converted_bytes, extension)
pub fn convert_image(
    bytes: &[u8],
    source_format: SupportedFormat,
) -> Result<(Vec<u8>, &'static str)> {
    // If JPEG or PNG and doesn't need conversion, return as-is
    if !source_format.needs_conversion() {
        let extension = match source_format {
            SupportedFormat::Jpeg => "jpg",
            SupportedFormat::Png => "png",
            _ => unreachable!(),
        };
        return Ok((bytes.to_vec(), extension));
    }

    #[cfg(feature = "heic")]
    if matches!(source_format, SupportedFormat::Heic) {
        return convert_heic_to_jpeg(bytes);
    }

    // Load image
    let img = validate_image(bytes)?;

    // Convert format
    let target_format = source_format.target_format();
    let mut output = Vec::new();

    match target_format {
        ImageFormat::Jpeg => {
            // JPEG quality set to 92
            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, 92);
            encoder.encode_image(&img)
                .context("转换为 JPEG 格式失败")?;
            Ok((output, "jpg"))
        }
        ImageFormat::Png => {
            // PNG lossless compression
            let encoder = image::codecs::png::PngEncoder::new(&mut output);
            encoder.write_image(
                img.as_bytes(),
                img.width(),
                img.height(),
                img.color().into(),
            )
            .context("转换为 PNG 格式失败")?;
            Ok((output, "png"))
        }
        _ => unreachable!(),
    }
}

/// Convert HEIC/HEIF to JPEG
pub fn convert_heic_to_jpeg(bytes: &[u8]) -> Result<(Vec<u8>, &'static str)> {
    #[cfg(not(feature = "heic"))]
    {
        let _ = bytes;
        Err(anyhow::anyhow!(
            "HEIC/HEIF is not supported in this build (enable feature: heic)"
        ))
    }

    #[cfg(feature = "heic")]
    {
        let image = decode_heic(bytes).context("HEIC 解码失败")?;

        let mut output = Vec::new();
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, 92);
        encoder.encode_image(&image)
            .context("转换为 JPEG 格式失败")?;

        Ok((output, "jpg"))
    }
}

/// Rasterize SVG into PNG
pub fn convert_svg_to_png(bytes: &[u8]) -> Result<(Vec<u8>, &'static str)> {
    let tree = load_svg_tree(bytes)?;
    let pixmap = render_svg(&tree).context("SVG 渲染失败")?;
    let png_data = pixmap.encode_png().context("SVG 转换为 PNG 失败")?;
    Ok((png_data, "png"))
}

/// Extract first frame from GIF and convert to JPEG
pub fn extract_gif_first_frame(bytes: &[u8]) -> Result<(Vec<u8>, &'static str)> {
    let img = image::load_from_memory_with_format(bytes, ImageFormat::Gif)
        .context("无法解析 GIF 图片")?;

    let mut output = Vec::new();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, 92);
    encoder.encode_image(&img)
        .context("提取 GIF 第一帧失败")?;

    Ok((output, "jpg"))
}

fn matches_content_type(content_type: Option<&str>, expected: &str) -> bool {
    content_type
        .map(|value| value.eq_ignore_ascii_case(expected))
        .unwrap_or(false)
}

fn is_svg_content_type(content_type: Option<&str>) -> bool {
    matches_content_type(content_type, "image/svg+xml")
        || matches_content_type(content_type, "image/svg")
}

#[cfg(feature = "heic")]
fn is_heic_content_type(content_type: Option<&str>) -> bool {
    matches_content_type(content_type, "image/heic")
        || matches_content_type(content_type, "image/heif")
        || matches_content_type(content_type, "image/heic-sequence")
        || matches_content_type(content_type, "image/heif-sequence")
}

fn matches_extension(filename: Option<&str>, exts: &[&str]) -> bool {
    let Some(name) = filename else { return false };
    let ext = std::path::Path::new(name)
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase());
    let Some(ext) = ext else { return false };
    exts.iter().any(|candidate| *candidate == ext)
}

#[cfg(feature = "heic")]
fn is_heic_payload(bytes: &[u8]) -> bool {
    let header = &bytes.get(0..64).unwrap_or(bytes);
    header.windows(8).any(|window| {
        window == b"ftypheic"
            || window == b"ftypheif"
            || window == b"ftyphevc"
            || window == b"ftyphevx"
            || window == b"ftypmif1"
    })
}

fn is_svg_payload(bytes: &[u8]) -> bool {
    let sample = &bytes.get(0..2048).unwrap_or(bytes);
    let text = String::from_utf8_lossy(sample).to_ascii_lowercase();
    text.contains("<svg")
}

#[cfg(feature = "heic")]
fn decode_heic(bytes: &[u8]) -> Result<DynamicImage> {
    use libheif_rs::{ColorSpace, HeifContext, RgbChroma};

    let ctx = HeifContext::read_from_bytes(bytes)
        .context("无法读取 HEIC 数据")?;
    let handle = ctx.primary_image_handle().context("无法解析 HEIC 主图像")?;
    let image = handle
        .decode(ColorSpace::Rgb(RgbChroma::Rgb), None)
        .context("HEIC 解码失败")?;

    let width = image.width();
    let height = image.height();
    let planes = image.planes();
    let interleaved = planes
        .interleaved
        .as_ref()
        .context("HEIC 图片数据缺失")?;
    let stride = interleaved.stride;
    let data = &interleaved.data;

    let mut rgba = Vec::with_capacity((width * height * 3) as usize);
    for y in 0..height {
        let row_start = (y * stride) as usize;
        let row_end = row_start + (width * 3) as usize;
        rgba.extend_from_slice(&data[row_start..row_end]);
    }

    let buffer = image::RgbImage::from_raw(width, height, rgba)
        .context("HEIC 像素缓冲区构建失败")?;
    Ok(DynamicImage::ImageRgb8(buffer))
}

fn load_svg_tree(bytes: &[u8]) -> Result<resvg::usvg::Tree> {
    let options = resvg::usvg::Options {
        font_family: "Sans".to_string(),
        ..Default::default()
    };
    resvg::usvg::Tree::from_data(bytes, &options)
        .context("无法解析 SVG 内容")
}

fn render_svg(tree: &resvg::usvg::Tree) -> Result<resvg::tiny_skia::Pixmap> {
    let size = tree.size().to_int_size();
    let mut width = size.width();
    let mut height = size.height();
    if width == 0 || height == 0 {
        width = 1024;
        height = 1024;
    }

    let mut pixmap =
        resvg::tiny_skia::Pixmap::new(width, height).context("创建 SVG 画布失败")?;
    resvg::render(tree, resvg::usvg::Transform::default(), &mut pixmap.as_mut());
    Ok(pixmap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_jpeg_format() {
        // JPEG file header: FF D8 FF
        let jpeg_header = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46];
        let result = detect_format(&jpeg_header, None, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), SupportedFormat::Jpeg);
    }

    #[test]
    fn test_detect_png_format() {
        // PNG file header: 89 50 4E 47 0D 0A 1A 0A
        let png_header = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        let result = detect_format(&png_header, None, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), SupportedFormat::Png);
    }

    #[test]
    fn test_invalid_format() {
        let invalid_data = vec![0x00, 0x01, 0x02, 0x03];
        let result = detect_format(&invalid_data, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_needs_conversion() {
        assert!(!SupportedFormat::Jpeg.needs_conversion());
        assert!(!SupportedFormat::Png.needs_conversion());
        assert!(SupportedFormat::Bmp.needs_conversion());
        assert!(SupportedFormat::Gif.needs_conversion());
        assert!(SupportedFormat::Tiff.needs_conversion());
        assert!(SupportedFormat::WebP.needs_conversion());
        #[cfg(feature = "heic")]
        assert!(SupportedFormat::Heic.needs_conversion());
        assert!(SupportedFormat::Svg.needs_conversion());
    }

    #[test]
    fn test_target_format() {
        assert_eq!(SupportedFormat::Png.target_format(), ImageFormat::Png);
        assert_eq!(SupportedFormat::Jpeg.target_format(), ImageFormat::Jpeg);
        assert_eq!(SupportedFormat::Bmp.target_format(), ImageFormat::Jpeg);
        assert_eq!(SupportedFormat::WebP.target_format(), ImageFormat::Jpeg);
    }

    #[test]
    fn test_extension() {
        assert_eq!(SupportedFormat::Jpeg.extension(), "jpg");
        assert_eq!(SupportedFormat::Png.extension(), "png");
        assert_eq!(SupportedFormat::Bmp.extension(), "bmp");
        assert_eq!(SupportedFormat::Gif.extension(), "gif");
        #[cfg(feature = "heic")]
        assert_eq!(SupportedFormat::Heic.extension(), "heic");
        assert_eq!(SupportedFormat::Svg.extension(), "svg");
    }
}
