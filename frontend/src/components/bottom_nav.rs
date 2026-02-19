use crate::components::{IconHistory, IconHome, IconUser};
use crate::stores::{AppState, TabRoute};
use crate::utils::local_storage;
use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};

fn tab_for_path(path: &str) -> TabRoute {
    if path == "/history" || path.starts_with("/history/") {
        TabRoute::History
    } else if path == "/profile"
        || path.starts_with("/profile/")
        || path == "/onboarding"
        || path.starts_with("/onboarding/")
    {
        TabRoute::Profile
    } else {
        TabRoute::Home
    }
}

fn should_record_last_path(path: &str) -> bool {
    !matches!(path, "/login" | "/register" | "/onboarding")
}

#[component]
pub fn BottomNav() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();
    let location = use_location();

    // 根据当前路径判断激活的 Tab
    let current_tab = create_memo(move |_| {
        let path = location.pathname.get();
        tab_for_path(path.as_str())
    });

    create_effect(move |_| {
        let path = location.pathname.get();
        if !should_record_last_path(&path) {
            return;
        }
        let tab = tab_for_path(&path);
        let search = location.search.get();
        let full_path = if search.is_empty() {
            path
        } else {
            format!("{}{}", path, search)
        };
        match tab {
            TabRoute::Home => state.last_home_path.set(full_path),
            TabRoute::History => state.last_history_path.set(full_path),
            TabRoute::Profile => state.last_profile_path.set(full_path),
        }
    });

    let on_tab_click = move |tab: TabRoute| {
        local_storage::set_last_tab(tab.path());

        // Check if we're already on this tab
        let current = current_tab.get();
        let is_same_tab = current == tab;

        state.current_tab.set(tab);

        let target = if is_same_tab {
            // If clicking the same tab, always go to root path
            tab.path().to_string()
        } else {
            // If switching tabs, go to last visited path
            let last_path = match tab {
                TabRoute::Home => state.last_home_path.get(),
                TabRoute::History => state.last_history_path.get(),
                TabRoute::Profile => state.last_profile_path.get(),
            };
            if last_path.is_empty() || last_path.starts_with("/login") || last_path.starts_with("/register") {
                tab.path().to_string()
            } else {
                last_path
            }
        };

        navigate(&target, Default::default());
    };

    view! {
        <nav class="bottom-nav">
            <For
                each=move || [TabRoute::Home, TabRoute::History, TabRoute::Profile]
                key=|tab| format!("{:?}", tab)
                children=move |tab| {
                    let is_active = move || current_tab.get() == tab;
                    let tab_clone = tab;
                    let on_click = on_tab_click.clone();

                    view! {
                        <button
                            class="tab-item"
                            class:active=is_active
                            on:click=move |_| on_click(tab_clone)
                            aria-label=tab.label()
                            aria-current=move || if is_active() { "page" } else { "" }
                        >
                            <span class="tab-icon">
                                {match tab {
                                    TabRoute::Home => view! { <IconHome /> }.into_any(),
                                    TabRoute::History => view! { <IconHistory /> }.into_any(),
                                    TabRoute::Profile => view! { <IconUser /> }.into_any(),
                                }}
                            </span>
                            <span class="tab-label">{tab.label()}</span>
                        </button>
                    }
                }
            />
        </nav>
    }
}
