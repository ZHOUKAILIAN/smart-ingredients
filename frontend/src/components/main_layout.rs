use leptos::prelude::*;
use crate::components::BottomNav;

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
