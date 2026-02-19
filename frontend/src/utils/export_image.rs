//! Export analysis result as a shareable long-image using Canvas 2D API.
//!
//! Renders a clean, readable card with health score, summary, and ingredient
//! list. Text layout uses `measure_text()` to guarantee no overflow.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

// ── Data types ──────────────────────────────────────────────────────────

/// A single ingredient row to render.
#[derive(Clone, Debug)]
pub struct ExportIngredient {
    pub name: String,
    pub risk_level: String,
    pub description: String,
    pub is_focus: bool,
}

/// All data needed to render the export image.
#[derive(Clone, Debug)]
pub struct ExportData {
    pub health_score: i32,
    pub recommendation: String,
    pub ingredients: Vec<ExportIngredient>,
    pub warnings: Vec<String>,
    pub summary: String,
    pub preference_label: String,
}

// ── Constants ───────────────────────────────────────────────────────────

const IMG_W: f64 = 480.0;       // mobile-friendly width
const PAD: f64 = 24.0;           // outer padding
const CONTENT_W: f64 = IMG_W - PAD * 2.0;  // 432
const CARD_PAD: f64 = 18.0;      // inner card padding
const CARD_R: f64 = 12.0;        // card border radius
const SECTION_GAP: f64 = 16.0;   // gap between cards

// Fonts – use system fonts available in WebView
const FONT_FAMILY: &str = "'PingFang SC', 'Noto Sans SC', 'Hiragino Sans GB', 'Microsoft YaHei', sans-serif";

// Colors
const BG_TOP: &str = "#ecfdf5";
const BG_BOT: &str = "#f0fdf4";
const CARD_BG: &str = "#ffffff";
const PRIMARY: &str = "#10b981";
const PRIMARY_DARK: &str = "#059669";
const TEXT_DARK: &str = "#1f2937";
const TEXT_BODY: &str = "#374151";
const TEXT_MUTED: &str = "#6b7280";
const TEXT_LIGHT: &str = "#9ca3af";
const RISK_HIGH_COLOR: &str = "#ef4444";
const RISK_MED_COLOR: &str = "#f59e0b";
const RISK_LOW_COLOR: &str = "#10b981";
const DIVIDER: &str = "rgba(229, 231, 235, 0.8)";

// ── Public API ──────────────────────────────────────────────────────────

/// Export and immediately download the image.
pub fn export_and_download(data: &ExportData) -> Result<(), String> {
    let (canvas, _ctx) = render_image(data)?;
    download_canvas(&canvas)
}

/// Export to a data URL for preview in the modal.
pub fn export_to_data_url(data: &ExportData) -> Result<String, String> {
    let (canvas, _ctx) = render_image(data)?;
    canvas
        .to_data_url_with_type("image/png")
        .map_err(|_| "生成图片数据失败".to_string())
}

/// Check if running inside Tauri (Android or desktop app).
pub fn is_tauri_available() -> bool {
    let result = js_sys::eval(
        r#"(function() {
            var has = typeof window.__TAURI__ !== 'undefined';
            console.log('[SI] is_tauri_available:', has);
            return has;
        })()"#,
    )
    .ok()
    .and_then(|v| v.as_bool())
    .unwrap_or(false);
    web_sys::console::log_1(&format!("[SI-WASM] is_tauri_available = {}", result).into());
    result
}

