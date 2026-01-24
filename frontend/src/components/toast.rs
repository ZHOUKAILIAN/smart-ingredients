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
            let title =
                detail_string(&detail, "title").unwrap_or_else(|| "提示".to_string());
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

        let _ = window.add_event_listener_with_callback(
            "global-toast",
            handler.as_ref().unchecked_ref(),
        );
        handler.forget();
    });

    view! {
        <div class="toast-host">
            <For
                each=move || toasts.get()
                key=|toast| toast.id
                children=move |toast| {
                    let class = match toast.level {
                        ToastLevel::Error => "toast toast-error",
                        ToastLevel::Warning => "toast toast-warning",
                        ToastLevel::Success => "toast toast-success",
                        ToastLevel::Info => "toast toast-info",
                    };
                    let id = toast.id;
                    let toasts_for_close = toasts.clone();
                    view! {
                        <div class=class>
                            <div class="toast-content">
                                <p class="toast-title">{toast.title}</p>
                                <p class="toast-message">{toast.message}</p>
                            </div>
                            <button
                                class="toast-close"
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
