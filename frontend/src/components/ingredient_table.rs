use leptos::prelude::*;

#[derive(Clone)]
pub struct IngredientRow {
    pub name: String,
    pub category: String,
    pub function: String,
    pub risk_level: String,
    pub note: String,
}

#[component]
pub fn IngredientTable(items: Vec<IngredientRow>) -> impl IntoView {
    view! {
        <div class="table-card">
            <div class="table-header">
                <span>"成分"</span>
                <span>"分类"</span>
                <span>"作用"</span>
                <span>"风险"</span>
                <span>"备注"</span>
            </div>
            <div class="table-body">
                {items
                    .into_iter()
                    .map(|item| {
                        view! {
                            <div class="table-row">
                                <span class="cell name">{item.name}</span>
                                <span class="cell category">{item.category}</span>
                                <span class="cell function">{item.function}</span>
                                <span class="cell risk">{item.risk_level}</span>
                                <span class="cell note">{item.note}</span>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
