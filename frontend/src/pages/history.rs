use js_sys::{Date, Object, Reflect};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::{use_location, use_navigate};
use wasm_bindgen::{JsCast, JsValue};

use crate::components::ConfirmModal;
use crate::services;
use crate::stores::{AnalysisSource, AppState, ToastLevel};
use crate::utils::navigation::build_full_path;
use crate::utils::{emit_toast, local_history};
use shared::{AnalysisResponse, AnalysisStatus, LlmStatus, OcrStatus};

fn format_datetime(date: &Date) -> String {
    let options = Object::new();
    let _ = Reflect::set(
        &options,
        &JsValue::from_str("year"),
        &JsValue::from_str("numeric"),
    );
    let _ = Reflect::set(
        &options,
        &JsValue::from_str("month"),
        &JsValue::from_str("2-digit"),
    );
    let _ = Reflect::set(
        &options,
        &JsValue::from_str("day"),
        &JsValue::from_str("2-digit"),
    );
    let _ = Reflect::set(
        &options,
        &JsValue::from_str("hour"),
        &JsValue::from_str("2-digit"),
    );
    let _ = Reflect::set(
        &options,
        &JsValue::from_str("minute"),
        &JsValue::from_str("2-digit"),
    );
    let _ = Reflect::set(
        &options,
        &JsValue::from_str("second"),
        &JsValue::from_str("2-digit"),
    );
    let _ = Reflect::set(
        &options,
        &JsValue::from_str("hour12"),
        &JsValue::from_bool(false),
    );
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
    let deleting_id = RwSignal::new(None::<uuid::Uuid>);
    let deleting_local_id = RwSignal::new(None::<String>);

    // Confirm modal state
    let show_confirm = RwSignal::new(false);
    let pending_delete_id = RwSignal::new(None::<uuid::Uuid>);
    let pending_delete_local_id = RwSignal::new(None::<String>);

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

    view! {
        <section class="page figma">
            <ConfirmModal
                show=show_confirm.into()
                title="删除记录".to_string()
                message="确定要删除这条记录吗？删除后无法恢复。".to_string()
                confirm_text="删除".to_string()
                cancel_text="取消".to_string()
                on_confirm=on_confirm_delete
                on_cancel=on_cancel_delete
            />

            <div class="page-scrollable-content pb-20">

                <Show when=move || state.auth_user.get().is_some() fallback=move || {
                view! {
                    <div>
                        <div class="px-5 mb-4">
                            <Show when=move || !local_items.get().is_empty() fallback=move || view! {
                                <a href="/" class="block mt-4 p-4 shadow-sm border border-emerald-100 bg-white-50/50 backdrop-blur-sm rounded-2xl transition-all hover:bg-emerald-50 cursor-pointer text-left" style="text-decoration: none;">
                                    <div class="animate-pulse">
                                        <div class="flex items-center justify-between mb-3">
                                            <div class="h-4 bg-emerald-100/50 rounded w-16"></div>
                                            <div class="h-3 bg-emerald-50 rounded w-20"></div>
                                        </div>
                                        <div class="flex items-start gap-3 mb-3">
                                            <div class="w-16 h-16 bg-emerald-50 rounded-lg flex-shrink-0 flex items-center justify-center">
                                                <div class="text-emerald-300 text-3xl font-light mb-1">"+"</div>
                                            </div>
                                            <div class="flex-1 space-y-2 py-1">
                                                <div class="h-4 bg-emerald-50 rounded w-3/4"></div>
                                                <div class="h-3 bg-emerald-50 rounded w-1/2"></div>
                                                <div class="h-3 bg-emerald-50 rounded w-5/6"></div>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="flex items-center justify-center pt-3 border-t border-emerald-50">
                                        <span class="text-sm font-medium text-emerald-600">"点击开启第一次分析"</span>
                                    </div>
                                </a>
                            }>
                                <div class="space-y-3">
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
                                            <div class="p-4 shadow-lg border-0 bg-white-95 backdrop-blur-sm rounded-2xl transition-all duration-300">
                                                <div class="flex justify-end mb-3">
                                                    <span class="text-xs text-gray-500">{timestamp}</span>
                                                </div>

                                                <div class="flex items-start gap-3 mb-3">
                                                    <Show when=move || has_image>
                                                        <img
                                                            src={image_url.clone().unwrap_or_default()}
                                                            alt=""
                                                            class="w-16 h-16 rounded-lg object-cover flex-shrink-0 border border-gray-100"
                                                            loading="lazy"
                                                            on:error=move |ev| {
                                                                if let Some(target) = ev.target() {
                                                                    if let Ok(el) = target.dyn_into::<web_sys::HtmlElement>() {
                                                                        let _ = el.style().set_property("display", "none");
                                                                    }
                                                                }
                                                            }
                                                        />
                                                    </Show>
                                                    <p class="text-sm text-gray-700 leading-relaxed m-0 flex-1 line-clamp-3">
                                                        {summary}
                                                    </p>
                                                </div>

                                                <div class="flex items-center justify-between pt-3 border-t border-gray-100">
                                                    <span class="text-sm text-gray-600">
                                                        "健康评分："
                                                        <span class=format!("font-bold {}",
                                                            if score >= 80 { "text-emerald-600" }
                                                            else if score >= 60 { "text-amber-600" }
                                                            else { "text-red-600" }
                                                        )>{score}</span>
                                                    </span>
                                                    <div class="flex items-center gap-2">
                                                        <a
                                                            class="h-8 px-3 text-sm text-gray-700 hover:text-gray-900 hover:bg-gray-100 rounded-lg flex items-center justify-center transition-colors cursor-pointer"
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
                                                            class="h-8 px-3 text-sm text-gray-700 hover:text-gray-900 hover:bg-gray-100 rounded-lg flex items-center justify-center transition-colors bg-transparent border-0 cursor-pointer"
                                                            disabled=move || deleting_local_id.get() == Some(id_value.get_value())
                                                            on:click=move |_| on_delete_local(id_value.get_value())
                                                        >
                                                            {move || if deleting_local_id.get() == Some(id_value.get_value()) { "删除中" } else { "删除" }}
                                                        </button>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            </Show>
                        </div>

                        <div class="px-5 mb-6">
                            <div class="text-center mb-4">
                                <a
                                    href="/profile"
                                    class="text-sm text-emerald-600 font-medium no-underline hover:text-emerald-700"
                                >
                                    "登录后可同步云端历史记录"
                                </a>
                            </div>
                        </div>
                    </div>
                }
            }>
                <div>
                    <div class="px-5 mb-4">
                        <Show when=move || !local_items.get().is_empty() fallback=move || view! {
                            <div class="text-center py-3">
                                <p class="text-xs text-gray-400">"暂无本地专属记录"</p>
                            </div>
                        }>
                            <div class="space-y-3 mb-6">
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
                                        <div class="p-4 shadow-lg border-0 bg-white-95 backdrop-blur-sm rounded-2xl transition-all duration-300">
                                            <div class="flex justify-end mb-3">
                                                <span class="text-xs text-gray-500">{timestamp}</span>
                                            </div>

                                            <div class="flex items-start gap-3 mb-3">
                                                <Show when=move || has_image>
                                                    <img
                                                        src={image_url.clone().unwrap_or_default()}
                                                        alt=""
                                                        class="w-16 h-16 rounded-lg object-cover flex-shrink-0 border border-gray-100"
                                                        loading="lazy"
                                                    />
                                                </Show>
                                                <p class="text-sm text-gray-700 leading-relaxed m-0 flex-1 line-clamp-3">
                                                    {summary}
                                                </p>
                                            </div>

                                            <div class="flex items-center justify-between pt-3 border-t border-gray-100">
                                                <span class="text-sm text-gray-600">
                                                    "健康评分："
                                                    <span class=format!("font-bold {}",
                                                        if score >= 80 { "text-emerald-600" }
                                                        else if score >= 60 { "text-amber-600" }
                                                        else { "text-red-600" }
                                                    )>{score}</span>
                                                </span>
                                                <div class="flex items-center gap-2">
                                                    <a
                                                        class="h-8 px-3 text-sm text-gray-700 hover:text-gray-900 hover:bg-gray-100 rounded-lg flex items-center justify-center transition-colors cursor-pointer"
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
                                                        class="h-8 px-3 text-sm text-gray-700 hover:text-gray-900 hover:bg-gray-100 rounded-lg flex items-center justify-center transition-colors bg-transparent border-0 cursor-pointer"
                                                        disabled=move || deleting_local_id.get() == Some(id_value.get_value())
                                                        on:click=move |_| on_delete_local(id_value.get_value())
                                                    >
                                                        {move || if deleting_local_id.get() == Some(id_value.get_value()) { "删除中" } else { "删除" }}
                                                    </button>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        </Show>
                    </div>

                    <div class="px-5 mb-6">
                        <div class="flex items-center justify-between mb-3 px-1">
                            <h2 class="text-base font-bold text-gray-900 m-0">"云端历史"</h2>
                            <span class="text-xs text-emerald-600 flex items-center gap-1">
                                <div class="w-2 h-2 bg-emerald-500 rounded-full animate-pulse"></div>
                                "已同步"
                            </span>
                        </div>
                        <Show when=move || loading.get() && items.get().is_empty() fallback=move || view! {
                            <Show when=move || !items.get().is_empty() fallback=move || view! {
                                <a href="/" class="block mt-4 p-4 shadow-sm border border-emerald-100 bg-white-50/50 backdrop-blur-sm rounded-2xl transition-all hover:bg-emerald-50 cursor-pointer text-left" style="text-decoration: none;">
                                    <div class="animate-pulse">
                                        <div class="flex items-center justify-between mb-3">
                                            <div class="h-4 bg-emerald-100/50 rounded w-16"></div>
                                            <div class="h-3 bg-emerald-50 rounded w-20"></div>
                                        </div>
                                        <div class="flex items-start gap-3 mb-3">
                                            <div class="w-16 h-16 bg-emerald-50 rounded-lg flex-shrink-0 flex items-center justify-center">
                                                <div class="text-emerald-300 text-3xl font-light mb-1">"+"</div>
                                            </div>
                                            <div class="flex-1 space-y-2 py-1">
                                                <div class="h-4 bg-emerald-50 rounded w-3/4"></div>
                                                <div class="h-3 bg-emerald-50 rounded w-1/2"></div>
                                                <div class="h-3 bg-emerald-50 rounded w-5/6"></div>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="flex items-center justify-center pt-3 border-t border-emerald-50">
                                        <span class="text-sm font-medium text-emerald-600">"云端暂无数据，去分析一条？"</span>
                                    </div>
                                </a>
                            }>
                                <div class="space-y-3">
                                    {move || items.get().into_iter().map(|item| {
                                        let id = item.id;
                                        let item_clone = item.clone();
                                        let summary = item.summary.clone().unwrap_or_default();
                                        let formatted_time = format_iso_datetime(&item.created_at);
                                        let image_url = StoredValue::new(item.image_url.clone());
                                        let resolved_image_url =
                                            StoredValue::new(services::resolve_media_url(&image_url.get_value()));
                                        let has_image = !resolved_image_url.get_value().is_empty();
                                        view! {
                                            <div class="p-4 shadow-lg border-0 bg-white-95 backdrop-blur-sm rounded-2xl transition-all duration-300">
                                                <div class="flex items-center justify-between mb-3">
                                                    <span class="bg-blue-500 text-white border-0 text-xs px-2.5 py-0.5 rounded-full font-medium">
                                                        "云端记录"
                                                    </span>
                                                    <span class="text-xs text-gray-500">{formatted_time}</span>
                                                </div>

                                                <div class="flex items-start gap-3 mb-3">
                                                    <Show when=move || has_image>
                                                        <img
                                                            src={resolved_image_url.get_value()}
                                                            alt=""
                                                            class="w-16 h-16 rounded-lg object-cover flex-shrink-0 border border-gray-100"
                                                            loading="lazy"
                                                            on:error=move |ev| {
                                                                if let Some(target) = ev.target() {
                                                                    if let Ok(el) = target.dyn_into::<web_sys::HtmlElement>() {
                                                                        let _ = el.style().set_property("display", "none");
                                                                    }
                                                                }
                                                            }
                                                        />
                                                    </Show>
                                                    <p class="text-sm text-gray-700 leading-relaxed m-0 flex-1 line-clamp-3">
                                                        {summary}
                                                    </p>
                                                </div>

                                                <div class="flex items-center justify-between pt-3 border-t border-gray-100">
                                                    <span class="text-sm text-gray-600">
                                                        "健康评分："
                                                        <span class=format!("font-bold {}",
                                                            if item.health_score.unwrap_or(0) >= 80 { "text-emerald-600" }
                                                            else if item.health_score.unwrap_or(0) >= 60 { "text-amber-600" }
                                                            else { "text-red-600" }
                                                        )>{item.health_score.unwrap_or(0)}</span>
                                                    </span>
                                                    <div class="flex items-center gap-2">
                                                        <a
                                                            class="h-8 px-3 text-sm text-gray-700 hover:text-gray-900 hover:bg-gray-100 rounded-lg flex items-center justify-center transition-colors cursor-pointer"
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
                                                            class="h-8 px-3 text-sm text-gray-700 hover:text-gray-900 hover:bg-gray-100 rounded-lg flex items-center justify-center transition-colors bg-transparent border-0 cursor-pointer"
                                                            disabled=move || deleting_id.get() == Some(id)
                                                            on:click=move |_| on_delete(id)
                                                        >
                                                            {move || if deleting_id.get() == Some(id) { "删除中" } else { "删除" }}
                                                        </button>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            </Show>
                            <Show when=move || { total.get() > 20 }>
                                <div class="flex justify-center items-center gap-4 mt-6">
                                    <button
                                        class="px-4 py-2 border-2 border-emerald-100 text-emerald-600 rounded-xl bg-white-80 hover:bg-emerald-50 transition-colors disabled:opacity-50"
                                        disabled=move || page.get() <= 1 || loading.get()
                                        on:click=move |_| {
                                            let new_page = page.get() - 1;
                                            page.set(new_page);
                                        }
                                    >
                                        {move || if loading.get() { "加载中…" } else { "上一页" }}
                                    </button>
                                    <span class="text-sm text-gray-500">{move || format!("{} / {}", page.get(), (total.get() + 19) / 20)}</span>
                                    <button
                                        class="px-4 py-2 border-2 border-emerald-100 text-emerald-600 rounded-xl bg-white-80 hover:bg-emerald-50 transition-colors disabled:opacity-50"
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
                            <div class="space-y-3">
                                {(0..3).map(|_| view! {
                                    <article class="rounded-2xl border border-emerald-300 bg-white-95 p-4 shadow-sm animate-pulse">
                                        <div class="flex items-center justify-between mb-3">
                                            <div class="h-5 w-16 rounded-full bg-emerald-100"></div>
                                            <div class="h-3 w-24 rounded bg-emerald-50"></div>
                                        </div>
                                        <div class="flex items-start gap-3 mb-3">
                                            <div class="w-16 h-16 rounded-lg bg-emerald-50"></div>
                                            <div class="flex-1 space-y-2">
                                                <div class="h-3 w-95 rounded bg-emerald-50"></div>
                                                <div class="h-3 w-80 rounded bg-emerald-50"></div>
                                                <div class="h-3 w-70 rounded bg-emerald-50"></div>
                                            </div>
                                        </div>
                                        <div class="flex items-center justify-between pt-3 border-t border-emerald-300">
                                            <div class="h-3 w-45 rounded bg-emerald-50"></div>
                                            <div class="h-3 w-20 rounded bg-emerald-50"></div>
                                        </div>
                                    </article>
                                }).collect_view()}
                            </div>
                        </Show>
                    </div>
                </div>
            </Show>
            </div>
        </section>
    }
}
