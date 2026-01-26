use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use serde_json::json;

use crate::components::IconArrowLeft;
use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::emit_toast;
use crate::utils::preference::load_preference;

#[component]
pub fn LoginPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();
    let navigate_for_back = navigate.clone();
    let navigate_for_skip = navigate.clone();
    let phone = create_rw_signal(String::new());
    let code = create_rw_signal(String::new());
    let sending = create_rw_signal(false);
    let verifying = create_rw_signal(false);
    let debug_code = create_rw_signal(None::<String>);

    let on_send = move |_| {
        let phone_value = phone.get().trim().to_string();
        if phone_value.is_empty() {
            emit_toast(ToastLevel::Warning, "请输入手机号", "手机号不能为空");
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
            <div class="page-topbar login-topbar">
                <button
                    class="icon-button login-back"
                    on:click=move |_| navigate_for_back("/", Default::default())
                    aria-label="返回首页"
                >
                    <IconArrowLeft />
                </button>
                <h1 class="page-topbar-title">"欢迎回来"</h1>
                <div class="icon-placeholder"></div>
            </div>

            <div class="login-hero">
                <div class="brand-mark">
                    <div class="brand-icon">"SI"</div>
                    <div class="brand-ai">"AI"</div>
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
                    on:click=move |_| navigate_for_skip("/", Default::default())
                >
                    "跳过，稍后登录 →"
                </button>
            </div>
        </section>
    }
}
