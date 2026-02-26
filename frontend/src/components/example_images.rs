//! Example images component

use leptos::prelude::*;

#[component]
pub fn ExampleImages() -> impl IntoView {
    view! {
        <div class="grid grid-cols-2 gap-3">
            <div class="rounded-xl border border-emerald-200 bg-emerald-50 p-4 text-center">
                <div class="text-sm font-semibold text-emerald-700">"✓ 清晰"</div>
            </div>
            <div class="rounded-xl border border-red-200 bg-red-50 p-4 text-center">
                <div class="text-sm font-semibold text-red-700">"✗ 模糊"</div>
            </div>
        </div>
    }
}
