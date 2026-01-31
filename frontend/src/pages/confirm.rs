use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

use crate::components::get_preference_label;
use crate::services;
use crate::stores::{AppState, LoadingState, ToastLevel};
use crate::utils::emit_toast;
use crate::utils::preference::{load_preference, save_preference};

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
    let preference = create_signal(initial_preference).0;

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
            if state.auth_user.get().is_none() {
                save_preference(&current_preference);
            }

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
            <div class="page-scrollable-content">
                <div class="text-editor-container">
                    <h3 class="section-label">"识别结果"</h3>
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
                    <p class="preference-tips">
                        {move || format!("💡 当前分析视角：{}", get_preference_label(&preference.get()))}
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
