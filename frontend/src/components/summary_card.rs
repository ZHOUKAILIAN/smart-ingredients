//! Summary card component

use leptos::prelude::*;
use shared::Warning;

#[component]
pub fn SummaryCard(
    summary: String,
    warnings: Vec<Warning>,
) -> impl IntoView {
    let has_warnings = !warnings.is_empty();
    let warning_section = if has_warnings {
        let warning_items = warnings.into_iter().map(|w| view! {
            <li>{w.message}</li>
        }).collect_view();

        Some(view! {
            <div class="warnings-section">
                <h4>"⚠️ 注意事项"</h4>
                <ul>
                    {warning_items}
                </ul>
            </div>
        })
    } else {
        None
    };

    view! {
        <details class="card summary-card" open>
            <summary class="summary-toggle">"摘要"</summary>
            <div class="summary-content">
                <p class="summary-text">{summary}</p>
                {warning_section}
            </div>
        </details>
    }
}
