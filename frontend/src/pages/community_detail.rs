//! Community detail page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::{use_navigate, use_params_map};

use crate::services;
use crate::stores::ToastLevel;
use crate::utils::{community_share_storage, community_ui, emit_toast};

#[component]
pub fn CommunityDetailPage() -> impl IntoView {
    let navigate = use_navigate();
    let navigate_for_back = navigate.clone();
    let navigate_for_delete = navigate.clone();
    let params = use_params_map();
    let detail = RwSignal::new(None::<shared::CommunityPostDetail>);
    let loading = RwSignal::new(false);
    let last_requested_id = RwSignal::new(None::<uuid::Uuid>);
    let share_records = RwSignal::new(community_share_storage::load_share_records());

    let on_back = move |_| {
        navigate_for_back("/community", Default::default());
    };
    let on_delete_success = Callback::new(move |_| {
        navigate_for_delete("/community", Default::default());
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
        <section class="page page-community-detail">
            <div class="page-topbar">
                <button class="icon-button" on:click=on_back aria-label="返回社区">
                    "←"
                </button>
                <div class="icon-placeholder"></div>
            </div>

            <div class="page-scrollable-content">
                <Show
                    when=move || detail.get().is_some()
                    fallback=move || view! { <p class="hint">"加载中…"</p> }
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
                            <div class="community-detail-card">
                                <div class="community-detail-meta">
                                    <span>{author_label}</span>
                                    <span>{created_at}</span>
                                </div>
                                <Show when=move || can_delete>
                                    <button
                                        class="community-delete-button"
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
                                            let analysis_id = record.analysis_id.clone();
                                            let share_token = record.share_token.clone();
                                            let on_delete_success = on_delete_success.clone();
                                            spawn_local(async move {
                                                match services::delete_community_post(post_id, share_token).await {
                                                    Ok(()) => {
                                                        let _ = community_share_storage::remove_share_record(&analysis_id);
                                                        share_records.set(community_share_storage::load_share_records());
                                                        emit_toast(ToastLevel::Success, "删除成功", "已从社区移除");
                                                        on_delete_success.run(());
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

                                    <Show
                                        when=move || has_image
                                    fallback=move || view! {
                                        <div class="community-detail-score">
                                            <span>"健康评分"</span>
                                            <strong>{card_score}</strong>
                                        </div>
                                        <p class="community-detail-summary">{card_summary.clone()}</p>
                                    }
                                >
                                    <img src={image_url_for_view.clone()} alt="社区分享图片" class="community-detail-image" />
                                </Show>

                                <Show when=move || has_image>
                                    <div class="community-detail-score">
                                        <span>"健康评分"</span>
                                        <strong>{health_score}</strong>
                                    </div>

                                    <p class="community-detail-summary">{summary_text.clone()}</p>
                                </Show>

                                <div class="community-detail-ingredients">
                                    <h3>"配料表"</h3>
                                    <p>{ingredients_raw}</p>
                                </div>
                            </div>
                        }
                    })}
                </Show>
            </div>
        </section>
    }
}
