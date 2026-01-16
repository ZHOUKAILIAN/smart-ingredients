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

/// Main App component
#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="app-container">
            <Home />
        </main>
    }
}

/// Home page component
#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="home">
            <h1>"Smart Ingredients"</h1>
            <p>"食品配料表分析助手"</p>
            <button>
                "开始分析"
            </button>
        </div>
    }
}
