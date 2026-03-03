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
        <div class="flex flex-col min-h-full h-full w-full overflow-hidden">
            <div
                class="flex-1 w-full max-w-[480px] mx-auto pb-[calc(56px+env(safe-area-inset-bottom,0))] box-border overflow-hidden flex flex-col min-h-0"
                id="main-content"
                tabindex="-1"
            >
                {children()}
            </div>
            <BottomNav />
        </div>
    }
}