/// Save image via Tauri IPC command (async). Returns the saved file path.
pub async fn save_via_tauri(data_url: &str) -> Result<String, String> {
    web_sys::console::log_1(&"[SI-WASM] save_via_tauri called".into());

    let parts: Vec<&str> = data_url.splitn(2, ',').collect();
    if parts.len() != 2 {
        return Err("无效的图片数据".to_string());
    }

    let base64_payload = parts[1];
    let filename = generate_filename();
    let window = web_sys::window().ok_or("无法获取 window")?;

    web_sys::console::log_1(
        &format!("[SI-WASM] saving as: {}, payload len: {}", filename, base64_payload.len()).into(),
    );

    // Store args as temporary window properties for JS eval access
    let _ = js_sys::Reflect::set(
        &window,
        &JsValue::from_str("__si_b64"),
        &JsValue::from_str(base64_payload),
    );
    let _ = js_sys::Reflect::set(
        &window,
        &JsValue::from_str("__si_fname"),
        &JsValue::from_str(&filename),
    );

    let promise = js_sys::eval(
        r#"(function() {
            console.log('[SI] invoking save_image_file via Tauri IPC');
            var b64 = window.__si_b64;
            var fname = window.__si_fname;
            delete window.__si_b64;
            delete window.__si_fname;
            return window.__TAURI__.core.invoke('save_image_file', {
                base64Data: b64,
                filename: fname
            });
        })()"#,
    )
    .map_err(|e| {
        let msg = format!("JS 调用失败: {:?}", e);
        web_sys::console::error_1(&msg.clone().into());
        msg
    })?;

    let promise: js_sys::Promise = promise
        .dyn_into()
        .map_err(|_| {
            let msg = "invoke 返回值不是 Promise".to_string();
            web_sys::console::error_1(&msg.clone().into());
            msg
        })?;

    let result = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|e| {
            let msg = e
                .as_string()
                .unwrap_or_else(|| format!("{:?}", e));
            let err = format!("保存失败: {}", msg);
            web_sys::console::error_1(&err.clone().into());
            err
        })?;

    let path = result
        .as_string()
        .ok_or_else(|| "返回路径格式错误".to_string())?;
    web_sys::console::log_1(&format!("[SI-WASM] save success: {}", path).into());
    Ok(path)
}

/// Download from a data URL string (browser fallback via `<a download>`).
pub fn download_from_data_url(data_url: &str) -> Result<(), String> {
    use js_sys::{Array, Uint8Array};

    let filename = generate_filename();

    let parts: Vec<&str> = data_url.splitn(2, ',').collect();
    if parts.len() != 2 {
        return Err("无效的图片数据".to_string());
    }

    let base64_payload = parts[1];
    let window = web_sys::window().ok_or("无法获取 window")?;

    let decoded = window
        .atob(base64_payload)
        .map_err(|_| "base64 解码失败".to_string())?;

    let len = decoded.len();
    let u8_array = Uint8Array::new_with_length(len as u32);
    for (i, ch) in decoded.chars().enumerate() {
        u8_array.set_index(i as u32, ch as u8);
    }

    let blob_parts = Array::new();
    blob_parts.push(&u8_array.buffer());
    let mut opts = web_sys::BlobPropertyBag::new();
    opts.set_type("image/png");
    let blob = web_sys::Blob::new_with_buffer_source_sequence_and_options(&blob_parts, &opts)
        .map_err(|_| "创建 Blob 失败".to_string())?;

    let object_url =
        web_sys::Url::create_object_url_with_blob(&blob).map_err(|_| "创建下载链接失败".to_string())?;

    let document = window.document().ok_or("无法获取 document")?;
    let anchor: web_sys::HtmlAnchorElement = document
        .create_element("a")
        .map_err(|_| "创建 anchor 失败")?
        .dyn_into()
        .map_err(|_| "anchor 类型转换失败")?;

    anchor.set_href(&object_url);
    anchor.set_download(&filename);
    anchor.style().set_property("display", "none").ok();

    let body = document.body().ok_or("获取 body 失败")?;
    body.append_child(&anchor).map_err(|_| "append 失败")?;
    anchor.click();
    body.remove_child(&anchor).ok();
    web_sys::Url::revoke_object_url(&object_url).ok();
    Ok(())
}

/// Generate and attempt native share; falls back to download.
pub fn export_and_share(data: &ExportData) -> Result<(), String> {
    export_and_download(data)
}

// ── Core rendering ──────────────────────────────────────────────────────

