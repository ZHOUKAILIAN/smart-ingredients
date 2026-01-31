//! Ingredient card list component

use crate::components::{IngredientCard, IngredientRow};
use leptos::prelude::*;

#[component]
pub fn IngredientCardList(items: Vec<IngredientRow>) -> impl IntoView {
    view! {
        <div class="ingredient-card-list">
            {items
                .into_iter()
                .map(|item| {
                    view! {
                        <IngredientCard
                            name={item.name}
                            category={item.category}
                            function={item.function}
                            risk_level={item.risk_level}
                            note={item.note}
                        />
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
