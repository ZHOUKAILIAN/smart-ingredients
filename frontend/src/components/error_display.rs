//! Error display component

use crate::stores::ErrorInfo;
use leptos::prelude::*;

#[component]
pub fn ErrorDisplay(
    /// Error information
    error: ErrorInfo,
) -> impl IntoView {
    let recoverable = error.recoverable;

    view! {
        <div class="error-container">
            <div class="error-icon">"⚠️"</div>
            <h2 class="error-title">{error.title}</h2>
            <p class="error-message">{error.message}</p>

            <Show when=move || recoverable>
                <p class="hint">"请返回首页重试"</p>
            </Show>
        </div>
    }
}
