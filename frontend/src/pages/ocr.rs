use leptos::leptos_dom::helpers::set_timeout;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use std::time::Duration;
use wasm_bindgen::JsCast;

use crate::components::IconArrowLeft;
use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::emit_toast;
use shared::AnalysisStatus;

#[component]
pub fn OcrPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();
    let fetching = RwSignal::new(false);
    let polling = RwSignal::new(false);
    let state_for_fetch = state.clone();
    let state_for_poll = state.clone();
    let state_for_retry = StoredValue::new(state.clone());
    let state_for_error = StoredValue::new(state.clone());
    let navigate_for_home = StoredValue::new(navigate.clone());

    create_effect(move |_| {
        if fetching.get() {
            return;
        }
        let analysis_id = state_for_fetch.analysis_id.get();
        if analysis_id.is_none() {
            return;
        }
        if state_for_fetch.analysis_result.get().is_some() {
            return;
        }

        fetching.set(true);
        let state = state_for_fetch.clone();
        spawn_local(async move {
            if let Some(id) = analysis_id {
                match services::fetch_analysis(id).await {
                    Ok(response) => {
                        if let Some(api_error) = response.error_message.clone() {
                            emit_toast(ToastLevel::Error, "识别失败", &api_error);
                        }
                        state.error_message.set(response.error_message.clone());
                        state.analysis_result.set(Some(response));
                    }
                    Err(err) => {
                        emit_toast(ToastLevel::Error, "识别失败", &err);
                        state.error_message.set(Some(err));
                    }
                }
            }
            fetching.set(false);
        });
    });

    create_effect(move |_| {
        if polling.get() {
            return;
        }

        let status = state_for_poll
            .analysis_result
            .get()
            .map(|response| response.status);
        let analysis_id = state_for_poll.analysis_id.get();

        match status {
            Some(AnalysisStatus::OcrCompleted) => {
                if let Some(response) = state_for_poll.analysis_result.get() {
                    state_for_poll.ocr_text.set(response.ocr_text.clone());
                    navigate("/confirm", Default::default());
                }
            }
            Some(AnalysisStatus::OcrPending) | Some(AnalysisStatus::OcrProcessing) => {
                if let Some(id) = analysis_id {
                    polling.set(true);
                    let state = state_for_poll.clone();
                    let polling = polling.clone();
                    set_timeout(
                        move || {
                            spawn_local(async move {
                                match services::fetch_analysis(id).await {
                                    Ok(response) => {
                                        if let Some(api_error) = response.error_message.clone() {
                                            emit_toast(ToastLevel::Error, "识别失败", &api_error);
                                        }
                                        state.error_message.set(response.error_message.clone());
                                        state.analysis_result.set(Some(response));
                                    }
                                    Err(err) => {
                                        emit_toast(ToastLevel::Error, "识别失败", &err);
                                        state.error_message.set(Some(err));
                                    }
                                }
                                polling.set(false);
                            });
                        },
                        Duration::from_secs(2),
                    );
                }
            }
            _ => {}
        }
    });

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
                <div class="w-full max-w-[360px] mx-auto my-4 flex flex-col gap-3 items-center text-center bg-white-80 border border-emerald-100 shadow-lg backdrop-blur-sm rounded-2xl p-5">
                    <div class="w-16 h-16 rounded-2xl bg-gradient-to-br from-emerald-500 to-teal-500 text-white flex items-center justify-center font-bold text-sm tracking-wider shadow-lg">
                        "OCR"
                    </div>
                    <div class="w-full h-1 rounded-full bg-emerald-100 overflow-hidden" aria-hidden="true">
                        <div class="h-full w-1/2 rounded-full bg-gradient-to-r from-transparent via-emerald-500 to-transparent animate-pulse"></div>
                    </div>
                    <h2 class="m-0 text-lg font-bold text-gray-900">"正在识别配料表…"</h2>
                    <p class="m-0 text-sm text-gray-600">"请稍候，通常需要3-5秒"</p>
                </div>

                <Show when=move || state_for_error.with_value(|state| state.error_message.get().is_some())>
                    <div class="mt-4 flex gap-3 px-5">
                    <button
                        class="flex-1 min-h-11 rounded-xl border border-emerald-100 bg-white-95 text-gray-800 text-sm font-semibold shadow-sm transition-all"
                        on:click=move |_| {
                            // Clear state and go back to home
                            state.analysis_id.set(None);
                            state.analysis_result.set(None);
                            state.ocr_text.set(None);
                            state.confirmed_text.set(None);
                            state.error_message.set(None);
                            navigate_for_home.with_value(|nav| nav("/", Default::default()));
                        }
                    >
                        "返回首页"
                    </button>
                    <button
                        class="flex-1 min-h-11 rounded-xl border-0 bg-gradient-to-br from-emerald-500 to-teal-500 text-white text-sm font-semibold shadow-lg transition-all"
                        on:click=move |_| {
                            let state = state_for_retry.get_value();
                            let analysis_id = state.analysis_id.get();
                            if let Some(id) = analysis_id {
                                spawn_local(async move {
                                    state.error_message.set(None);
                                    match services::retry_ocr(id).await {
                                        Ok(response) => {
                                            state.analysis_result.set(Some(response));
                                        }
                                        Err(err) => {
                                            emit_toast(ToastLevel::Error, "识别失败", &err);
                                            state.error_message.set(Some(err));
                                        }
                                    }
                                });
                            }
                        }
                    >
                        "重试"
                    </button>
                </div>
            </Show>
            </div>
        </section>
    }
}
