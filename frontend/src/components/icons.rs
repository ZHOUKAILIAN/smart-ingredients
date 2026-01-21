use leptos::prelude::*;

#[component]
pub fn IconArrowLeft() -> impl IntoView {
    view! {
        <svg
            class="icon"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
            focusable="false"
        >
            <path d="m12 19-7-7 7-7"></path>
            <path d="M19 12H5"></path>
        </svg>
    }
}

#[component]
pub fn IconCamera() -> impl IntoView {
    view! {
        <svg
            class="icon"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
            focusable="false"
        >
            <path d="M14.5 4h-5L8 7H5a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3.5Z"></path>
            <circle cx="12" cy="13" r="3"></circle>
        </svg>
    }
}

#[component]
pub fn IconCheckBadge() -> impl IntoView {
    view! {
        <svg
            class="icon"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
            focusable="false"
        >
            <path d="m9 12 2 2 4-4"></path>
            <path d="M12 3a9 9 0 1 0 9 9"></path>
        </svg>
    }
}

#[component]
pub fn IconChart() -> impl IntoView {
    view! {
        <svg
            class="icon"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
            focusable="false"
        >
            <path d="M3 3v18h18"></path>
            <path d="M7 14v4"></path>
            <path d="M12 10v8"></path>
            <path d="M17 6v12"></path>
        </svg>
    }
}
