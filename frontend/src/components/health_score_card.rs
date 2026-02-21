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
    let label_style = format!("color: {}", score_color);

    view! {
        <div class="surface-card score-card">
            <div class="score-header">
                <span class="score-label">"健康评分"</span>
                <span class="score-level" style={label_style.clone()}>{score_label}</span>
            </div>
            <div class="score-main">
                <span class="score-value" style={label_style.clone()}>{normalized_score}</span>
                <span class="score-unit">"/100"</span>
            </div>
            <p class="score-recommendation">{display_text}</p>
            <div class="score-progress">
                <div class="score-progress-bar" style={progress_style}></div>
            </div>
        </div>
    }
}
