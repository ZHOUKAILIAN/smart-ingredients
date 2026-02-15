//! Export preview modal — shows the generated image in a floating overlay.
//! User can long-press to save, tap the download button, or close the modal.

use leptos::prelude::*;
use crate::stores::ToastLevel;
use crate::utils::emit_toast;

#[component]
pub fn ExportPreviewModal(
    /// The data URL of the generated image (Some = visible, None = hidden).
    #[prop(into)]
    image_url: Signal<Option<String>>,
    /// Callback to close the modal.
    #[prop(into)]
    on_close: Callback<()>,
) -> impl IntoView {
    let on_download = move |_: web_sys::MouseEvent| {
        if let Some(url) = image_url.get() {
            if crate::utils::export_image::is_tauri_available() {
                // Tauri (Android/desktop app): async save via native IPC
                wasm_bindgen_futures::spawn_local(async move {
                    match crate::utils::export_image::save_via_tauri(&url).await {
                        Ok(path) => {
                            emit_toast(
                                ToastLevel::Success,
                                "保存成功",
                                &format!("图片已保存: {}", path),
                            );
                        }
                        Err(e) => {
                            emit_toast(ToastLevel::Error, "保存失败", &e);
                        }
                    }
                });
            } else {
                // Browser fallback: <a download>
                match crate::utils::export_image::download_from_data_url(&url) {
                    Ok(()) => {
                        emit_toast(ToastLevel::Success, "保存成功", "图片已开始下载");
                    }
                    Err(e) => {
                        emit_toast(ToastLevel::Error, "保存失败", &e);
                    }
                }
            }
        }
    };

    let on_backdrop = move |_: web_sys::MouseEvent| {
        on_close.run(());
    };

    let on_close_btn = move |_: web_sys::MouseEvent| {
        on_close.run(());
    };

    // Prevent clicks on the content from closing the modal
    let stop_propagation = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
    };

    view! {
        <Show when=move || image_url.get().is_some()>
            <div class="export-preview-backdrop" on:click=on_backdrop>
                <div class="export-preview-modal" on:click=stop_propagation>
                    <div class="export-preview-header">
                        <span class="export-preview-title">"分析结果图片"</span>
                        <button class="export-preview-close" on:click=on_close_btn>
                            "✕"
                        </button>
                    </div>

                    <div class="export-preview-body">
                        <p class="export-preview-hint">"长按图片可保存到相册"</p>
                        <div class="export-preview-image-container">
                            <img
                                src=move || image_url.get().unwrap_or_default()
                                alt="分析结果"
                                class="export-preview-image"
                            />
                        </div>
                    </div>

                    <div class="export-preview-footer">
                        <button class="export-preview-download-btn" on:click=on_download>
                            <span>"💾"</span>
                            <span>"保存图片"</span>
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
