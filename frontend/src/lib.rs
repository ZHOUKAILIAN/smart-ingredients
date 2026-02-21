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
    AnalyzingPage, CapturePage, CommunityDetailPage, CommunityPage, ConfirmPage, DetailPage,
    HistoryPage, LoginPage, OcrPage, OnboardingPage, ProfilePage, RegisterPage, ResultPage,
    SummaryPage,
};
use crate::stores::{AnalysisSource, AppState, LoadingState, ResultPageState, TabRoute, ToastLevel};
use crate::utils::preference::{load_preference, merge_preferences, save_preference};
use crate::utils::{emit_toast, local_storage};
use serde_json::json;

/// Main App component
#[component]
pub fn App() -> impl IntoView {
    let analysis_id = RwSignal::new(None);
    let analysis_result = RwSignal::new(None);
    let initial_preference = match load_preference() {
        Some(value) => value,
        None => {
            let value = "normal".to_string();
            save_preference(&value);
            value
        }
    };
    let analysis_preference = RwSignal::new(Some(initial_preference));
    let has_seen_onboarding = RwSignal::new(local_storage::get_has_seen_onboarding());
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
    let last_community_path = RwSignal::new("/community".to_string());
    let last_profile_path = RwSignal::new("/profile".to_string());
    let open_in_scan_mode = RwSignal::new(false);

    provide_context(AppState {
        analysis_id,
        analysis_result,
        analysis_preference,
        has_seen_onboarding,
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
        last_community_path,
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
                        let mut base = prefs.preferences;
                        if !base.is_object() {
                            base = json!({});
                        }
                        let mut selection_to_set: Option<String> = None;
                        let mut seen_to_set: Option<bool> = None;

                        if let Some(value) = base.get("selection").and_then(|v| v.as_str()) {
                            save_preference(value);
                            auth_state.analysis_preference.set(Some(value.to_string()));
                        } else if let Some(local_value) = auth_state.analysis_preference.get() {
                            selection_to_set = Some(local_value);
                        }

                        if let Some(flag) =
                            base.get("has_seen_onboarding").and_then(|v| v.as_bool())
                        {
                            local_storage::set_has_seen_onboarding(flag);
                            auth_state.has_seen_onboarding.set(flag);
                        } else if auth_state.has_seen_onboarding.get() {
                            seen_to_set = Some(true);
                        }

                        if selection_to_set.is_some() || seen_to_set.is_some() {
                            let merged = merge_preferences(
                                base,
                                selection_to_set.as_deref(),
                                seen_to_set,
                            );
                            if let Err(err) = services::update_preferences(merged).await {
                                emit_toast(ToastLevel::Error, "同步失败", &err);
                            }
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
                    <Route path=path!("/community") view=move || view! {
                        <MainLayout>
                            <CommunityPage />
                        </MainLayout>
                    } />
                    <Route path=path!("/community/:id") view=move || view! {
                        <MainLayout>
                            <CommunityDetailPage />
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
