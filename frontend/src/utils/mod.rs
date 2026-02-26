//! Utility functions

pub mod auth_storage;
pub mod community_share;
pub mod community_share_storage;
pub mod community_ui;
pub mod error_messages;
pub mod export_image;
pub mod local_history;
pub mod local_storage;
pub mod navigation;
pub mod preference;
pub mod presentation;

use wasm_bindgen::JsValue;
use web_sys::{CustomEvent, CustomEventInit};

use crate::stores::ToastLevel;

/// Get the color for a health score (hex values matching Tailwind palette)
pub fn get_health_score_color(score: i32) -> &'static str {
    match score {
        0..=49 => "#dc2626",   // red-600
        50..=69 => "#d97706",  // amber-600
        70..=100 => "#059669", // emerald-600
        _ => "#6b7280",        // gray-500
    }
}

/// Get the label for a health score
pub fn get_health_score_label(score: i32) -> &'static str {
    match score {
        0..=49 => "需要注意",
        50..=69 => "一般",
        70..=100 => "良好",
        _ => "未知",
    }
}

/// Translate ingredient category to a user-facing label.
pub fn category_label(value: &str) -> String {
    match value.to_lowercase().as_str() {
        "additive" => "additive/添加剂".to_string(),
        "allergen" => "allergen/过敏原".to_string(),
        "nutrition" => "nutrition/营养成分".to_string(),
        "other" => "other/其他".to_string(),
        _ => value.to_string(),
    }
}

pub fn emit_toast(level: ToastLevel, title: &str, message: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let detail = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &detail,
        &JsValue::from_str("title"),
        &JsValue::from_str(title),
    );
    let _ = js_sys::Reflect::set(
        &detail,
        &JsValue::from_str("message"),
        &JsValue::from_str(message),
    );
    let _ = js_sys::Reflect::set(
        &detail,
        &JsValue::from_str("level"),
        &JsValue::from_str(match level {
            ToastLevel::Error => "error",
            ToastLevel::Warning => "warning",
            ToastLevel::Success => "success",
            ToastLevel::Info => "info",
        }),
    );

    let mut init = CustomEventInit::new();
    init.detail(&detail);
    if let Ok(event) = CustomEvent::new_with_event_init_dict("global-toast", &init) {
        let _ = window.dispatch_event(&event);
    }
}
