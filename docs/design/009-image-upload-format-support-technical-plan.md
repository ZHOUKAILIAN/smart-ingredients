# 009-图片上传格式支持优化技术方案

## 元数据

| 字段     | 值                               |
| -------- | -------------------------------- |
| 文档编号 | 009-image-upload-format-support  |
| 标题     | 图片上传格式支持优化技术方案     |
| 版本     | 1.0                              |
| 状态     | 草稿                             |
| 创建日期 | 2026-01-21                       |
| 更新日期 | 2026-01-21                       |
| 作者     | Claude Code                      |
| 关联需求 | 009-image-upload-format-support  |

## 概述

### 目的

本技术方案旨在解决图片上传格式限制问题，并优化全局错误提示体验：

1. **扩展格式支持**：支持 HEIC、BMP、TIFF、GIF 等常见格式
2. **服务端转换**：统一转换为标准格式（JPEG/PNG）
3. **智能检测**：自动检测图片格式，无需依赖 Content-Type
4. **全局错误提示**：实现 Toast 组件，统一处理所有 API 错误
5. **优化错误信息**：提供友好的中文错误信息和操作建议

### 范围

本设计涵盖：
- **后端**：图片格式验证逻辑重构、图片格式转换实现、友好错误信息
- **前端**：全局 Toast 组件、API 错误拦截器、错误信息映射
- 性能优化方案

不涉及：
- 前端图片格式转换（由服务端处理）
- 图片压缩算法优化
- OCR 识别逻辑修改

### 假设

1. 使用 `image` crate 作为主要图片处理库
2. HEIC 格式通过 `libheif-rs` 或转换服务处理
3. 转换后的图片质量满足 OCR 要求
4. 服务器有足够的内存和 CPU 资源进行格式转换

## 架构设计

### 高层架构

```
┌─────────────────────────────────────────────────────────┐
│                   Client (Mobile/Web)                   │
│  - 拍照/选择图片                                         │
│  - 上传原始格式（HEIC/BMP/PNG/JPEG 等）                 │
└─────────────────────────────────────────────────────────┘
                          │
                          │ HTTP Multipart
                          ↓
┌─────────────────────────────────────────────────────────┐
│              Backend - Upload Handler                   │
│  1. 接收文件 (multipart/form-data)                      │
│  2. 验证文件大小 (< 10MB)                               │
│  3. 检测图片格式 (auto-detect)                          │
└─────────────────────────────────────────────────────────┘
                          │
                          ↓
┌─────────────────────────────────────────────────────────┐
│          Format Conversion Service (新增)               │
│  ┌───────────────────────────────────────────────────┐  │
│  │ 1. 格式检测                                       │  │
│  │    - 使用 image crate 检测格式                    │  │
│  │    - 支持: JPEG, PNG, WebP, BMP, GIF, TIFF       │  │
│  │    - HEIC: 使用 libheif-rs (可选)                │  │
│  ├───────────────────────────────────────────────────┤  │
│  │ 2. 格式转换                                       │  │
│  │    - HEIC/BMP/TIFF → JPEG (质量 92)              │  │
│  │    - PNG → PNG (保留透明通道)                     │  │
│  │    - WebP → JPEG (质量 92)                       │  │
│  │    - GIF → JPEG (第一帧)                         │  │
│  ├───────────────────────────────────────────────────┤  │
│  │ 3. 质量控制                                       │  │
│  │    - 保持原始分辨率                               │  │
│  │    - JPEG 质量: 92                               │  │
│  │    - PNG: 无损压缩                               │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
                          │
                          ↓
┌─────────────────────────────────────────────────────────┐
│              Storage Service (现有)                     │
│  - 存储转换后的图片到磁盘                               │
│  - 生成 image_url                                       │
└─────────────────────────────────────────────────────────┘
                          │
                          ↓
┌─────────────────────────────────────────────────────────┐
│              OCR Processing (现有)                      │
│  - 使用标准格式图片进行 OCR                             │
└─────────────────────────────────────────────────────────┘
```

### 组件层次

