//! Loading spinner component

use leptos::prelude::*;

#[component]
pub fn LoadingSpinner(
    /// Loading message to display
    #[prop(into)]
    message: String,
) -> impl IntoView {
    view! {
        <div class="loading-container">
            <div class="spinner"></div>
            <p class="loading-message">{message}</p>
        </div>
    }
}