fn render_image(data: &ExportData) -> Result<(HtmlCanvasElement, CanvasRenderingContext2d), String> {
    let document = web_sys::window()
        .and_then(|w| w.document())
        .ok_or("无法获取 document")?;

    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .map_err(|_| "创建 canvas 失败")?
        .dyn_into()
        .map_err(|_| "canvas 类型转换失败")?;

    // We need a temporary context to measure text for height pre-calculation
    canvas.set_width(IMG_W as u32);
    canvas.set_height(4000); // temporary large height

    let ctx: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .map_err(|_| "获取 2d context 失败")?
        .ok_or("2d context 为空")?
        .dyn_into()
        .map_err(|_| "context 类型转换失败")?;

    // Pre-calculate actual height
    let total_height = calc_total_height(&ctx, data);
    canvas.set_height(total_height as u32);

    // Re-acquire context after resize
    let ctx: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .map_err(|_| "获取 2d context 失败")?
        .ok_or("2d context 为空")?
        .dyn_into()
        .map_err(|_| "context 类型转换失败")?;

    // ── Background gradient ─────────────────────────────────────────
    let gradient = ctx.create_linear_gradient(0.0, 0.0, 0.0, total_height);
    gradient.add_color_stop(0.0, BG_TOP).map_err(|_| "gradient err")?;
    gradient.add_color_stop(1.0, BG_BOT).map_err(|_| "gradient err")?;
    ctx.set_fill_style(&JsValue::from(&gradient));
    ctx.fill_rect(0.0, 0.0, IMG_W, total_height);

    let mut y = PAD;

    // ── 1. Header ───────────────────────────────────────────────────
    y = draw_header(&ctx, y, data);

    // ── 2. Score card ───────────────────────────────────────────────
    y = draw_score_section(&ctx, y, data);

    // ── 3. Summary card ─────────────────────────────────────────────
    if !data.summary.is_empty() {
        y = draw_summary_section(&ctx, y, data);
    }

    // ── 4. Ingredients table card ───────────────────────────────────
    if !data.ingredients.is_empty() {
        y = draw_ingredients_section(&ctx, y, data);
    }

    // ── 5. Watermark ────────────────────────────────────────────────
    draw_watermark(&ctx, total_height);

    let _ = y; // suppress unused warning
    Ok((canvas, ctx))
}

// ── Height pre-calculation ──────────────────────────────────────────────

fn calc_total_height(ctx: &CanvasRenderingContext2d, data: &ExportData) -> f64 {
    let mut h = PAD;

    // Header: brand + subtitle
    h += 60.0;
    h += SECTION_GAP;

    // Score section (dynamic height based on recommendation text wrapping)
    h += calc_score_section_height(ctx, data);
    h += SECTION_GAP;

    // Summary
    if !data.summary.is_empty() {
        let text_w = CONTENT_W - CARD_PAD * 2.0;
        let lines = measure_wrap_lines(ctx, &data.summary, text_w, 14.0);
        h += CARD_PAD * 2.0 + 28.0 + (lines as f64) * 22.0;
        h += SECTION_GAP;
    }

    // Ingredients
    if !data.ingredients.is_empty() {
        let count = data.ingredients.len().min(20);
        // title row + header row + items
        h += CARD_PAD * 2.0 + 32.0 + 32.0 + (count as f64) * 42.0;
        if data.ingredients.len() > 20 {
            h += 32.0; // "more" hint
        }
        h += SECTION_GAP;
    }

    // Watermark
    h += 48.0;
    h += PAD;

    h
}

/// Measure how many lines text will wrap into at a given font size.
fn measure_wrap_lines(ctx: &CanvasRenderingContext2d, text: &str, max_w: f64, font_size: f64) -> usize {
    ctx.set_font(&format!("{}px {}", font_size, FONT_FAMILY));
    let mut lines = 0usize;
    for paragraph in text.split('\n') {
        if paragraph.trim().is_empty() {
            lines += 1;
            continue;
        }
        let chars: Vec<char> = paragraph.chars().collect();
        let mut start = 0;
        while start < chars.len() {
            // Binary-ish search for how many chars fit
            let mut end = start + 1;
            while end <= chars.len() {
                let segment: String = chars[start..end].iter().collect();
                if let Ok(m) = ctx.measure_text(&segment) {
                    if m.width() > max_w {
                        break;
                    }
                }
                end += 1;
            }
            // end-1 chars fit on this line (or all remaining)
            let line_end = if end > chars.len() { chars.len() } else { (end - 1).max(start + 1) };
            lines += 1;
            start = line_end;
        }
    }
    lines.max(1)
}

