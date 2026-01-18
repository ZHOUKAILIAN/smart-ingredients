use leptos::leptos_dom::helpers::set_timeout;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use std::time::Duration;

use crate::services;
use crate::stores::AppState;
use shared::AnalysisStatus;

#[component]
pub fn OcrPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();
    let fetching = create_rw_signal(false);
    let polling = create_rw_signal(false);

    create_effect(move |_| {
        if fetching.get() {
            return;
        }
        let analysis_id = state.analysis_id.get();
        if analysis_id.is_none() {
            return;
        }
        if state.analysis_result.get().is_some() {
            return;
        }

        fetching.set(true);
        let state = state.clone();
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

        let status = state
            .analysis_result
            .get()
            .map(|response| response.status);
        let analysis_id = state.analysis_id.get();

        match status {
            Some(AnalysisStatus::OcrCompleted) => {
                if let Some(response) = state.analysis_result.get() {
                    state.ocr_text.set(response.ocr_text.clone());
                    navigate("/confirm", Default::default());
                }
            }
            Some(AnalysisStatus::OcrPending) | Some(AnalysisStatus::OcrProcessing) => {
                if let Some(id) = analysis_id {
                    polling.set(true);
                    let state = state.clone();
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

    let on_retry = move |_| {
        let analysis_id = state.analysis_id.get();
        if let Some(id) = analysis_id {
            let state = state.clone();
            spawn_local(async move {
                state.error_message.set(None);
                match services::retry_ocr(id).await {
                    Ok(response) => {
                        state.analysis_result.set(Some(response));
                    }
                    Err(err) => state.error_message.set(Some(err)),
                }
            });
        }
    };
    let on_retry = Callback::new(on_retry);

    view! {
        <section class="page page-ocr">
            <div class="loading-card">
                <div class="spinner" aria-hidden="true"></div>
                <p class="loading-text">"正在识别配料表..."</p>
                <p class="loading-hint">"请稍候，通常需要3-5秒"</p>
            </div>

            <Show when=move || state.error_message.get().is_some()>
                <p class="hint error">
                    {move || state.error_message.get().unwrap_or_default()}
                </p>
                <button class="btn-retry" on:click=move |_| on_retry.call(())>
                    "重试"
                </button>
            </Show>
        </section>
    }
}
