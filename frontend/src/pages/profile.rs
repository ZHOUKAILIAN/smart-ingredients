use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::emit_toast;

#[component]
pub fn ProfilePage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = StoredValue::new(use_navigate());

    let state_for_logout = state.clone();
    let on_logout = Callback::new(move |_| {
        let state = state_for_logout.clone();
        let navigate = navigate.get_value();
        spawn_local(async move {
            let _ = services::logout().await;
            state.auth_user.set(None);
            emit_toast(ToastLevel::Success, "已退出", "已退出登录");
            navigate("/", Default::default());
        });
    });

    let state_for_delete = state.clone();
    let on_delete = Callback::new(move |_| {
        let confirmed = web_sys::window()
            .and_then(|w| w.confirm_with_message("确认注销账号吗？该操作不可恢复。").ok())
            .unwrap_or(false);
        if !confirmed {
            return;
        }
        let state = state_for_delete.clone();
        let navigate = navigate.get_value();
        spawn_local(async move {
            match services::delete_account().await {
                Ok(()) => {
                    state.auth_user.set(None);
                    emit_toast(ToastLevel::Success, "已注销", "账号已注销");
                    navigate("/", Default::default());
                }
                Err(err) => {
                    emit_toast(ToastLevel::Error, "注销失败", &err);
                }
            }
        });
    });

    view! {
        <section class="page page-profile">
            <div class="page-header">
                <div>
                    <h2>"个人中心"</h2>
                    <p class="subtitle">"管理个人信息"</p>
                </div>
            </div>
            <Show when=move || state.auth_user.get().is_some() fallback=move || {
                view! {
                    <div class="unauth-card">
                        <div class="unauth-header">
                            <h3 class="unauth-title">"登录使用更多功能"</h3>
                            <p class="unauth-subtitle">"注册账号，体验完整服务"</p>
                        </div>
                        <div class="benefit-list">
                            <div class="benefit-item-card">
                                <div class="benefit-check">"✓"</div>
                                <div class="benefit-text">"查看分析统计"</div>
                            </div>
                            <div class="benefit-item-card">
                                <div class="benefit-check">"✓"</div>
                                <div class="benefit-text">"管理偏好设置"</div>
                            </div>
                            <div class="benefit-item-card">
                                <div class="benefit-check">"✓"</div>
                                <div class="benefit-text">"同步跨设备数据"</div>
                            </div>
                        </div>
                        <button class="primary-button" style="width:100%" on:click=move |_| {
                            let navigate = navigate.get_value();
                            navigate("/login", Default::default());
                        }>"登录 / 注册"</button>
                        
                        <div style="margin-top: 16px; display: flex; flex-direction: column; gap: 8px;">
                            <button class="link-button" on:click=move |_| {
                                let navigate = navigate.get_value();
                                navigate("/?modal=preference", Default::default());
                            }>"偏好设置"</button>
                        </div>
                    </div>
                }
            }>
                <div class="surface-card">
                    <Show
                        when=move || state.auth_user.get().is_some()
                        fallback=move || view! { <p>"加载中..."</p> }
                    >
                        {move || {
                            state.auth_user.get().map(|user| {
                                view! {
                                    <div class="profile-info">
                                        <p>{format!("手机号: {}", user.phone_masked)}</p>
                                        <p>{format!("注册时间: {}", user.created_at)}</p>
                                        <p>{format!("分析次数: {}", user.analysis_count)}</p>
                                    </div>
                                }
                            })
                        }}
                    </Show>
                    <div class="profile-actions">
                        <button class="secondary-cta" on:click=move |_| {
                            let navigate = navigate.get_value();
                            navigate("/?modal=preference", Default::default());
                        }>"偏好设置"</button>
                        <button class="secondary-cta" on:click=move |_| on_logout.run(())>
                            "退出登录"
                        </button>
                        <button class="primary-cta" on:click=move |_| on_delete.run(())>
                            "注销账号"
                        </button>
                    </div>
                </div>
            </Show>
        </section>
    }
}
