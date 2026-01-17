//! Usage tips component

use leptos::prelude::*;

#[component]
pub fn UsageTips() -> impl IntoView {
    let expanded = RwSignal::new(false);

    view! {
        <div class="collapsible" class:collapsible-open=expanded>
            <div class="collapsible-header" on:click=move |_| expanded.update(|v| *v = !*v)>
                <span>"📋 拍摄技巧"</span>
                <span class="collapsible-icon">"▼"</span>
            </div>
            <Show when=move || expanded.get()>
                <div class="collapsible-content">
                    <ul>
                        <li>"确保配料表清晰可见"</li>
                        <li>"保持良好的光线条件"</li>
                        <li>"避免反光和阴影"</li>
                        <li>"对准配料表区域拍摄"</li>
                    </ul>
                </div>
            </Show>
        </div>
    }
}
