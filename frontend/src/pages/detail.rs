//! Detail page - shows full ingredient list

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::{IngredientRow, IngredientCardList};
use crate::stores::AppState;

fn risk_label(level: &str) -> String {
    level.to_string()
}

fn to_rows(table: &[shared::TableRow]) -> Vec<IngredientRow> {
    table
        .iter()
        .map(|row| IngredientRow {
            name: row.name.clone(),
            category: row.category.clone(),
            function: row.function.clone(),
            risk_level: risk_label(&row.risk_level),
            note: row.note.clone(),
        })
        .collect()
}

fn ingredient_rows(items: &[shared::IngredientInfo]) -> Vec<IngredientRow> {
    items
        .iter()
        .map(|item| IngredientRow {
            name: item.name.clone(),
            category: item.category.clone(),
            function: item.description.clone().unwrap_or_default(),
            risk_level: risk_label(&item.risk_level),
            note: String::new(),
        })
        .collect()
}

#[component]
pub fn DetailPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();

    let on_back = move |_| {
        navigate("/summary", Default::default());
    };

    let table_rows = move || {
        state
            .analysis_result
            .get()
            .and_then(|response| response.result)
            .map(|result| {
                if result.table.is_empty() {
                    ingredient_rows(&result.ingredients)
                } else {
                    to_rows(&result.table)
                }
            })
            .unwrap_or_default()
    };

    view! {
        <section class="page page-detail figma">
            <div class="figma-body">
                <header class="figma-header">
                    <button class="icon-button" on:click=on_back aria-label="返回概要">
                        "←"
                    </button>
                    <h1 class="figma-title">"详细配料列表"</h1>
                    <span class="icon-placeholder"></span>
                </header>

                // Ingredient card list
                <Show
                    when=move || !table_rows().is_empty()
                    fallback=move || view! { <p class="hint">"暂无配料数据"</p> }
                >
                    <IngredientCardList items=table_rows() />
                </Show>
            </div>
        </section>
    }
}
