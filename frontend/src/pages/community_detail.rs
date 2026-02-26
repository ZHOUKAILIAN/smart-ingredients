//! Community detail page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::{use_navigate, use_params_map};

use crate::components::ConfirmModal;
use crate::services;
use crate::stores::ToastLevel;
use crate::utils::{community_share_storage, community_ui, emit_toast};

#[derive(Clone)]
struct PendingDelete {
    post_id: uuid::Uuid,
    analysis_id: String,
    share_token: Option<String>,
}

#[component]
pub fn CommunityDetailPage() -> impl IntoView {
    let navigate = use_navigate();
    let navigate_for_delete = navigate.clone();
    let params = use_params_map();
    let detail = RwSignal::new(None::<shared::CommunityPostDetail>);
    let loading = RwSignal::new(false);
    let last_requested_id = RwSignal::new(None::<uuid::Uuid>);
    let share_records = RwSignal::new(community_share_storage::load_share_records());
    let show_confirm = RwSignal::new(false);
    let pending_delete = RwSignal::new(None::<PendingDelete>);

    let on_delete_success = Callback::new(move |_| {
        navigate_for_delete("/community", Default::default());
    });

    let on_confirm_delete = Callback::new(move |_| {
        show_confirm.set(false);
        let Some(pending) = pending_delete.get() else {
            return;
        };
        pending_delete.set(None);
        let share_records = share_records;
        let on_delete_success = on_delete_success.clone();
        spawn_local(async move {
            match services::delete_community_post(pending.post_id, pending.share_token).await {
                Ok(()) => {
                    let _ = community_share_storage::remove_share_record(&pending.analysis_id);
                    share_records.set(community_share_storage::load_share_records());
                    emit_toast(ToastLevel::Success, "删除成功", "已从社区移除");
                    on_delete_success.run(());
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

    create_effect(move |_| {
        let Some(id_raw) = params.get().get("id") else {
            return;
        };
        let Ok(id) = uuid::Uuid::parse_str(&id_raw) else {
            emit_toast(ToastLevel::Error, "加载失败", "无效的帖子编号");
            return;
        };
        if loading.get_untracked() {
            return;
        }
        let last_id = last_requested_id.get_untracked();
        if !community_ui::should_fetch_key(&id, last_id.as_ref()) {
            return;
        }
        loading.set(true);
        last_requested_id.set(Some(id));
        let detail_signal = detail;
        spawn_local(async move {
            match services::fetch_community_post(id).await {
                Ok(response) => {
                    detail_signal.set(Some(response));
                }
                Err(err) => {
                    emit_toast(ToastLevel::Error, "加载失败", &err);
                }
            }
            loading.set(false);
        });
    });

    view! {
        <section class="page figma">
            <div class="flex items-center px-4 py-3 bg-white-80 backdrop-blur-xl sticky top-0 z-10 shadow-sm">
                <a
                    class="mr-3 -ml-2 w-10 h-10 rounded-full border-0 bg-transparent flex items-center justify-center text-gray-700 hover:text-gray-900 hover:bg-gray-100 transition-colors no-underline"
                    href="/community"
                    aria-label="返回社区"
                >
                    "←"
                </a>
                <div class="w-10 h-10"></div>
            </div>

            <ConfirmModal
                show=show_confirm.into()
                title="确认删除".to_string()
                message="确定要删除这条分享吗？删除后无法恢复。"
                confirm_text="删除".to_string()
                cancel_text="取消".to_string()
                on_confirm=on_confirm_delete
                on_cancel=on_cancel_delete
            />

            <div class="page-scrollable-content px-5 py-5">
                <h1 class="sr-only">"社区分享详情"</h1>
                <Show
                    when=move || detail.get().is_some()
                    fallback=move || view! { <p class="text-sm text-gray-600 text-center m-0 py-6">"加载中…"</p> }
                >
                    {move || detail.get().map(|item| {
                        let item_id = item.id;
                        let item_id_string = item.id.to_string();
                        let can_delete = community_ui::should_show_delete_button(
                            community_ui::find_share_record_by_post_id(
                                &share_records.get(),
                                &item_id_string,
                            )
                            .is_some(),
                        );
                        let author_label = item.author_label.clone();
                        let created_at = community_ui::format_community_datetime(&item.created_at);
                        let summary_text = item.summary_text.clone();
                        let ingredients_raw = item.ingredients_raw.clone();
                        let card_summary = item.card_payload.summary.clone();
                        let card_score = item.card_payload.health_score;
                        let health_score = item.health_score;
                        let image_url = item
                            .card_image_url
                            .clone()
                            .map(|url| services::resolve_media_url(&url))
                            .unwrap_or_default();
                        let has_image = !image_url.is_empty();
                        let image_url_for_view = image_url.clone();
                        view! {
                            <div class="rounded-2xl border border-emerald-100 bg-white-95 shadow-lg p-4">
                                <div class="flex items-center justify-between text-xs text-gray-500 mb-3">
                                    <span>{author_label}</span>
                                    <span>{created_at}</span>
                                </div>
                                <Show when=move || can_delete>
                                    <button
                                        class="mb-3 h-8 px-3 text-sm text-red-600 hover:text-red-700 hover:bg-red-50 rounded-lg flex items-center justify-center transition-colors bg-transparent border-0 cursor-pointer"
                                        on:click=move |_| {
                                            let item_id_string = item_id.to_string();
                                            let records = community_share_storage::load_share_records();
                                            let Some(record) = community_ui::find_share_record_by_post_id(
                                                &records,
                                                &item_id_string,
                                            ) else {
                                                emit_toast(ToastLevel::Error, "删除失败", "无效的分享记录");
                                                return;
                                            };
                                            let Ok(post_id) = uuid::Uuid::parse_str(&record.post_id) else {
                                                emit_toast(ToastLevel::Error, "删除失败", "无效的分享记录");
                                                return;
                                            };
                                            pending_delete.set(Some(PendingDelete {
                                                post_id,
                                                analysis_id: record.analysis_id.clone(),
                                                share_token: record.share_token.clone(),
                                            }));
                                            show_confirm.set(true);
                                        }
                                    >
                                        "删除"
                                    </button>
                                </Show>

                                    <Show
                                        when=move || has_image
                                    fallback=move || view! {
                                        <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-50 text-emerald-700 font-medium text-sm">
                                            <span>"健康评分"</span>
                                            <strong class="text-base">{card_score}</strong>
                                        </div>
                                        <p class="mt-3 mb-0 text-sm text-gray-700 leading-relaxed">{card_summary.clone()}</p>
                                    }
                                >
                                    <img
                                        src={image_url_for_view.clone()}
                                        alt="社区分享图片"
                                        class="w-full rounded-xl border border-gray-100 object-cover mb-3"
                                        width="800"
                                        height="600"
                                        fetchpriority="high"
                                    />
                                </Show>

                                <Show when=move || has_image>
                                    <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-50 text-emerald-700 font-medium text-sm">
                                        <span>"健康评分"</span>
                                        <strong class="text-base">{health_score}</strong>
                                    </div>

                                    <p class="mt-3 mb-0 text-sm text-gray-700 leading-relaxed">{summary_text.clone()}</p>
                                </Show>

                                <div class="mt-4 p-3 rounded-xl bg-emerald-50/60 border border-emerald-100">
                                    <h2 class="m-0 mb-2 text-sm font-semibold text-gray-900">"配料表"</h2>
                                    <p class="m-0 text-sm text-gray-700 leading-relaxed whitespace-pre-wrap">{ingredients_raw}</p>
                                </div>
                            </div>
                        }
                    })}
                </Show>
            </div>
        </section>
    }
}