/// Actually wrap text into lines that fit within max_w pixels.
fn wrap_text_measured(ctx: &CanvasRenderingContext2d, text: &str, max_w: f64, font_size: f64) -> Vec<String> {
    ctx.set_font(&format!("{}px {}", font_size, FONT_FAMILY));
    let mut result = Vec::new();
    for paragraph in text.split('\n') {
        if paragraph.trim().is_empty() {
            result.push(String::new());
            continue;
        }
        let chars: Vec<char> = paragraph.chars().collect();
        let mut start = 0;
        while start < chars.len() {
            let mut end = start + 1;
            while end <= chars.len() {
                let segment: String = chars[start..end].iter().collect();
                if let Ok(m) = ctx.measure_text(&segment) {
                    if m.width() > max_w {
                        break;
                    }
                }
                end += 1;
            }
            let line_end = if end > chars.len() { chars.len() } else { (end - 1).max(start + 1) };
            let line: String = chars[start..line_end].iter().collect();
            result.push(line);
            start = line_end;
        }
    }
    if result.is_empty() {
        result.push(String::new());
    }
    result
}

// ── Section renderers ───────────────────────────────────────────────────

fn draw_header(ctx: &CanvasRenderingContext2d, y: f64, _data: &ExportData) -> f64 {
    let mut cy = y;

    // Brand name
    set_font(ctx, "bold", 22.0);
    ctx.set_fill_style_str(PRIMARY_DARK);
    ctx.set_text_align("center");
    ctx.set_text_baseline("alphabetic");
    let _ = ctx.fill_text("Smart Ingredients", IMG_W / 2.0, cy + 26.0);
    cy += 32.0;

    // Subtitle
    set_font(ctx, "normal", 13.0);
    ctx.set_fill_style_str(TEXT_MUTED);
    let _ = ctx.fill_text("食品配料表智能分析报告", IMG_W / 2.0, cy + 16.0);
    cy += 28.0;

    ctx.set_text_align("left");
    cy
}

fn calc_score_section_height(ctx: &CanvasRenderingContext2d, data: &ExportData) -> f64 {
    let max_text_w = CONTENT_W - 104.0 - CARD_PAD;
    let base_h = 48.0; // title + score label
    let rec_lines = if data.recommendation.is_empty() {
        0
    } else {
        measure_wrap_lines(ctx, &data.recommendation, max_text_w, 12.0)
    };
    CARD_PAD * 2.0 + base_h + (rec_lines as f64) * 18.0
}

fn draw_score_section(ctx: &CanvasRenderingContext2d, y: f64, data: &ExportData) -> f64 {
    let card_h = calc_score_section_height(ctx, data);
    let card_x = PAD;
    let card_y = y;

    draw_card_bg(ctx, card_x, card_y, CONTENT_W, card_h);

    // Left side: score circle
    let cx = card_x + 54.0;
    let cy_center = card_y + card_h / 2.0;
    let r = 30.0;

    // Circle track
    ctx.begin_path();
    let _ = ctx.arc(cx, cy_center, r, 0.0, std::f64::consts::PI * 2.0);
    ctx.set_fill_style_str("#f0fdf4");
    ctx.fill();

    // Circle ring
    let color = score_color(data.health_score);
    ctx.begin_path();
    let _ = ctx.arc(cx, cy_center, r, 0.0, std::f64::consts::PI * 2.0);
    ctx.set_stroke_style_str(color);
    ctx.set_line_width(5.0);
    ctx.stroke();

    // Score number
    set_font(ctx, "bold", 24.0);
    ctx.set_fill_style_str(TEXT_DARK);
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");
    let _ = ctx.fill_text(&data.health_score.to_string(), cx, cy_center);

    // Right side: label, description
    let lx = card_x + 104.0;
    let max_text_w = CONTENT_W - 104.0 - CARD_PAD;
    ctx.set_text_align("left");
    ctx.set_text_baseline("alphabetic");

    set_font(ctx, "bold", 16.0);
    ctx.set_fill_style_str(TEXT_DARK);
    let _ = ctx.fill_text("健康评分", lx, card_y + CARD_PAD + 20.0);

    set_font(ctx, "normal", 13.0);
    ctx.set_fill_style_str(color);
    let label = score_label(data.health_score);
    let _ = ctx.fill_text(label, lx, card_y + CARD_PAD + 40.0);

    // Recommendation text (wrapped, not truncated)
    if !data.recommendation.is_empty() {
        set_font(ctx, "normal", 12.0);
        ctx.set_fill_style_str(TEXT_MUTED);
        let lines = wrap_text_measured(ctx, &data.recommendation, max_text_w, 12.0);
        let mut rec_y = card_y + CARD_PAD + 56.0;
        for line in &lines {
            let _ = ctx.fill_text(line, lx, rec_y);
            rec_y += 18.0;
        }
    }

    card_y + card_h + SECTION_GAP
}

