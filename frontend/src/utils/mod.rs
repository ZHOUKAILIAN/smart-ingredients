//! Utility functions

/// Get the color for a health score
pub fn get_health_score_color(score: i32) -> &'static str {
    match score {
        0..=49 => "var(--risk-high)",
        50..=69 => "var(--risk-medium)",
        70..=100 => "var(--risk-low)",
        _ => "var(--muted)",
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

/// Get the CSS class for a risk badge
pub fn risk_badge_class(level: &str) -> &'static str {
    match level {
        "low" => "risk-badge risk-badge-low",
        "medium" => "risk-badge risk-badge-medium",
        "high" => "risk-badge risk-badge-high",
        _ => "risk-badge",
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
