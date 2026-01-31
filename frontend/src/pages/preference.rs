use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use serde_json::json;

use crate::components::{
    get_preference_icon, get_preference_label, ConfirmModal, IconArrowLeft, PreferenceCard,
};
use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::emit_toast;
use crate::utils::preference::{load_preference, save_preference};

#[component]
pub fn PreferencePage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = StoredValue::new(use_navigate());

    // 加载当前偏好设置
    let initial_preference = state
        .analysis_preference
        .get()
        .or_else(|| load_preference())
        .unwrap_or_else(|| "none".to_string());

    let preference = RwSignal::new(initial_preference);
    let show_confirm = RwSignal::new(false);

    let on_save_click = move |_| {
        show_confirm.set(true);
    };

    let on_confirm_save = move |_| {
        show_confirm.set(false);
        let pref_value = preference.get();
        save_preference(&pref_value);
        state.analysis_preference.set(Some(pref_value.clone()));

        // 如果已登录，同步到服务器
        if state.auth_user.get().is_some() {
            let val_clone = pref_value.clone();
            spawn_local(async move {
                match services::update_preferences(json!({ "selection": val_clone })).await {
                    Ok(_) => {
                        emit_toast(ToastLevel::Success, "已保存", "偏好设置已保存");
                    }
                    Err(err) => {
                        emit_toast(ToastLevel::Error, "保存失败", &err);
                    }
                }
            });
        } else {
            emit_toast(ToastLevel::Success, "已保存", "偏好设置已保存到本地");
        }

        let nav = navigate.get_value();
        nav("/profile", Default::default());
    };

    let on_cancel = move |_| {
        let nav = navigate.get_value();
        nav("/profile", Default::default());
    };

    let on_cancel_confirm = Callback::new(move |_| {
        show_confirm.set(false);
    });

    let confirm_message = Signal::derive(move || {
        format!(
            "确定保存为「{}」吗？",
            get_preference_label(&preference.get())
        )
    });

    view! {
        <section class="page page-preference">
            <ConfirmModal
                show=show_confirm.into()
                title="保存偏好设置".to_string()
                message=confirm_message
                confirm_text="保存".to_string()
                cancel_text="取消".to_string()
                on_confirm=Callback::new(move |_| on_confirm_save(()))
                on_cancel=on_cancel_confirm
            />

            <div class="page-scrollable-content">
                <button class="preference-back-btn" on:click=on_cancel aria-label="返回">
                    <IconArrowLeft />
                </button>
                <div class="preference-page-container">
                    <PreferenceCard
                        value=Signal::derive(move || preference.get())
                        on_change=Callback::new(move |value: String| {
                            preference.set(value);
                        })
                    />

                    <div class="preference-actions">
                        <button class="secondary-cta" on:click=on_cancel>
                            "取消"
                        </button>
                        <button class="primary-cta" on:click=on_save_click>
                            "保存设置"
                        </button>
                    </div>

                    <div class="preference-hint">
                        "💡 AI 将重点分析您关注的成分（如糖分、添加剂等），并提供针对性建议"
                    </div>
                </div>
            </div>
        </section>
    }
}
