//! Health score card component

use crate::utils::{get_health_score_color, get_health_score_label};
use leptos::prelude::*;

#[component]
pub fn HealthScoreCard(score: i32, recommendation: String) -> impl IntoView {
    let normalized_score = score.clamp(0, 100);
    let score_color = get_health_score_color(normalized_score);
    let score_label = get_health_score_label(normalized_score);
    let display_text = if recommendation.trim().is_empty() {
        format!("综合评估：{}", score_label)
    } else {
        recommendation
    };
    let progress_style = format!("width: {}%; background: {}", normalized_score, score_color);
    let score_class = move || {
        if normalized_score >= 70 {
            "text-emerald-600"
        } else if normalized_score >= 50 {
            "text-amber-600"
        } else {
            "text-red-600"
        }
    };

    view! {
        <div class="rounded-2xl border border-emerald-100 bg-white-95 shadow-lg p-4">
            <div class="flex items-center justify-between mb-2">
                <span class="text-sm font-semibold text-gray-800">"健康评分"</span>
                <span class=move || format!("text-sm font-semibold {}", score_class())>{score_label}</span>
            </div>
            <div class="flex items-end gap-1 mb-2">
                <span class=move || format!("text-4xl font-bold leading-none {}", score_class())>{normalized_score}</span>
                <span class="text-sm text-gray-500 mb-1">"/100"</span>
            </div>
            <p class="text-sm text-gray-700 m-0 mb-3">{display_text}</p>
            <div class="w-full h-2 rounded-full bg-emerald-100 overflow-hidden">
                <div class="h-full rounded-full transition-all" style={progress_style}></div>
            </div>
        </div>
    }
}
