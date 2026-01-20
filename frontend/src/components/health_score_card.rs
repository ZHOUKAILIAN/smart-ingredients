//! Health score card component

use leptos::prelude::*;
use crate::utils::{get_health_score_color, get_health_score_label};

#[component]
pub fn HealthScoreCard(
    score: i32,
    recommendation: String,
) -> impl IntoView {
    let normalized_score = score.clamp(0, 100);
    let score_color = get_health_score_color(normalized_score);
    let score_label = get_health_score_label(normalized_score);
    let display_text = if recommendation.trim().is_empty() {
        score_label.to_string()
    } else {
        recommendation
    };
    let progress_style = format!(
        "width: {}%; background: {}",
        normalized_score, score_color
    );

    view! {
        <div class="surface-card score-card">
            <div class="score-circle" style={format!("border-color: {}", score_color)}>
                <span class="score-value">{normalized_score}</span>
            </div>
            <div class="score-meta">
                <h2>"健康评分"</h2>
                <p>{display_text}</p>
            </div>
            <div class="score-progress">
                <div class="score-progress-bar" style={progress_style}></div>
            </div>
        </div>
    }
}
