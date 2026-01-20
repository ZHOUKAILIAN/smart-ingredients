use leptos::leptos_dom::helpers::set_timeout;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use std::time::Duration;

use crate::services;
use crate::stores::AppState;
use shared::AnalysisStatus;

#[component]
pub fn AnalyzingPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();
    let fetching = create_rw_signal(false);
    let polling = create_rw_signal(false);
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
                        state.error_message.set(response.error_message.clone());
                        state.analysis_result.set(Some(response));
                    }
                    Err(err) => state.error_message.set(Some(err)),
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
            Some(AnalysisStatus::Completed) => {
                navigate("/summary", Default::default());
            }
            Some(AnalysisStatus::LlmPending) | Some(AnalysisStatus::LlmProcessing) => {
                if let Some(id) = analysis_id {
                    polling.set(true);
                    let state = state_for_poll.clone();
                    let polling = polling.clone();
                    set_timeout(
                        move || {
                            spawn_local(async move {
                                match services::fetch_analysis(id).await {
                                    Ok(response) => {
                                        state.error_message.set(response.error_message.clone());
                                        state.analysis_result.set(Some(response));
                                    }
                                    Err(err) => state.error_message.set(Some(err)),
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

    view! {
        <section class="page page-analyzing figma">
            <div class="waiting-body">
                <div class="surface-card status-card">
                    <div class="status-icon">"AI"</div>
                    <div class="progress-bar" aria-hidden="true">
                        <div class="progress-bar-fill"></div>
                    </div>
                    <h2 class="status-title">"AI 分析中..."</h2>
                    <p class="status-text">"请稍候，通常需要5-10秒"</p>
                </div>
            </div>

            <Show when=move || {
                state_for_error.with_value(|state| state.error_message.get().is_some())
            }>
                <p class="hint error section-padding">
                    {move || {
                        state_for_error
                            .with_value(|state| state.error_message.get().unwrap_or_default())
                    }}
                </p>
                <div class="error-actions">
                    <button
                        class="secondary-cta"
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
                        class="primary-cta"
                        on:click=move |_| {
                            let state = state_for_retry.get_value();
                            let analysis_id = state.analysis_id.get();
                            if let Some(id) = analysis_id {
                                spawn_local(async move {
                                    state.error_message.set(None);
                                    match services::retry_llm(id).await {
                                        Ok(response) => {
                                            state.analysis_result.set(Some(response));
                                        }
                                        Err(err) => state.error_message.set(Some(err)),
                                    }
                                });
                            }
                        }
                    >
                        "重试"
                    </button>
                </div>
            </Show>
        </section>
    }
}