```
backend/src/
├── handlers/
│   └── analysis.rs                    (修改)
│       ├── upload_handler()           - 上传入口
│       └── validate_content_type()    - 格式验证 (重构)
├── services/
│   ├── storage.rs                     (修改)
│   │   └── store_image()              - 添加格式转换调用
│   └── image_converter.rs             (新增)
│       ├── detect_format()            - 自动检测格式
│       ├── convert_to_jpeg()          - 转换为 JPEG
│       ├── convert_to_png()           - 转换为 PNG
│       └── validate_image()           - 验证图片有效性
└── errors.rs                          (修改)
    └── AppError::UnsupportedFormat    - 优化错误信息
```

### 技术栈

| 组件          | 技术                   | 选择理由                                   |
| ------------- | ---------------------- | ------------------------------------------ |
| 图片处理      | `image` crate          | 纯 Rust，支持多种格式，性能好              |
| HEIC 支持     | `libheif-rs` (可选)    | 专门处理 HEIC/HEIF 格式                    |
| 格式检测      | `image::guess_format`  | 自动检测格式，不依赖 Content-Type          |
| 内存管理      | `bytes` crate          | 高效的字节处理                             |
| 错误处理      | `anyhow` + `thiserror` | 现有方案，保持一致                         |

## 详细设计

### 1. 格式验证逻辑重构

**当前代码** (`backend/src/handlers/analysis.rs:234-245`):

```rust
fn validate_content_type(content_type: Option<&str>) -> Result<Option<&'static str>, AppError> {
    match content_type {
        Some("image/jpeg") => Ok(Some("jpg")),
        Some("image/png") => Ok(Some("png")),
        Some("image/webp") => Ok(Some("webp")),
        Some(other) => Err(AppError::UnsupportedMediaType(format!(
            "unsupported content type: {}",
            other
        ))),
        None => Ok(None),
    }
}
```

**问题**：
1. 仅支持 3 种格式
2. 依赖 Content-Type（可能不准确）
3. 错误信息为英文

**重构后代码**:

```rust
fn validate_content_type(content_type: Option<&str>) -> Result<(), AppError> {
    // 只做基本验证，实际格式由 image crate 自动检测
    match content_type {
        Some(ct) if ct.starts_with("image/") => Ok(()),
        Some(other) => Err(AppError::UnsupportedMediaType(format!(
            "不支持的文件类型: {}。请上传图片文件（支持 JPEG、PNG、HEIC、BMP、WebP、GIF、TIFF 格式）",
            other
        ))),
        None => Ok(()), // 允许无 Content-Type，由后续自动检测
    }
}
```

**改进**：
1. 接受所有 `image/*` 类型
2. 中文错误信息
3. 明确列出支持的格式

### 2. 图片格式转换服务

**新增文件**: `backend/src/services/image_converter.rs`

