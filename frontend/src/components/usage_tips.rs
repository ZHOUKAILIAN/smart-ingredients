//! Usage tips component

use leptos::prelude::*;

#[component]
pub fn UsageTips() -> impl IntoView {
    let expanded = RwSignal::new(false);

    view! {
        <div class="rounded-2xl border border-emerald-100 bg-white-95 shadow-sm overflow-hidden">
            <div class="flex items-center justify-between px-4 py-3 cursor-pointer select-none" on:click=move |_| expanded.update(|v| *v = !*v)>
                <span class="text-sm font-semibold text-gray-900">"📋 拍摄技巧"</span>
                <span class=move || if expanded.get() { "text-xs text-gray-500 rotate-180 transition-transform" } else { "text-xs text-gray-500 transition-transform" }>"▼"</span>
            </div>
            <Show when=move || expanded.get()>
                <div class="px-4 pb-4">
                    <ul class="m-0 pl-4 text-sm text-gray-600 space-y-1.5">
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
