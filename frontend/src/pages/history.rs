use leptos::prelude::*;
use leptos::task::spawn_local;
use js_sys::{Date, Object, Reflect};
use leptos_router::hooks::{use_location, use_navigate};
use wasm_bindgen::{JsCast, JsValue};

use crate::components::{ConfirmModal, ExportPreviewModal, LoadingSpinner};
use crate::services;
use crate::stores::{AnalysisSource, AppState, ToastLevel};
use crate::utils::navigation::build_full_path;
use crate::utils::export_image::{ExportData, ExportIngredient};
use crate::utils::{emit_toast, local_history};
use shared::{AnalysisResponse, AnalysisStatus, LlmStatus, OcrStatus};

fn format_datetime(date: &Date) -> String {
    let options = Object::new();
    let _ = Reflect::set(&options, &JsValue::from_str("year"), &JsValue::from_str("numeric"));
    let _ = Reflect::set(&options, &JsValue::from_str("month"), &JsValue::from_str("2-digit"));
    let _ = Reflect::set(&options, &JsValue::from_str("day"), &JsValue::from_str("2-digit"));
    let _ = Reflect::set(&options, &JsValue::from_str("hour"), &JsValue::from_str("2-digit"));
    let _ = Reflect::set(&options, &JsValue::from_str("minute"), &JsValue::from_str("2-digit"));
    let _ = Reflect::set(&options, &JsValue::from_str("second"), &JsValue::from_str("2-digit"));
    let _ = Reflect::set(&options, &JsValue::from_str("hour12"), &JsValue::from_bool(false));
    date.to_locale_string("zh-CN", &options.into())
        .as_string()
        .unwrap_or_default()
        .replace('/', "-")
}

fn format_timestamp(timestamp: i64) -> String {
    let date = Date::new(&JsValue::from_f64(timestamp as f64));
    format_datetime(&date)
}

fn format_iso_datetime(iso_string: &str) -> String {
    let date = Date::new(&JsValue::from_str(iso_string));
    if date.get_time().is_nan() {
        return iso_string.to_string();
    }
    format_datetime(&date)
}

fn page_from_search(search: &str) -> i64 {
    let trimmed = search.trim_start_matches('?');
    for pair in trimmed.split('&') {
        let mut parts = pair.splitn(2, '=');
        let key = parts.next().unwrap_or_default();
        if key == "page" {
            if let Some(value) = parts.next() {
                if let Ok(parsed) = value.parse::<i64>() {
                    if parsed > 0 {
                        return parsed;
                    }
                }
            }
        }
    }
    1
}

fn build_page_target(path: &str, page: i64) -> String {
    if page <= 1 {
        path.to_string()
    } else {
        format!("{}?page={}", path, page)
    }
}

