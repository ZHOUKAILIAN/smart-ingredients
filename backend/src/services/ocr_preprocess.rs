//! OCR preprocessing using OpenCV

use std::path::{Path, PathBuf};

use anyhow::Result;
use opencv::{core, imgcodecs, imgproc, prelude::*};
use tracing::warn;

use crate::config::OcrPreprocessConfig;

pub fn preprocess_image(input: &Path, config: &OcrPreprocessConfig) -> Result<Option<PathBuf>> {
    if !config.enabled {
        return Ok(None);
    }

    let mut image = imgcodecs::imread(
        input
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("invalid image path"))?,
        imgcodecs::IMREAD_COLOR,
    )?;
    if image.empty() {
        return Err(anyhow::anyhow!("failed to read image for preprocessing"));
    }

    image = resize_if_needed(&image, config.min_width, config.max_width)?;
    let mut gray = Mat::default();
    imgproc::cvt_color(&image, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    let mut processed = if config.clahe {
        let mut clahe_output = Mat::default();
        let mut clahe = imgproc::create_clahe(2.0, core::Size::new(8, 8))?;
        clahe.apply(&gray, &mut clahe_output)?;
        clahe_output
    } else {
        gray.clone()
    };

    if config.denoise {
        let mut denoised = Mat::default();
        imgproc::gaussian_blur(
            &processed,
            &mut denoised,
            core::Size::new(3, 3),
            0.0,
            0.0,
            core::BORDER_DEFAULT,
        )?;
        processed = denoised;
    }

    if config.sharpen {
        let mut blurred = Mat::default();
        imgproc::gaussian_blur(
            &processed,
            &mut blurred,
            core::Size::new(0, 0),
            1.0,
            0.0,
            core::BORDER_DEFAULT,
        )?;
        let mut sharpened = Mat::default();
        core::add_weighted(&processed, 1.5, &blurred, -0.5, 0.0, &mut sharpened, -1)?;
        processed = sharpened;
    }

    if config.binary {
        let mut binary = Mat::default();
        imgproc::adaptive_threshold(
            &processed,
            &mut binary,
            255.0,
            imgproc::ADAPTIVE_THRESH_GAUSSIAN_C,
            imgproc::THRESH_BINARY,
            31,
            10.0,
        )?;
        processed = binary;
    }

    if config.morph_close {
        let kernel = imgproc::get_structuring_element(
            imgproc::MORPH_RECT,
            core::Size::new(3, 3),
            core::Point::new(-1, -1),
        )?;
        let mut closed = Mat::default();
        imgproc::morphology_ex(
            &processed,
            &mut closed,
            imgproc::MORPH_CLOSE,
            &kernel,
            core::Point::new(-1, -1),
            1,
            core::BORDER_CONSTANT,
            core::Scalar::default(),
        )?;
        processed = closed;
    }

    if config.deskew {
        if let Err(err) = deskew(&mut processed) {
            warn!("OCR deskew failed: {}", err);
        }
    }

    let output_path = std::env::temp_dir()
        .join(format!("ocr_preprocess_{}.png", uuid::Uuid::new_v4()));
    imgcodecs::imwrite(
        output_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("invalid output path"))?,
        &processed,
        &core::Vector::new(),
    )?;

    Ok(Some(output_path))
}

fn resize_if_needed(image: &Mat, min_width: i32, max_width: i32) -> Result<Mat> {
    let size = image.size()?;

    if size.width >= min_width && size.width <= max_width {
        return Ok(image.clone());
    }

    let target_width = if size.width > max_width {
        max_width
    } else {
        min_width
    };
    let scale = target_width as f64 / size.width as f64;
    let new_height = (size.height as f64 * scale).round() as i32;
    let mut resized = Mat::default();
    imgproc::resize(
        image,
        &mut resized,
        core::Size::new(target_width, new_height),
        0.0,
        0.0,
        if target_width > size.width {
            imgproc::INTER_CUBIC
        } else {
            imgproc::INTER_AREA
        },
    )?;
    Ok(resized)
}

fn deskew(image: &mut Mat) -> Result<()> {
    let mut edges = Mat::default();
    imgproc::canny(image, &mut edges, 50.0, 150.0, 3, false)?;

    let mut points = Mat::default();
    core::find_non_zero(&edges, &mut points)?;
    if points.total() == 0 {
        return Ok(());
    }

    let rect = imgproc::min_area_rect(&points)?;
    let mut angle = rect.angle as f64;
    if angle < -45.0 {
        angle += 90.0;
    }

    let center = core::Point2f::new(rect.center.x, rect.center.y);
    let rotation = imgproc::get_rotation_matrix_2d(center, angle, 1.0)?;
    let size = image.size()?;
    let mut rotated = Mat::default();
    imgproc::warp_affine(
        image,
        &mut rotated,
        &rotation,
        core::Size::new(size.width, size.height),
        imgproc::INTER_LINEAR,
        core::BORDER_REPLICATE,
        core::Scalar::default(),
    )?;
    *image = rotated;
    Ok(())
}
