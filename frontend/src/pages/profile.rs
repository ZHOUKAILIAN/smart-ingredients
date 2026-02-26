use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

use crate::services;
use crate::stores::{AppState, ToastLevel};
use crate::utils::emit_toast;

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
            .and_then(|w| {
                w.confirm_with_message("确认注销账号吗？该操作不可恢复。")
                    .ok()
            })
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
        <section class="page figma">
            <div class="page-scrollable-content">
                <Show when=move || state.auth_user.get().is_some() fallback=move || {
                view! {
                    // 未登录状态
                    <div class="px-5 pt-8 pb-6">
                        <div class="rounded-3xl border border-emerald-100 bg-white-95 shadow-lg p-5 text-center">
                            <div class="w-16 h-16 mx-auto rounded-full bg-emerald-100 text-2xl flex items-center justify-center">"👤"</div>
                            <div class="mt-3 text-lg font-bold text-gray-900">"未登录"</div>
                            <button class="mt-3 h-10 px-4 rounded-xl border-0 bg-gradient-to-br from-emerald-500 to-teal-500 text-white text-sm font-semibold shadow-md" on:click=move |_| {
                                let navigate = navigate.get_value();
                                navigate("/login", Default::default());
                            }>"登录 / 注册"</button>
                        </div>

                        <div class="mt-4 rounded-2xl border border-emerald-100 bg-white-95 shadow-sm overflow-hidden">
                            <button class="w-full h-12 px-4 flex items-center justify-between text-left bg-transparent border-0 hover:bg-emerald-50 transition-colors" on:click=move |_| {
                                let navigate = navigate.get_value();
                                navigate("/onboarding", Default::default());
                            }>
                                <span class="flex items-center gap-2 text-gray-800">
                                    <span>"⚙️"</span>
                                    <span class="text-sm font-medium">"人群设置"</span>
                                </span>
                                <span class="text-gray-400">"›"</span>
                            </button>
                        </div>
                    </div>
                }
            }>
                // 已登录状态
                {move || {
                    state.auth_user.get().map(|user| {
                        view! {
                            <div class="px-5 pt-8 pb-6">
                                <div class="rounded-3xl border border-emerald-100 bg-white-95 shadow-lg p-5 text-center">
                                    <div class="w-16 h-16 mx-auto rounded-full bg-emerald-100 text-2xl flex items-center justify-center">"👤"</div>
                                    <div class="mt-3 text-lg font-bold text-gray-900">{user.login_id}</div>
                                    <div class="mt-2 inline-flex items-center gap-2 rounded-full bg-emerald-50 px-3 py-1">
                                        <span class="text-xs text-gray-500">"分析次数"</span>
                                        <span class="text-sm font-semibold text-emerald-700">{user.analysis_count}</span>
                                    </div>
                                </div>

                                <div class="mt-4 rounded-2xl border border-emerald-100 bg-white-95 shadow-sm overflow-hidden">
                                    <button class="w-full h-12 px-4 flex items-center justify-between text-left bg-transparent border-0 hover:bg-emerald-50 transition-colors" on:click=move |_| {
                                        let navigate = navigate.get_value();
                                        navigate("/onboarding", Default::default());
                                    }>
                                        <span class="flex items-center gap-2 text-gray-800">
                                            <span>"⚙️"</span>
                                            <span class="text-sm font-medium">"人群设置"</span>
                                        </span>
                                        <span class="text-gray-400">"›"</span>
                                    </button>
                                    <div class="h-px bg-emerald-100"></div>
                                    <button class="w-full h-12 px-4 flex items-center justify-between text-left bg-transparent border-0 hover:bg-emerald-50 transition-colors" on:click=move |_| on_logout.run(())>
                                        <span class="flex items-center gap-2 text-gray-800">
                                            <span>"🚪"</span>
                                            <span class="text-sm font-medium">"退出登录"</span>
                                        </span>
                                        <span class="text-gray-400">"›"</span>
                                    </button>
                                    <button class="w-full h-12 px-4 flex items-center justify-between text-left bg-transparent border-0 hover:bg-red-50 transition-colors" on:click=move |_| on_delete.run(())>
                                        <span class="flex items-center gap-2 text-red-600">
                                            <span>"⚠️"</span>
                                            <span class="text-sm font-medium">"注销账号"</span>
                                        </span>
                                        <span class="text-red-300">"›"</span>
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
