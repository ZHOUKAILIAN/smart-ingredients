//! Loading spinner component

use leptos::prelude::*;

#[component]
pub fn LoadingSpinner(
    /// Loading message to display
    #[prop(into)]
    message: String,
) -> impl IntoView {
    view! {
        <div class="mx-auto my-8 w-full max-w-[320px] rounded-2xl border border-emerald-100 bg-white-95 px-5 py-6 text-center shadow-sm">
            <div class="mx-auto h-8 w-8 rounded-full border-2 border-emerald-200 border-t-emerald-500 animate-spin"></div>
            <p class="m-0 mt-3 text-sm font-medium text-gray-700">{message}</p>
        </div>
    }
}