fn is_modified_click(ev: &web_sys::MouseEvent) -> bool {
    ev.meta_key() || ev.ctrl_key() || ev.shift_key() || ev.alt_key() || ev.button() != 0
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
    let location = use_location();
    let loading = RwSignal::new(false);
    let page = RwSignal::new(1_i64);
    let total = RwSignal::new(0_i64);
    let items = RwSignal::new(Vec::<shared::HistoryItem>::new());
    let local_items = RwSignal::new(Vec::<local_history::LocalHistoryItem>::new());
    let last_load_key = RwSignal::new(None::<(uuid::Uuid, i64)>);
    let viewing_id = RwSignal::new(None::<uuid::Uuid>);
    let exporting_id = RwSignal::new(None::<uuid::Uuid>);
    let deleting_id = RwSignal::new(None::<uuid::Uuid>);
    let exporting_local_id = RwSignal::new(None::<String>);
    let deleting_local_id = RwSignal::new(None::<String>);

    // Confirm modal state
    let show_confirm = RwSignal::new(false);
    let pending_delete_id = RwSignal::new(None::<uuid::Uuid>);
    let pending_delete_local_id = RwSignal::new(None::<String>);

    // Export preview modal state
    let export_preview_url = RwSignal::new(None::<String>);

    create_effect(move |_| {
        let search = location.search.get();
        let new_page = page_from_search(&search);
        if page.get() != new_page {
            page.set(new_page);
        }
    });

    let navigate_for_page = navigate.clone();
    create_effect(move |_| {
        let current_page = page.get();
        let path = location.pathname.get_untracked();
        let search = location.search.get_untracked();
        let target = build_page_target(path.as_str(), current_page);
        let current = build_full_path(path.as_str(), search.as_str());
        if current != target {
            let navigate = navigate_for_page.get_value();
            navigate(&target, Default::default());
        }
    });

    let load_page = Callback::new(move |page_number: i64| {
        if loading.get_untracked() {
            return;
        }
        loading.set(true);
        let items = items.clone();
        let total = total.clone();
        spawn_local(async move {
            match services::fetch_user_history(page_number, 20).await {
                Ok(response) => {
                    items.set(response.items);
                    total.set(response.total);
                    // 不要在这里设置 page，避免触发循环
                }
                Err(err) => {
                    emit_toast(ToastLevel::Error, "加载失败", &err);
                }
            }
            loading.set(false);
        });
    });

    create_effect(move |_| {
        if let Some(user) = state.auth_user.get() {
            let current_page = page.get();
            let key = (user.id, current_page);
            if last_load_key.get_untracked().as_ref() == Some(&key) {
                return;
            }
            last_load_key.set(Some(key));
            load_page.run(current_page);
        }
    });

    create_effect(move |_| {
        if state.auth_user.get().is_none() {
            last_load_key.set(None);
            local_items.set(local_history::load_local_history());
        }
    });

    let on_delete = move |id: uuid::Uuid| {
        pending_delete_id.set(Some(id));
        pending_delete_local_id.set(None);
        show_confirm.set(true);
    };

    let on_delete_local = move |id: String| {
        pending_delete_local_id.set(Some(id));
        pending_delete_id.set(None);
        show_confirm.set(true);
    };

    let on_confirm_delete = Callback::new(move |_| {
        show_confirm.set(false);

        // Delete cloud record
        if let Some(id) = pending_delete_id.get() {
            deleting_id.set(Some(id));
            let deleting_id = deleting_id.clone();
            spawn_local(async move {
                match services::delete_history(id).await {
                    Ok(()) => {
                        emit_toast(ToastLevel::Success, "已删除", "记录已删除");
                        // 强制重新加载：清除缓存的 key
                        last_load_key.set(None);
                    }
                    Err(err) => {
                        emit_toast(ToastLevel::Error, "删除失败", &err);
                    }
                }
                deleting_id.set(None);
            });
        }

        // Delete local record
        if let Some(id) = pending_delete_local_id.get() {
            deleting_local_id.set(Some(id.clone()));
            match local_history::delete_local_history(&id) {
                Ok(()) => {
                    local_items.set(local_history::load_local_history());
                    emit_toast(ToastLevel::Success, "已删除", "本地记录已删除");
                }
                Err(err) => {
                    emit_toast(ToastLevel::Error, "删除失败", &err);
                }
            }
            deleting_local_id.set(None);
        }

        pending_delete_id.set(None);
        pending_delete_local_id.set(None);
    });

    let on_cancel_delete = Callback::new(move |_| {
        show_confirm.set(false);
        pending_delete_id.set(None);
        pending_delete_local_id.set(None);
    });

    let on_view_local = {
        let navigate = navigate.clone();
        move |item: local_history::LocalHistoryItem| {
            if let Some(response) = local_to_response(&item) {
                state.analysis_id.set(Some(response.id));
                state.analysis_result.set(Some(response));
                state.analysis_source.set(AnalysisSource::History);
                let navigate = navigate.get_value();
                navigate("/summary", Default::default());
            } else {
                emit_toast(ToastLevel::Error, "加载失败", "记录格式无效");
            }
        }
    };

    let on_view_cloud = {
        let navigate = navigate.clone();
        let viewing_id = viewing_id.clone();
        move |item: shared::HistoryItem| {
            let id = item.id;
            if viewing_id.get() == Some(id) {
                return;
            }
            viewing_id.set(Some(id));
            state.analysis_id.set(Some(id));
            state.analysis_source.set(AnalysisSource::History);

            // 异步获取完整的分析结果
            let viewing_id = viewing_id.clone();
            spawn_local(async move {
                match services::fetch_analysis(id).await {
                    Ok(response) => {
                        state.analysis_result.set(Some(response));
                        let navigate = navigate.get_value();
                        navigate("/summary", Default::default());
                    }
                    Err(err) => {
                        emit_toast(ToastLevel::Error, "加载失败", &err);
                    }
                }
                viewing_id.set(None);
            });
        }
    };

    let on_close_preview = Callback::new(move |_: ()| {
        export_preview_url.set(None);
    });
    let export_preview_signal = Signal::derive(move || export_preview_url.get());

    view! {
        <section class="page page-history figma">
            <ConfirmModal
                show=show_confirm.into()
                title="删除记录".to_string()
                message="确定要删除这条记录吗？删除后无法恢复。".to_string()
                confirm_text="删除".to_string()
                cancel_text="取消".to_string()
                on_confirm=on_confirm_delete
                on_cancel=on_cancel_delete
            />

            <ExportPreviewModal
                image_url=export_preview_signal
                on_close=on_close_preview
            />

            <div class="page-scrollable-content">
                <Show when=move || state.auth_user.get().is_some() fallback=move || {
                view! {
                    <div>
                        <Show when=move || !local_items.get().is_empty() fallback=move || view! {
                            <div class="empty-state">
                                <p class="hint">"暂无本地记录"</p>
                            </div>
                        }>
                            <ul class="history-list">
                                {move || local_items.get().into_iter().map(|item| {
                                    let id = item.id.clone();
                                    let id_value = StoredValue::new(id.clone());
                                    let summary = item.summary.clone();
                                    let score = item.health_score;
                                    let timestamp = format_timestamp(item.timestamp);
                                    let item_clone = item.clone();
                                    let image_path = StoredValue::new(item.image_path.clone());
                                    let image_url = image_path
                                        .get_value()
                                        .map(|path| services::resolve_media_url(&path));
                                    let has_image = image_url
                                        .as_ref()
                                        .map(|url| !url.is_empty())
                                        .unwrap_or(false);
                                    view! {
                                        <li class="history-item-card">
                                            <div class="history-card-main">
                                                <div class="history-card-content">
                                                    <div class="history-badges">
                                                        <span class="history-badge local">"本地记录"</span>
                                                    </div>

                                                    <div class="history-meta-row">
                                                        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" focusable="false">
                                                            <circle cx="12" cy="12" r="10"></circle>
                                                            <polyline points="12 6 12 12 16 14"></polyline>
                                                        </svg>
                                                        <span>{timestamp}</span>
                                                    </div>
                                                </div>

                                                <div class="history-thumb-wrapper">
                                                    <Show when=move || has_image fallback=move || view! {
                                                        <div class="history-thumb-img">"📷"</div>
                                                    }>
                                                        <img
                                                            src={image_url.clone().unwrap_or_default()}
                                                            alt="缩略图"
                                                            class="history-thumb-img"
                                                            loading="lazy"
                                                            width="72"
                                                            height="72"
                                                            on:error=move |ev| {
                                                                if let Some(target) = ev.target() {
                                                                    if let Ok(img) = target.dyn_into::<web_sys::HtmlImageElement>() {
                                                                        img.set_attribute("data-error", "true").ok();
                                                                    }
                                                                }
                                                            }
                                                        />
                                                    </Show>
                                                </div>
                                            </div>

                                            <p class="history-description">{summary}</p>

                                            <div class="history-divider"></div>

                                            <div class="history-footer">
                                                <div class="history-score-display">
                                                    "健康评分 "
                                                    <span class={format!("history-score-value {}",
                                                        if score >= 80 { "score-high" }
                                                        else if score >= 60 { "score-medium" }
                                                        else { "score-low" }
                                                    )}>
                                                        {score}
                                                    </span>
                                                </div>
                                                <div class="history-actions">
                                                    <a
                                                        class="history-action-btn"
                                                        href="/summary"
                                                        on:click=move |ev: web_sys::MouseEvent| {
                                                            if is_modified_click(&ev) {
                                                                return;
                                                            }
                                                            ev.prevent_default();
                                                            on_view_local(item_clone.clone());
                                                        }
                                                    >
                                                        "查看"
                                                    </a>
                                                    <button
                                                        class="history-action-btn export"
                                                        disabled=move || exporting_local_id.get() == Some(id_value.get_value())
                                                        aria-label="导出记录"
                                                        on:click={
                                                            let result = item.result.clone();
                                                            let exporting_local_id = exporting_local_id.clone();
                                                            move |_: web_sys::MouseEvent| {
                                                                let export_id = id_value.get_value();
                                                                if exporting_local_id.get() == Some(export_id.clone()) {
                                                                    return;
                                                                }
                                                                exporting_local_id.set(Some(export_id.clone()));
                                                                let result = result.clone();
                                                                let exporting_local_id = exporting_local_id.clone();
                                                                spawn_local(async move {
                                                                    let data = ExportData {
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
                                                                        preference_label: String::new(),
                                                                    };
                                                                    match crate::utils::export_image::export_to_data_url(&data) {
                                                                        Ok(url) => export_preview_url.set(Some(url)),
                                                                        Err(e) => emit_toast(ToastLevel::Error, "导出失败", &e),
                                                                    }
                                                                    exporting_local_id.set(None);
                                                                });
                                                            }
                                                        }
                                                    >
                                                        {move || if exporting_local_id.get() == Some(id_value.get_value()) { "导出中" } else { "📤" }}
                                                    </button>
                                                    <button
                                                        class="history-action-btn delete"
                                                        disabled=move || deleting_local_id.get() == Some(id_value.get_value())
                                                        on:click=move |_| on_delete_local(id_value.get_value())
                                                    >
                                                        {move || if deleting_local_id.get() == Some(id_value.get_value()) { "删除中" } else { "删除" }}
                                                    </button>
                                                </div>
                                            </div>
                                        </li>
                                    }
                                }).collect_view()}
                            </ul>
                        </Show>
                    </div>
                }
            }>
                <div>
                    <Show when=move || loading.get() fallback=move || view! {
                        <Show when=move || !items.get().is_empty() fallback=move || view! {
                            <div class="empty-state">
                                <p class="hint">"暂无历史记录"</p>
                            </div>
                        }>
                            <ul class="history-list">
                                {move || items.get().into_iter().map(|item| {
                                    let id = item.id;
                                    let item_clone = item.clone();
                                    let summary = item.summary.clone().unwrap_or_default();
                                    let formatted_time = format_iso_datetime(&item.created_at);
                                    let image_url = StoredValue::new(item.image_url.clone());
                                    let resolved_image_url =
                                        StoredValue::new(services::resolve_media_url(&image_url.get_value()));
                                        view! {
                                            <li class="history-item-card">
                                                <div class="history-card-main">
                                                    <div class="history-card-content">
                                                        <div class="history-badges">
                                                            <span class="history-badge cloud">"云端记录"</span>
                                                        </div>

                                                        <div class="history-meta-row">
                                                            <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" focusable="false">
                                                                <circle cx="12" cy="12" r="10"></circle>
                                                                <polyline points="12 6 12 12 16 14"></polyline>
                                                            </svg>
                                                            <span>{formatted_time}</span>
                                                        </div>
                                                    </div>

                                                    <div class="history-thumb-wrapper">
                                                        <Show when=move || !resolved_image_url.get_value().is_empty() fallback=move || view! {
                                                            <div class="history-thumb-img">"📷"</div>
                                                        }>
                                                            <img
                                                                src={resolved_image_url.get_value()}
                                                                alt="缩略图"
                                                                class="history-thumb-img"
                                                                loading="lazy"
                                                                width="72"
                                                                height="72"
                                                                on:error=move |ev| {
                                                                    if let Some(target) = ev.target() {
                                                                        if let Ok(img) = target.dyn_into::<web_sys::HtmlImageElement>() {
                                                                            img.set_attribute("data-error", "true").ok();
                                                                        }
                                                                    }
                                                                }
                                                            />
                                                        </Show>
                                                    </div>
                                                </div>

                                                <p class="history-description">{summary}</p>

                                                <div class="history-divider"></div>

                                                <div class="history-footer">
                                                    <div class="history-score-display">
                                                        "健康评分 "
                                                        <span class={format!("history-score-value {}",
                                                            if item.health_score.unwrap_or(0) >= 80 { "score-high" }
                                                            else if item.health_score.unwrap_or(0) >= 60 { "score-medium" }
                                                            else { "score-low" }
                                                        )}>
                                                            {item.health_score.unwrap_or(0)}
                                                        </span>
                                                    </div>
                                                    <div class="history-actions">
                                                        <a
                                                            class="history-action-btn"
                                                            href="/summary"
                                                            aria-disabled=move || viewing_id.get() == Some(id)
                                                            on:click=move |ev: web_sys::MouseEvent| {
                                                                if is_modified_click(&ev) {
                                                                    return;
                                                                }
                                                                ev.prevent_default();
                                                                if viewing_id.get() == Some(id) {
                                                                    return;
                                                                }
                                                                on_view_cloud(item_clone.clone());
                                                            }
                                                        >
                                                            {move || if viewing_id.get() == Some(id) { "加载中" } else { "查看" }}
                                                        </a>
                                                        <button
                                                            class="history-action-btn export"
                                                            disabled=move || exporting_id.get() == Some(id)
                                                            aria-label="导出记录"
                                                            on:click=move |_: web_sys::MouseEvent| {
                                                                if exporting_id.get() == Some(id) {
                                                                    return;
                                                                }
                                                                exporting_id.set(Some(id));
                                                                let analysis_id = id;
                                                                let exporting_id = exporting_id.clone();
                                                                spawn_local(async move {
                                                                    match services::fetch_analysis(analysis_id).await {
                                                                        Ok(response) => {
                                                                            if let Some(result) = response.result {
                                                                                let data = ExportData {
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
                                                                                    preference_label: String::new(),
                                                                                };
                                                                                match crate::utils::export_image::export_to_data_url(&data) {
                                                                                    Ok(url) => export_preview_url.set(Some(url)),
                                                                                    Err(e) => emit_toast(ToastLevel::Error, "导出失败", &e),
                                                                                }
                                                                            } else {
                                                                                emit_toast(ToastLevel::Error, "导出失败", "该记录没有分析结果");
                                                                            }
                                                                        }
                                                                        Err(err) => {
                                                                            emit_toast(ToastLevel::Error, "导出失败", &err);
                                                                        }
                                                                    }
                                                                    exporting_id.set(None);
                                                                });
                                                            }
                                                        >
                                                            {move || if exporting_id.get() == Some(id) { "导出中" } else { "📤" }}
                                                        </button>
                                                        <button
                                                            class="history-action-btn delete"
                                                            disabled=move || deleting_id.get() == Some(id)
                                                            on:click=move |_| on_delete(id)
                                                        >
                                                            {move || if deleting_id.get() == Some(id) { "删除中" } else { "删除" }}
                                                        </button>
                                                    </div>
                                                </div>
                                            </li>
                                        }                          }).collect_view()}
                            </ul>
                        </Show>
                        <Show when=move || { total.get() > 20 }>
                            <div class="history-pagination">
                                <button
                                    class="secondary-cta"
                                    disabled=move || page.get() <= 1 || loading.get()
                                    on:click=move |_| {
                                        let new_page = page.get() - 1;
                                        page.set(new_page);
                                    }
                                >
                                    {move || if loading.get() { "加载中…" } else { "上一页" }}
                                </button>
                                <span>{move || format!("第 {} 页 / 共 {} 条", page.get(), total.get())}</span>
                                <button
                                    class="secondary-cta"
                                    disabled=move || page.get() * 20 >= total.get() || loading.get()
                                    on:click=move |_| {
                                        let new_page = page.get() + 1;
                                        page.set(new_page);
                                    }
                                >
                                    {move || if loading.get() { "加载中…" } else { "下一页" }}
                                </button>
                            </div>
                        </Show>
                    }>
                        <div class="history-loading">
                            <LoadingSpinner message="加载历史记录中…" />
                        </div>
                    </Show>
                </div>
            </Show>
            </div>
        </section>
    }
}