```rust
use anyhow::{Context, Result};
use image::{DynamicImage, ImageFormat, ImageOutputFormat};
use std::io::Cursor;

/// 支持的图片格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedFormat {
    Jpeg,
    Png,
    WebP,
    Bmp,
    Gif,
    Tiff,
    // HEIC 需要特殊处理
}

impl SupportedFormat {
    /// 判断是否需要转换
    pub fn needs_conversion(&self) -> bool {
        matches!(
            self,
            SupportedFormat::Bmp
                | SupportedFormat::Gif
                | SupportedFormat::Tiff
                | SupportedFormat::WebP
        )
    }

    /// 获取目标格式
    pub fn target_format(&self) -> ImageFormat {
        match self {
            SupportedFormat::Png => ImageFormat::Png,
            _ => ImageFormat::Jpeg, // 其他格式统一转为 JPEG
        }
    }
}

/// 自动检测图片格式
pub fn detect_format(bytes: &[u8]) -> Result<SupportedFormat> {
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
            "不支持的图片格式: {:?}。支持的格式: JPEG, PNG, WebP, BMP, GIF, TIFF",
            other
        )),
    }
}

/// 验证图片有效性
pub fn validate_image(bytes: &[u8]) -> Result<DynamicImage> {
    image::load_from_memory(bytes)
        .context("图片文件损坏或格式不正确，无法解析")
}

/// 转换图片格式
pub fn convert_image(
    bytes: &[u8],
    source_format: SupportedFormat,
) -> Result<(Vec<u8>, &'static str)> {
    // 如果是 JPEG 或 PNG，且不需要转换，直接返回
    if !source_format.needs_conversion() {
        let extension = match source_format {
            SupportedFormat::Jpeg => "jpg",
            SupportedFormat::Png => "png",
            _ => unreachable!(),
        };
        return Ok((bytes.to_vec(), extension));
    }

    // 加载图片
    let img = validate_image(bytes)?;

    // 转换格式
    let target_format = source_format.target_format();
    let mut output = Vec::new();

    match target_format {
        ImageFormat::Jpeg => {
            // JPEG 质量设为 92
            img.write_to(
                &mut Cursor::new(&mut output),
                ImageOutputFormat::Jpeg(92),
            )
            .context("转换为 JPEG 格式失败")?;
            Ok((output, "jpg"))
        }
        ImageFormat::Png => {
            // PNG 无损压缩
            img.write_to(
                &mut Cursor::new(&mut output),
                ImageOutputFormat::Png,
            )
            .context("转换为 PNG 格式失败")?;
            Ok((output, "png"))
        }
        _ => unreachable!(),
    }
}

/// 处理 GIF 图片（提取第一帧）
pub fn extract_gif_first_frame(bytes: &[u8]) -> Result<(Vec<u8>, &'static str)> {
    let img = image::load_from_memory_with_format(bytes, ImageFormat::Gif)
        .context("无法解析 GIF 图片")?;

    let mut output = Vec::new();
    img.write_to(
        &mut Cursor::new(&mut output),
        ImageOutputFormat::Jpeg(92),
    )
    .context("提取 GIF 第一帧失败")?;

    Ok((output, "jpg"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_jpeg_format() {
        // JPEG 文件头: FF D8 FF
        let jpeg_header = vec![0xFF, 0xD8, 0xFF, 0xE0];
        let result = detect_format(&jpeg_header);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), SupportedFormat::Jpeg);
    }

    #[test]
    fn test_detect_png_format() {
        // PNG 文件头: 89 50 4E 47 0D 0A 1A 0A
        let png_header = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        let result = detect_format(&png_header);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), SupportedFormat::Png);
    }

    #[test]
    fn test_invalid_format() {
        let invalid_data = vec![0x00, 0x01, 0x02, 0x03];
        let result = detect_format(&invalid_data);
        assert!(result.is_err());
    }
}
```

### 3. 集成到上传流程

**修改文件**: `backend/src/handlers/analysis.rs`

```rust
async fn upload_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
    let mut file_bytes = None;
    let mut filename = None;
    let mut content_type = None;

    while let Some(field) = multipart.next_field().await.map_err(|err| {
        AppError::BadRequest(format!("无效的表单字段: {}", err))
    })? {
        if field.name() != Some("file") {
            continue;
        }

        filename = field.file_name().map(|name| name.to_string());
        content_type = field.content_type().map(|ct| ct.to_string());

        let bytes = field.bytes().await.map_err(|err| {
            AppError::BadRequest(format!("读取文件失败: {}", err))
        })?;

        if bytes.len() > MAX_UPLOAD_BYTES {
            return Err(AppError::PayloadTooLarge(
                format!("文件大小超过限制。当前: {:.2}MB，最大: 10MB",
                    bytes.len() as f64 / 1024.0 / 1024.0)
            ));
        }

        file_bytes = Some(bytes.to_vec());
        break;
    }

    let file_bytes = file_bytes.ok_or_else(|| {
        AppError::BadRequest("缺少文件字段".to_string())
    })?;

    // 基本验证 Content-Type
    validate_content_type(content_type.as_deref())?;

    // 存储图片（内部会进行格式检测和转换）
    let image_url = storage::store_image(
        &file_bytes,
        &state.config.upload_dir,
        filename.as_deref(),
    )
    .await
    .map_err(|err| AppError::Storage(err.to_string()))?;

    let id = db::insert_analysis(&state.pool, &image_url).await?;

    let pool = state.pool.clone();
    let config = state.config.clone();
    let image_url_clone = image_url.clone();
    tokio::spawn(async move {
        run_ocr_task(pool, config, id, image_url_clone).await;
    });

    Ok(Json(UploadResponse {
        id,
        status: AnalysisStatus::OcrPending,
        image_url,
    }))
}
```

