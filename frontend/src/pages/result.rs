use leptos::prelude::*;
use crate::components::{IngredientRow, IngredientCardList, HealthScoreCard, SummaryCard};
use leptos::leptos_dom::helpers::set_timeout;
use leptos::task::spawn_local;
use crate::services;
use crate::stores::AppState;
use shared::AnalysisStatus;
use std::time::Duration;

fn risk_label(level: &str) -> String {
    level.to_string()
}

fn to_rows(table: &[shared::TableRow]) -> Vec<IngredientRow> {
    table
        .iter()
        .map(|row| IngredientRow {
            name: row.name.clone(),
            category: row.category.clone(),
            function: row.function.clone(),
            risk_level: risk_label(&row.risk_level),
            note: row.note.clone(),
        })
        .collect()
}

fn ingredient_rows(items: &[shared::IngredientInfo]) -> Vec<IngredientRow> {
    items
        .iter()
        .map(|item| IngredientRow {
            name: item.name.clone(),
            category: item.category.clone(),
            function: item.description.clone().unwrap_or_default(),
            risk_level: risk_label(&item.risk_level),
            note: String::new(),
        })
        .collect()
}

#[component]
pub fn ResultPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let state_for_effect = state.clone();
    let state_for_summary = state.clone();
    let state_for_table = state.clone();
    let state_for_error = state.clone();
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
                        let api_error = response.error_message.clone();
                        state.analysis_result.set(Some(response));
                        state.error_message.set(api_error);
                    }
                    Err(err) => state.error_message.set(Some(err)),
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
                                    let api_error = response.error_message.clone();
                                    state.analysis_result.set(Some(response));
                                    state.error_message.set(api_error);
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
    });

    let error_text = move || {
        state_for_error.error_message.get().or_else(|| {
            state_for_error
                .analysis_result
                .get()
                .and_then(|response| response.error_message)
        })
    };

    let summary_text = move || {
        state_for_summary
            .analysis_result
            .get()
            .and_then(|response| response.result)
            .map(|result| {
                if result.summary.trim().is_empty() {
                    if result.ingredients.is_empty() {
                        "暂无摘要".to_string()
                    } else {
                        format!("识别到 {} 项配料，可查看表格详情。", result.ingredients.len())
                    }
                } else {
                    result.summary
                }
            })
            .unwrap_or_else(|| "暂无摘要".to_string())
    };

    let table_rows = move || {
        state_for_table
            .analysis_result
            .get()
            .and_then(|response| response.result)
            .map(|result| {
                if result.table.is_empty() {
                    ingredient_rows(&result.ingredients)
                } else {
                    to_rows(&result.table)
                }
            })
            .unwrap_or_default()
    };

    view! {
        <section class="page page-result">
            <header class="page-header">
                <h1 class="title">"分析结果"</h1>
                <p class="subtitle">"以下为模型分析结果"</p>
            </header>

            // Error message
            <Show when=move || error_text().is_some()>
                <p class="summary-text error">
                    {move || error_text().unwrap_or_default()}
                </p>
            </Show>

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
                                    if result.ingredients.is_empty() {
                                        "暂无摘要".to_string()
                                    } else {
                                        format!("识别到 {} 项配料，可查看详情。", result.ingredients.len())
                                    }
                                } else {
                                    result.summary.clone()
                                }}
                                warnings={result.warnings.clone()}
                            />
                        })
                }}
            </Show>

            // Section title
            <h2 class="section-title">"配料详情"</h2>

            // Ingredient card list
            <Show
                when=move || !table_rows().is_empty()
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
                <IngredientCardList items=table_rows() />
            </Show>

            <div class="action-area">
                <a class="primary-button" href="/">
                    "重新拍照"
                </a>
            </div>
        </section>
    }
}
