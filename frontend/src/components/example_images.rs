//! Example images component

use leptos::prelude::*;

#[component]
pub fn ExampleImages() -> impl IntoView {
    let expanded = RwSignal::new(false);

    view! {
        <div class="collapsible" class:collapsible-open=expanded>
            <div class="collapsible-header" on:click=move |_| expanded.update(|v| *v = !*v)>
                <span>"📸 示例图片"</span>
                <span class="collapsible-icon">"▼"</span>
            </div>
            <Show when=move || expanded.get()>
                <div class="collapsible-content">
                    <div class="example-grid">
                        <div class="example-item good">
                            <div class="example-placeholder">"✓ 清晰"</div>
                        </div>
                        <div class="example-item bad">
                            <div class="example-placeholder">"✗ 模糊"</div>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}
