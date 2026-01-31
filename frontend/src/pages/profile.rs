use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::emit_toast;

fn mask_phone(phone: &str) -> String {
    if phone.len() >= 11 {
        format!("{}****{}", &phone[..3], &phone[7..])
    } else {
        phone.to_string()
    }
}

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
            <div class="page-scrollable-content">
                <Show when=move || state.auth_user.get().is_some() fallback=move || {
                view! {
                    // 未登录状态
                    <div class="profile-container">
                        <div class="profile-header">
                            <div class="profile-avatar">
                                <span class="avatar-icon">"👤"</span>
                            </div>
                            <div class="profile-name">"未登录"</div>
                            <button class="profile-login-btn" on:click=move |_| {
                                let navigate = navigate.get_value();
                                navigate("/login", Default::default());
                            }>"登录 / 注册"</button>
                        </div>

                        <div class="profile-menu">
                            <button class="menu-item" on:click=move |_| {
                                let navigate = navigate.get_value();
                                navigate("/preference", Default::default());
                            }>
                                <span class="menu-icon">"⚙️"</span>
                                <span class="menu-label">"偏好设置"</span>
                                <span class="menu-arrow">"›"</span>
                            </button>
                        </div>
                    </div>
                }
            }>
                // 已登录状态
                {move || {
                    state.auth_user.get().map(|user| {
                        let phone_display = mask_phone(&user.phone_masked);
                        view! {
                            <div class="profile-container">
                                <div class="profile-header">
                                    <div class="profile-avatar">
                                        <span class="avatar-icon">"👤"</span>
                                    </div>
                                    <div class="profile-name">{phone_display}</div>
                                    <div class="profile-stats">
                                        <span class="stat-item">
                                            <span class="stat-label">"分析次数"</span>
                                            <span class="stat-value">{user.analysis_count}</span>
                                        </span>
                                    </div>
                                </div>

                                <div class="profile-menu">
                                    <button class="menu-item" on:click=move |_| {
                                        let navigate = navigate.get_value();
                                        navigate("/preference", Default::default());
                                    }>
                                        <span class="menu-icon">"⚙️"</span>
                                        <span class="menu-label">"偏好设置"</span>
                                        <span class="menu-arrow">"›"</span>
                                    </button>

                                    <div class="menu-divider"></div>

                                    <button class="menu-item" on:click=move |_| on_logout.run(())>
                                        <span class="menu-icon">"🚪"</span>
                                        <span class="menu-label">"退出登录"</span>
                                        <span class="menu-arrow">"›"</span>
                                    </button>

                                    <button class="menu-item danger" on:click=move |_| on_delete.run(())>
                                        <span class="menu-icon">"⚠️"</span>
                                        <span class="menu-label">"注销账号"</span>
                                        <span class="menu-arrow">"›"</span>
                                    </button>
                                </div>
                            </div>
                        }
                    })
                }}
            </Show>
            </div>
        </section>
    }
}
