//! Example images component

use leptos::prelude::*;

#[component]
pub fn ExampleImages() -> impl IntoView {
    view! {
        <div class="example-grid">
            <div class="example-item good">
                <div class="example-placeholder">"✓ 清晰"</div>
            </div>
            <div class="example-item bad">
                <div class="example-placeholder">"✗ 模糊"</div>
            </div>
        </div>
    }
}
