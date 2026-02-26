//! Export preview modal — shows the generated image in a floating overlay.
//! User can long-press to save, tap the download button, or close the modal.

use crate::stores::ToastLevel;
use crate::utils::emit_toast;
use leptos::prelude::*;

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

    // Escape 键关闭模态框
    let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Escape" {
            on_close.run(());
        }
    };

    view! {
        <Show when=move || image_url.get().is_some()>
            <div
                class="fixed inset-0 z-[9999] flex items-center justify-center bg-slate-900/55 px-4"
                on:click=on_backdrop
                on:keydown=on_keydown
                role="dialog"
                aria-modal="true"
                aria-label="导出预览"
            >
                <div class="w-full max-w-[420px] rounded-3xl border border-emerald-100 bg-white-95 shadow-2xl overflow-hidden" on:click=stop_propagation>
                    <div class="flex items-center justify-between px-5 py-4 border-b border-emerald-100">
                        <span class="text-base font-semibold text-gray-900">"分析结果图片"</span>
                        <button class="w-9 h-9 rounded-full border-0 bg-transparent text-gray-500 hover:bg-gray-100 hover:text-gray-700 transition-colors" on:click=on_close_btn aria-label="关闭">
                            "✕"
                        </button>
                    </div>

                    <div class="px-5 py-4">
                        <p class="m-0 mb-3 text-center text-xs text-gray-500">"长按图片可保存到相册"</p>
                        <div class="rounded-2xl border border-emerald-100 bg-emerald-50/50 p-2">
                            <img
                                src=move || image_url.get().unwrap_or_default()
                                alt="分析结果"
                                class="block w-full rounded-xl object-contain max-h-[60vh]"
                            />
                        </div>
                    </div>

                    <div class="px-5 pb-5">
                        <button class="w-full h-11 rounded-xl border-0 bg-gradient-to-br from-emerald-500 to-teal-500 text-white text-sm font-semibold shadow-lg transition-all flex items-center justify-center gap-2" on:click=on_download>
                            <span aria-hidden="true">"💾"</span>
                            <span>"保存图片"</span>
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
