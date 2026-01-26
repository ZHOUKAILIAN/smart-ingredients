use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use wasm_bindgen::JsValue;

use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::{emit_toast, local_history};
use shared::{AnalysisResponse, AnalysisStatus, LlmStatus, OcrStatus};

fn format_timestamp(timestamp: i64) -> String {
    let date = js_sys::Date::new(&JsValue::from_f64(timestamp as f64));
    let iso = date.to_iso_string().as_string().unwrap_or_default();
    let trimmed = iso.get(0..19).unwrap_or(&iso);
    trimmed.replace('T', " ")
}

fn local_to_response(item: &local_history::LocalHistoryItem) -> Option<AnalysisResponse> {
    let id = uuid::Uuid::parse_str(&item.id).ok()?;
    let created_at = format_timestamp(item.timestamp);
    Some(AnalysisResponse {
        id,
        status: AnalysisStatus::Completed,
        ocr_status: OcrStatus::Completed,
        llm_status: LlmStatus::Completed,
        ocr_text: None,
        confirmed_text: None,
        ocr_completed_at: None,
        result: Some(item.result.clone()),
        error_message: None,
        created_at: created_at.clone(),
        updated_at: created_at,
    })
}

#[component]
pub fn HistoryPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = StoredValue::new(use_navigate());
    let loading = create_rw_signal(false);
    let page = create_rw_signal(1_i64);
    let total = create_rw_signal(0_i64);
    let items = create_rw_signal(Vec::<shared::HistoryItem>::new());
    let local_items = create_rw_signal(Vec::<local_history::LocalHistoryItem>::new());

    let load_page = Callback::new(move |page_number: i64| {
        if loading.get() {
            return;
        }
        loading.set(true);
        let items = items.clone();
        let total = total.clone();
        let page_signal = page.clone();
        spawn_local(async move {
            match services::fetch_user_history(page_number, 20).await {
                Ok(response) => {
                    items.set(response.items);
                    total.set(response.total);
                    page_signal.set(response.page);
                }
                Err(err) => {
                    emit_toast(ToastLevel::Error, "加载失败", &err);
                }
            }
            loading.set(false);
        });
    });

    create_effect(move |_| {
        if state.auth_user.get().is_some() {
            load_page.run(page.get());
        }
    });

    create_effect(move |_| {
        if state.auth_user.get().is_none() {
            local_items.set(local_history::load_local_history());
        }
    });

    let on_delete = {
        let load_page = load_page.clone();
        move |id: uuid::Uuid| {
        let current_page = page.get();
        spawn_local(async move {
            match services::delete_history(id).await {
                Ok(()) => {
                    emit_toast(ToastLevel::Success, "已删除", "记录已删除");
                    load_page.run(current_page);
                }
                Err(err) => {
                    emit_toast(ToastLevel::Error, "删除失败", &err);
                }
            }
        });
    }};

    let on_delete_local = {
        let local_items = local_items.clone();
        move |id: String| {
            match local_history::delete_local_history(&id) {
                Ok(()) => {
                    local_items.set(local_history::load_local_history());
                    emit_toast(ToastLevel::Success, "已删除", "本地记录已删除");
                }
                Err(err) => {
                    emit_toast(ToastLevel::Error, "删除失败", &err);
                }
            }
        }
    };

    let on_view_local = {
        let navigate = navigate.clone();
        move |item: local_history::LocalHistoryItem| {
            if let Some(response) = local_to_response(&item) {
                state.analysis_id.set(Some(response.id));
                state.analysis_result.set(Some(response));
                let navigate = navigate.get_value();
                navigate("/summary", Default::default());
            } else {
                emit_toast(ToastLevel::Error, "加载失败", "记录格式无效");
            }
        }
    };

    view! {
        <section class="page page-history">
            <div class="page-header">
                <div>
                    <h2>"分析历史"</h2>
                    <p class="subtitle">"本地记录与云端历史"</p>
                </div>
            </div>
            <Show when=move || state.auth_user.get().is_some() fallback=move || {
                view! {
                    <div class="surface-card history-local-card">
                        <div class="history-section-header">
                            <h3>"本地记录"</h3>
                            <span class="history-tag">"仅本设备可见"</span>
                        </div>
                        <Show when=move || !local_items.get().is_empty() fallback=move || view! {
                            <p class="hint">"暂无本地记录"</p>
                        }>
                            <ul class="history-list">
                                {move || local_items.get().into_iter().map(|item| {
                                    let id = item.id.clone();
                                    let summary = item.summary.clone();
                                    let score = item.health_score;
                                    let timestamp = format_timestamp(item.timestamp);
                                    let item_clone = item.clone();
                                    view! {
                                        <li class="history-item">
                                            <div class="history-item-main">
                                                <div class="history-item-meta">
                                                    <span class="history-label">"本地记录"</span>
                                                    <span class="history-time">{timestamp}</span>
                                                </div>
                                                <p class="history-summary">{summary}</p>
                                                <p class="history-score">{format!("健康评分: {}", score)}</p>
                                            </div>
                                            <div class="history-item-actions">
                                                <button class="secondary-cta" on:click=move |_| on_view_local(item_clone.clone())>
                                                    "查看"
                                                </button>
                                                <button class="secondary-cta" on:click=move |_| on_delete_local(id.clone())>
                                                    "删除"
                                                </button>
                                            </div>
                                        </li>
                                    }
                                }).collect_view()}
                            </ul>
                        </Show>
                        <div class="history-login-cta">
                            <p class="hint">"登录后可同步云端历史记录"</p>
                            <button class="primary-button" on:click=move |_| {
                                let navigate = navigate.get_value();
                                navigate("/login", Default::default());
                            }>"去登录"</button>
                        </div>
                    </div>
                }
            }>
                <div class="surface-card">
                    <Show when=move || !items.get().is_empty() fallback=move || view! {
                        <p class="hint">"暂无历史记录"</p>
                    }>
                        <ul class="history-list">
                            {move || items.get().into_iter().map(|item| {
                                let id = item.id;
                                view! {
                                    <li class="history-item">
                                        <div class="history-item-main">
                                            <div class="history-item-meta">
                                                <span class="history-label">"云端记录"</span>
                                                <span class="history-time">{item.created_at.clone()}</span>
                                            </div>
                                            <p class="history-score">{format!("健康评分: {}", item.health_score.unwrap_or(0))}</p>
                                        </div>
                                        <div class="history-item-actions">
                                            <button class="secondary-cta" on:click=move |_| on_delete(id)>
                                                "删除"
                                            </button>
                                        </div>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    </Show>
                    <div class="history-pagination">
                        <button
                            class="secondary-cta"
                            disabled=move || page.get() <= 1
                            on:click=move |_| load_page.run(page.get() - 1)
                        >
                            "上一页"
                        </button>
                        <span>{move || format!("第 {} 页 / 共 {} 条", page.get(), total.get())}</span>
                        <button
                            class="secondary-cta"
                            disabled=move || (page.get() * 20) >= total.get()
                            on:click=move |_| load_page.run(page.get() + 1)
                        >
                            "下一页"
                        </button>
                    </div>
                </div>
            </Show>
        </section>
    }
}
