use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use serde_json::json;

use crate::components::{get_preference_label, ConfirmModal, PreferenceCard};
use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::emit_toast;
use crate::utils::preference::save_preference;

const ONBOARDING_STEPS: &[&str] = &[
    "选择您的关注人群",
    "AI 会优先解读相关风险",
    "结果页会对准关注点提示",
];

#[component]
pub fn OnboardingPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = StoredValue::new(use_navigate());

    let initial_preference = state
        .analysis_preference
        .get()
        .unwrap_or_else(|| "normal".to_string());

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

        if state.auth_user.get().is_some() {
            let val_clone = pref_value.clone();
            spawn_local(async move {
                match services::update_preferences(json!({ "selection": val_clone })).await {
                    Ok(_) => {
                        emit_toast(ToastLevel::Success, "已保存", "人群设置已保存");
                    }
                    Err(err) => {
                        emit_toast(ToastLevel::Error, "保存失败", &err);
                    }
                }
            });
        } else {
            emit_toast(ToastLevel::Success, "已保存", "人群设置已保存到本地");
        }

        let nav = navigate.get_value();
        nav("/?view=scan", Default::default());
    };

    let on_skip = move |_| {
        preference.set("normal".to_string());
        on_confirm_save(());
    };

    let on_cancel_confirm = Callback::new(move |_| {
        show_confirm.set(false);
    });

    let confirm_message = Signal::derive(move || {
        format!("确定选择「{}」吗？", get_preference_label(&preference.get()))
    });

    view! {
        <section class="page page-preference">
            <ConfirmModal
                show=show_confirm.into()
                title="确认人群定位".to_string()
                message=confirm_message
                confirm_text="确认".to_string()
                cancel_text="返回修改".to_string()
                on_confirm=Callback::new(move |_| on_confirm_save(()))
                on_cancel=on_cancel_confirm
            />

            <div class="page-scrollable-content">
                <div class="preference-page-container">
                    <div class="preference-intro">
                        <h2>"欢迎使用 Smart Ingredients"</h2>
                        <p>"先选人群定位，AI 会把分析结果对准你的关注点"</p>
                    </div>

                    <div class="analysis-list">
                        {ONBOARDING_STEPS
                            .iter()
                            .map(|step| view! { <div class="analysis-item">{*step}</div> })
                            .collect_view()}
                    </div>

                    <PreferenceCard
                        value=Signal::derive(move || preference.get())
                        on_change=Callback::new(move |value: String| {
                            preference.set(value);
                        })
                    />

                    <div class="preference-actions">
                        <button class="secondary-cta" on:click=on_skip>
                            "先用普通人群"
                        </button>
                        <button class="primary-cta" on:click=on_save_click>
                            "确认并开始"
                        </button>
                    </div>

                    <div class="preference-hint">
                        "💡 之后可在个人中心随时修改"
                    </div>
                </div>
            </div>
        </section>
    }
}
