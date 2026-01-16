use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn CapturePage() -> impl IntoView {
    view! {
        <section class="page page-capture">
            <header class="page-header">
                <h1 class="title">"配料表分析"</h1>
                <p class="subtitle">"拍一张配料表，快速看到风险信息"</p>
            </header>
            <div class="action-area">
                <A class="primary-button" href="/analyzing">
                    "拍照"
                </A>
                <p class="hint">"目前为静态展示，后续接入相机"</p>
            </div>
        </section>
    }
}
