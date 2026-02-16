use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use serde_json::json;
use uuid::Uuid;

use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::preference::{load_preference, save_preference};
use crate::utils::{emit_toast, local_history};

fn validate_username(username: &str) -> Result<(), &'static str> {
    let trimmed = username.trim();
    let length = trimmed.chars().count();
    if length < 4 || length > 20 {
        return Err("账号长度需为 4-20 位");
    }
    if !trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err("账号仅允许字母、数字和下划线");
    }
    Ok(())
}

fn validate_password(password: &str) -> Result<(), &'static str> {
    if password.trim().len() < 6 {
        return Err("密码长度至少 6 位");
    }
    Ok(())
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = StoredValue::new(use_navigate());
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let confirm_password = RwSignal::new(String::new());
    let registering = RwSignal::new(false);

    let post_auth = {
        let navigate = navigate.clone();
        move |state: AppState, auth: shared::AuthResponse| async move {
            state.auth_user.set(Some(auth.user));
            emit_toast(ToastLevel::Success, "注册成功", "欢迎加入");
            let local_items = local_history::load_local_history();
            if !local_items.is_empty() {
                let should_migrate = web_sys::window()
                    .and_then(|w| {
                        w.confirm_with_message(&format!(
                            "检测到 {} 条本地记录，是否迁移到云端？",
                            local_items.len()
                        ))
                        .ok()
                    })
                    .unwrap_or(false);
                if should_migrate {
                    let ids: Vec<Uuid> = local_items
                        .iter()
                        .filter_map(|item| Uuid::parse_str(&item.id).ok())
                        .collect();
                    if !ids.is_empty() {
                        match services::migrate_local_history(ids).await {
                            Ok(resp) => {
                                if let Err(err) = local_history::clear_local_history() {
                                    emit_toast(ToastLevel::Warning, "清理本地记录失败", &err);
                                }
                                if resp.total_after > 500 {
                                    let delete_count = resp.total_after - 500;
                                    let should_prune = web_sys::window()
                                        .and_then(|w| {
                                            w.confirm_with_message(&format!(
                                                "云端历史记录已超过上限，将删除最旧的 {} 条，是否继续？",
                                                delete_count
                                            ))
                                            .ok()
                                        })
                                        .unwrap_or(false);
                                    if should_prune {
                                        match services::prune_history(delete_count).await {
                                            Ok(pruned) => {
                                                emit_toast(
                                                    ToastLevel::Success,
                                                    "历史记录已清理",
                                                    &format!("已删除 {} 条旧记录", pruned.deleted),
                                                );
                                            }
                                            Err(err) => {
                                                emit_toast(ToastLevel::Error, "清理失败", &err);
                                            }
                                        }
                                    }
                                }
                                emit_toast(
                                    ToastLevel::Success,
                                    "迁移完成",
                                    &format!("成功迁移 {} 条记录", resp.migrated),
                                );
                            }
                            Err(err) => {
                                emit_toast(ToastLevel::Error, "迁移失败", &err);
                            }
                        }
                    }
                }
            }
            let pref = match load_preference() {
                Some(value) => value,
                None => {
                    let value = "normal".to_string();
                    save_preference(&value);
                    value
                }
            };
            if let Err(err) = services::update_preferences(json!({ "selection": pref })).await {
                emit_toast(ToastLevel::Error, "同步失败", &err);
            }
            navigate.get_value()("/", Default::default());
        }
    };

    let on_register = {
        let post_auth = post_auth.clone();
        move |_| {
            let username_value = username.get().trim().to_string();
            let password_value = password.get().trim().to_string();
            let confirm_value = confirm_password.get().trim().to_string();
            if username_value.is_empty() || password_value.is_empty() {
                emit_toast(ToastLevel::Warning, "请输入完整信息", "账号与密码不能为空");
                return;
            }
            if let Err(msg) = validate_username(&username_value) {
                emit_toast(ToastLevel::Warning, "账号格式错误", msg);
                return;
            }
            if let Err(msg) = validate_password(&password_value) {
                emit_toast(ToastLevel::Warning, "密码格式错误", msg);
                return;
            }
            if password_value != confirm_value {
                emit_toast(
                    ToastLevel::Warning,
                    "密码不一致",
                    "请确认两次输入的密码一致",
                );
                return;
            }
            if registering.get() {
                return;
            }
            registering.set(true);
            let state = state.clone();
            spawn_local(async move {
                match services::register(username_value, password_value).await {
                    Ok(auth) => {
                        post_auth(state, auth).await;
                    }
                    Err(err) => {
                        emit_toast(ToastLevel::Error, "注册失败", &err);
                    }
                }
                registering.set(false);
            });
        }
    };

    view! {
        <section class="page page-login figma">
            <div class="page-scrollable-content">
                <div class="login-hero">
                <div class="brand-mark">
                    <div class="brand-icon brand-icon-float">"SI"</div>
                    <div class="brand-ai brand-ai-float">"AI"</div>
                </div>
                <h2 class="login-title">"Smart Ingredients"</h2>
                <p class="login-subtitle">"注册新账号以同步您的数据"</p>
            </div>

            <div class="login-form">
                <div class="input-field-group">
                    <label class="input-label">"账号"</label>
                    <div class="input-wrapper">
                        <input
                            class="custom-input"
                            type="text"
                            placeholder="4-20 位字母/数字/下划线"
                            on:input=move |ev| username.set(event_target_value(&ev))
                        />
                    </div>
                </div>

                <div class="input-field-group">
                    <label class="input-label">"密码"</label>
                    <div class="input-wrapper">
                        <input
                            class="custom-input"
                            type="password"
                            placeholder="至少 6 位"
                            on:input=move |ev| password.set(event_target_value(&ev))
                        />
                    </div>
                </div>

                <div class="input-field-group">
                    <label class="input-label">"确认密码"</label>
                    <div class="input-wrapper">
                        <input
                            class="custom-input"
                            type="password"
                            placeholder="注册时需确认"
                            on:input=move |ev| confirm_password.set(event_target_value(&ev))
                        />
                    </div>
                </div>

                <div class="input-field-group">
                    <p class="hint">"⚠️ 忘记密码无法找回，仅可重新注册"</p>
                </div>

                <button
                    class="primary-button"
                    style="width: 100%;"
                    on:click=on_register
                    disabled=move || registering.get()
                >
                    {move || if registering.get() { "注册中..." } else { "注册账号" }}
                </button>

                <button
                    class="link-button login-switch"
                    on:click=move |_| {
                        let navigate = navigate.get_value();
                        navigate("/login", Default::default());
                    }
                >
                    "已有账号？去登录"
                </button>
            </div>

            <div class="login-footer">
                <button
                    class="link-button"
                    on:click=move |_| {
                        if let Some(window) = web_sys::window() {
                            if let Some(history) = window.history().ok() {
                                let _ = history.back();
                            }
                        }
                    }
                >
                    "跳过，稍后登录 →"
                </button>
            </div>
            </div>
        </section>
    }
}
