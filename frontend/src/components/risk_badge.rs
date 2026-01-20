//! Risk badge component

use leptos::prelude::*;
use crate::utils::risk_badge_class;

#[component]
pub fn RiskBadge(level: String) -> impl IntoView {
    let badge_class = risk_badge_class(&level);
    let label = match level.as_str() {
        "low" => "安全",
        "medium" => "注意",
        "high" => "风险",
        _ => "未知",
    };

    view! {
        <span class={badge_class}>{label}</span>
    }
}
