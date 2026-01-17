//! Risk badge component

use leptos::prelude::*;
use crate::utils::risk_badge_class;

#[component]
pub fn RiskBadge(level: String) -> impl IntoView {
    let badge_class = risk_badge_class(&level);
    let label = match level.as_str() {
        "low" => "低风险",
        "medium" => "中风险",
        "high" => "高风险",
        _ => "未知",
    };

    view! {
        <span class={badge_class}>{label}</span>
    }
}
