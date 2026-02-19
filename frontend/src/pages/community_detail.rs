//! Community detail page

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use leptos_router::params::use_params_map;

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
        let nav = navigate.get_value();
        nav("/community", Default::default());
    };

    create_effect(move |_| {
        let Some(id_raw) = params.get().get("id").cloned() else {
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
                        let image_url = item
                            .card_image_url
                            .clone()
                            .map(|url| services::resolve_media_url(&url))
                            .unwrap_or_default();
                        view! {
                            <div class="community-detail-card">
                                <div class="community-detail-meta">
                                    <span>{item.author_label}</span>
                                    <span>{item.created_at}</span>
                                </div>

                                <Show when=move || !image_url.is_empty()>
                                    <img src={image_url} alt="社区分享图片" class="community-detail-image" />
                                </Show>

                                <div class="community-detail-score">
                                    <span>"健康评分"</span>
                                    <strong>{item.health_score}</strong>
                                </div>

                                <p class="community-detail-summary">{item.summary_text}</p>

                                <div class="community-detail-ingredients">
                                    <h3>"配料表"</h3>
                                    <p>{item.ingredients_raw}</p>
                                </div>
                            </div>
                        }
                    })}
                </Show>
            </div>
        </section>
    }
}