**修改文件**: `backend/src/services/storage.rs`

```rust
use crate::services::image_converter;

pub async fn store_image(
    bytes: &[u8],
    upload_dir: &str,
    original_filename: Option<&str>,
) -> Result<String> {
    // 1. 检测图片格式
    let format = image_converter::detect_format(bytes)
        .context("图片格式检测失败")?;

    // 2. 转换格式（如果需要）
    let (final_bytes, extension) = if format.needs_conversion() {
        tracing::info!(
            "转换图片格式: {:?} -> {}",
            format,
            format.target_format()
        );
        image_converter::convert_image(bytes, format)
            .context("图片格式转换失败")?
    } else {
        let ext = match format {
            image_converter::SupportedFormat::Jpeg => "jpg",
            image_converter::SupportedFormat::Png => "png",
            _ => unreachable!(),
        };
        (bytes.to_vec(), ext)
    };

    // 3. 生成文件名
    let filename = generate_filename(original_filename, extension);
    let file_path = Path::new(upload_dir).join(&filename);

    // 4. 确保目录存在
    if let Some(parent) = file_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    // 5. 写入文件
    tokio::fs::write(&file_path, &final_bytes).await?;

    // 6. 返回相对路径
    Ok(format!("/uploads/{}", filename))
}

fn generate_filename(original: Option<&str>, extension: &str) -> String {
    let uuid = uuid::Uuid::new_v4();
    let timestamp = chrono::Utc::now().timestamp();

    if let Some(original) = original {
        let stem = Path::new(original)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("image");
        format!("{}_{}_{}. {}", stem, timestamp, uuid, extension)
    } else {
        format!("image_{}_{}.{}", timestamp, uuid, extension)
    }
}
```

### 4. HEIC 格式支持（可选）

如果需要支持 HEIC 格式，有两种方案：

#### 方案 A：使用 `libheif-rs`（推荐）

```rust
// Cargo.toml
[dependencies]
libheif-rs = "0.22"

// image_converter.rs
#[cfg(feature = "heic")]
pub fn convert_heic_to_jpeg(bytes: &[u8]) -> Result<(Vec<u8>, &'static str)> {
    use libheif_rs::{HeifContext, RgbChroma, ColorSpace};

    let ctx = HeifContext::read_from_bytes(bytes)
        .context("无法解析 HEIC 文件")?;

    let handle = ctx.primary_image_handle()
        .context("无法获取 HEIC 主图")?;

    let image = handle.decode(ColorSpace::Rgb(RgbChroma::Rgb), None)
        .context("解码 HEIC 图片失败")?;

    let planes = image.planes();
    let interleaved = planes.interleaved
        .ok_or_else(|| anyhow::anyhow!("HEIC 图片数据格式错误"))?;

    // 转换为 DynamicImage
    let img = image::RgbImage::from_raw(
        image.width(),
        image.height(),
        interleaved.data.to_vec(),
    )
    .ok_or_else(|| anyhow::anyhow!("HEIC 转换失败"))?;

    let dynamic_img = image::DynamicImage::ImageRgb8(img);

    // 输出为 JPEG
    let mut output = Vec::new();
    dynamic_img.write_to(
        &mut Cursor::new(&mut output),
        ImageOutputFormat::Jpeg(92),
    )?;

    Ok((output, "jpg"))
}
```

**优点**：原生支持，质量高
**缺点**：需要系统安装 libheif 库

#### 方案 B：使用云端转换服务

```rust
pub async fn convert_heic_via_service(bytes: &[u8]) -> Result<(Vec<u8>, &'static str)> {
    // 调用云端 API 进行转换
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.example.com/convert/heic-to-jpeg")
        .body(bytes.to_vec())
        .send()
        .await?;

    let jpeg_bytes = response.bytes().await?.to_vec();
    Ok((jpeg_bytes, "jpg"))
}
```

**优点**：无需本地依赖
**缺点**：依赖外部服务，有延迟

## 错误处理

### 错误类型定义

