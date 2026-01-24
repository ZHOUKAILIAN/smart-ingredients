use leptos::prelude::*;
use crate::components::{HealthScoreCard, IconArrowLeft, RiskBadge};
use leptos::leptos_dom::helpers::set_timeout;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::emit_toast;
use shared::AnalysisStatus;
use std::time::Duration;

#[derive(Clone)]
struct AnalysisItem {
    name: String,
    risk_level: String,
    description: String,
}

fn analysis_items(result: &shared::AnalysisResult) -> Vec<AnalysisItem> {
    if !result.ingredients.is_empty() {
        return result
            .ingredients
            .iter()
            .map(|item| AnalysisItem {
                name: item.name.clone(),
                risk_level: item.risk_level.clone(),
                description: item.description.clone().unwrap_or_default(),
            })
            .collect();
    }

    result
        .table
        .iter()
        .map(|row| AnalysisItem {
            name: row.name.clone(),
            risk_level: row.risk_level.clone(),
            description: if row.function.is_empty() {
                row.note.clone()
            } else {
                row.function.clone()
            },
        })
        .collect()
}

#[component]
pub fn ResultPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();
    let state_for_effect = state.clone();
    let state_for_status = state.clone();
    let fetching = create_rw_signal(false);
    let polling = create_rw_signal(false);
    let state_for_polling = state.clone();

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
            });
        }
    });

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
                let polling = polling.clone();
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
                            polling.set(false);
                        });
                    },
                    Duration::from_secs(2),
                );
            }
        }
    });

    let ingredient_items = move || {
        state
            .analysis_result
            .get()
            .and_then(|response| response.result)
            .map(|result| analysis_items(&result))
            .unwrap_or_default()
    };

    let advice_items = move || {
        state
            .analysis_result
            .get()
            .and_then(|response| response.result)
            .map(|result| {
                if !result.warnings.is_empty() {
                    result
                        .warnings
                        .iter()
                        .map(|warning| warning.message.clone())
                        .collect::<Vec<_>>()
                } else if result.recommendation.trim().is_empty() {
                    vec!["建议搭配新鲜水果蔬菜，保持均衡饮食".to_string()]
                } else {
                    vec![result.recommendation.clone()]
                }
            })
            .unwrap_or_default()
    };

    let on_back_home_top = {
        let state = state.clone();
        let navigate = navigate.clone();
        move |_| {
            state.analysis_id.set(None);
            state.analysis_result.set(None);
            navigate("/", Default::default());
        }
    };
    let on_back_home_bottom = {
        let state = state.clone();
        let navigate = navigate.clone();
        move |_| {
            state.analysis_id.set(None);
            state.analysis_result.set(None);
            navigate("/", Default::default());
        }
    };
    let on_new_analysis = {
        let state = state.clone();
        let navigate = navigate.clone();
        move |_| {
            state.analysis_id.set(None);
            state.analysis_result.set(None);
            navigate("/?view=scan", Default::default());
        }
    };

    view! {
        <section class="page page-result figma">
            <div class="scan-header">
                <button class="icon-button" on:click=on_back_home_top>
                    <IconArrowLeft />
                </button>
                <h1 class="figma-title">"分析报告"</h1>
                <button class="icon-button" type="button" aria-label="分享" disabled>
                    "↗"
                </button>
            </div>

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

            <div class="section-padding">
                <div class="surface-card result-section">
                    <h2 class="card-title">"配料分析"</h2>
                    <Show
                        when=move || !ingredient_items().is_empty()
                        fallback=move || {
                            let status = state_for_status
                                .analysis_result
                                .get()
                                .map(|response| response.status);
                            let message = match status {
                                Some(AnalysisStatus::OcrPending)
                                | Some(AnalysisStatus::OcrProcessing) => "正在识别配料表，请稍候…",
                                Some(AnalysisStatus::OcrCompleted) => "等待确认文本后进行分析。",
                                Some(AnalysisStatus::LlmPending)
                                | Some(AnalysisStatus::LlmProcessing) => "正在分析中，请稍候…",
                                _ => "暂无配料数据",
                            };
                            view! { <p class="hint">{message}</p> }
                        }
                    >
                        <div class="analysis-list">
                            {move || {
                                ingredient_items()
                                    .into_iter()
                                    .map(|item| {
                                        view! {
                                            <div class="analysis-item">
                                                <div class="analysis-header">
                                                    <span class="analysis-name">{item.name}</span>
                                                    <RiskBadge level={item.risk_level} />
                                                </div>
                                                <p class="analysis-desc">
                                                    {if item.description.is_empty() {
                                                        "暂无描述".to_string()
                                                    } else {
                                                        item.description
                                                    }}
                                                </p>
                                            </div>
                                        }
                                    })
                                    .collect_view()
                            }}
                        </div>
                    </Show>
                </div>

                <div class="surface-card result-section">
                    <h2 class="card-title">"营养成分 (每100ml)"</h2>
                    <p class="hint">"暂无营养成分数据"</p>
                </div>

                <div class="surface-card result-section">
                    <h2 class="card-title">"健康建议"</h2>
                    <ul class="advice-list">
                        {move || {
                            advice_items()
                                .into_iter()
                                .map(|item| view! { <li>{item}</li> })
                                .collect_view()
                        }}
                    </ul>
                </div>
            </div>

            <div class="result-actions">
                <button class="secondary-cta" on:click=on_back_home_bottom>
                    "返回首页"
                </button>
                <button class="primary-cta" on:click=on_new_analysis>
                    "分析新产品"
                </button>
            </div>
        </section>
    }
}
