use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use crate::components::{ExampleImages, ImagePreview};
use crate::services;
use crate::stores::AppState;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, Url};

#[component]
pub fn CapturePage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();
    let selected_file: RwSignal<Option<web_sys::File>, LocalStorage> = RwSignal::new_local(None);
    let preview_url = create_rw_signal(None::<String>);
    let local_error = create_rw_signal(None);
    let file_input_ref = NodeRef::<leptos::html::Input>::new();

    let on_file_change = move |ev: leptos::ev::Event| {
        let input = ev
            .target()
            .and_then(|target| target.dyn_into::<HtmlInputElement>().ok());
        let file = input
            .and_then(|input| input.files())
            .and_then(|files| files.get(0));

        // Create preview URL
        if let Some(ref f) = file {
            if let Ok(url) = Url::create_object_url_with_blob(f) {
                preview_url.set(Some(url));
            }
        }

        selected_file.set(file);
    };

    let on_select_image = move |_| {
        if let Some(input) = file_input_ref.get() {
            input.click();
        }
    };

    let on_remove_preview = move || {
        // Revoke object URL to prevent memory leak
        if let Some(url) = preview_url.get() {
            let _ = Url::revoke_object_url(&url);
        }
        preview_url.set(None);
        selected_file.set(None);

        // Clear file input
        if let Some(input) = file_input_ref.get() {
            input.set_value("");
        }
    };

    let on_upload = store_value(move |_| {
        let file = selected_file.get();
        if file.is_none() {
            local_error.set(Some("请先选择一张图片".to_string()));
            return;
        }

        local_error.set(None);
        state.error_message.set(None);
        let state = state.clone();
        let navigate = navigate.clone();

        spawn_local(async move {
            match services::upload_image(file.unwrap()).await {
                Ok(response) => {
                    state.analysis_id.set(Some(response.id));
                    state.analysis_result.set(None);
                    navigate("/analyzing", Default::default());
                }
                Err(err) => {
                    state.error_message.set(Some(err.clone()));
                    local_error.set(Some(err));
                }
            }
        });
    });

    view! {
        <section class="page page-home">
            // Brand section
            <div class="brand-section">
                <div class="brand-icon">"🥗"</div>
                <h1 class="brand-name">"Smart Ingredients"</h1>
                <p class="brand-tagline">"食品配料表分析助手"</p>
            </div>

            // Feature card
            <div class="card feature-card">
                <p class="feature-text">
                    "智能识别配料表，快速了解食品成分和健康风险"
                </p>
            </div>

            // Steps card
            <div class="card steps-card">
                <div class="step-item">
                    <span class="step-icon">"1️⃣"</span>
                    <span class="step-text">"拍摄配料表"</span>
                </div>
                <div class="step-item">
                    <span class="step-icon">"2️⃣"</span>
                    <span class="step-text">"AI智能分析"</span>
                </div>
                <div class="step-item">
                    <span class="step-icon">"3️⃣"</span>
                    <span class="step-text">"查看健康报告"</span>
                </div>
            </div>

            // Example images (collapsible)
            <ExampleImages />

            // Hidden file input
            <input
                node_ref=file_input_ref
                class="file-input-hidden"
                type="file"
                accept="image/*"
                on:change=on_file_change
            />

            // Main action button (only show when no preview)
            <Show when=move || preview_url.get().is_none()>
                <div class="action-area">
                    <button
                        class="primary-button-large"
                        on:click=on_select_image
                    >
                        <span class="button-icon">"📷"</span>
                        <span class="button-text">"开始分析"</span>
                    </button>
                </div>
            </Show>

            // Image preview (show after selection)
            <ImagePreview
                preview_url=preview_url.into()
                on_remove=on_remove_preview
            />

            // Upload button (show when preview exists)
            <Show when=move || preview_url.get().is_some()>
                <button
                    class="secondary-button"
                    on:click=move |ev| on_upload.with_value(|f| f(ev))
                >
                    "确认上传"
                </button>
            </Show>

            // Error message
            <Show when=move || local_error.get().is_some()>
                <p class="hint error">
                    {move || local_error.get().unwrap_or_default()}
                </p>
            </Show>
        </section>
    }
}
