use crate::components::{
    get_preference_icon, get_preference_label, HealthScoreCard, IconArrowLeft, RiskBadge,
};
use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::emit_toast;
use crate::utils::preference::load_preference;
use leptos::leptos_dom::helpers::set_timeout;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use shared::{AnalysisStatus, RuleHit};
use std::collections::HashSet;
use std::time::Duration;
use wasm_bindgen::JsCast;

#[derive(Clone)]
struct AnalysisItem {
    name: String,
    risk_level: String,
    description: String,
    is_focus: bool,
    index: usize,
}

fn analysis_items(result: &shared::AnalysisResult) -> Vec<AnalysisItem> {
    let focus_set = result
        .focus_ingredients
        .as_ref()
        .map(|items| {
            items
                .iter()
                .map(|item| item.trim().to_lowercase())
                .collect::<std::collections::HashSet<_>>()
        })
        .unwrap_or_default();

    if !result.ingredients.is_empty() {
        let mut items: Vec<AnalysisItem> = result
            .ingredients
            .iter()
            .map(|item| AnalysisItem {
                name: item.name.clone(),
                risk_level: item.risk_level.clone(),
                description: item.description.clone().unwrap_or_default(),
                is_focus: focus_set.contains(&item.name.trim().to_lowercase()),
                index: 0,
            })
            .collect();
        assign_indices(&mut items);
        sort_items(&mut items);
        return items;
    }

    let mut items: Vec<AnalysisItem> = result
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
            is_focus: focus_set.contains(&row.name.trim().to_lowercase()),
            index: 0,
        })
        .collect();
    assign_indices(&mut items);
    sort_items(&mut items);
    items
}

fn assign_indices(items: &mut [AnalysisItem]) {
    for (index, item) in items.iter_mut().enumerate() {
        item.index = index;
    }
}

fn sort_items(items: &mut Vec<AnalysisItem>) {
    items.sort_by_key(|item| {
        (
            risk_rank(&item.risk_level),
            if item.is_focus { 0 } else { 1 },
            item.index,
        )
    });
}

fn risk_rank(level: &str) -> i32 {
    match level.trim().to_lowercase().as_str() {
        "high" => 0,
        "medium" => 1,
        "low" => 2,
        _ => 3,
    }
}

fn confidence_label(level: &str) -> &'static str {
    match level.trim().to_lowercase().as_str() {
        "high" => "高",
        "medium" => "中",
        "low" => "低",
        _ => "未知",
    }
}

fn tag_label(tag: &str) -> &'static str {
    match tag.trim().to_lowercase().as_str() {
        "allergy" => "过敏人群",
        "kids" => "儿童/婴幼儿",
        "pregnancy" => "孕妇",
        _ => "关注人群",
    }
}

fn format_group_tags(tags: &[String]) -> String {
    let mut seen = HashSet::new();
    let mut labels = Vec::new();
    for tag in tags {
        let label = tag_label(tag);
        if seen.insert(label) {
            labels.push(label);
        }
    }
    labels.join(" / ")
}

fn build_strong_alerts(rule_hits: &[RuleHit]) -> Vec<String> {
    let mut has_allergy = false;
    let mut has_kids = false;
    let mut has_pregnancy = false;

    for hit in rule_hits {
        for tag in &hit.group_tags {
            match tag.trim().to_lowercase().as_str() {
                "allergy" => has_allergy = true,
                "kids" => has_kids = true,
                "pregnancy" => has_pregnancy = true,
                _ => {}
            }
        }
    }

    let mut alerts = Vec::new();
    if has_allergy {
        alerts.push("包含过敏原成分，过敏人群必须严格避免。".to_string());
    }
    if has_kids {
        alerts.push("含儿童敏感成分，儿童/婴幼儿需谨慎或避免食用。".to_string());
    }
    if has_pregnancy {
        alerts.push("含孕期敏感成分，孕妇建议谨慎或咨询医生。".to_string());
    }

    alerts
}

