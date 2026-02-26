//! Global toast component

use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;

use crate::stores::{AppState, ToastLevel, ToastMessage};

#[component]
pub fn ToastHost() -> impl IntoView {
    let state = use_context::<AppState>().expect("missing app state");
    let toasts = state.toasts;

    create_effect(move |_| {
        let Some(window) = web_sys::window() else {
            return;
        };

        let toasts_for_event = toasts.clone();
        let handler = wasm_bindgen::closure::Closure::wrap(Box::new(move |event: web_sys::Event| {
            let Some(custom_event) = event.dyn_ref::<web_sys::CustomEvent>() else {
                return;
            };
            let detail = custom_event.detail();
            let title = detail_string(&detail, "title").unwrap_or_else(|| "提示".to_string());
            let message =
                detail_string(&detail, "message").unwrap_or_else(|| "请求失败".to_string());
            let level = match detail_string(&detail, "level")
                .unwrap_or_else(|| "error".to_string())
                .as_str()
            {
                "warning" => ToastLevel::Warning,
                "success" => ToastLevel::Success,
                "info" => ToastLevel::Info,
                _ => ToastLevel::Error,
            };

            let id = js_sys::Date::now() as u64;
            let toast = ToastMessage {
                id,
                level,
                title,
                message,
            };

            toasts_for_event.update(|items| items.push(toast));
            let toasts_for_cleanup = toasts_for_event.clone();
            spawn_local(async move {
                TimeoutFuture::new(4000).await;
                toasts_for_cleanup.update(|items| {
                    items.retain(|item| item.id != id);
                });
            });
        }) as Box<dyn FnMut(_)>);

        let _ = window
            .add_event_listener_with_callback("global-toast", handler.as_ref().unchecked_ref());
        handler.forget();
    });

    view! {
        <div class="fixed top-4 right-4 z-[1200] flex w-[min(92vw,360px)] flex-col gap-2" aria-live="polite" role="status">
            <For
                each=move || toasts.get()
                key=|toast| toast.id
                children=move |toast| {
                    let class = match toast.level {
                        ToastLevel::Error => "rounded-xl border px-3 py-2 shadow-xl backdrop-blur-sm flex items-start gap-2 border-red-200 bg-red-50 text-red-800",
                        ToastLevel::Warning => "rounded-xl border px-3 py-2 shadow-xl backdrop-blur-sm flex items-start gap-2 border-amber-200 bg-amber-50 text-amber-800",
                        ToastLevel::Success => "rounded-xl border px-3 py-2 shadow-xl backdrop-blur-sm flex items-start gap-2 border-emerald-200 bg-emerald-50 text-emerald-800",
                        ToastLevel::Info => "rounded-xl border px-3 py-2 shadow-xl backdrop-blur-sm flex items-start gap-2 border-sky-200 bg-sky-50 text-sky-800",
                    };
                    let id = toast.id;
                    let toasts_for_close = toasts.clone();
                    view! {
                        <div class=class>
                            <div class="min-w-0 flex-1">
                                <p class="m-0 text-sm font-semibold leading-tight">{toast.title}</p>
                                <p class="m-0 mt-1 text-xs leading-relaxed opacity-90">{toast.message}</p>
                            </div>
                            <button
                                class="h-7 w-7 rounded-md border-0 bg-transparent text-base leading-none opacity-70 hover:opacity-100 hover:bg-black/5"
                                aria-label="关闭"
                                on:click=move |_| {
                                    toasts_for_close.update(|items| {
                                        items.retain(|item| item.id != id);
                                    });
                                }
                            >
                                "×"
                            </button>
                        </div>
                    }
                }
            />
        </div>
    }
}

fn detail_string(detail: &JsValue, key: &str) -> Option<String> {
    js_sys::Reflect::get(detail, &JsValue::from_str(key))
        .ok()
        .and_then(|value| value.as_string())
}
