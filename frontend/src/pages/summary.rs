//! Summary page - shows analysis overview

use leptos::leptos_dom::helpers::set_timeout;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use std::time::Duration;
use wasm_bindgen::JsCast;

use crate::components::{
    get_preference_label, CommunityShareButton, HealthScoreCard, IconArrowLeft, ShareButton,
    ShareExportProps, SummaryCard,
};
use crate::services;
use crate::stores::{AnalysisSource, AppState, ToastLevel};
use crate::utils::export_image::ExportIngredient;
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
    let last_saved = RwSignal::new(None::<uuid::Uuid>);

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
        if state.analysis_source.get() == AnalysisSource::History {
            return;
        }
        let Some(result) = response.result.clone() else {
            return;
        };
        if last_saved.get() == Some(response.id) {
            return;
        }
        let response_id = response.id;
        let response_id_str = response_id.to_string();
        let exists = local_history::load_local_history().into_iter().any(|item| {
            if item.id == response_id_str {
                return true;
            }
            matches!(
                uuid::Uuid::parse_str(&item.id),
                Ok(existing) if existing == response_id
            )
        });
        if exists {
            last_saved.set(Some(response_id));
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
        last_saved.set(Some(response_id));
    });

    let navigate_detail = navigate.clone();
    let on_view_detail = move |_| {
        navigate_detail("/detail", Default::default());
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
                <div class="flex flex-col gap-3 pb-5">
                    <button
                        class="w-full h-12 px-5 rounded-2xl border-0 bg-gradient-to-br from-emerald-500 to-teal-500 text-white font-semibold shadow-lg transition-all flex items-center justify-center gap-2"
                        on:click=on_view_detail
                    >
                        <span class="text-xl leading-none">"📋"</span>
                        <span class="text-base font-semibold leading-none">"查看详细配料表"</span>
                    </button>

                    {move || {
                        state.analysis_result.get()
                            .and_then(|r| r.result)
                            .map(|result| {
                                let pref = state.analysis_preference.get()
                                    .unwrap_or_else(|| "normal".to_string());
                                let props = ShareExportProps {
                                    health_score: result.health_score,
                                    recommendation: result.recommendation.clone(),
                                    ingredients: result.ingredients.iter().map(|i| {
                                        ExportIngredient {
                                            name: i.name.clone(),
                                            risk_level: i.risk_level.clone(),
                                            description: i.description.clone().unwrap_or_default(),
                                            is_focus: false,
                                        }
                                    }).collect(),
                                    warnings: result.warnings.iter().map(|w| w.message.clone()).collect(),
                                    summary: result.summary.clone(),
                                    preference_label: get_preference_label(&pref).to_string(),
                                };
                                view! { <ShareButton props=props /> }
                            })
                    }}
                    {move || {
                        let analysis_id = state.analysis_id.get();
                        let response = state.analysis_result.get();
                        let confirmed_text = response
                            .as_ref()
                            .and_then(|r| r.confirmed_text.clone())
                            .or_else(|| state.confirmed_text.get());
                        let ocr_text = response
                            .as_ref()
                            .and_then(|r| r.ocr_text.clone())
                            .or_else(|| state.ocr_text.get());
                        response.and_then(|r| r.result).map(|result| {
                            let preference = state.analysis_preference.get()
                                .unwrap_or_else(|| "normal".to_string());
                            let preference_label = Some(get_preference_label(&preference).to_string());
                            view! {
                                <CommunityShareButton
                                    analysis_id=analysis_id
                                    analysis_result=result
                                    confirmed_text=confirmed_text
                                    ocr_text=ocr_text
                                    preference_label=preference_label
                                />
                            }
                        })
                    }}
                </div>
            </div>
        </section>
    }
}
