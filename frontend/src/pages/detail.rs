//! Detail page - shows full ingredient list

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::{IconArrowLeft, IngredientCardList, IngredientRow};
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

fn risk_rank(level: &str) -> i32 {
    let trimmed = level.trim();
    let lowered = trimmed.to_lowercase();
    match lowered.as_str() {
        "high" => 0,
        "medium" => 1,
        "low" => 2,
        "unknown" => 3,
        _ => match trimmed {
            "高" => 0,
            "中" => 1,
            "低" => 2,
            "未知" => 3,
            _ => 3,
        },
    }
}

fn sort_rows_by_risk(rows: Vec<IngredientRow>) -> Vec<IngredientRow> {
    let mut indexed_rows: Vec<(usize, IngredientRow)> =
        rows.into_iter().enumerate().collect();
    indexed_rows.sort_by_key(|(index, row)| (risk_rank(&row.risk_level), *index));
    indexed_rows.into_iter().map(|(_, row)| row).collect()
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
                    sort_rows_by_risk(ingredient_rows(&result.ingredients))
                } else {
                    sort_rows_by_risk(to_rows(&result.table))
                }
            })
            .unwrap_or_default()
    };

    view! {
        <section class="page page-detail figma">
            <div class="page-topbar">
                <button class="icon-button" on:click=on_back aria-label="返回概要">
                    <IconArrowLeft />
                </button>
                <h1 class="page-topbar-title">"详细配料列表"</h1>
                <div class="icon-placeholder"></div>
            </div>

            <div class="page-scrollable-content">
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
