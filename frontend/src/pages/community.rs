//! Community list page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::{use_location, use_navigate};
use web_sys::MouseEvent;

use crate::components::ConfirmModal;
use crate::services;
use crate::stores::ToastLevel;
use crate::utils::navigation::build_full_path;
use crate::utils::{community_share_storage, community_ui, emit_toast};

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

fn is_modified_click(ev: &MouseEvent) -> bool {
    ev.meta_key() || ev.ctrl_key() || ev.shift_key() || ev.alt_key() || ev.button() != 0
}

#[derive(Clone)]
struct PendingDelete {
    post_id: uuid::Uuid,
    analysis_id: String,
    share_token: Option<String>,
}

#[component]
pub fn CommunityPage() -> impl IntoView {
    let navigate = use_navigate();
    let navigate_for_page = navigate.clone();
    let location = use_location();
    let items = RwSignal::new(Vec::<shared::CommunityPostListItem>::new());
    let total = RwSignal::new(0i64);
    let page = RwSignal::new(1i64);
    let loading = RwSignal::new(false);
    let last_requested_page = RwSignal::new(None::<i64>);
    let share_records = RwSignal::new(community_share_storage::load_share_records());
    let show_confirm = RwSignal::new(false);
    let pending_delete = RwSignal::new(None::<PendingDelete>);

    let fetch_page = move |page_number: i64| {
        if loading.get_untracked() {
            return;
        }
        if !community_ui::should_fetch_page(page_number, last_requested_page.get_untracked()) {
            return;
        }
        loading.set(true);
        last_requested_page.set(Some(page_number));
        let items_signal = items;
        let total_signal = total;
        spawn_local(async move {
            match services::fetch_community_posts(page_number, 20).await {
                Ok(response) => {
                    items_signal.set(response.items);
                    total_signal.set(response.total);
                }
                Err(err) => {
                    emit_toast(ToastLevel::Error, "加载失败", &err);
                }
            }
            loading.set(false);
        });
    };

    create_effect(move |_| {
        let current = page.get();
        fetch_page(current);
    });

    create_effect(move |_| {
        let search = location.search.get();
        let new_page = page_from_search(&search);
        if page.get() != new_page {
            page.set(new_page);
        }
    });

    create_effect(move |_| {
        let current_page = page.get();
        let path = location.pathname.get_untracked();
        let search = location.search.get_untracked();
        let target = build_page_target(path.as_str(), current_page);
        let current = build_full_path(path.as_str(), search.as_str());
        if current != target {
            let nav = navigate_for_page.clone();
            nav(&target, Default::default());
        }
    });

    let has_pages = move || total.get() > 20;
    let page_title = community_ui::community_page_title();
    let on_open_detail = Callback::new({
        let nav = navigate.clone();
        move |id: uuid::Uuid| {
            nav(&format!("/community/{}", id), Default::default());
        }
    });

    let on_confirm_delete = Callback::new(move |_| {
        show_confirm.set(false);
        let Some(pending) = pending_delete.get() else {
            return;
        };
        pending_delete.set(None);
        let items_signal = items;
        let total_signal = total;
        let share_records = share_records;
        spawn_local(async move {
            match services::delete_community_post(pending.post_id, pending.share_token).await {
                Ok(()) => {
                    let _ = community_share_storage::remove_share_record(&pending.analysis_id);
                    items_signal.update(|list| {
                        list.retain(|item| item.id != pending.post_id);
                    });
                    total_signal.update(|value| {
                        if *value > 0 {
                            *value -= 1;
                        }
                    });
                    share_records.set(community_share_storage::load_share_records());
                    emit_toast(ToastLevel::Success, "删除成功", "已从社区移除");
                }
                Err(err) => {
                    emit_toast(ToastLevel::Error, "删除失败", &err);
                }
            }
        });
    });

    let on_cancel_delete = Callback::new(move |_| {
        show_confirm.set(false);
        pending_delete.set(None);
    });

    view! {
        <section class="page figma">
            <ConfirmModal
                show=show_confirm.into()
                title="确认删除".to_string()
                message="确定要删除这条分享吗？删除后无法恢复。"
                confirm_text="删除".to_string()
                cancel_text="取消".to_string()
                on_confirm=on_confirm_delete
                on_cancel=on_cancel_delete
            />
            <div class="page-scrollable-content pb-20">
                <div class="px-5 pb-4">
                    <Show when=move || page_title.is_some() fallback=move || ()>
                        <h1 class="text-2xl font-bold text-gray-900 m-0">{page_title.unwrap_or_default()}</h1>
                    </Show>
                </div>

                <div class="px-5 mb-6">
                    <Show
                        when=move || loading.get() && items.get().is_empty()
                        fallback=move || view! {
                            <Show
                                when=move || !items.get().is_empty()
                                fallback=move || view! {
                                    <a href="/" class="block mt-4 p-4 shadow-sm border border-emerald-100 bg-white-50/50 backdrop-blur-sm rounded-2xl transition-all hover:bg-emerald-50 cursor-pointer text-left" style="text-decoration: none;">
                                        <div class="text-center py-3">
                                            <div class="text-2xl leading-none text-emerald-500">"✦"</div>
                                            <h3 class="m-0 mt-2 text-base font-semibold text-gray-900">"社区还没有分享"</h3>
                                            <p class="m-0 mt-1 text-sm text-gray-500">"去完成一次分析并分享，成为第一个发布者"</p>
                                        </div>
                                    </a>
                                }
                            >
                                <div class="space-y-3">
                                    {move || items.get().into_iter().map(|item| {
                                        let id = item.id;
                                        let summary = item.summary_text.clone();
                                        let author = item.author_label.clone();
                                        let created_at = community_ui::format_community_datetime(&item.created_at);
                                        let detail_href = format!("/community/{}", id);
                                        let on_open_detail = on_open_detail;
                                        let image_url = item
                                            .card_image_url
                                            .clone()
                                            .map(|url| services::resolve_media_url(&url))
                                            .unwrap_or_default();
                                        let has_image = !image_url.is_empty();
                                        let image_url_for_view = image_url.clone();
                                        let score = item.health_score;
                                        let can_delete = community_ui::should_show_delete_button(
                                            community_ui::find_share_record_by_post_id(
                                                &share_records.get(),
                                                &id.to_string(),
                                            )
                                            .is_some(),
                                        );
                                        view! {
                                            <div class="p-4 shadow-lg border-0 bg-white-95 backdrop-blur-sm rounded-2xl transition-all duration-300">
                                                <div class="flex items-center justify-between mb-3">
                                                    <span class="bg-blue-500 text-white border-0 text-xs px-2.5 py-0.5 rounded-full font-medium">
                                                        {author}
                                                    </span>
                                                    <span class="text-xs text-gray-500">{created_at}</span>
                                                </div>

                                                <a
                                                    href=detail_href
                                                    class="block cursor-pointer text-gray-900 no-underline"
                                                    on:click=move |ev: MouseEvent| {
                                                        if is_modified_click(&ev) {
                                                            return;
                                                        }
                                                        ev.prevent_default();
                                                        on_open_detail.run(id);
                                                    }
                                                >
                                                    <div class="flex items-start gap-3 mb-3">
                                                        <Show when=move || has_image>
                                                            <img
                                                                src={image_url_for_view.clone()}
                                                                alt="社区分享图片"
                                                                class="w-16 h-16 rounded-lg object-cover flex-shrink-0 border border-gray-100"
                                                                loading="lazy"
                                                            />
                                                        </Show>
                                                        <p class="text-sm text-gray-700 leading-relaxed m-0 flex-1 line-clamp-3">
                                                            {summary}
                                                        </p>
                                                    </div>
                                                </a>

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
                                                        <Show when=move || can_delete>
                                                            <button
                                                                class="h-8 px-3 text-sm text-red-600 hover:text-red-700 hover:bg-red-50 rounded-lg flex items-center justify-center transition-colors bg-transparent border-0 cursor-pointer"
                                                                on:click=move |ev| {
                                                                    ev.stop_propagation();
                                                                    let records = community_share_storage::load_share_records();
                                                                    let Some(record) = community_ui::find_share_record_by_post_id(
                                                                        &records,
                                                                        &id.to_string(),
                                                                    ) else {
                                                                        emit_toast(ToastLevel::Error, "删除失败", "无效的分享记录");
                                                                        return;
                                                                    };
                                                                    pending_delete.set(Some(PendingDelete {
                                                                        post_id: id,
                                                                        analysis_id: record.analysis_id.clone(),
                                                                        share_token: record.share_token.clone(),
                                                                    }));
                                                                    show_confirm.set(true);
                                                                }
                                                            >
                                                                "删除"
                                                            </button>
                                                        </Show>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            </Show>
                        }
                    >
                        <div class="space-y-3">
                            {(0..3).map(|_| view! {
                                <article class="rounded-2xl border border-emerald-100 bg-white-95 p-4 shadow-sm animate-pulse">
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
                                    <div class="flex items-center justify-between pt-3 border-t border-emerald-100">
                                        <div class="h-3 w-45 rounded bg-emerald-50"></div>
                                        <div class="h-3 w-20 rounded bg-emerald-50"></div>
                                    </div>
                                </article>
                            }).collect_view()}
                        </div>
                    </Show>

                    <Show when=has_pages>
                        <div class="flex justify-center items-center gap-4 mt-6">
                            <button
                                class="px-4 py-2 border-2 border-emerald-100 text-emerald-600 rounded-xl bg-white-80 hover:bg-emerald-50 transition-colors disabled:opacity-50 cursor-pointer"
                                disabled=move || page.get() <= 1
                                on:click=move |_| page.set(page.get() - 1)
                            >
                                "上一页"
                            </button>
                            <span class="text-sm text-gray-500">{move || format!("{} / {}", page.get(), (total.get() + 19) / 20)}</span>
                            <button
                                class="px-4 py-2 border-2 border-emerald-100 text-emerald-600 rounded-xl bg-white-80 hover:bg-emerald-50 transition-colors disabled:opacity-50 cursor-pointer"
                                disabled=move || page.get() * 20 >= total.get()
                                on:click=move |_| page.set(page.get() + 1)
                            >
                                "下一页"
                            </button>
                        </div>
                    </Show>
                </div>
            </div>
        </section>
    }
}
