use leptos::prelude::*;

#[derive(Clone)]
pub struct IngredientRow {
    pub name: &'static str,
    pub risk: &'static str,
    pub note: &'static str,
}

#[component]
pub fn IngredientTable(items: Vec<IngredientRow>) -> impl IntoView {
    view! {
        <div class="table-card">
            <div class="table-header">
                <span>"成分"</span>
                <span>"风险"</span>
                <span>"说明"</span>
            </div>
            <div class="table-body">
                {items
                    .into_iter()
                    .map(|item| {
                        view! {
                            <div class="table-row">
                                <span class="cell name">{item.name}</span>
                                <span class="cell risk">{item.risk}</span>
                                <span class="cell note">{item.note}</span>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
