//! Image preview component

use leptos::prelude::*;

#[component]
pub fn ImagePreview<F>(preview_url: Signal<Option<String>>, on_remove: F) -> impl IntoView
where
    F: Fn() + Copy + Send + Sync + 'static,
{
    let is_loading = RwSignal::new(false);
    create_effect(move |_| {
        if preview_url.get().is_some() {
            is_loading.set(true);
        } else {
            is_loading.set(false);
        }
    });

    view! {
        <Show when=move || preview_url.get().is_some()>
            {move || view! {
                <div class="relative w-full h-full min-h-[240px] rounded-2xl overflow-hidden border border-emerald-100 bg-white">
                    <img
                        class=move || {
                            if is_loading.get() {
                                "w-full h-full object-cover opacity-30 transition-opacity"
                            } else {
                                "w-full h-full object-cover opacity-100 transition-opacity"
                            }
                        }
                        src={move || preview_url.get().unwrap_or_default()}
                        alt="预览"
                        on:load=move |_| is_loading.set(false)
                        on:error=move |_| is_loading.set(false)
                    />
                    <Show when=move || is_loading.get()>
                        <div class="absolute inset-0 flex items-center justify-center bg-white/70" aria-hidden="true">
                            <span class="h-7 w-7 rounded-full border-2 border-emerald-200 border-t-emerald-500 animate-spin"></span>
                        </div>
                    </Show>
                    <button class="absolute top-2 right-2 w-8 h-8 rounded-full border-0 bg-black/50 text-white text-sm flex items-center justify-center hover:bg-black/65 transition-colors" on:click=move |_| on_remove() aria-label="移除图片">
                        "✕"
                    </button>
                </div>
            }}
        </Show>
    }
}
