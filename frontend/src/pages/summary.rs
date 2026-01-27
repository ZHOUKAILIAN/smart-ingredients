//! Summary page - shows analysis overview

use leptos::prelude::*;
use leptos::leptos_dom::helpers::set_timeout;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use std::time::Duration;

use crate::components::{HealthScoreCard, SummaryCard};
use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::{emit_toast, local_history};
use shared::AnalysisStatus;

#[component]
pub fn SummaryPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();
    let state_for_effect = state.clone();
    let state_for_polling = state.clone();
    let fetching = RwSignal::new(false);
    let polling = RwSignal::new(false);
    let last_saved = create_rw_signal(None::<uuid::Uuid>);

    // Fetch analysis result if not present
    create_effect(move |_| {
        if fetching.get() {
            return;
        }
        if state.analysis_result.get().is_some() {
            return;
        }
        let analysis_id = state_for_effect.analysis_id.get();
        if let Some(id) = analysis_id {
            fetching.set(true);
            let state = state_for_effect.clone();
            spawn_local(async move {
                match services::fetch_analysis(id).await {
                    Ok(response) => {
                        if let Some(api_error) = response.error_message.clone() {
                            emit_toast(ToastLevel::Error, "分析失败", &api_error);
                        }
                        state.analysis_result.set(Some(response));
                    }
                    Err(err) => {
                        emit_toast(ToastLevel::Error, "分析失败", &err);
                    }
                }
                fetching.set(false);
            });
        }
    });

    // Poll for completion if still processing
    create_effect(move |_| {
        if polling.get() {
            return;
        }

        let status = state_for_polling
            .analysis_result
            .get()
            .map(|response| response.status);
        let analysis_id = state_for_polling.analysis_id.get();

        let should_poll = matches!(
            status,
            Some(AnalysisStatus::LlmPending | AnalysisStatus::LlmProcessing)
        );

        if should_poll {
            if let Some(id) = analysis_id {
                polling.set(true);
                let state = state_for_polling.clone();
                let polling_signal = polling;
                set_timeout(
                    move || {
                        spawn_local(async move {
                            match services::fetch_analysis(id).await {
                                Ok(response) => {
                                    if let Some(api_error) = response.error_message.clone() {
                                        emit_toast(ToastLevel::Error, "分析失败", &api_error);
                                    }
                                    state.analysis_result.set(Some(response));
                                }
                                Err(err) => {
                                    emit_toast(ToastLevel::Error, "分析失败", &err);
                                }
                            }
                            polling_signal.set(false);
                        });
                    },
                    Duration::from_secs(2),
                );
            }
        }
    });

    create_effect(move |_| {
        if state.auth_user.get().is_some() {
            return;
        }
        let Some(response) = state.analysis_result.get() else {
            return;
        };
        if response.status != AnalysisStatus::Completed {
            return;
        }
        let Some(result) = response.result.clone() else {
            return;
        };
        if last_saved.get() == Some(response.id) {
            return;
        }

        let summary = if result.summary.trim().is_empty() {
            format!("识别到 {} 项配料", result.ingredients.len())
        } else {
            result.summary.clone()
        };
        let item = local_history::LocalHistoryItem {
            id: response.id.to_string(),
            timestamp: js_sys::Date::now() as i64,
            health_score: result.health_score,
            summary,
            result,
            image_path: state.selected_image_path.get(),
        };

        if let Err(err) = local_history::add_local_history(item) {
            emit_toast(ToastLevel::Warning, "本地记录保存失败", &err);
        }
        last_saved.set(Some(response.id));
    });

    let navigate_detail = navigate.clone();
    let on_view_detail = move |_| {
        navigate_detail("/detail", Default::default());
    };

    let ingredient_count = move || {
        state
            .analysis_result
            .get()
            .and_then(|r| r.result)
            .map(|result| result.ingredients.len())
            .unwrap_or(0)
    };

    view! {
        <section class="page page-summary figma">
            <div class="figma-body">
                <header class="page-header">
                    <div class="figma-header">
                        <span class="icon-placeholder"></span>
                        <h1 class="figma-title">"分析结果概要"</h1>
                        <span class="icon-placeholder"></span>
                    </div>
                    <p class="subtitle">"以下为模型分析概要"</p>
                </header>

                // Health score card
                <Show when=move || {
                    state.analysis_result.get()
                        .and_then(|r| r.result)
                        .is_some()
                }>
                    {move || {
                        state.analysis_result.get()
                        .and_then(|r| r.result)
                        .map(|result| view! {
                            <HealthScoreCard
                                score={result.health_score}
                                recommendation={result.recommendation.clone()}
                            />
                        })
                }}
            </Show>

                // Summary card
                <Show when=move || {
                    state.analysis_result.get()
                        .and_then(|r| r.result)
                        .is_some()
                }>
                    {move || {
                        state.analysis_result.get()
                            .and_then(|r| r.result)
                            .map(|result| view! {
                                <SummaryCard
                                    summary={if result.summary.trim().is_empty() {
                                        format!("识别到 {} 项配料", result.ingredients.len())
                                    } else {
                                        result.summary.clone()
                                    }}
                                    warnings={result.warnings.clone()}
                                />
                            })
                    }}
                </Show>

                // Action buttons
                <div class="summary-actions">
                    <button class="btn-view-detail" on:click=on_view_detail>
                        <span class="btn-icon">"📋"</span>
                        <span class="btn-title">"查看详细配料表"</span>
                        <span class="btn-arrow">"→"</span>
                    </button>
                </div>
            </div>
        </section>
    }
}
