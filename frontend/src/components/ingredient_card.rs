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
    let has_note = !note.is_empty();
    let note_view = if has_note {
        Some(view! {
            <p class="ingredient-note">{note}</p>
        })
    } else {
        None
    };

    view! {
        <div class="card ingredient-card">
            <div class="ingredient-card-header">
                <h3 class="ingredient-name">{name}</h3>
                <RiskBadge level={risk_level.clone()} />
            </div>
            <div class="ingredient-meta">
                <span class="category">{category}</span>
            </div>
            <p class="ingredient-function">{function}</p>
            {note_view}
        </div>
    }
}
