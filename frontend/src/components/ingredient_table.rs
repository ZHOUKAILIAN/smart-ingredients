use crate::utils::category_label;
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
        <div class="rounded-2xl border border-emerald-100 bg-white-95 shadow-sm overflow-hidden">
            <div class="grid grid-cols-[1.2fr_1fr_1.1fr_0.8fr_1.3fr] gap-2 px-3 py-2 bg-emerald-50 text-xs font-semibold text-emerald-700">
                <span>"成分"</span>
                <span>"分类"</span>
                <span>"作用"</span>
                <span>"风险"</span>
                <span>"备注"</span>
            </div>
            <div class="divide-y divide-emerald-100">
                {items
                    .into_iter()
                    .map(|item| {
                        view! {
                            <div class="grid grid-cols-[1.2fr_1fr_1.1fr_0.8fr_1.3fr] gap-2 px-3 py-2 text-xs text-gray-700 leading-relaxed">
                                <span class="font-medium text-gray-900">{item.name}</span>
                                <span>{category_label(&item.category)}</span>
                                <span>{item.function}</span>
                                <span>{item.risk_level}</span>
                                <span>{item.note}</span>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
