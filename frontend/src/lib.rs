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
use leptos::task::spawn_local;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::components::{MainLayout, ToastHost};
use crate::pages::{
    AnalyzingPage, CapturePage, ConfirmPage, DetailPage, HistoryPage, LoginPage, OcrPage,
    OnboardingPage, ProfilePage, RegisterPage, ResultPage, SummaryPage,
};
use crate::stores::{AnalysisSource, AppState, LoadingState, ResultPageState, TabRoute};
use crate::utils::preference::save_preference;

/// Main App component
#[component]
pub fn App() -> impl IntoView {
    let analysis_id = RwSignal::new(None);
    let analysis_result = RwSignal::new(None);
    let analysis_preference = RwSignal::new(None);
    let error_message = RwSignal::new(None);
    let ocr_text = RwSignal::new(None);
    let confirmed_text = RwSignal::new(None);

    // New state for interaction optimization
    let loading_state = RwSignal::new(LoadingState::Idle);
    let result_page_state = RwSignal::new(ResultPageState::Summary);
    let error = RwSignal::new(None);
    let selected_image_path = RwSignal::new(None);
    let toasts = RwSignal::new(Vec::new());
    let auth_user = RwSignal::new(None);
    let auth_loading = RwSignal::new(true);
    let analysis_source = RwSignal::new(AnalysisSource::NewAnalysis);

    let current_tab = RwSignal::new(TabRoute::Home);
    let last_home_path = RwSignal::new("/".to_string());
    let last_history_path = RwSignal::new("/history".to_string());
    let last_profile_path = RwSignal::new("/profile".to_string());
    let open_in_scan_mode = RwSignal::new(false);

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
        auth_user,
        auth_loading,
        analysis_source,
        current_tab,
        last_home_path,
        last_history_path,
        last_profile_path,
        open_in_scan_mode,
    });

    let auth_state = use_context::<AppState>().expect("AppState not found");
    spawn_local(async move {
        auth_state.auth_loading.set(true);
        match services::ensure_session().await {
            Ok(user) => {
                auth_state.auth_user.set(user.clone());
                if user.is_some() {
                    if let Ok(prefs) = services::fetch_preferences().await {
                        if let Some(value) =
                            prefs.preferences.get("selection").and_then(|v| v.as_str())
                        {
                            save_preference(value);
                            auth_state.analysis_preference.set(Some(value.to_string()));
                        }
                    }
                }
            }
            Err(_) => {
                auth_state.auth_user.set(None);
            }
        }
        auth_state.auth_loading.set(false);
    });

    view! {
        <Router>
            <main class="app-shell">
                <ToastHost />
                <Routes fallback=|| view! { <p>"Not found"</p> }>
                    <Route path=path!("/login") view=LoginPage />
                    <Route path=path!("/register") view=RegisterPage />

                    <Route path=path!("/") view=move || view! {
                        <MainLayout>
                            <CapturePage />
                        </MainLayout>
                    } />
                    <Route path=path!("/history") view=move || view! {
                        <MainLayout>
                            <HistoryPage />
                        </MainLayout>
                    } />
                    <Route path=path!("/profile") view=move || view! {
                        <MainLayout>
                            <ProfilePage />
                        </MainLayout>
                    } />
                    <Route path=path!("/onboarding") view=move || view! {
                        <MainLayout>
                            <OnboardingPage />
                        </MainLayout>
                    } />
                    <Route path=path!("/ocr") view=move || view! {
                        <MainLayout>
                            <OcrPage />
                        </MainLayout>
                    } />
                    <Route path=path!("/confirm") view=move || view! {
                        <MainLayout>
                            <ConfirmPage />
                        </MainLayout>
                    } />
                    <Route path=path!("/analyzing") view=move || view! {
                        <MainLayout>
                            <AnalyzingPage />
                        </MainLayout>
                    } />
                    <Route path=path!("/result") view=move || view! {
                        <MainLayout>
                            <ResultPage />
                        </MainLayout>
                    } />
                    <Route path=path!("/summary") view=move || view! {
                        <MainLayout>
                            <SummaryPage />
                        </MainLayout>
                    } />
                    <Route path=path!("/detail") view=move || view! {
                        <MainLayout>
                            <DetailPage />
                        </MainLayout>
                    } />
                </Routes>
            </main>
        </Router>
    }
}
