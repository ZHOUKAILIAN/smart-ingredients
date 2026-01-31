use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use serde_json::json;

use crate::components::IconArrowLeft;
use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::{emit_toast, local_history};
use crate::utils::preference::load_preference;
use uuid::Uuid;

#[component]
pub fn LoginPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();
    let phone = RwSignal::new(String::new());
    let code = RwSignal::new(String::new());
    let sending = RwSignal::new(false);
    let verifying = RwSignal::new(false);
    let debug_code = RwSignal::new(None::<String>);

    // 验证中国手机号格式
    let validate_phone = |phone: &str| -> bool {
        // 中国手机号：1开头，第二位是3-9，总共11位数字
        if phone.len() != 11 {
            return false;
        }
        let chars: Vec<char> = phone.chars().collect();
        if chars[0] != '1' {
            return false;
        }
        if !('3'..='9').contains(&chars[1]) {
            return false;
        }
        phone.chars().all(|c| c.is_ascii_digit())
    };

    let on_send = move |_| {
        let phone_value = phone.get().trim().to_string();
        if phone_value.is_empty() {
            emit_toast(ToastLevel::Warning, "请输入手机号", "手机号不能为空");
            return;
        }
        if !validate_phone(&phone_value) {
            emit_toast(ToastLevel::Warning, "手机号格式错误", "请输入正确的中国大陆手机号");
            return;
        }
        if sending.get() {
            return;
        }
        sending.set(true);
        spawn_local(async move {
            match services::send_sms(phone_value).await {
                Ok(resp) => {
                    if let Some(code) = resp.debug_code {
                        debug_code.set(Some(code));
                    }
                    emit_toast(ToastLevel::Success, "验证码已发送", "请注意查收短信");
                }
                Err(err) => {
                    emit_toast(ToastLevel::Error, "发送失败", &err);
                }
            }
            sending.set(false);
        });
    };

    let on_verify = {
        let navigate = navigate.clone();
        move |_| {
            let phone_value = phone.get().trim().to_string();
            let code_value = code.get().trim().to_string();
            if phone_value.is_empty() || code_value.is_empty() {
                emit_toast(ToastLevel::Warning, "请输入完整信息", "手机号与验证码不能为空");
                return;
            }
            if !validate_phone(&phone_value) {
                emit_toast(ToastLevel::Warning, "手机号格式错误", "请输入正确的中国大陆手机号");
                return;
            }
            if verifying.get() {
                return;
            }
            verifying.set(true);
            let state = state.clone(); // AppState is Copy now, so this is just Copy. But explicit clone is fine.
            let navigate = navigate.clone();
            spawn_local(async move {
                match services::verify_sms(phone_value, code_value).await {
                    Ok(auth) => {
                        state.auth_user.set(Some(auth.user));
                        emit_toast(ToastLevel::Success, "登录成功", "欢迎回来");
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
                        if let Some(pref) = load_preference() {
                            let _ = services::update_preferences(json!({ "selection": pref })).await;
                            navigate("/", Default::default());
                        } else {
                            navigate("/?modal=preference", Default::default());
                        }
                    }
                    Err(err) => {
                        emit_toast(ToastLevel::Error, "登录失败", &err);
                    }
                }
                verifying.set(false);
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
                <p class="login-subtitle">"登录以同步您的数据"</p>
            </div>

            <div class="login-form">
                <div class="input-field-group">
                    <label class="input-label">"手机号"</label>
                    <div class="input-wrapper">
                        <input
                            class="custom-input"
                            type="tel"
                            placeholder="请输入手机号"
                            on:input=move |ev| phone.set(event_target_value(&ev))
                        />
                    </div>
                </div>

                <div class="input-field-group">
                    <label class="input-label">"验证码"</label>
                    <div class="input-wrapper">
                        <input
                            class="custom-input"
                            type="text"
                            placeholder="请输入验证码"
                            on:input=move |ev| code.set(event_target_value(&ev))
                        />
                        <button 
                            class="btn-send-code" 
                            on:click=on_send 
                            disabled=move || sending.get()
                        >
                            {move || if sending.get() { "发送中..." } else { "获取验证码" }}
                        </button>
                    </div>
                </div>

                <button 
                    class="primary-button" 
                    style="width: 100%;"
                    on:click=on_verify 
                    disabled=move || verifying.get()
                >
                    {move || if verifying.get() { "登录中..." } else { "登录" }}
                </button>

                <Show when=move || debug_code.get().is_some()>
                    <div class="focus-hint" style="text-align: center; margin-top: 10px;">
                        {move || format!("调试验证码: {}", debug_code.get().unwrap_or_default())}
                    </div>
                </Show>
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
