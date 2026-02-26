//! Risk badge component

use leptos::prelude::*;

#[component]
pub fn RiskBadge(level: String) -> impl IntoView {
    let (badge_class, label) = match level.as_str() {
        "low" => (
            "inline-flex items-center px-2 py-1 rounded-full text-xs font-semibold bg-emerald-50 text-emerald-700 border border-emerald-100",
            "健康",
        ),
        "medium" => (
            "inline-flex items-center px-2 py-1 rounded-full text-xs font-semibold bg-amber-50 text-amber-700 border border-amber-100",
            "注意",
        ),
        "high" => (
            "inline-flex items-center px-2 py-1 rounded-full text-xs font-semibold bg-red-50 text-red-700 border border-red-100",
            "风险",
        ),
        _ => (
            "inline-flex items-center px-2 py-1 rounded-full text-xs font-semibold bg-gray-50 text-gray-600 border border-gray-100",
            "未知",
        ),
    };

    view! {
        <span class={badge_class}>{label}</span>
    }
}