```rust
// backend/src/errors.rs
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    // ... 现有错误类型 ...

    #[error("不支持的图片格式: {0}")]
    UnsupportedImageFormat(String),

    #[error("图片文件损坏: {0}")]
    CorruptedImage(String),

    #[error("图片格式转换失败: {0}")]
    ImageConversionFailed(String),
}
```

### 错误响应示例

```json
{
  "error": "UnsupportedImageFormat",
  "message": "不支持的图片格式: HEIC。支持的格式: JPEG, PNG, WebP, BMP, GIF, TIFF",
  "details": {
    "supported_formats": ["JPEG", "PNG", "WebP", "BMP", "GIF", "TIFF"],
    "detected_format": "HEIC"
  }
}
```

## 性能考虑
### 5. 前端全局 Toast 组件设计

#### 组件结构

**新增文件**: `frontend/src/components/toast.rs`

```rust
use leptos::prelude::*;
use std::time::Duration;

/// Toast 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastType {
    Success,  // 成功（绿色）
    Error,    // 错误（红色）
    Warning,  // 警告（黄色）
    Info,     // 信息（蓝色）
}

/// Toast 消息
#[derive(Debug, Clone)]
pub struct ToastMessage {
    pub id: uuid::Uuid,
    pub message: String,
    pub toast_type: ToastType,
    pub duration: Duration,  // 显示时长
}

/// Toast 容器组件
#[component]
pub fn ToastContainer() -> impl IntoView {
    // 全局 Toast 状态
    let toasts = use_context::<RwSignal<Vec<ToastMessage>>>()
        .expect("ToastContainer must be used within ToastProvider");

    view! {
        <div class="toast-container">
            <For
                each=move || toasts.get()
                key=|toast| toast.id
                children=move |toast| {
                    view! {
                        <ToastItem toast=toast />
                    }
                }
            />
        </div>
    }
}

/// 单个 Toast 项
#[component]
fn ToastItem(toast: ToastMessage) -> impl IntoView {
    let toasts = use_context::<RwSignal<Vec<ToastMessage>>>()
        .expect("ToastItem must be used within ToastProvider");

    let toast_id = toast.id;
    let (visible, set_visible) = create_signal(true);

    // 自动消失
    create_effect(move |_| {
        let duration = toast.duration;
        set_timeout(
            move || {
                set_visible.set(false);
                // 动画结束后移除
                set_timeout(
                    move || {
                        toasts.update(|list| {
                            list.retain(|t| t.id != toast_id);
                        });
                    },
                    Duration::from_millis(300),
                );
            },
            duration,
        );
    });

    let toast_class = move || {
        let base = "toast-item";
        let type_class = match toast.toast_type {
            ToastType::Success => "toast-success",
            ToastType::Error => "toast-error",
            ToastType::Warning => "toast-warning",
            ToastType::Info => "toast-info",
        };
        let visible_class = if visible.get() { "visible" } else { "hidden" };
        format!("{} {} {}", base, type_class, visible_class)
    };

    let icon = match toast.toast_type {
        ToastType::Success => "✓",
        ToastType::Error => "✕",
        ToastType::Warning => "⚠",
        ToastType::Info => "ℹ",
    };

    let close_toast = move |_| {
        set_visible.set(false);
        set_timeout(
            move || {
                toasts.update(|list| {
                    list.retain(|t| t.id != toast_id);
                });
            },
            Duration::from_millis(300),
        );
    };

    view! {
        <div class=toast_class>
            <div class="toast-icon">{icon}</div>
            <div class="toast-message">{toast.message.clone()}</div>
            <button class="toast-close" on:click=close_toast>
                "×"
            </button>
        </div>
    }
}

/// Toast Provider（在 App 根组件中使用）
#[component]
pub fn ToastProvider(children: Children) -> impl IntoView {
    let toasts = create_rw_signal(Vec::<ToastMessage>::new());
    provide_context(toasts);

    view! {
        {children()}
        <ToastContainer />
    }
}

/// 显示 Toast 的辅助函数
pub fn show_toast(message: impl Into<String>, toast_type: ToastType) {
    if let Some(toasts) = use_context::<RwSignal<Vec<ToastMessage>>>() {
        let toast = ToastMessage {
            id: uuid::Uuid::new_v4(),
            message: message.into(),
            toast_type,
            duration: Duration::from_secs(4),
        };
        toasts.update(|list| list.push(toast));
    }
}

/// 便捷方法
pub fn show_error(message: impl Into<String>) {
    show_toast(message, ToastType::Error);
}

pub fn show_success(message: impl Into<String>) {
    show_toast(message, ToastType::Success);
}

pub fn show_warning(message: impl Into<String>) {
    show_toast(message, ToastType::Warning);
}

pub fn show_info(message: impl Into<String>) {
    show_toast(message, ToastType::Info);
}
```