fn draw_summary_section(ctx: &CanvasRenderingContext2d, y: f64, data: &ExportData) -> f64 {
    let text_w = CONTENT_W - CARD_PAD * 2.0;
    let lines = wrap_text_measured(ctx, &data.summary, text_w, 14.0);
    let title_h = 28.0;
    let line_h = 22.0;
    let card_h = CARD_PAD * 2.0 + title_h + (lines.len() as f64) * line_h;
    let card_x = PAD;

    draw_card_bg(ctx, card_x, y, CONTENT_W, card_h);

    // Title
    let mut cy = y + CARD_PAD;
    set_font(ctx, "bold", 15.0);
    ctx.set_fill_style_str(TEXT_DARK);
    ctx.set_text_align("left");
    ctx.set_text_baseline("alphabetic");
    let _ = ctx.fill_text("📋 分析摘要", card_x + CARD_PAD, cy + 18.0);
    cy += title_h;

    // Summary lines
    set_font(ctx, "normal", 14.0);
    ctx.set_fill_style_str(TEXT_BODY);
    for line in &lines {
        let _ = ctx.fill_text(line, card_x + CARD_PAD, cy + 16.0);
        cy += line_h;
    }

    y + card_h + SECTION_GAP
}

fn draw_ingredients_section(ctx: &CanvasRenderingContext2d, y: f64, data: &ExportData) -> f64 {
    let items = &data.ingredients;
    let count = items.len().min(20);
    let title_h = 32.0;
    let header_h = 32.0;
    let row_h = 42.0;
    let more_h = if items.len() > 20 { 32.0 } else { 0.0 };
    let card_h = CARD_PAD * 2.0 + title_h + header_h + (count as f64) * row_h + more_h;
    let card_x = PAD;

    draw_card_bg(ctx, card_x, y, CONTENT_W, card_h);

    let inner_x = card_x + CARD_PAD;
    let inner_w = CONTENT_W - CARD_PAD * 2.0;
    let mut cy = y + CARD_PAD;

    // Title
    set_font(ctx, "bold", 15.0);
    ctx.set_fill_style_str(TEXT_DARK);
    ctx.set_text_align("left");
    ctx.set_text_baseline("alphabetic");
    let _ = ctx.fill_text("🧪 配料分析", inner_x, cy + 20.0);
    cy += title_h;

    // Column layout: Name | Description | Risk
    let col_name_w = 80.0;
    let col_risk_w = 56.0;
    let col_desc_w = inner_w - col_name_w - col_risk_w - 16.0;

    // Table header
    set_font(ctx, "bold", 12.0);
    ctx.set_fill_style_str(TEXT_MUTED);
    let _ = ctx.fill_text("配料名称", inner_x, cy + 20.0);
    let _ = ctx.fill_text("说明", inner_x + col_name_w + 8.0, cy + 20.0);
    ctx.set_text_align("center");
    let _ = ctx.fill_text("风险", inner_x + inner_w - col_risk_w / 2.0, cy + 20.0);
    ctx.set_text_align("left");

    // Header divider
    cy += header_h;
    ctx.set_stroke_style_str(DIVIDER);
    ctx.set_line_width(1.0);
    ctx.begin_path();
    ctx.move_to(inner_x, cy);
    ctx.line_to(inner_x + inner_w, cy);
    ctx.stroke();

    // Rows
    for (i, item) in items.iter().take(20).enumerate() {
        let ry = cy + (i as f64) * row_h;

        // Alternating background
        if i % 2 == 0 {
            draw_rounded_rect(ctx, inner_x, ry, inner_w, row_h, 4.0);
            ctx.set_fill_style_str("rgba(248, 250, 252, 0.6)");
            ctx.fill();
        }

        // Name
        set_font(ctx, "bold", 13.0);
        ctx.set_fill_style_str(TEXT_DARK);
        ctx.set_text_align("left");
        let name_display = if item.is_focus {
            format!("⭐ {}", item.name)
        } else {
            item.name.clone()
        };
        let name_fitted = fit_text(ctx, &name_display, col_name_w);
        let _ = ctx.fill_text(&name_fitted, inner_x + 6.0, ry + 26.0);

        // Description
        set_font(ctx, "normal", 12.0);
        ctx.set_fill_style_str(TEXT_MUTED);
        let desc = if item.description.is_empty() {
            "—".to_string()
        } else {
            item.description.clone()
        };
        let desc_fitted = fit_text(ctx, &desc, col_desc_w);
        let _ = ctx.fill_text(&desc_fitted, inner_x + col_name_w + 8.0, ry + 26.0);

        // Risk badge
        let (badge_text, badge_color) = risk_badge(&item.risk_level);
        let badge_w = 48.0;
        let badge_h = 22.0;
        let badge_x = inner_x + inner_w - col_risk_w / 2.0 - badge_w / 2.0;
        let badge_y = ry + (row_h - badge_h) / 2.0;

        draw_rounded_rect(ctx, badge_x, badge_y, badge_w, badge_h, 11.0);
        ctx.set_fill_style_str(badge_color);
        ctx.set_global_alpha(0.12);
        ctx.fill();
        ctx.set_global_alpha(1.0);

        set_font(ctx, "bold", 11.0);
        ctx.set_fill_style_str(badge_color);
        ctx.set_text_align("center");
        let _ = ctx.fill_text(badge_text, badge_x + badge_w / 2.0, badge_y + 15.0);
        ctx.set_text_align("left");

        // Row divider
        if i < count - 1 {
            ctx.set_stroke_style_str(DIVIDER);
            ctx.set_line_width(0.5);
            ctx.begin_path();
            ctx.move_to(inner_x, ry + row_h);
            ctx.line_to(inner_x + inner_w, ry + row_h);
            ctx.stroke();
        }
    }

    // "More" hint
    if items.len() > 20 {
        let more_y = cy + (count as f64) * row_h + 6.0;
        set_font(ctx, "normal", 12.0);
        ctx.set_fill_style_str(TEXT_MUTED);
        ctx.set_text_align("center");
        let _ = ctx.fill_text(
            &format!("还有 {} 项配料未显示…", items.len() - 20),
            IMG_W / 2.0,
            more_y + 14.0,
        );
        ctx.set_text_align("left");
    }

    y + card_h + SECTION_GAP
}

