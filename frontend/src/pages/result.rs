use leptos::prelude::*;
use crate::components::{IngredientRow, IngredientTable};
use leptos::leptos_dom::helpers::set_timeout;
use leptos::task::spawn_local;
use crate::services;
use crate::stores::AppState;
use shared::AnalysisStatus;
use std::time::Duration;

fn risk_label(level: &str) -> String {
    match level {
        "low" => "低".to_string(),
        "medium" => "中".to_string(),
        "high" => "高".to_string(),
        _ => "未知".to_string(),
    }
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
            Some(AnalysisStatus::Pending | AnalysisStatus::Processing)
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
                <h1 class="title">"配料表"</h1>
                <p class="subtitle">"以下为模型分析结果"</p>
            </header>
            <Show
                when=move || error_text().is_some()
                fallback=move || view! { <p class="summary-text">{summary_text()}</p> }
            >
                <p class="summary-text error">
                    {move || error_text().unwrap_or_default()}
                </p>
            </Show>
            <Show
                when=move || !table_rows().is_empty()
                fallback=move || {
                    let status = state_for_status
                        .analysis_result
                        .get()
                        .map(|response| response.status);
                    let message = match status {
                        Some(AnalysisStatus::Pending) | Some(AnalysisStatus::Processing) => {
                            "正在分析中，请稍候…"
                        }
                        _ => "暂无表格数据",
                    };
                    view! { <p class="hint">{message}</p> }
                }
            >
                <IngredientTable items=table_rows() />
            </Show>
            <div class="action-area">
                <a class="primary-button" href="/">
                    "重新拍照"
                </a>
            </div>
        </section>
    }
}
