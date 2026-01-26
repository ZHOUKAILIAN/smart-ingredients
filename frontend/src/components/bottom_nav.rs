use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};
use crate::stores::{AppState, TabRoute};
use crate::utils::local_storage;

#[component]
pub fn BottomNav() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();
    let location = use_location();

    // 根据当前路径判断激活的 Tab
    let current_tab = create_memo(move |_| {
        let path = location.pathname.get();
        match path.as_str() {
            "/" => TabRoute::Home,
            "/history" => TabRoute::History,
            "/profile" => TabRoute::Profile,
            _ => TabRoute::Home,
        }
    });

    let on_tab_click = move |tab: TabRoute| {
        local_storage::set_last_tab(tab.path());
        state.current_tab.set(tab);
        navigate(tab.path(), Default::default());
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
                            <span class="tab-icon">{tab.icon()}</span>
                            <span class="tab-label">{tab.label()}</span>
                        </button>
                    }
                }
            />
        </nav>
    }
}
