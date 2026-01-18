//! Smart Ingredients Frontend
//!
//! Tauri + Leptos desktop application

mod components;
mod pages;
mod services;
mod stores;
mod types;
mod utils;

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::pages::{AnalyzingPage, CapturePage, ConfirmPage, OcrPage, ResultPage};
use crate::stores::AppState;

/// Main App component
#[component]
pub fn App() -> impl IntoView {
    let analysis_id = create_rw_signal(None);
    let analysis_result = create_rw_signal(None);
    let error_message = create_rw_signal(None);
    let ocr_text = create_rw_signal(None);
    let confirmed_text = create_rw_signal(None);

    provide_context(AppState {
        analysis_id,
        analysis_result,
        error_message,
        ocr_text,
        confirmed_text,
    });

    view! {
        <Router>
            <main class="app-shell">
                <Routes fallback=|| view! { <p>"Not found"</p> }>
                    <Route path=path!("/") view=CapturePage />
                    <Route path=path!("/ocr") view=OcrPage />
                    <Route path=path!("/confirm") view=ConfirmPage />
                    <Route path=path!("/analyzing") view=AnalyzingPage />
                    <Route path=path!("/result") view=ResultPage />
                </Routes>
            </main>
        </Router>
    }
}
