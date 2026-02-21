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
                <div class="image-preview" class:loading=move || is_loading.get()>
                    <img
                        src={move || preview_url.get().unwrap_or_default()}
                        alt="预览"
                        on:load=move |_| is_loading.set(false)
                        on:error=move |_| is_loading.set(false)
                    />
                    <div class="image-preview-loading" aria-hidden="true">
                        <span class="image-preview-spinner"></span>
                    </div>
                    <button class="image-preview-remove" on:click=move |_| on_remove() aria-label="移除图片">
                        "✕"
                    </button>
                </div>
            }}
        </Show>
    }
}
