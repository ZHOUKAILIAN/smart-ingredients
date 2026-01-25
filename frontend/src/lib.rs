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

use crate::components::ToastHost;
use crate::pages::{AnalyzingPage, CapturePage, ConfirmPage, DetailPage, OcrPage, ResultPage, SummaryPage};
use crate::stores::{AppState, LoadingState, ResultPageState};

/// Main App component
#[component]
pub fn App() -> impl IntoView {
    let analysis_id = create_rw_signal(None);
    let analysis_result = create_rw_signal(None);
    let analysis_preference = create_rw_signal(None);
    let error_message = create_rw_signal(None);
    let ocr_text = create_rw_signal(None);
    let confirmed_text = create_rw_signal(None);

    // New state for interaction optimization
    let loading_state = create_rw_signal(LoadingState::Idle);
    let result_page_state = create_rw_signal(ResultPageState::Summary);
    let error = create_rw_signal(None);
    let selected_image_path = create_rw_signal(None);
    let toasts = create_rw_signal(Vec::new());

    provide_context(AppState {
        analysis_id,
        analysis_result,
        analysis_preference,
        error_message,
        ocr_text,
        confirmed_text,
        loading_state,
        result_page_state,
        error,
        selected_image_path,
        toasts,
    });

    view! {
        <Router>
            <main class="app-shell">
                <ToastHost />
                <Routes fallback=|| view! { <p>"Not found"</p> }>
                    <Route path=path!("/") view=CapturePage />
                    <Route path=path!("/ocr") view=OcrPage />
                    <Route path=path!("/confirm") view=ConfirmPage />
                    <Route path=path!("/analyzing") view=AnalyzingPage />
                    <Route path=path!("/result") view=ResultPage />
                    <Route path=path!("/summary") view=SummaryPage />
                    <Route path=path!("/detail") view=DetailPage />
                </Routes>
            </main>
        </Router>
    }
}
