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

use crate::pages::{AnalyzingPage, CapturePage, ResultPage};

/// Main App component
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main class="app-shell">
                <Routes>
                    <Route path="/" view=CapturePage />
                    <Route path="/analyzing" view=AnalyzingPage />
                    <Route path="/result" view=ResultPage />
                </Routes>
            </main>
        </Router>
    }
}
