//! Image preview component

use leptos::prelude::*;

#[component]
pub fn ImagePreview<F>(
    preview_url: Signal<Option<String>>,
    on_remove: F,
) -> impl IntoView
where
    F: Fn() + Copy + Send + Sync + 'static,
{
    view! {
        <Show when=move || preview_url.get().is_some()>
            {move || view! {
                <div class="image-preview">
                    <img src={move || preview_url.get().unwrap_or_default()} alt="预览" />
                    <button class="image-preview-remove" on:click=move |_| on_remove()>
                        "✕"
                    </button>
                </div>
            }}
        </Show>
    }
}