#### CSS 样式

**修改文件**: `frontend/src/styles/app.css`

```css
/* Toast 容器 */
.toast-container {
  position: fixed;
  top: 20px;
  right: 20px;
  z-index: 9999;
  display: flex;
  flex-direction: column;
  gap: 12px;
  pointer-events: none;
}

/* Toast 项 */
.toast-item {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 300px;
  max-width: 400px;
  padding: 16px 20px;
  background: white;
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  pointer-events: auto;
  transform: translateX(400px);
  opacity: 0;
  transition: all 0.3s ease;
}

.toast-item.visible {
  transform: translateX(0);
  opacity: 1;
}

.toast-item.hidden {
  transform: translateX(400px);
  opacity: 0;
}

/* Toast 图标 */
.toast-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  font-size: 16px;
  font-weight: bold;
  flex-shrink: 0;
}

/* Toast 消息 */
.toast-message {
  flex: 1;
  font-size: 14px;
  line-height: 1.5;
  color: #374151;
}

/* Toast 关闭按钮 */
.toast-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  color: #9ca3af;
  font-size: 20px;
  cursor: pointer;
  flex-shrink: 0;
  transition: color 0.2s;
}

.toast-close:hover {
  color: #374151;
}

/* Toast 类型样式 */
.toast-success {
  border-left: 4px solid #10b981;
}

.toast-success .toast-icon {
  background: #d1fae5;
  color: #059669;
}

.toast-error {
  border-left: 4px solid #ef4444;
}

.toast-error .toast-icon {
  background: #fee2e2;
  color: #dc2626;
}

.toast-warning {
  border-left: 4px solid #f59e0b;
}

.toast-warning .toast-icon {
  background: #fef3c7;
  color: #d97706;
}

.toast-info {
  border-left: 4px solid #3b82f6;
}

.toast-info .toast-icon {
  background: #dbeafe;
  color: #2563eb;
}

/* 移动端适配 */
@media (max-width: 640px) {
  .toast-container {
    top: 16px;
    right: 16px;
    left: 16px;
  }

  .toast-item {
    min-width: auto;
    max-width: none;
  }
}
```

#### API 错误拦截器

**新增文件**: `frontend/src/utils/api_error_handler.rs`

```rust
use crate::components::toast::{show_error, show_warning};

/// 统一处理 API 错误
pub fn handle_api_error(status: u16, message: Option<String>) {
    let user_message = match status {
        400 => {
            // 解析后端返回的错误信息
            if let Some(msg) = message {
                if msg.contains("不支持") || msg.contains("格式") {
                    "不支持该图片格式，请选择 JPEG、PNG 或 HEIC 格式的照片"
                } else if msg.contains("过大") || msg.contains("超过") {
                    "图片文件过大，请选择小于 10MB 的图片"
                } else {
                    &msg
                }
            } else {
                "请求参数错误，请重试"
            }
        }
        401 => "登录已过期，请重新登录",
        403 => "没有权限执行此操作",
        404 => "请求的资源不存在",
        413 => "文件过大，请选择小于 10MB 的图片",
        415 => "不支持该文件格式",
        500 => "服务器错误，请稍后重试",
        502 | 503 | 504 => "服务暂时不可用，请稍后重试",
        _ => "操作失败，请重试",
    };

    show_error(user_message);
}

/// 处理网络错误
pub fn handle_network_error() {
    show_error("网络连接失败，请检查网络后重试");
}

/// 处理超时错误
pub fn handle_timeout_error() {
    show_warning("请求超时，请重试");
}
```

#### 集成到 App

**修改文件**: `frontend/src/lib.rs` 或 `frontend/src/app.rs`

