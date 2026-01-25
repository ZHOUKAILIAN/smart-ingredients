use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

use crate::components::PreferenceSelector;
use crate::services;
use crate::stores::{AppState, LoadingState, ToastLevel};
use crate::utils::emit_toast;
use crate::utils::preference::load_preference;

#[component]
pub fn ConfirmPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();
    let state_for_confirm = state.clone();
    let navigate_for_retake = navigate.clone();
    let navigate_for_confirm = navigate.clone();

    let initial_text = state
        .confirmed_text
        .get()
        .or_else(|| state.ocr_text.get())
        .unwrap_or_default();
    let (edited_text, set_edited_text) = create_signal(initial_text);

    let initial_preference = state
        .analysis_preference
        .get()
        .or_else(|| load_preference())
        .unwrap_or_else(|| "none".to_string());
    let (preference, set_preference) = create_signal(initial_preference);

    let on_confirm = move |_| {
        let text = edited_text.get();
        let analysis_id = state_for_confirm.analysis_id.get();
        let current_preference = preference.get();

        if let Some(id) = analysis_id {
            let state = state_for_confirm.clone();
            let navigate = navigate_for_confirm.clone();

            // Set loading state
            state.loading_state.set(LoadingState::LlmAnalyzing);
            state
                .analysis_preference
                .set(Some(current_preference.clone()));

            spawn_local(async move {
                state.error_message.set(None);
                match services::confirm_and_analyze(id, text, Some(current_preference)).await {
                    Ok(response) => {
                        state.analysis_result.set(Some(response));
                        state.confirmed_text.set(Some(edited_text.get()));
                        state.loading_state.set(LoadingState::Idle);
                        navigate("/analyzing", Default::default());
                    }
                    Err(err) => {
                        emit_toast(ToastLevel::Error, "提交失败", &err);
                        state.error_message.set(Some(err));
                        state.loading_state.set(LoadingState::Idle);
                    }
                }
            });
        }
    };

    let on_retake = {
        let state = state.clone();
        move |_| {
            state.analysis_preference.set(None);
            navigate_for_retake("/", Default::default());
        }
    };

    view! {
        <section class="page page-confirm figma">
            <div class="figma-body">
                <header class="page-header">
                    <div class="figma-header">
                        <span class="icon-placeholder"></span>
                        <h1 class="figma-title">"识别结果确认"</h1>
                        <span class="icon-placeholder"></span>
                    </div>
                    <p class="subtitle">"请确认识别文本是否正确，可以编辑修改"</p>
                </header>

                <div class="text-editor-container">
                    <textarea
                        class="text-editor"
                        rows="10"
                        placeholder="OCR识别的文本..."
                        prop:value=move || edited_text.get()
                        on:input=move |ev| {
                            set_edited_text.set(event_target_value(&ev));
                        }
                    />
                    <p class="edit-tips">
                        "💡 提示：您可以修改识别错误的文字，以提高分析准确性"
                    </p>
                </div>

                <div class="preference-container">
                    <PreferenceSelector
                        value=Signal::derive(move || preference.get())
                        on_change=Callback::new(move |value: String| {
                            set_preference.set(value);
                        })
                        label="分析视角"
                        show_description=true
                    />
                    <p class="preference-tips">
                        "💡 默认来自首页首次选择的偏好，可在此临时修改本次分析视角。"
                    </p>
                </div>

                <div class="action-buttons">
                    <button
                        class="btn-secondary"
                        on:click=on_retake
                        disabled=move || state.loading_state.get() != LoadingState::Idle
                    >
                        "重新拍照"
                    </button>
                    <button
                        class="btn-primary"
                        on:click=on_confirm
                        disabled=move || state.loading_state.get() != LoadingState::Idle
                    >
                        {move || {
                            if state.loading_state.get() == LoadingState::LlmAnalyzing {
                                "提交中..."
                            } else {
                                "确认并分析"
                            }
                        }}
                    </button>
                </div>
            </div>
        </section>
    }
}
