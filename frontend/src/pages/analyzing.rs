use leptos::prelude::*;
#[component]
pub fn AnalyzingPage() -> impl IntoView {
    view! {
        <section class="page page-analyzing">
            <div class="loading-card">
                <div class="spinner" aria-hidden="true"></div>
                <p class="loading-text">"正在分析配料表…"</p>
            </div>
            <a class="secondary-link" href="/result">
                "跳到结果页"
            </a>
        </section>
    }
}
