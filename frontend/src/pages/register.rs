use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use serde_json::json;
use uuid::Uuid;

use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::local_storage;
use crate::utils::preference::{load_preference, merge_preferences, save_preference};
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
            let local_seen = local_storage::get_has_seen_onboarding();
            if local_seen {
                state.has_seen_onboarding.set(true);
            }
            match services::fetch_preferences().await {
                Ok(prefs) => {
                    let mut base = prefs.preferences;
                    if !base.is_object() {
                        base = json!({});
                    }
                    let mut selection_to_set: Option<String> = None;
                    let mut seen_to_set: Option<bool> = None;

                    if let Some(value) = base.get("selection").and_then(|v| v.as_str()) {
                        save_preference(value);
                        state.analysis_preference.set(Some(value.to_string()));
                    } else {
                        selection_to_set = Some(pref.clone());
                    }

                    if let Some(flag) = base.get("has_seen_onboarding").and_then(|v| v.as_bool()) {
                        local_storage::set_has_seen_onboarding(flag);
                        state.has_seen_onboarding.set(flag);
                    } else if local_seen {
                        seen_to_set = Some(true);
                    }

                    if selection_to_set.is_some() || seen_to_set.is_some() {
                        let merged =
                            merge_preferences(base, selection_to_set.as_deref(), seen_to_set);
                        if let Err(err) = services::update_preferences(merged).await {
                            emit_toast(ToastLevel::Error, "同步失败", &err);
                        }
                    }
                }
                Err(err) => {
                    emit_toast(ToastLevel::Warning, "读取偏好失败", &err);
                    let merged = merge_preferences(
                        json!({}),
                        Some(pref.as_str()),
                        if local_seen { Some(true) } else { None },
                    );
                    if let Err(update_err) = services::update_preferences(merged).await {
                        emit_toast(ToastLevel::Error, "同步失败", &update_err);
                    }
                }
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
        <section class="page figma">
            <div class="page-scrollable-content">
                <div class="px-6 pt-12 pb-6 text-center">
                <div class="relative inline-block mb-3">
                    <div class="w-16 h-16 rounded-3xl bg-gradient-to-br from-emerald-500 to-teal-600 text-white text-xl font-bold flex items-center justify-center shadow-xl animate-float">"SI"</div>
                    <div class="absolute -top-1 -right-1 w-7 h-7 rounded-full bg-gradient-to-br from-amber-400 to-orange-400 text-white text-xs font-bold flex items-center justify-center shadow-md animate-bounce-slow">"AI"</div>
                </div>
                <h2 class="m-0 text-2xl font-bold text-gray-900">"Smart Ingredients"</h2>
                <p class="mt-2 mb-0 text-sm text-gray-600">"注册新账号以同步您的数据"</p>
            </div>

            <div class="px-6 pb-4 space-y-4">
                <div class="space-y-1.5">
                    <label class="text-sm font-semibold text-gray-700" for="register-username">"账号"</label>
                    <div>
                        <input
                            id="register-username"
                            class="w-full h-11 rounded-xl border border-emerald-100 bg-white-95 px-3 text-sm text-gray-800 shadow-sm focus:outline-none focus:border-emerald-500"
                            type="text"
                            name="username"
                            autocomplete="username"
                            spellcheck="false"
                            placeholder="4-20 位字母/数字/下划线…"
                            on:input=move |ev| username.set(event_target_value(&ev))
                        />
                    </div>
                </div>

                <div class="space-y-1.5">
                    <label class="text-sm font-semibold text-gray-700" for="register-password">"密码"</label>
                    <div>
                        <input
                            id="register-password"
                            class="w-full h-11 rounded-xl border border-emerald-100 bg-white-95 px-3 text-sm text-gray-800 shadow-sm focus:outline-none focus:border-emerald-500"
                            type="password"
                            name="new-password"
                            autocomplete="new-password"
                            placeholder="至少 6 位…"
                            on:input=move |ev| password.set(event_target_value(&ev))
                        />
                    </div>
                </div>

                <div class="space-y-1.5">
                    <label class="text-sm font-semibold text-gray-700" for="register-confirm-password">"确认密码"</label>
                    <div>
                        <input
                            id="register-confirm-password"
                            class="w-full h-11 rounded-xl border border-emerald-100 bg-white-95 px-3 text-sm text-gray-800 shadow-sm focus:outline-none focus:border-emerald-500"
                            type="password"
                            name="confirm-password"
                            autocomplete="new-password"
                            placeholder="再次输入密码…"
                            on:input=move |ev| confirm_password.set(event_target_value(&ev))
                        />
                    </div>
                </div>

                <div>
                    <p class="text-xs text-amber-700 bg-amber-50 border border-amber-100 rounded-xl px-3 py-2 m-0">"⚠️ 忘记密码无法找回，仅可重新注册"</p>
                </div>

                <button
                    class="w-full h-11 rounded-xl border-0 bg-gradient-to-br from-emerald-500 to-teal-500 text-white text-sm font-semibold shadow-lg transition-all disabled:opacity-50"
                    on:click=on_register
                    disabled=move || registering.get()
                >
                    {move || if registering.get() { "注册中…" } else { "注册账号" }}
                </button>

                <button
                    class="w-full text-sm text-emerald-600 font-medium bg-transparent border-0 py-2"
                    on:click=move |_| {
                        let navigate = navigate.get_value();
                        navigate("/login", Default::default());
                    }
                >
                    "已有账号？去登录"
                </button>
            </div>

            <div class="px-6 pb-8 text-center">
                <button
                    class="text-sm text-gray-500 bg-transparent border-0 py-2"
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
