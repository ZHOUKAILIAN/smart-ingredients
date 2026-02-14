use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use serde_json::json;

use crate::components::{get_preference_label, ConfirmModal, PreferenceCard};
use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::emit_toast;
use crate::utils::preference::save_preference;

const ONBOARDING_STEPS: &[(&str, &str)] = &[
    ("选人群", "告诉我们你更在意哪类风险"),
    ("拍配料表", "上传清晰配料表，AI 自动识别"),
    ("看识别结果", "先拿到文本，再决定下一步"),
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
                        <h2>"先选人群定位"</h2>
                        <p>"只做最关键的判断，把结果对准你的关注点"</p>
                        <p class="analysis-desc">"随时可在个人中心修改"</p>
                    </div>

                    <div class="analysis-list">
                        {ONBOARDING_STEPS
                            .iter()
                            .map(|(title, desc)| view! {
                                <div class="analysis-item">
                                    <p class="analysis-summary">{*title}</p>
                                    <p class="analysis-desc">{*desc}</p>
                                </div>
                            })
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
                            "先体验，后设置"
                        </button>
                        <button class="primary-cta" on:click=on_save_click>
                            "确认人群并开始"
                        </button>
                    </div>

                    <div class="preference-hint">
                        "💡 识别到配料文本后，你可以继续深入分析"
                    </div>
                </div>
            </div>
        </section>
    }
}
