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
        <div class="mx-auto my-8 w-full max-w-[360px] rounded-2xl border border-red-100 bg-red-50 px-5 py-6 text-center shadow-sm">
            <div class="text-3xl leading-none">"⚠️"</div>
            <h2 class="m-0 mt-3 text-lg font-semibold text-red-700">{error.title}</h2>
            <p class="m-0 mt-2 text-sm text-red-600 leading-relaxed">{error.message}</p>

            <Show when=move || recoverable>
                <p class="m-0 mt-3 text-xs text-red-500">"请返回首页重试"</p>
            </Show>
        </div>
    }
}
