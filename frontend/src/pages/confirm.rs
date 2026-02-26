use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use wasm_bindgen::JsCast;

use crate::components::{get_preference_description, IconArrowLeft};
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
        .unwrap_or_else(|| "normal".to_string());
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

    let on_back = move |_| {
        // Navigate back using browser history
        if let Some(window) = web_sys::window() {
            if let Some(history) = window.history().ok() {
                let _ = history.back();
            }
        }
    };

    view! {
        <section class="page figma">
            <div class="flex items-center px-4 py-3 bg-white-80 backdrop-blur-xl sticky top-0 z-10 shadow-sm">
                <button
                    class="mr-3 -ml-2 w-10 h-10 rounded-full border-0 bg-transparent flex items-center justify-center text-gray-700 hover:text-gray-900 hover:bg-gray-100 transition-colors"
                    on:click=on_back
                    aria-label="返回上一页"
                >
                    <IconArrowLeft />
                </button>
                <div class="w-10 h-10"></div>
            </div>

            <div class="page-scrollable-content px-5 py-5">
                <div class="mt-2">
                    <h3 class="m-0 mb-2 text-base font-semibold text-gray-900 text-center">"识别结果"</h3>
                    <textarea
                        class="w-full min-h-[220px] rounded-2xl border border-emerald-100 bg-white-95 p-4 text-sm leading-relaxed text-gray-800 shadow-sm focus:outline-none focus:border-emerald-500"
                        rows="10"
                        name="ocr-text"
                        placeholder="OCR识别的文本…"
                        prop:value=move || edited_text.get()
                        on:input=move |ev| {
                            set_edited_text.set(event_target_value(&ev));
                        }
                    />
                    <p class="mt-2 mb-0 text-xs text-gray-600">
                        "💡 提示：您可以修改识别错误的文字，以提高分析准确性"
                    </p>
                </div>

                <div class="mt-3 px-1">
                    <p class="m-0 text-xs text-gray-600">
                        {move || format!(
                            "💡 当前分析更注重：{}。如需修改请前往「我的」页面",
                            get_preference_description(&preference.get()),
                        )}
                    </p>
                </div>

                <div class="mt-4 flex gap-3">
                    <button
                        class="flex-1 min-h-11 rounded-xl border border-emerald-100 bg-white-95 text-gray-800 text-sm font-semibold shadow-sm transition-all disabled:opacity-50"
                        on:click=on_retake
                        disabled=move || state.loading_state.get() != LoadingState::Idle
                    >
                        "重新拍照"
                    </button>
                    <button
                        class="flex-1 min-h-11 rounded-xl border-0 bg-gradient-to-br from-emerald-500 to-teal-500 text-white text-sm font-semibold shadow-lg transition-all disabled:opacity-50"
                        on:click=on_confirm
                        disabled=move || state.loading_state.get() != LoadingState::Idle
                    >
                        {move || {
                            if state.loading_state.get() == LoadingState::LlmAnalyzing {
                                "提交中…"
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
