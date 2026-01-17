//! Health score card component

use leptos::prelude::*;
use crate::utils::{get_health_score_color, get_health_score_label};

#[component]
pub fn HealthScoreCard(
    score: i32,
    recommendation: String,
) -> impl IntoView {
    let score_color = get_health_score_color(score);
    let score_label = get_health_score_label(score);

    // Create conic gradient style for the gauge
    let gauge_style = format!(
        "background: conic-gradient(from 0deg, {} 0%, {} {}%, var(--bg) {}%, var(--bg) 100%)",
        score_color, score_color, score, score
    );

    view! {
        <div class="card health-score-card">
            <div class="gauge-container">
                <div class="gauge-circle" style={gauge_style}></div>
                <div class="gauge-score">{score}</div>
            </div>
            <div class="score-label">{score_label}</div>
            <p class="recommendation">{recommendation}</p>
        </div>
    }
}