#[component]
pub fn ResultPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();
    let state_for_effect = state.clone();
    let state_for_status = state.clone();
    let fetching = RwSignal::new(false);
    let polling = RwSignal::new(false);
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

    let summary_text = move || {
        state
            .analysis_result
            .get()
            .and_then(|response| response.result)
            .map(|result| {
                result
                    .focus_summary
                    .clone()
                    .filter(|value| !value.trim().is_empty())
                    .unwrap_or_else(|| result.summary.clone())
            })
            .unwrap_or_else(|| "暂无分析总结".to_string())
    };

    let rule_hits = move || {
        state
            .analysis_result
            .get()
            .and_then(|response| response.result)
            .map(|result| result.rule_hits)
            .unwrap_or_default()
    };

    let confidence_info = move || {
        state
            .analysis_result
            .get()
            .and_then(|response| response.result)
            .and_then(|result| result.confidence)
    };

    let strong_alerts = move || {
        state
            .analysis_result
            .get()
            .and_then(|response| response.result)
            .map(|result| build_strong_alerts(&result.rule_hits))
            .unwrap_or_default()
    };

    let current_preference = move || {
        state
            .analysis_preference
            .get()
            .or_else(|| load_preference())
            .unwrap_or_else(|| "none".to_string())
    };

    let on_back = move |_| {
        // Navigate back using browser history
        if let Some(window) = web_sys::window() {
            if let Some(history) = window.history().ok() {
                let _ = history.back();
            }
        }
    };
    let on_back_home_bottom = {
        let state = state.clone();
        let navigate = navigate.clone();
        move |_| {
            state.analysis_id.set(None);
            state.analysis_result.set(None);
            state.analysis_preference.set(None);
            navigate("/", Default::default());
        }
    };
    let on_new_analysis = {
        let state = state.clone();
        let navigate = navigate.clone();
        move |_| {
            state.analysis_id.set(None);
            state.analysis_result.set(None);
            state.analysis_preference.set(None);
            navigate("/?view=scan", Default::default());
        }
    };

    view! {
        <section class="page page-result figma">
            <div class="page-topbar">
                <button class="icon-button" on:click=on_back aria-label="返回上一页">
                    <IconArrowLeft />
                </button>
                <div class="icon-placeholder"></div>
            </div>

            <div class="page-scrollable-content">

            // Preference badge
            <div class="preference-badge-container">
                <span class="preference-badge">
                    {move || get_preference_icon(&current_preference())}
                    " "
                    {move || get_preference_label(&current_preference())}
                </span>
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

            <Show when=move || !strong_alerts().is_empty()>
                <div class="surface-card result-section">
                    <h2 class="card-title">"重要提醒"</h2>
                    <ul class="advice-list">
                        {move || {
                            strong_alerts()
                                .into_iter()
                                .map(|item| view! { <li>{item}</li> })
                                .collect_view()
                        }}
                    </ul>
                </div>
            </Show>

            <Show when=move || confidence_info().is_some()>
                {move || {
                    confidence_info().map(|confidence| {
                        let reasons = confidence.reasons.clone();
                        let reasons_for_check = reasons.clone();
                        view! {
                            <div class="surface-card result-section">
                                <h2 class="card-title">"可信度"</h2>
                                <p class="analysis-summary">
                                    {format!("可信度：{}", confidence_label(&confidence.level))}
                                </p>
                                <Show when=move || !reasons_for_check.is_empty()>
                                    <ul class="advice-list">
                                        {reasons.iter().map(|reason| view! { <li>{reason.clone()}</li> }).collect_view()}
                                    </ul>
                                </Show>
                            </div>
                        }
                    })
                }}
            </Show>

            <div class="section-padding">
                <Show when=move || !rule_hits().is_empty()>
                    <div class="surface-card result-section">
                        <h2 class="card-title">"规则命中"</h2>
                        <div class="analysis-list">
                            {move || {
                                rule_hits()
                                    .into_iter()
                                    .map(|item| {
                                        let group_tags = item.group_tags.clone();
                                        let group_tags_for_check = group_tags.clone();
                                        view! {
                                            <div class="analysis-item">
                                                <div class="analysis-header">
                                                    <span class="analysis-name">{item.name}</span>
                                                    <RiskBadge level={item.risk_level} />
                                                </div>
                                                <p class="analysis-desc">{item.description}</p>
                                                <Show when=move || !group_tags_for_check.is_empty()>
                                                    <p class="analysis-summary">
                                                        {format!("适用人群：{}", format_group_tags(&group_tags))}
                                                    </p>
                                                </Show>
                                            </div>
                                        }
                                    })
                                    .collect_view()
                            }}
                        </div>
                    </div>
                </Show>

                <div class="surface-card result-section">
                    <h2 class="card-title">"配料分析"</h2>
                    {move || {
                        let has_focus_items = ingredient_items().iter().any(|item| item.is_focus);
                        has_focus_items.then(|| view! {
                            <p class="focus-hint">
                                "⭐ 带星标的成分是您关注的偏好相关成分"
                            </p>
                        })
                    }}
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
                                        let is_focus = item.is_focus;
                                        view! {
                                            <div class="analysis-item" class:focus-item=is_focus>
                                                <div class="analysis-header">
                                                    <span class="analysis-name">
                                                        {item.name}
                                                        {is_focus.then(|| view! {
                                                            <span class="focus-indicator" title="偏好关注成分">"⭐"</span>
                                                        })}
                                                    </span>
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

                <div class="surface-card result-section recommendation-card">
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

                <div class="surface-card result-section">
                    <h2 class="card-title">"配料表分析"</h2>
                    <p class="analysis-summary">{summary_text}</p>
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
            </div>
        </section>
    }
}
