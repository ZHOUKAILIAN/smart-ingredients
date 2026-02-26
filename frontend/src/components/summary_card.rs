//! Summary card component

use leptos::prelude::*;
use shared::Warning;

#[component]
pub fn SummaryCard(summary: String, warnings: Vec<Warning>) -> impl IntoView {
    let has_warnings = !warnings.is_empty();
    let warning_section = if has_warnings {
        let warning_items = warnings
            .into_iter()
            .map(|w| {
                view! {
                    <li>{w.message}</li>
                }
            })
            .collect_view();

        Some(view! {
            <div class="mt-3 rounded-xl border border-amber-100 bg-amber-50 p-3">
                <h4 class="m-0 mb-2 text-sm font-semibold text-amber-700">"⚠️ 注意事项"</h4>
                <ul class="m-0 pl-4 text-sm text-amber-800 space-y-1">
                    {warning_items}
                </ul>
            </div>
        })
    } else {
        None
    };

    view! {
        <details class="rounded-2xl border border-emerald-100 bg-white-95 shadow-lg p-4">
            <summary class="list-none cursor-pointer text-sm font-semibold text-gray-800">"摘要"</summary>
            <div class="mt-3">
                <p class="m-0 text-sm text-gray-700 leading-relaxed">{summary}</p>
                {warning_section}
            </div>
        </details>
    }
}
