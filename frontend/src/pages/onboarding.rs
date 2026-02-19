use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use serde_json::json;

use crate::components::{get_preference_label, ConfirmModal, PREFERENCE_OPTIONS};
use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::emit_toast;
use crate::utils::preference::{load_preference, save_preference};

#[component]
pub fn OnboardingPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = StoredValue::new(use_navigate());

    let initial_preference = state
        .analysis_preference
        .get()
        .unwrap_or_else(|| "normal".to_string());

    // If user already has a preference, they're re-visiting from profile settings
    let is_returning = state.analysis_preference.get().is_some() || load_preference().is_some();

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

        // Reset tab memory so bottom-nav doesn't return to /onboarding
        state.last_home_path.set("/".to_string());
        state.last_profile_path.set("/profile".to_string());

        if is_returning {
            // Came from profile settings — go back to profile
            navigate.get_value()("/profile", Default::default());
        } else {
            // First-time onboarding — go to scan mode
            state.open_in_scan_mode.set(true);
            navigate.get_value()("/", Default::default());
        }
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
                <div class="onboarding-compact">
                    <div class="onboarding-header">
                        <h2>"选人群，对准你的关注点"</h2>
                        <p class="onboarding-flow-hint">
                            "📋 选人群 → 📸 拍配料表 → ✅ 看结果"
                        </p>
                    </div>

                    <div class="onboarding-grid">
                        {PREFERENCE_OPTIONS
                            .iter()
                            .map(|opt| {
                                let value = opt.value;
                                let is_selected = move || preference.get() == value;
                                view! {
                                    <button
                                        class="onboarding-option"
                                        class:selected=is_selected
                                        on:click=move |_| {
                                            preference.set(value.to_string());
                                        }
                                    >
                                        <span class="onboarding-option-icon">{opt.icon}</span>
                                        <span class="onboarding-option-label">{opt.label}</span>
                                    </button>
                                }
                            })
                            .collect_view()}
                    </div>

                    <div class="onboarding-actions">
                        <button class="secondary-cta" on:click=on_skip>
                            "跳过"
                        </button>
                        <button class="primary-cta" on:click=on_save_click>
                            "确认并开始"
                        </button>
                    </div>
                </div>
            </div>
        </section>
    }
}
