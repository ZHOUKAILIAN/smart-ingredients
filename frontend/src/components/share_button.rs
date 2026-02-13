//! Share/Export button component for analysis result pages.

use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::ExportPreviewModal;
use crate::stores::ToastLevel;
use crate::utils::emit_toast;
use crate::utils::export_image::{ExportData, ExportIngredient};

/// Props extracted from analysis result, passed to the button.
#[derive(Clone, Debug)]
pub struct ShareExportProps {
    pub health_score: i32,
    pub recommendation: String,
    pub ingredients: Vec<ExportIngredient>,
    pub warnings: Vec<String>,
    pub summary: String,
    pub preference_label: String,
}

impl From<ShareExportProps> for ExportData {
    fn from(p: ShareExportProps) -> Self {
        ExportData {
            health_score: p.health_score,
            recommendation: p.recommendation,
            ingredients: p.ingredients,
            warnings: p.warnings,
            summary: p.summary,
            preference_label: p.preference_label,
        }
    }
}

#[component]
pub fn ShareButton(
    #[prop(into)] props: ShareExportProps,
) -> impl IntoView {
    let exporting = RwSignal::new(false);
    let preview_url = RwSignal::new(None::<String>);
    let data = props.clone();

    let on_export = move |_: web_sys::MouseEvent| {
        if exporting.get() {
            return;
        }
        exporting.set(true);
        let export_data: ExportData = data.clone().into();
        spawn_local({
            let exporting = exporting;
            async move {
                match crate::utils::export_image::export_to_data_url(&export_data) {
                    Ok(url) => {
                        preview_url.set(Some(url));
                    }
                    Err(err) => {
                        emit_toast(ToastLevel::Error, "导出失败", &err);
                    }
                }
                exporting.set(false);
            }
        });
    };

    let on_close_preview = Callback::new(move |_: ()| {
        preview_url.set(None);
    });

    let preview_signal = Signal::derive(move || preview_url.get());

    view! {
        <button
            class="export-btn"
            on:click=on_export
            disabled=move || exporting.get()
            aria-label="导出分析结果为图片"
        >
            <Show
                when=move || exporting.get()
                fallback=|| view! {
                    <span class="export-btn-icon">"📤"</span>
                    <span class="export-btn-text">"导出图片"</span>
                }
            >
                <span class="export-btn-icon export-loading">"⏳"</span>
                <span class="export-btn-text">"生成中…"</span>
            </Show>
        </button>

        <ExportPreviewModal
            image_url=preview_signal
            on_close=on_close_preview
        />
    }
}
