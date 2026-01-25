use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use crate::components::{
    ExampleImages, IconArrowLeft, IconCamera, IconChart, IconCheckBadge, ImagePreview,
    PreferenceCard,
};
use crate::services;
use crate::stores::{AppState, LoadingState, ToastLevel};
use crate::utils::emit_toast;
use crate::utils::preference::{load_preference, save_preference};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, Url};
use web_sys::window;

#[component]
pub fn CapturePage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();
    let selected_file: RwSignal<Option<web_sys::File>, LocalStorage> = RwSignal::new_local(None);
    let preview_url = create_rw_signal(None::<String>);
    let camera_input_ref = NodeRef::<leptos::html::Input>::new();
    let album_input_ref = NodeRef::<leptos::html::Input>::new();
    let show_scan = create_rw_signal(false);
    let initialized = create_rw_signal(false);
    let stored_preference = load_preference();
    let initial_preference = stored_preference
        .clone()
        .unwrap_or_else(|| "none".to_string());
    let (preference, set_preference) = create_signal(initial_preference);
    let (show_preference_guide, set_show_preference_guide) =
        create_signal(stored_preference.is_none());

    create_effect(move |_| {
        if initialized.get() {
            return;
        }
        initialized.set(true);
        if let Some(win) = window() {
            if let Ok(search) = win.location().search() {
                if search.contains("view=scan") {
                    show_scan.set(true);
                }
            }
        }
    });

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

    let on_select_camera = move |_| {
        if let Some(input) = camera_input_ref.get() {
            input.click();
        }
    };

    let on_select_album = move |_| {
        if let Some(input) = album_input_ref.get() {
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
        if let Some(input) = camera_input_ref.get() {
            input.set_value("");
        }
        if let Some(input) = album_input_ref.get() {
            input.set_value("");
        }
    };

    let state_for_upload = state.clone();
    let on_upload = store_value(move |_| {
        let file = selected_file.get();
        if file.is_none() {
            emit_toast(ToastLevel::Warning, "需要图片", "请先选择一张图片");
            return;
        }

        state_for_upload.error_message.set(None);
        let state = state_for_upload.clone();
        let navigate = navigate.clone();

        // Set loading state
        state.loading_state.set(LoadingState::OcrProcessing);

        spawn_local(async move {
            match services::upload_image(file.unwrap()).await {
                Ok(response) => {
                    state.analysis_id.set(Some(response.id));
                    state.analysis_result.set(None);
                    state.error_message.set(None);
                    state.ocr_text.set(None);
                    state.confirmed_text.set(None);
                    state.loading_state.set(LoadingState::Idle);
                    navigate("/ocr", Default::default());
                }
                Err(err) => {
                    state.error_message.set(Some(err));
                    state.loading_state.set(LoadingState::Idle);
                }
            }
        });
    });

    view! {
        <section class="page page-capture figma">
            <Show when=move || show_preference_guide.get()>
                <div class="preference-guide-overlay">
                    <div class="surface-card preference-guide-card">
                        <h2 class="preference-guide-title">"选择分析偏好"</h2>
                        <p class="preference-guide-subtitle">
                            "告诉我们您更关注哪些点，我们会提供更符合需求的分析结果。"
                        </p>
                        <PreferenceCard
                            value=Signal::derive(move || preference.get())
                            on_change=Callback::new(move |value: String| {
                                save_preference(&value);
                                set_preference.set(value.clone());
                                state.analysis_preference.set(Some(value));
                            })
                        />
                        <div class="preference-guide-actions">
                            <button
                                class="secondary-cta"
                                on:click=move |_| {
                                    save_preference("none");
                                    set_preference.set("none".to_string());
                                    state.analysis_preference.set(Some("none".to_string()));
                                    set_show_preference_guide.set(false);
                                }
                            >
                                "暂不选择"
                            </button>
                            <button
                                class="primary-cta"
                                on:click=move |_| {
                                    let value = preference.get();
                                    save_preference(&value);
                                    state.analysis_preference.set(Some(value));
                                    set_show_preference_guide.set(false);
                                }
                            >
                                "保存偏好"
                            </button>
                        </div>
                    </div>
                </div>
            </Show>
            <Show when=move || !show_scan.get()>
                <div class="home-hero">
                    <div class="brand-mark">
                        <div class="brand-icon">"SI"</div>
                        <div class="brand-ai">"AI"</div>
                    </div>
                    <h1 class="hero-title">"Smart Ingredients"</h1>
                    <p class="hero-subtitle">"AI智能配料表分析"</p>
                    <p class="hero-description">"拍摄识别配料表，AI分析健康风险，让您吃得更安心"</p>
                </div>

                <div class="surface-card steps-card">
                    <h2 class="card-title centered">"使用步骤"</h2>
                    <div class="steps-list">
                        <div class="step-item">
                            <div class="step-icon">
                                <span class="step-number">"1"</span>
                                <IconCamera />
                            </div>
                            <div class="step-content">
                                <h3>"拍摄配料表"</h3>
                                <p>"对准食品包装上的配料表拍照"</p>
                            </div>
                        </div>
                        <div class="step-item">
                            <div class="step-icon">
                                <span class="step-number">"2"</span>
                                <IconCheckBadge />
                            </div>
                            <div class="step-content">
                                <h3>"确认识别文本"</h3>
                                <p>"AI自动识别配料信息"</p>
                            </div>
                        </div>
                        <div class="step-item">
                            <div class="step-icon">
                                <span class="step-number">"3"</span>
                                <IconChart />
                            </div>
                            <div class="step-content">
                                <h3>"查看健康报告"</h3>
                                <p>"获取详细的成分分析和建议"</p>
                            </div>
                        </div>
                    </div>
                </div>

                <details class="example-section">
                    <summary class="link-button">"查看示例"</summary>
                    <ExampleImages />
                </details>

                <div class="home-actions">
                    <button class="primary-cta" on:click=move |_| show_scan.set(true)>
                        "开始分析"
                    </button>
                </div>
            </Show>

            <Show when=move || show_scan.get()>
                <div class="scan-header">
                    <button
                        class="icon-button"
                        on:click=move |_| {
                            show_scan.set(false);
                            on_remove_preview();
                        }
                    >
                        <IconArrowLeft />
                    </button>
                    <h1>"拍摄配料表"</h1>
                    <div class="icon-placeholder"></div>
                </div>

                <div class="scan-content">
                    <input
                        node_ref=camera_input_ref
                        class="file-input-hidden"
                        type="file"
                        accept="image/*"
                        capture="environment"
                        on:change=on_file_change
                    />
                    <input
                        node_ref=album_input_ref
                        class="file-input-hidden"
                        type="file"
                        accept="image/*"
                        on:change=on_file_change
                    />

                    <Show when=move || preview_url.get().is_some()>
                        <div class="surface-card preview-card">
                            <ImagePreview
                                preview_url=preview_url.into()
                                on_remove=on_remove_preview
                            />
                            <div class="status-banner">
                                "图片已上传，点击分析开始识别"
                            </div>
                            <button
                                class="primary-cta"
                                on:click=move |ev| on_upload.with_value(|f| f(ev))
                                disabled=move || state.loading_state.get() != LoadingState::Idle
                            >
                                {move || {
                                    if state.loading_state.get() == LoadingState::OcrProcessing {
                                        "AI 分析中..."
                                    } else {
                                        "开始分析"
                                    }
                                }}
                            </button>
                        </div>
                    </Show>

                    <Show when=move || preview_url.get().is_none()>
                        <div class="surface-card upload-card">
                            <div class="upload-hero">
                                <div class="upload-icon">"CAM"</div>
                                <div>
                                    <h2>"上传配料表照片"</h2>
                                    <p>"请确保配料表文字清晰可见"</p>
                                </div>
                            </div>
                            <div class="upload-actions">
                                <button class="primary-cta" on:click=on_select_camera>
                                    "拍照"
                                </button>
                                <button class="secondary-cta" on:click=on_select_album>
                                    "从相册选择"
                                </button>
                            </div>
                        </div>
                    </Show>

                    <div class="surface-card tips-card">
                        <h3>"拍摄小贴士："</h3>
                        <ul>
                            <li>"确保配料表文字清晰，光线充足"</li>
                            <li>"尽量平行拍摄，避免文字倾斜"</li>
                            <li>"避免反光和阴影遮挡文字"</li>
                        </ul>
                    </div>

                </div>
            </Show>
        </section>
    }
}
