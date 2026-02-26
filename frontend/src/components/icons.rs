use leptos::prelude::*;

#[component]
pub fn IconArrowLeft(#[prop(into, default = "w-5 h-5".into())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
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
pub fn IconCamera(#[prop(into, default = "w-5 h-5".into())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
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
pub fn IconCheckBadge(#[prop(into, default = "w-5 h-5".into())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
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
            class="w-5 h-5"
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

#[component]
pub fn IconSparkles(#[prop(into, default = "w-5 h-5".into())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M9.937 15.5A2 2 0 0 0 8.5 14.063l-6.135-1.582a.5.5 0 0 1 0-.962L8.5 9.936A2 2 0 0 0 9.937 8.5l1.582-6.135a.5.5 0 0 1 .963 0L14.063 8.5A2 2 0 0 0 15.5 9.937l6.135 1.581a.5.5 0 0 1 0 .964L15.5 14.063a2 2 0 0 0-1.437 1.437l-1.582 6.135a.5.5 0 0 1-.963 0z"/>
            <path d="M20 3v4"/>
            <path d="M22 5h-4"/>
            <path d="M4 17v2"/>
            <path d="M5 18H3"/>
        </svg>
    }
}

#[component]
pub fn IconTrendingUp(#[prop(into, default = "w-5 h-5".into())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <polyline points="22 7 13.5 15.5 8.5 10.5 2 17"/>
            <polyline points="16 7 22 7 22 13"/>
        </svg>
    }
}

#[component]
pub fn IconUpload(#[prop(into, default = "w-5 h-5".into())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="17 8 12 3 7 8"/>
            <line x1="12" x2="12" y1="3" y2="15"/>
        </svg>
    }
}

#[component]
pub fn IconFileText(#[prop(into, default = "w-5 h-5".into())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/>
            <polyline points="14 2 14 8 20 8"/>
            <line x1="16" x2="8" y1="13" y2="13"/>
            <line x1="16" x2="8" y1="17" y2="17"/>
            <line x1="10" x2="8" y1="9" y2="9"/>
        </svg>
    }
}
