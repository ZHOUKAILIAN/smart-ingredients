use crate::components::BottomNav;
use crate::stores::AppState;
use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let location = use_location();
    let navigate = use_navigate();

    create_effect(move |_| {
        if state.auth_loading.get() || state.has_seen_onboarding.get() {
            return;
        }
        let path = location.pathname.get();
        if path == "/onboarding" || path.starts_with("/onboarding/") {
            return;
        }
        if path == "/login" || path == "/register" {
            return;
        }
        navigate("/onboarding", Default::default());
    });

    view! {
        <div class="main-layout">
            <div class="main-content">
                {children()}
            </div>
            <BottomNav />
        </div>
    }
}
