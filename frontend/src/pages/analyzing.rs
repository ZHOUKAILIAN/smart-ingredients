use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

use crate::services;
use crate::stores::AppState;

#[component]
pub fn AnalyzingPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();
    let started = create_rw_signal(false);

    create_effect(move |_| {
        if started.get() {
            return;
        }
        let analysis_id = state.analysis_id.get();
        if let Some(id) = analysis_id {
            started.set(true);
            let state = state.clone();
            let navigate = navigate.clone();
            spawn_local(async move {
                match services::analyze_image(id).await {
                    Ok(response) => {
                        let api_error = response.error_message.clone();
                        state.analysis_result.set(Some(response));
                        state.error_message.set(api_error);
                    }
                    Err(err) => {
                        state.error_message.set(Some(err));
                    }
                }
                navigate("/result", Default::default());
            });
        }
    });

    view! {
        <section class="page page-analyzing">
            <div class="loading-card">
                <div class="spinner" aria-hidden="true"></div>
                <p class="loading-text">"正在分析配料表…"</p>
            </div>
            <a class="secondary-link" href="/result">
                "跳到结果页"
            </a>
        </section>
    }
}