```rust
use crate::components::toast::ToastProvider;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ToastProvider>
            // 现有的 App 内容
            <Router>
                // ... routes ...
            </Router>
        </ToastProvider>
    }
}
```

#### 使用示例

```rust
// 在上传处理中使用
async fn handle_upload(file: File) -> Result<(), String> {
    match upload_image(file).await {
        Ok(response) => {
            show_success("图片上传成功");
            Ok(())
        }
        Err(err) => {
            // 自动转换为用户友好的错误提示
            handle_api_error(err.status(), Some(err.message()));
            Err(err.message())
        }
    }
}
```

#### 错误信息映射表

| 后端错误 | HTTP 状态码 | 用户提示 |
|---------|------------|---------|
| UnsupportedMediaType | 415 | 不支持该图片格式，请选择 JPEG、PNG 或 HEIC 格式的照片 |
| PayloadTooLarge | 413 | 图片文件过大（当前 X MB），请选择小于 10MB 的图片 |
| BadRequest (格式) | 400 | 图片格式不正确，请重新拍照或选择其他图片 |
| BadRequest (损坏) | 400 | 图片文件损坏，无法处理，请重新拍照 |
| Storage | 500 | 图片保存失败，请重试 |
| ImageConversionFailed | 500 | 图片处理失败，请重新拍照或选择其他图片 |
| Network Error | - | 上传失败，请检查网络连接后重试 |
| Timeout | - | 上传超时，请重试 |


### 内存管理

1. **流式处理**：
   - 使用 `bytes::Bytes` 避免多次复制
   - 转换完成后立即释放原始数据

2. **并发限制**：
   ```rust
   // 使用 tokio 的 Semaphore 限制并发转换数
   static CONVERSION_SEMAPHORE: Lazy<Semaphore> = Lazy::new(|| Semaphore::new(5));

   pub async fn convert_with_limit(bytes: &[u8]) -> Result<Vec<u8>> {
       let _permit = CONVERSION_SEMAPHORE.acquire().await?;
       tokio::task::spawn_blocking(move || {
           // 在独立线程中进行 CPU 密集型转换
           convert_image(bytes)
       })
       .await?
   }
   ```

3. **内存监控**：
   - 记录每次转换的内存使用
   - 超过阈值时拒绝新的转换请求

### 性能基准

| 操作 | 图片大小 | 预期时间 |
|------|---------|---------|
| 格式检测 | 任意 | < 10ms |
| JPEG → JPEG（无转换） | 5MB | < 50ms |
| PNG → JPEG | 5MB | < 500ms |
| BMP → JPEG | 5MB | < 800ms |
| HEIC → JPEG | 5MB | < 1.5s |