fn draw_watermark(ctx: &CanvasRenderingContext2d, total_h: f64) {
    let wm_y = total_h - PAD - 12.0;

    // Divider
    ctx.set_stroke_style_str("rgba(148, 163, 184, 0.3)");
    ctx.set_line_width(1.0);
    ctx.begin_path();
    ctx.move_to(PAD + 40.0, wm_y - 20.0);
    ctx.line_to(IMG_W - PAD - 40.0, wm_y - 20.0);
    ctx.stroke();

    set_font(ctx, "normal", 11.0);
    ctx.set_fill_style_str(TEXT_LIGHT);
    ctx.set_text_align("center");
    let _ = ctx.fill_text(
        "由 Smart Ingredients 智能分析生成 · 仅供参考",
        IMG_W / 2.0,
        wm_y,
    );
    ctx.set_text_align("left");
}

// ── Primitive helpers ───────────────────────────────────────────────────

fn set_font(ctx: &CanvasRenderingContext2d, weight: &str, size: f64) {
    ctx.set_font(&format!("{} {}px {}", weight, size, FONT_FAMILY));
}

fn draw_card_bg(ctx: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64) {
    // Shadow
    ctx.set_shadow_color("rgba(15, 23, 42, 0.06)");
    ctx.set_shadow_blur(12.0);
    ctx.set_shadow_offset_x(0.0);
    ctx.set_shadow_offset_y(4.0);

    draw_rounded_rect(ctx, x, y, w, h, CARD_R);
    ctx.set_fill_style_str(CARD_BG);
    ctx.fill();

    // Reset shadow
    ctx.set_shadow_color("transparent");
    ctx.set_shadow_blur(0.0);
    ctx.set_shadow_offset_y(0.0);

    // Border
    draw_rounded_rect(ctx, x, y, w, h, CARD_R);
    ctx.set_stroke_style_str(DIVIDER);
    ctx.set_line_width(1.0);
    ctx.stroke();
}

