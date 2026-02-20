//! Community list page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

use crate::services;
use crate::stores::ToastLevel;
use crate::utils::{community_share_storage, community_ui, emit_toast};

#[component]
pub fn CommunityPage() -> impl IntoView {
    let navigate = use_navigate();
    let items = RwSignal::new(Vec::<shared::CommunityPostListItem>::new());
    let total = RwSignal::new(0i64);
    let page = RwSignal::new(1i64);
    let loading = RwSignal::new(false);
    let last_requested_page = RwSignal::new(None::<i64>);
    let share_records = RwSignal::new(community_share_storage::load_share_records());

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

    let on_open_detail = Callback::new({
        let nav = navigate.clone();
        move |id: uuid::Uuid| {
            nav(&format!("/community/{}", id), Default::default());
        }
    });
    let has_pages = move || total.get() > 20;
    let refresh_share_records = move || {
        share_records.set(community_share_storage::load_share_records());
    };
    let page_title = community_ui::community_page_title();

    view! {
        <section class="page page-community">
            <div class="page-scrollable-content">
                <Show when=move || page_title.is_some() fallback=move || ()>
                    <h2 class="page-title">{page_title.unwrap_or_default()}</h2>
                </Show>
                <Show
                    when=move || !items.get().is_empty()
                    fallback=move || view! { <p class="hint">"暂无社区分享"</p> }
                >
                    <ul class="community-list">
                        {move || items.get().into_iter().map(|item| {
                            let id = item.id;
                            let summary = item.summary_text.clone();
                            let author = item.author_label.clone();
                            let created_at = community_ui::format_community_datetime(&item.created_at);
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
                            let items_signal = items;
                            let total_signal = total;
                            let refresh_share_records = refresh_share_records;

                            view! {
                                <li class="community-card" on:click=move |_| on_open_detail.run(id)>
                                    <div class="community-card-main">
                                        <div class="community-card-content">
                                            <div class="community-card-meta">
                                                <span>{author}</span>
                                                <span>{created_at}</span>
                                            </div>
                                            <p class="community-summary">{summary}</p>
                                        </div>
                                        <div class="community-thumb">
                                            <Show when=move || has_image fallback=move || view! {
                                                <div class="community-thumb-placeholder">"📷"</div>
                                            }>
                                                <img
                                                    src={image_url_for_view.clone()}
                                                    alt="社区分享图片"
                                                    class="community-thumb-image"
                                                />
                                            </Show>
                                        </div>
                                    </div>
                                    <div class="community-card-footer">
                                        <span class="community-score-label">"健康评分"</span>
                                        <span class="community-score-value">{score}</span>
                                        <Show when=move || can_delete>
                                            <button
                                                class="community-delete-button"
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
                                                    let Ok(post_id) = uuid::Uuid::parse_str(&record.post_id) else {
                                                        emit_toast(ToastLevel::Error, "删除失败", "无效的分享记录");
                                                        return;
                                                    };
                                                    let analysis_id = record.analysis_id.clone();
                                                    let share_token = record.share_token.clone();
                                                    spawn_local(async move {
                                                        match services::delete_community_post(post_id, share_token).await {
                                                            Ok(()) => {
                                                                let _ = community_share_storage::remove_share_record(&analysis_id);
                                                                items_signal.update(|list| {
                                                                    list.retain(|item| item.id != id);
                                                                });
                                                                total_signal.update(|value| {
                                                                    if *value > 0 {
                                                                        *value -= 1;
                                                                    }
                                                                });
                                                                refresh_share_records();
                                                                emit_toast(ToastLevel::Success, "删除成功", "已从社区移除");
                                                            }
                                                            Err(err) => {
                                                                emit_toast(ToastLevel::Error, "删除失败", &err);
                                                            }
                                                        }
                                                    });
                                                }
                                            >
                                                "删除"
                                            </button>
                                        </Show>
                                    </div>
                                </li>
                            }
                        }).collect_view()}
                    </ul>
                </Show>

                <Show when=has_pages>
                    <div class="community-pagination">
                        <button
                            class="secondary-cta"
                            disabled=move || page.get() <= 1
                            on:click=move |_| page.set(page.get() - 1)
                        >
                            "上一页"
                        </button>
                        <span>{move || format!("第 {} 页 / 共 {} 条", page.get(), total.get())}</span>
                        <button
                            class="secondary-cta"
                            disabled=move || page.get() * 20 >= total.get()
                            on:click=move |_| page.set(page.get() + 1)
                        >
                            "下一页"
                        </button>
                    </div>
                </Show>
            </div>
        </section>
    }
}
