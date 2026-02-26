use crate::components::{IconCommunity, IconHistory, IconHome, IconUser};
use crate::stores::{AppState, TabRoute};
use crate::utils::local_storage;
use crate::utils::navigation::build_full_path;
use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};
use web_sys::MouseEvent;

fn tab_for_path(path: &str) -> TabRoute {
    if path == "/history" || path.starts_with("/history/") {
        TabRoute::History
    } else if path == "/community" || path.starts_with("/community/") {
        TabRoute::Community
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
    path != "/login" && path != "/register"
}

fn is_modified_click(ev: &MouseEvent) -> bool {
    ev.meta_key() || ev.ctrl_key() || ev.shift_key() || ev.alt_key() || ev.button() != 0
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
        if !should_record_last_path(path.as_str()) {
            return;
        }
        let search = location.search.get();
        let full_path = build_full_path(path.as_str(), search.as_str());
        match tab_for_path(path.as_str()) {
            TabRoute::Home => state.last_home_path.set(full_path),
            TabRoute::History => state.last_history_path.set(full_path),
            TabRoute::Community => state.last_community_path.set(full_path),
            TabRoute::Profile => state.last_profile_path.set(full_path),
        }
    });

    let tab_target = move |tab: TabRoute| {
        let current = current_tab.get();
        let is_same_tab = current == tab;
        if is_same_tab {
            tab.path().to_string()
        } else {
            let last_path = match tab {
                TabRoute::Home => state.last_home_path.get(),
                TabRoute::History => state.last_history_path.get(),
                TabRoute::Community => state.last_community_path.get(),
                TabRoute::Profile => state.last_profile_path.get(),
            };
            if last_path.is_empty()
                || last_path.starts_with("/login")
                || last_path.starts_with("/register")
            {
                tab.path().to_string()
            } else {
                last_path
            }
        }
    };

    let on_tab_click = move |tab: TabRoute| {
        local_storage::set_last_tab(tab.path());

        state.current_tab.set(tab);
        let target = tab_target(tab);
        navigate(&target, Default::default());
    };

    view! {
        <nav class="fixed bottom-0 left-0 right-0 h-[clamp(48px,6.2vh,60px)] bg-white-80 border-t border-emerald-100/40 backdrop-blur-xl flex justify-around items-center px-[env(safe-area-inset-left)] pr-[env(safe-area-inset-right)] pb-[env(safe-area-inset-bottom)] z-[100] shadow-[0_-10px_24px_rgba(15,23,42,0.12)]">
            <For
                each=move || [
                    TabRoute::Home,
                    TabRoute::History,
                    TabRoute::Community,
                    TabRoute::Profile,
                ]
                key=|tab| format!("{:?}", tab)
                children=move |tab| {
                    let is_active = move || current_tab.get() == tab;
                    let tab_clone = tab;
                    let on_click = on_tab_click.clone();

                    view! {
                        <a
                            class=move || {
                                if is_active() {
                                    "relative flex flex-col items-center justify-center flex-1 h-full bg-transparent border-0 no-underline px-0 py-1 gap-1 text-emerald-600"
                                } else {
                                    "relative flex flex-col items-center justify-center flex-1 h-full bg-transparent border-0 no-underline px-0 py-1 gap-1 text-slate-500 hover:text-emerald-600"
                                }
                            }
                            on:click=move |ev: MouseEvent| {
                                if is_modified_click(&ev) {
                                    return;
                                }
                                ev.prevent_default();
                                on_click(tab_clone);
                            }
                            href=move || tab_target(tab_clone)
                            aria-label=tab.label()
                            aria-current=move || if is_active() { "page" } else { "" }
                        >
                            <span class="relative flex items-center justify-center pb-[2px]">
                                {match tab {
                                    TabRoute::Home => view! { <IconHome /> }.into_any(),
                                    TabRoute::History => view! { <IconHistory /> }.into_any(),
                                    TabRoute::Community => view! { <IconCommunity /> }.into_any(),
                                    TabRoute::Profile => view! { <IconUser /> }.into_any(),
                                }}
                            </span>
                            <span class="text-[11px] font-medium leading-[1.2]">{tab.label()}</span>
                        </a>
                    }
                }
            />
        </nav>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tab_for_path_maps_community_root() {
        assert_eq!(tab_for_path("/community"), TabRoute::Community);
    }

    #[test]
    fn tab_for_path_maps_community_detail() {
        assert_eq!(tab_for_path("/community/123"), TabRoute::Community);
    }

    #[test]
    fn tab_for_path_maps_history_root() {
        assert_eq!(tab_for_path("/history"), TabRoute::History);
    }
}
