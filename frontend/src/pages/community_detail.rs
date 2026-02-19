//! Community detail page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::{use_navigate, use_params_map};

use crate::services;
use crate::stores::ToastLevel;
use crate::utils::emit_toast;

#[component]
pub fn CommunityDetailPage() -> impl IntoView {
    let navigate = use_navigate();
    let params = use_params_map();
    let detail = RwSignal::new(None::<shared::CommunityPostDetail>);
    let loading = RwSignal::new(false);

    let on_back = move |_| {
        navigate("/community", Default::default());
    };

    create_effect(move |_| {
        let Some(id_raw) = params.get().get("id") else {
            return;
        };
        let Ok(id) = uuid::Uuid::parse_str(&id_raw) else {
            emit_toast(ToastLevel::Error, "加载失败", "无效的帖子编号");
            return;
        };
        if loading.get() {
            return;
        }
        loading.set(true);
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
                        let author_label = item.author_label.clone();
                        let created_at = item.created_at.clone();
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
