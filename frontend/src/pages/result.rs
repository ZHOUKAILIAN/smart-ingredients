use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::{IngredientRow, IngredientTable};

#[component]
pub fn ResultPage() -> impl IntoView {
    let items = vec![
        IngredientRow {
            name: "白砂糖",
            risk: "中",
            note: "摄入过量可能增加热量负担",
        },
        IngredientRow {
            name: "山梨酸钾",
            risk: "低",
            note: "常见防腐剂，符合限量使用",
        },
        IngredientRow {
            name: "食用香精",
            risk: "中",
            note: "建议关注具体来源与用量",
        },
    ];

    view! {
        <section class="page page-result">
            <header class="page-header">
                <h1 class="title">"配料表"</h1>
                <p class="subtitle">"以下为模型分析后的静态展示"</p>
            </header>
            <IngredientTable items=items />
            <div class="action-area">
                <A href="/">
                    "重新拍照"
                </A>
            </div>
        </section>
    }
}
