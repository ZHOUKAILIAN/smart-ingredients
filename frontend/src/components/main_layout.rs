use crate::components::BottomNav;
use leptos::prelude::*;

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    view! {
        <div class="main-layout">
            <div class="main-content">
                {children()}
            </div>
            <BottomNav />
        </div>
    }
}
