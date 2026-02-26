use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use serde_json::json;

use crate::components::{get_preference_label, ConfirmModal, PreferenceCard};
use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::emit_toast;
use crate::utils::local_storage;
use crate::utils::preference::{merge_preferences, save_preference};

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
        local_storage::set_has_seen_onboarding(true);
        state.has_seen_onboarding.set(true);

        if state.auth_user.get().is_some() {
            let val_clone = pref_value.clone();
            spawn_local(async move {
                let base = services::fetch_preferences()
                    .await
                    .map(|prefs| prefs.preferences)
                    .unwrap_or_else(|_| json!({}));
                let merged = merge_preferences(base, Some(val_clone.as_str()), Some(true));
                match services::update_preferences(merged).await {
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
        nav("/", Default::default());
    };

    let on_skip = move |_| {
        preference.set("normal".to_string());
        on_confirm_save(());
    };

    let on_cancel_confirm = Callback::new(move |_| {
        show_confirm.set(false);
    });

    let confirm_message = Signal::derive(move || {
        format!(
            "确定选择「{}」吗？",
            get_preference_label(&preference.get())
        )
    });

    view! {
        <section class="page figma">
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
                <div class="px-5 pt-8 pb-6">

                    <PreferenceCard
                        value=Signal::derive(move || preference.get())
                        on_change=Callback::new(move |value: String| {
                            preference.set(value);
                        })
                    />

                    <div class="flex items-center gap-4 mt-8 px-2">
                        <button class="flex-1 py-3 px-4 bg-gray-100 hover:bg-gray-200 text-gray-700 text-sm font-semibold rounded-2xl transition-all cursor-pointer border-0" on:click=on_skip>
                            "先体验，后设置"
                        </button>
                        <button class="flex-1 py-3 px-4 bg-gradient-to-r from-emerald-500 to-teal-600 hover:from-emerald-600 hover:to-teal-700 text-white text-sm font-semibold rounded-2xl transition-all cursor-pointer border-0 shadow-lg shadow-emerald-500/20" on:click=on_save_click>
                            "确认人群并开始"
                        </button>
                    </div>

                    <div class="mt-4 text-xs text-gray-600 bg-emerald-50 border border-emerald-100 rounded-xl px-3 py-2">
                        "💡 识别到配料文本后，你可以继续深入分析"
                    </div>
                </div>
            </div>
        </section>
    }
}
