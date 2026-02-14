use crate::components::{
    get_preference_icon, get_preference_label, HealthScoreCard, IconArrowLeft, RiskBadge,
    ShareButton, ShareExportProps,
};
use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::emit_toast;
use crate::utils::preference::load_preference;
use leptos::either::Either;
use leptos::leptos_dom::helpers::set_timeout;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use shared::{AnalysisStatus, RuleHit};
use std::collections::HashSet;
use std::time::Duration;
use wasm_bindgen::JsCast;

#[derive(Clone, PartialEq)]
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

fn confidence_percent(level: &str) -> i32 {
    match level.trim().to_lowercase().as_str() {
        "high" => 85,
        "medium" => 60,
        "low" => 35,
        _ => 50,
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

fn format_rule_evidence(evidence: &Option<String>, source: &Option<String>) -> Option<String> {
    match (evidence.as_ref(), source.as_ref()) {
        (Some(ev), Some(src)) => Some(format!("证据：{}（{}）", ev, src)),
        (Some(ev), None) => Some(format!("证据：{}", ev)),
        (None, Some(src)) => Some(format!("证据来源：{}", src)),
        (None, None) => None,
    }
}

fn build_risk_chain(rule_hits: &[RuleHit]) -> Vec<String> {
    let mut chains = Vec::new();
    for hit in rule_hits {
        let mut chain = format!("{} → {} 风险", hit.name, hit.risk_level);
        if !hit.group_tags.is_empty() {
            let tags = format_group_tags(&hit.group_tags);
            if !tags.is_empty() {
                chain = format!("{} → {}", chain, tags);
            }
        }
        chains.push(chain);
    }
    chains
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

fn conclusion_label(score: i32) -> &'static str {
    match score {
        75..=100 => "可吃",
        50..=74 => "谨慎",
        _ => "不推荐",
    }
}

fn split_key_risks(rule_hits: &[RuleHit], max_items: usize) -> (Vec<RuleHit>, Vec<RuleHit>) {
    let mut hits: Vec<RuleHit> = rule_hits
        .iter()
        .filter(|item| risk_rank(&item.risk_level) <= 1)
        .cloned()
        .collect();
    hits.sort_by_key(|item| risk_rank(&item.risk_level));
    let remaining = if hits.len() > max_items {
        hits.split_off(max_items)
    } else {
        Vec::new()
    };
    (hits, remaining)
}

fn format_dimension_label(value: &str) -> &'static str {
    match value.trim().to_lowercase().as_str() {
        "additives_processing" => "添加剂/加工",
        "sugar_fat" => "糖/脂",
        "nutrition_value" => "营养价值",
        "sensitive" => "敏感人群",
        "formula_complexity" => "配方复杂度",
        _ => "其他",
    }
}

fn build_preference_guidance(preference: &str, rule_hits: &[RuleHit]) -> Vec<String> {
    let mut guidance = Vec::new();
    let pref = preference.trim().to_lowercase();

    if pref == "allergy" {
        guidance.push("过敏人群优先关注过敏原与交叉污染提示。".to_string());
    } else if pref == "kids" {
        guidance.push("儿童人群建议避免高糖与高风险添加剂。".to_string());
    } else if pref == "pregnancy" {
        guidance.push("孕期建议避开刺激性成分与不明确添加剂。".to_string());
    } else if pref == "weight_loss" {
        guidance.push("控重人群建议优先选择低糖/低脂产品。".to_string());
    } else if pref == "health" {
        guidance.push("健康人群建议减少高糖与高盐摄入。".to_string());
    }

    let has_allergen = rule_hits
        .iter()
        .any(|item| item.category.trim().to_lowercase() == "allergen");
    if has_allergen {
        guidance.push("检测到过敏原，建议优先选择无过敏原替代品。".to_string());
    }

    guidance
}

fn build_alternative_suggestions(rule_hits: &[RuleHit]) -> Vec<String> {
    let mut suggestions = Vec::new();
    let mut has_allergen = false;
    let mut has_additives = false;
    let mut has_sugar = false;

    for hit in rule_hits {
        let category = hit.category.trim().to_lowercase();
        if category == "allergen" {
            has_allergen = true;
        }
        if category == "additive" && risk_rank(&hit.risk_level) <= 1 {
            has_additives = true;
        }
        let name = hit.name.trim();
        if name.contains("糖") || name.contains("糖浆") {
            has_sugar = true;
        }
    }

    if has_allergen {
        suggestions.push("优先选择无过敏原或有清晰过敏原标识的替代品。".to_string());
    }
    if has_sugar {
        suggestions.push("可尝试低糖/无糖版本，或用天然甜味来源替代。".to_string());
    }
    if has_additives {
        suggestions.push("优先选择配料表更简洁、添加剂更少的产品。".to_string());
    }

    if suggestions.is_empty() {
        suggestions.push("当前配料结构较简单，可优先选择配料更短、更透明的产品。".to_string());
    }

    suggestions
}

#[component]
pub fn ResultPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();
    let state_for_effect = state.clone();
    let state_for_status = state.clone();
    let fetching = RwSignal::new(false);
    let polling = RwSignal::new(false);
    let poll_attempts = RwSignal::new(0u32);
    let state_for_polling = state.clone();
    let selected_rule = RwSignal::new(None::<String>);

    create_effect(move |_| {
        let _ = state.analysis_id.get();
        selected_rule.set(None);
        polling.set(false);
        poll_attempts.set(0);
        fetching.set(false);
    });

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
            let fetching = fetching.clone();
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

        if !should_poll {
            poll_attempts.set(0);
            return;
        }

        let attempts = poll_attempts.get();
        let max_attempts = 20u32;
        if attempts >= max_attempts {
            emit_toast(ToastLevel::Error, "分析超时", "分析耗时较长，请稍后刷新结果。");
            poll_attempts.set(0);
            return;
        }

        if let Some(id) = analysis_id {
            polling.set(true);
            poll_attempts.set(attempts + 1);
            let state = state_for_polling.clone();
            let polling = polling.clone();
            let delay_secs = (2 + attempts as u64).min(6);
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
                Duration::from_secs(delay_secs),
            );
        }
    });

    let ingredient_items = create_memo(move |_| {
        state
            .analysis_result
            .get()
            .and_then(|response| response.result)
            .map(|result| analysis_items(&result))
            .unwrap_or_default()
    });

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

            // Conclusion card
            <Show when=move || {
                state.analysis_result.get()
                    .and_then(|r| r.result)
                    .is_some()
            }>
                {move || {
                    state.analysis_result.get()
                        .and_then(|r| r.result)
                        .map(|result| {
                            let conclusion = conclusion_label(result.health_score);
                            let reason = summary_text();
                            view! {
                                <div class="surface-card result-section">
                                    <h2 class="card-title">"结论"</h2>
                                    <p class="analysis-summary">{format!("综合判断：{}", conclusion)}</p>
                                    <p class="analysis-desc">{reason}</p>
                                </div>
                            }
                        })
                }}
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

            <Show when=move || {
                state.analysis_result.get()
                    .and_then(|r| r.result)
                    .is_some()
            }>
                <div class="surface-card result-section">
                    <h2 class="card-title">"关键风险"</h2>
                    <p class="hint">"仅展示中高风险成分"</p>
                    {move || {
                        let (primary, remaining) = split_key_risks(&rule_hits(), 3);
                        let content = if primary.is_empty() {
                            Either::Right(view! { <p class="analysis-summary">"暂未识别中高风险成分"</p> })
                        } else {
                            let has_remaining = !remaining.is_empty();
                            let remaining_block = has_remaining.then(|| {
                                view! {
                                    <details class="analysis-item">
                                        <summary class="analysis-header">
                                            <span class="analysis-name">
                                                {format!("查看更多（{}）", remaining.len())}
                                            </span>
                                        </summary>
                                        <div class="analysis-list">
                                            {remaining
                                                .iter()
                                                .cloned()
                                                .map(|item| {
                                                    let name = item.name.clone();
                                                    let name_for_click = name.clone();
                                                    let is_selected = selected_rule
                                                        .get()
                                                        .map(|value| value == name)
                                                        .unwrap_or(false);
                                                    view! {
                                                        <div
                                                            role="button"
                                                            tabindex="0"
                                                            class="analysis-item"
                                                            class:selected-item=is_selected
                                                            on:click=move |_| {
                                                                selected_rule.set(Some(name_for_click.clone()));
                                                            }
                                                        >
                                                            <div class="analysis-header">
                                                                <span class="analysis-name">{item.name}</span>
                                                                <RiskBadge level={item.risk_level} />
                                                            </div>
                                                            <p class="analysis-desc">{item.description}</p>
                                                            <Show when=move || is_selected>
                                                                <p class="analysis-summary">"已定位到规则命中"</p>
                                                            </Show>
                                                        </div>
                                                    }
                                                })
                                                .collect_view()}
                                        </div>
                                    </details>
                                }
                            });
                            Either::Left(view! {
                                <div class="analysis-list">
                                    {primary
                                        .iter()
                                        .cloned()
                                        .map(|item| {
                                            let name = item.name.clone();
                                            let name_for_click = name.clone();
                                            let is_selected = selected_rule
                                                .get()
                                                .map(|value| value == name)
                                                .unwrap_or(false);
                                            view! {
                                                <div
                                                    role="button"
                                                    tabindex="0"
                                                    class="analysis-item"
                                                    class:selected-item=is_selected
                                                    on:click=move |_| {
                                                        selected_rule.set(Some(name_for_click.clone()));
                                                    }
                                                >
                                                    <div class="analysis-header">
                                                        <span class="analysis-name">{item.name}</span>
                                                        <RiskBadge level={item.risk_level} />
                                                    </div>
                                                    <p class="analysis-desc">{item.description}</p>
                                                    <Show when=move || is_selected>
                                                        <p class="analysis-summary">"已定位到规则命中"</p>
                                                    </Show>
                                                </div>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                                {remaining_block}
                            })
                        };
                        content
                    }}
                </div>
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

            <div class="section-padding">
                <Show when=move || confidence_info().is_some()>
                    {move || {
                        confidence_info().map(|confidence| {
                            let reasons = confidence.reasons.clone();
                            let reasons_for_check = reasons.clone();
                            let factors = confidence.factors.clone();
                            let factors_for_check = factors.clone();
                            view! {
                                <div class="surface-card result-section">
                                    <h2 class="card-title">"可信度"</h2>
                                    <p class="hint">"可信度分数越高，结果越可靠"</p>
                                    <p class="analysis-summary">
                                        {format!("可信度：{}", confidence_label(&confidence.level))}
                                    </p>
                                    <div class="analysis-item">
                                        <div class="analysis-header">
                                            <span class="analysis-name">"可信度进度"</span>
                                            <span class="analysis-desc">
                                                {format!("{}%", confidence_percent(&confidence.level))}
                                            </span>
                                        </div>
                                        <div class="progress-track">
                                            <div
                                                class="progress-bar"
                                                style={format!("width: {}%", confidence_percent(&confidence.level))}
                                            ></div>
                                        </div>
                                    </div>
                                    <Show when=move || !reasons_for_check.is_empty()>
                                        <ul class="advice-list">
                                            {reasons.iter().map(|reason| view! { <li>{reason.clone()}</li> }).collect_view()}
                                        </ul>
                                    </Show>
                                    <Show when=move || !factors_for_check.is_empty()>
                                        <div class="analysis-list">
                                            {factors.iter().map(|factor| {
                                                let detail = factor.detail.clone().unwrap_or_default();
                                                let detail_for_check = detail.clone();
                                                view! {
                                                    <div class="analysis-item">
                                                        <div class="analysis-header">
                                                            <span class="analysis-name">{factor.label.clone()}</span>
                                                            <span class="analysis-desc">
                                                                {format!("{}分", factor.score)}
                                                            </span>
                                                        </div>
                                                        <Show when=move || !detail_for_check.is_empty()>
                                                            <p class="analysis-summary">{detail.clone()}</p>
                                                        </Show>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </Show>
                                </div>
                            }
                        })
                    }}
                </Show>
                <Show when=move || {
                    state.analysis_result.get()
                        .and_then(|r| r.result)
                        .and_then(|result| result.score_breakdown)
                        .map(|items| !items.is_empty())
                        .unwrap_or(false)
                } fallback=move || view! {
                    <div class="surface-card result-section">
                        <h2 class="card-title">"风险维度"</h2>
                        <p class="hint">"同类对比暂无，风险维度将随数据完善"</p>
                    </div>
                }>
                    {move || {
                        state.analysis_result.get()
                            .and_then(|r| r.result)
                            .and_then(|result| result.score_breakdown)
                            .map(|items| view! {
                                <div class="surface-card result-section">
                                    <h2 class="card-title">"风险维度"</h2>
                                    <p class="hint">"评分仅用于相对参考"</p>
                                    <div class="analysis-list">
                                        {items.iter().map(|item| {
                                            let label = format_dimension_label(&item.dimension);
                                            view! {
                                                <div class="analysis-item">
                                                    <div class="analysis-header">
                                                        <span class="analysis-name">{label}</span>
                                                        <span class="analysis-desc">
                                                            {format!("{}分", item.score)}
                                                        </span>
                                                    </div>
                                                    <div class="progress-track">
                                                        <div
                                                            class="progress-bar"
                                                            style={format!("width: {}%", item.score.clamp(0, 100))}
                                                        ></div>
                                                    </div>
                                                    <p class="analysis-summary">{item.reason.clone()}</p>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            })
                    }}
                </Show>

                <Show when=move || !rule_hits().is_empty()>
                    <div class="surface-card result-section">
                        <h2 class="card-title">"风险链路"</h2>
                        <ul class="advice-list">
                            {move || {
                                build_risk_chain(&rule_hits())
                                    .into_iter()
                                    .map(|item| view! { <li>{item}</li> })
                                    .collect_view()
                            }}
                        </ul>
                    </div>
                </Show>

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
                                        let evidence = format_rule_evidence(&item.evidence, &item.source);
                                        let evidence_text = evidence.clone().unwrap_or_default();
                                        let evidence_for_check = evidence_text.clone();
                                        let name = item.name.clone();
                                        let is_selected = selected_rule
                                            .get()
                                            .map(|value| value == name)
                                            .unwrap_or(false);
                                        view! {
                                            <details class="analysis-item" open=is_selected>
                                                <summary class="analysis-header">
                                                    <span class="analysis-name">{item.name}</span>
                                                    <RiskBadge level={item.risk_level} />
                                                </summary>
                                                <Show when=move || is_selected>
                                                    <p class="analysis-summary">"来自关键风险定位"</p>
                                                </Show>
                                                <p class="analysis-desc">{item.description}</p>
                                                <Show when=move || !group_tags_for_check.is_empty()>
                                                    <p class="analysis-summary">
                                                        {format!("适用人群：{}", format_group_tags(&group_tags))}
                                                    </p>
                                                </Show>
                                                <Show when=move || !evidence_for_check.is_empty()>
                                                    <p class="analysis-summary">{evidence_text.clone()}</p>
                                                </Show>
                                            </details>
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
                        let has_focus_items = ingredient_items.get().iter().any(|item| item.is_focus);
                        has_focus_items.then(|| view! {
                            <p class="focus-hint">
                                "⭐ 带星标的成分是您关注的偏好相关成分"
                            </p>
                        })
                    }}
                    <Show
                        when=move || !ingredient_items.get().is_empty()
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
                                ingredient_items.get()
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

                <div class="surface-card result-section">
                    <h2 class="card-title">"人群建议"</h2>
                    {move || {
                        let guidance = build_preference_guidance(&current_preference(), &rule_hits());
                        let content = if guidance.is_empty() {
                            Either::Right(view! { <p class="analysis-summary">"暂无特定人群建议"</p> })
                        } else {
                            Either::Left(view! {
                                <ul class="advice-list">
                                    {guidance
                                        .iter()
                                        .cloned()
                                        .map(|item| view! { <li>{item}</li> })
                                        .collect_view()}
                                </ul>
                            })
                        };
                        content
                    }}
                </div>

                <div class="surface-card result-section">
                    <h2 class="card-title">"可替代建议"</h2>
                    <ul class="advice-list">
                        {move || {
                            build_alternative_suggestions(&rule_hits())
                                .into_iter()
                                .map(|item| view! { <li>{item}</li> })
                                .collect_view()
                        }}
                    </ul>
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
                {move || {
                    state.analysis_result.get()
                        .and_then(|r| r.result)
                        .map(|result| {
                            let pref = current_preference();
                            let props = ShareExportProps {
                                health_score: result.health_score,
                                recommendation: result.recommendation.clone(),
                                ingredients: result.ingredients.iter().map(|i| {
                                    crate::utils::export_image::ExportIngredient {
                                        name: i.name.clone(),
                                        risk_level: i.risk_level.clone(),
                                        description: i.description.clone().unwrap_or_default(),
                                        is_focus: result.focus_ingredients.as_ref()
                                            .map(|fi| fi.iter().any(|f| f.trim().to_lowercase() == i.name.trim().to_lowercase()))
                                            .unwrap_or(false),
                                    }
                                }).collect(),
                                warnings: result.warnings.iter().map(|w| w.message.clone()).collect(),
                                summary: result.focus_summary.clone()
                                    .filter(|s| !s.trim().is_empty())
                                    .unwrap_or_else(|| result.summary.clone()),
                                preference_label: get_preference_label(&pref).to_string(),
                            };
                            view! { <ShareButton props=props /> }
                        })
                }}
                <button class="primary-cta" on:click=on_new_analysis>
                    "分析新产品"
                </button>
            </div>
            </div>
        </section>
    }
}
