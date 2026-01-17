use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use crate::services;
use crate::stores::AppState;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[component]
pub fn CapturePage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();
    let selected_file: RwSignal<Option<web_sys::File>, LocalStorage> = RwSignal::new_local(None);
    let local_error = create_rw_signal(None);

    let on_file_change = move |ev: leptos::ev::Event| {
        let input = ev
            .target()
            .and_then(|target| target.dyn_into::<HtmlInputElement>().ok());
        let file = input
            .and_then(|input| input.files())
            .and_then(|files| files.get(0));
        selected_file.set(file);
    };

    let on_upload = move |_| {
        let file = selected_file.get();
        if file.is_none() {
            local_error.set(Some("请先选择一张图片".to_string()));
            return;
        }

        local_error.set(None);
        state.error_message.set(None);
        let state = state.clone();
        let navigate = navigate.clone();

        spawn_local(async move {
            match services::upload_image(file.unwrap()).await {
                Ok(response) => {
                    state.analysis_id.set(Some(response.id));
                    state.analysis_result.set(None);
                    navigate("/analyzing", Default::default());
                }
                Err(err) => {
                    state.error_message.set(Some(err.clone()));
                    local_error.set(Some(err));
                }
            }
        });
    };

    view! {
        <section class="page page-capture">
            <header class="page-header">
                <h1 class="title">"配料表分析"</h1>
                <p class="subtitle">"拍一张配料表，快速看到风险信息"</p>
            </header>
            <div class="action-area">
                <input
                    class="file-input"
                    type="file"
                    accept="image/*"
                    on:change=on_file_change
                />
                <button
                    class="primary-button"
                    on:click=on_upload
                    disabled=move || selected_file.get().is_none()
                >
                    "上传并分析"
                </button>
                <Show
                    when=move || local_error.get().is_some()
                    fallback=|| view! { <p class="hint">"请选择清晰的配料表图片"</p> }
                >
                    <p class="hint error">
                        {move || local_error.get().unwrap_or_default()}
                    </p>
                </Show>
            </div>
        </section>
    }
}