## 测试策略

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_upload_jpeg() {
        let jpeg_bytes = include_bytes!("../../tests/fixtures/test.jpg");
        let result = store_image(jpeg_bytes, "/tmp", Some("test.jpg")).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_upload_heic() {
        let heic_bytes = include_bytes!("../../tests/fixtures/test.heic");
        let result = store_image(heic_bytes, "/tmp", Some("test.heic")).await;
        assert!(result.is_ok());
        // 验证转换为 JPEG
        assert!(result.unwrap().ends_with(".jpg"));
    }

    #[tokio::test]
    async fn test_upload_invalid_format() {
        let invalid_bytes = b"not an image";
        let result = store_image(invalid_bytes, "/tmp", None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_upload_too_large() {
        let large_bytes = vec![0u8; 11 * 1024 * 1024]; // 11MB
        let result = upload_handler(large_bytes).await;
        assert!(matches!(result, Err(AppError::PayloadTooLarge(_))));
    }
}
```

### 集成测试

```rust
#[tokio::test]
async fn test_full_upload_flow() {
    let app = create_test_app().await;

    // 上传 HEIC 图片
    let response = app
        .post("/api/analysis/upload")
        .multipart(/* HEIC file */)
        .await;

    assert_eq!(response.status(), 200);

    let body: UploadResponse = response.json().await;
    assert!(body.image_url.ends_with(".jpg")); // 已转换为 JPEG
}
```

### 性能测试

```rust
#[tokio::test]
async fn benchmark_image_conversion() {
    let bmp_bytes = include_bytes!("../../tests/fixtures/large.bmp");

    let start = std::time::Instant::now();
    let result = convert_image(bmp_bytes, SupportedFormat::Bmp).await;
    let duration = start.elapsed();

    assert!(result.is_ok());
    assert!(duration.as_secs() < 2, "转换时间过长: {:?}", duration);
}
```

## 实施阶段

### 阶段 1：后端基础实现（1-2 天）

- [ ] 创建 `image_converter.rs` 模块
- [ ] 实现 `detect_format()` 函数
- [ ] 实现 `convert_image()` 函数（JPEG/PNG/BMP/TIFF）
- [ ] 重构 `validate_content_type()` 函数
- [ ] 修改 `storage::store_image()` 集成转换逻辑
- [ ] 优化后端错误信息（中文、友好）
- [ ] 添加单元测试

### 阶段 2：前端 Toast 组件（1 天）

- [ ] 创建 `toast.rs` 组件
- [ ] 实现 Toast 容器和 Toast 项
- [ ] 添加 Toast CSS 样式（与 Figma 风格一致）
- [ ] 创建 `api_error_handler.rs` 错误拦截器
- [ ] 集成 ToastProvider 到 App 根组件
- [ ] 更新所有 API 调用使用 Toast 提示
- [ ] 移除页面底部的静态错误显示

### 阶段 3：扩展格式支持（1 天）

- [ ] 实现 GIF 第一帧提取
- [ ] 实现 WebP 转换
- [ ] （可选）集成 HEIC 支持
- [ ] 添加格式特定的测试用例
- [ ] 测试所有格式的错误提示

### 阶段 4：优化和测试（1-2 天）

- [ ] 添加并发限制（Semaphore）
- [ ] 优化内存使用
- [ ] 性能基准测试
- [ ] 集成测试（后端 + 前端）
- [ ] 真机测试（iOS HEIC、Android 各种格式）
- [ ] 测试 Toast 在不同设备上的显示效果

### 阶段 5：文档和部署（0.5 天）

- [ ] 更新 API 文档
- [ ] 更新用户指南
- [ ] 更新前端组件文档
- [ ] 部署到测试环境
- [ ] 收集用户反馈

## 风险与缓解

| 风险 | 影响 | 可能性 | 缓解措施 |
|------|------|--------|----------|
| HEIC 格式转换失败 | 高 | 中 | 使用 libheif-rs 或云端服务，提供降级方案 |
| 格式转换耗时过长 | 中 | 中 | 使用 spawn_blocking 避免阻塞，添加超时机制 |
| 内存占用过高 | 高 | 低 | 使用 Semaphore 限制并发，监控内存使用 |
| 转换后图片质量下降影响 OCR | 高 | 低 | 设置高质量参数（JPEG 92），充分测试 |
| libheif 库跨平台兼容性问题 | 中 | 中 | 提供 feature flag，可选择不支持 HEIC |

## 待解决问题

| 问题 | 影响 | 负责人 | 状态 |
|------|------|--------|------|
| 是否需要支持 HEIC 格式 | 高 | 产品团队 | 开放 |
| libheif 库的部署复杂度 | 中 | 运维团队 | 开放 |
| 云端转换服务的成本 | 中 | 产品团队 | 开放 |
| SVG 光栅化方案 | 低 | 开发团队 | 开放 |

## 参考资料

- [image crate 文档](https://docs.rs/image/latest/image/)
- [libheif-rs 文档](https://docs.rs/libheif-rs/)
- [HEIC 格式规范](https://en.wikipedia.org/wiki/High_Efficiency_Image_File_Format)
- [图片格式对比](https://developer.mozilla.org/en-US/docs/Web/Media/Formats/Image_types)
- 项目文档：
  - `docs/requirements/009-image-upload-format-support-requirements.md`
  - `backend/src/handlers/analysis.rs`
  - `backend/src/services/storage.rs`

---

## 变更记录

| 版本 | 日期 | 作者 | 描述 |
|------|------|------|------|
| 1.0 | 2026-01-21 | Claude Code | 初始版本，包含格式检测、转换和错误处理方案 |
| 1.1 | 2026-01-21 | Claude Code | 添加前端全局 Toast 组件设计、API 错误拦截器、错误信息映射表 |
