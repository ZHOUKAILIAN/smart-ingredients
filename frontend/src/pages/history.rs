use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::emit_toast;

#[component]
pub fn HistoryPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = StoredValue::new(use_navigate());
    let loading = create_rw_signal(false);
    let page = create_rw_signal(1_i64);
    let total = create_rw_signal(0_i64);
    let items = create_rw_signal(Vec::<shared::HistoryItem>::new());

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

    view! {
        <section class="page page-history">
            <div class="page-header">
                <div>
                    <h2>"分析历史"</h2>
                    <p class="subtitle">"查看登录后的分析记录"</p>
                </div>
            </div>
            <Show when=move || state.auth_user.get().is_some() fallback=move || {
                view! {
                    <div class="unauth-card">
                        <div class="unauth-header">
                            <h3 class="unauth-title">"登录后可查看历史记录"</h3>
                        </div>
                        <div class="benefit-list">
                            <div class="benefit-item-card">
                                <div class="benefit-check">"✓"</div>
                                <div class="benefit-text">"保存所有分析记录"</div>
                            </div>
                            <div class="benefit-item-card">
                                <div class="benefit-check">"✓"</div>
                                <div class="benefit-text">"随时查看历史结果"</div>
                            </div>
                            <div class="benefit-item-card">
                                <div class="benefit-check">"✓"</div>
                                <div class="benefit-text">"跨设备同步数据"</div>
                            </div>
                        </div>
                        <button class="primary-button" style="width:100%" on:click=move |_| {
                            let navigate = navigate.get_value();
                            navigate("/login", Default::default());
                        }>"去登录"</button>
                    </div>
                }
            }>
                <div class="surface-card">
                    <Show when=move || !items.get().is_empty() fallback=move || view! {
                        <p>"暂无历史记录"</p>
                    }>
                        <ul class="history-list">
                            {move || items.get().into_iter().map(|item| {
                                let id = item.id;
                                view! {
                                    <li class="history-item">
                                        <div>
                                            <p>{format!("分析时间: {}", item.created_at)}</p>
                                            <p>{format!("健康评分: {}", item.health_score.unwrap_or(0))}</p>
                                        </div>
                                        <button class="secondary-cta" on:click=move |_| on_delete(id)>
                                            "删除"
                                        </button>
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