fn draw_rounded_rect(ctx: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64, r: f64) {
    ctx.begin_path();
    ctx.move_to(x + r, y);
    ctx.line_to(x + w - r, y);
    ctx.quadratic_curve_to(x + w, y, x + w, y + r);
    ctx.line_to(x + w, y + h - r);
    ctx.quadratic_curve_to(x + w, y + h, x + w - r, y + h);
    ctx.line_to(x + r, y + h);
    ctx.quadratic_curve_to(x, y + h, x, y + h - r);
    ctx.line_to(x, y + r);
    ctx.quadratic_curve_to(x, y, x + r, y);
    ctx.close_path();
}

/// Truncate text to fit within max_w pixels, appending "…" if needed.
fn fit_text(ctx: &CanvasRenderingContext2d, text: &str, max_w: f64) -> String {
    if let Ok(m) = ctx.measure_text(text) {
        if m.width() <= max_w {
            return text.to_string();
        }
    }
    let chars: Vec<char> = text.chars().collect();
    for len in (1..chars.len()).rev() {
        let candidate: String = chars[..len].iter().collect();
        let with_ellipsis = format!("{}…", candidate);
        if let Ok(m) = ctx.measure_text(&with_ellipsis) {
            if m.width() <= max_w {
                return with_ellipsis;
            }
        }
    }
    "…".to_string()
}

fn score_color(score: i32) -> &'static str {
    match score {
        0..=49 => RISK_HIGH_COLOR,
        50..=69 => RISK_MED_COLOR,
        _ => RISK_LOW_COLOR,
    }
}

fn score_label(score: i32) -> &'static str {
    match score {
        0..=49 => "⚠️ 需要注意",
        50..=69 => "🔶 一般",
        _ => "✅ 良好",
    }
}

fn risk_badge(level: &str) -> (&'static str, &'static str) {
    match level.to_lowercase().as_str() {
        "high" => ("高风险", RISK_HIGH_COLOR),
        "medium" => ("中风险", RISK_MED_COLOR),
        "low" => ("低风险", RISK_LOW_COLOR),
        _ => ("未知", TEXT_LIGHT),
    }
}

fn generate_filename() -> String {
    let date = js_sys::Date::new_0();
    let y = date.get_full_year();
    let m = date.get_month() + 1;
    let d = date.get_date();
    format!("smart-ingredients-{y}{m:02}{d:02}.png")
}

fn download_canvas(canvas: &HtmlCanvasElement) -> Result<(), String> {
    let data_url = canvas
        .to_data_url_with_type("image/png")
        .map_err(|_| "生成图片数据失败")?;
    download_from_data_url(&data_url)
}
