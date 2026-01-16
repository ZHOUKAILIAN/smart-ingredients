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

use crate::pages::{AnalyzingPage, CapturePage, ResultPage};

/// Main App component
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main class="app-shell">
                <Routes fallback=|| view! { <p>"Not found"</p> }>
                    <Route path=path!("/") view=CapturePage />
                    <Route path=path!("/analyzing") view=AnalyzingPage />
                    <Route path=path!("/result") view=ResultPage />
                </Routes>
            </main>
        </Router>
    }
}
