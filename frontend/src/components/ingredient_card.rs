//! Ingredient card component

use leptos::prelude::*;
use crate::components::RiskBadge;

#[component]
pub fn IngredientCard(
    name: String,
    category: String,
    function: String,
    risk_level: String,
    note: String,
) -> impl IntoView {
    let is_valid = |value: &str| !value.is_empty() && value != "未知" && value != "暂无";
    let category_value = store_value(category);
    let function_value = store_value(function);
    let note_value = store_value(note);
    let show_category = is_valid(&category_value.get_value());
    let show_function = is_valid(&function_value.get_value());
    let show_note = is_valid(&note_value.get_value());

    view! {
        <div class="ingredient-card-compact">
            <div class="card-header">
                <h3 class="ingredient-name">{name}</h3>
                <RiskBadge level={risk_level.clone()} />
            </div>
            <Show when=move || show_category || show_function>
                <div class="tags-row">
                    <Show when=move || show_category>
                        <span class="tag tag-category">{category_value.get_value()}</span>
                    </Show>
                    <Show when=move || show_function>
                        <span class="tag tag-function">{function_value.get_value()}</span>
                    </Show>
                </div>
            </Show>
            <Show when=move || show_note>
                <p class="ingredient-note">{note_value.get_value()}</p>
            </Show>
        </div>
    }
}
