//! Ingredient card component

use crate::components::RiskBadge;
use crate::utils::category_label;
use leptos::prelude::*;

#[component]
pub fn IngredientCard(
    name: String,
    category: String,
    function: String,
    risk_level: String,
    note: String,
) -> impl IntoView {
    let is_valid = |value: &str| !value.is_empty() && value != "未知" && value != "暂无";
    let category_value = RwSignal::new(category_label(&category));
    let function_value = RwSignal::new(function);
    let note_value = RwSignal::new(note);
    let function_tag_class = match risk_level.as_str() {
        "high" => "inline-flex items-center px-2 py-1 rounded-full text-xs font-medium border text-red-700 border-red-100",
        "medium" => "inline-flex items-center px-2 py-1 rounded-full text-xs font-medium border text-amber-700 border-amber-100",
        "low" => "inline-flex items-center px-2 py-1 rounded-full text-xs font-medium border text-emerald-700 border-emerald-100",
        _ => "inline-flex items-center px-2 py-1 rounded-full text-xs font-medium border text-gray-600 border-gray-100",
    };
    let show_category = is_valid(&category_value.get_untracked());
    let show_function = is_valid(&function_value.get_untracked());
    let show_note = is_valid(&note_value.get_untracked());

    view! {
        <div class="rounded-2xl border border-emerald-100 bg-white-95 shadow-sm p-4">
            <div class="flex items-start justify-between gap-3">
                <h3 class="m-0 text-sm font-semibold text-gray-900">{name}</h3>
                <RiskBadge level={risk_level.clone()} />
            </div>
            <Show when=move || show_category || show_function>
                <div class="mt-2 flex flex-wrap gap-2">
                    <Show when=move || show_category>
                        <span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-emerald-50 text-emerald-700 border border-emerald-100">{category_value.get()}</span>
                    </Show>
                    <Show when=move || show_function>
                        <span class={function_tag_class}>{function_value.get()}</span>
                    </Show>
                </div>
            </Show>
            <Show when=move || show_note>
                <p class="mt-2 mb-0 text-xs text-gray-600 leading-relaxed">{note_value.get()}</p>
            </Show>
        </div>
    }
}
