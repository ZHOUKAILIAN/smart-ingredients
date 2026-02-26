use crate::components::{
    IconArrowLeft, IconCamera, IconCheckBadge, IconFileText, IconSparkles, IconUpload, ImagePreview,
};
use crate::services;
use crate::stores::{AnalysisSource, AppState, LoadingState, ToastLevel};
use crate::utils::emit_toast;
use js_sys::Function;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::window;
use web_sys::{HtmlInputElement, Url};

#[component]
pub fn CapturePage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = StoredValue::new(use_navigate());
    let selected_file: RwSignal<Option<web_sys::File>, LocalStorage> = RwSignal::new_local(None);
    let preview_url = RwSignal::new(None::<String>);
    let camera_input_ref = NodeRef::<leptos::html::Input>::new();
    let album_input_ref = NodeRef::<leptos::html::Input>::new();

    // 线程局部信号（非 Send），用于控制扫描视图切换
    let show_scan: RwSignal<bool, LocalStorage> = RwSignal::new_local(false);
    let initialized = RwSignal::new(false);

    create_effect(move |_| {
        if initialized.get() {
            return;
        }
        initialized.set(true);
        if let Some(win) = window() {
            if let Ok(search) = win.location().search() {
                if search.contains("view=scan") {
                    show_scan.set(true);
                }
            }
        }
    });

    let on_file_change = move |ev: leptos::ev::Event| {
        let input = ev
            .target()
            .and_then(|target| target.dyn_into::<HtmlInputElement>().ok());
        let file = input
            .and_then(|input| input.files())
            .and_then(|files| files.get(0));

        // Create preview URL
        if let Some(ref f) = file {
            if let Ok(url) = Url::create_object_url_with_blob(f) {
                preview_url.set(Some(url));
            }
        }

        selected_file.set(file);
    };

    let on_select_camera = move |_| {
        if let Some(input) = camera_input_ref.get() {
            if let Ok(show_picker) =
                js_sys::Reflect::get(input.as_ref(), &JsValue::from_str("showPicker"))
            {
                if let Some(func) = show_picker.dyn_ref::<Function>() {
                    let _ = func.call0(input.as_ref());
                    return;
                }
            }
            input.click();
        }
    };

    let on_remove_preview = move || {
        // Revoke object URL to prevent memory leak
        if let Some(url) = preview_url.get() {
            let _ = Url::revoke_object_url(&url);
        }
        preview_url.set(None);
        selected_file.set(None);

        // Clear file input
        if let Some(input) = camera_input_ref.get() {
            input.set_value("");
        }
        if let Some(input) = album_input_ref.get() {
            input.set_value("");
        }
    };

    let state_for_upload = state.clone();
    let on_upload = store_value(move |_| {
        let file = selected_file.get();
        if file.is_none() {
            emit_toast(ToastLevel::Warning, "需要图片", "请先选择一张图片");
            return;
        }

        state_for_upload.error_message.set(None);
        let state = state_for_upload.clone();
        let navigate = navigate.get_value();

        // Set loading state
        state.loading_state.set(LoadingState::OcrProcessing);

        spawn_local(async move {
            match services::upload_image(file.unwrap()).await {
                Ok(response) => {
                    state.analysis_id.set(Some(response.id));
                    state.analysis_result.set(None);
                    state.error_message.set(None);
                    state.ocr_text.set(None);
                    state.confirmed_text.set(None);
                    state.selected_image_path.set(Some(response.image_url));
                    state.analysis_source.set(AnalysisSource::NewAnalysis);
                    state.loading_state.set(LoadingState::Idle);
                    // 清除 show_scan 状态
                    show_scan.set(false);
                    navigate("/ocr", Default::default());
                }
                Err(err) => {
                    state.error_message.set(Some(err));
                    state.loading_state.set(LoadingState::Idle);
                }
            }
        });
    });

    view! {
        <section class="page figma">
            {/* 可滚动内容区域 */}
            <div class="page-scrollable-content">
                <Show when=move || !show_scan.get()>
                    <div class="text-center pt-5 pb-3 px-6">
                        <div class="flex justify-center mb-3">
                            <div class="relative">
                                <div class="w-20 h-20 bg-gradient-to-br from-emerald-500 to-teal-600 rounded-[28px] flex items-center justify-center shadow-2xl animate-float">
                                    <IconSparkles class="w-10 h-10 text-white" />
                                </div>
                                <div class="absolute -top-2 -right-2 w-8 h-8 bg-gradient-to-br from-amber-400 to-orange-400 rounded-full flex items-center justify-center shadow-xl animate-bounce-slow">
                                    <span class="text-sm font-bold text-white">"AI"</span>
                                </div>
                                <div class="absolute inset-0 bg-gradient-to-br from-emerald-400 to-teal-500 rounded-[28px] blur-2xl opacity-30 -z-10 animate-pulse-gentle"></div>
                            </div>
                        </div>

                        <h1 class="text-2xl font-bold text-gray-900 mb-2 tracking-tight">
                            "Smart Ingredients"
                        </h1>
                        <p class="text-sm font-medium text-emerald-600 mb-2">
                            "AI智能配料表分析"
                        </p>
                        <p class="text-xs text-gray-600 px-4 leading-relaxed">
                            "拍摄识别配料表，AI分析"<br/>"健康风险，让您吃得更安心"
                        </p>
                    </div>

                    <div class="px-5 mb-3">
                        <div class="mx-2 p-4 shadow-2xl border bg-white-95 backdrop-blur-xl rounded-3xl">
                            <div class="flex items-center justify-center gap-2 mb-3">
                                <div class="w-1 h-5 bg-gradient-to-b from-emerald-500 to-teal-600 rounded-full"></div>
                                <h2 class="text-base font-bold text-gray-900">
                                    "使用步骤"
                                </h2>
                                <div class="w-1 h-5 bg-gradient-to-b from-emerald-500 to-teal-600 rounded-full"></div>
                            </div>
                            <div class="px-2 py-2 flex flex-col justify-center my-auto">
                                <div class="bg-white rounded-2xl p-4 mx-2 sm:mx-4 shadow-[0_4px_20px_-4px_rgba(16,185,129,0.1)] border border-emerald-50">
                                    <div class="flex flex-col gap-4 max-w-[220px] mx-auto">
                                        <div class="flex items-center justify-start gap-3 group">
                                            <div class="relative flex-shrink-0">
                                                <div class="w-12 h-12 bg-gradient-to-br from-emerald-100 via-teal-50 to-emerald-50 rounded-2xl flex items-center justify-center shadow-sm transition-all duration-300">
                                                    <IconCamera class="w-6 h-6 text-emerald-600" />
                                                </div>
                                                <div class="absolute z-10 w-5 h-5 bg-gradient-to-br from-emerald-500 to-teal-600 text-white rounded-full flex items-center justify-center text-[10px] font-bold shadow-md" style="top: -6px; right: -6px;">
                                                    "1"
                                                </div>
                                            </div>
                                            <div class="text-left flex-1">
                                                <h3 class="font-bold text-gray-900 mb-0.5 text-sm mt-0">"拍摄配料表"</h3>
                                                <p class="text-xs text-gray-500 leading-relaxed m-0 whitespace-nowrap">"对准配料表垂直拍照"</p>
                                            </div>
                                        </div>
                                        <div class="flex items-center justify-start gap-3 group">
                                            <div class="relative flex-shrink-0">
                                                <div class="w-12 h-12 bg-gradient-to-br from-emerald-100 via-teal-50 to-emerald-50 rounded-2xl flex items-center justify-center shadow-sm transition-all duration-300">
                                                    <IconFileText class="w-6 h-6 text-emerald-600" />
                                                </div>
                                                <div class="absolute z-10 w-5 h-5 bg-gradient-to-br from-emerald-500 to-teal-600 text-white rounded-full flex items-center justify-center text-[10px] font-bold shadow-md" style="top: -6px; right: -6px;">
                                                    "2"
                                                </div>
                                            </div>
                                            <div class="text-left flex-1">
                                                <h3 class="font-bold text-gray-900 mb-0.5 text-sm mt-0">"确认识别文本"</h3>
                                                <p class="text-xs text-gray-500 leading-relaxed m-0 whitespace-nowrap">"AI自动识别配料信息"</p>
                                            </div>
                                        </div>
                                        <div class="flex items-center justify-start gap-3 group">
                                            <div class="relative flex-shrink-0">
                                                <div class="w-12 h-12 bg-gradient-to-br from-emerald-100 via-teal-50 to-emerald-50 rounded-2xl flex items-center justify-center shadow-sm transition-all duration-300">
                                                    <IconCheckBadge class="w-6 h-6 text-emerald-600" />
                                                </div>
                                                <div class="absolute z-10 w-5 h-5 bg-gradient-to-br from-emerald-500 to-teal-600 text-white rounded-full flex items-center justify-center text-[10px] font-bold shadow-md" style="top: -6px; right: -6px;">
                                                    "3"
                                                </div>
                                            </div>
                                            <div class="text-left flex-1">
                                                <h3 class="font-bold text-gray-900 mb-0.5 text-sm mt-0">"查看健康报告"</h3>
                                                <p class="text-xs text-gray-500 leading-relaxed m-0 whitespace-nowrap">"获取详细的分析和建议"</p>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="px-5 pb-4 pt-1">
                        <button
                            on:click=move |_| show_scan.set(true)
                            class="w-full max-w-xs mx-auto block h-12 text-base font-semibold rounded-2xl transition-all relative overflow-hidden group border-0 bg-gradient-to-br from-emerald-500 to-teal-500 text-white shadow-lg"
                        >
                            <span class="flex items-center justify-center">
                                <IconCamera class="w-5 h-5 mr-2" />
                                "开始分析"
                            </span>
                        </button>

                    </div>
                </Show>

                <Show when=move || show_scan.get()>
                    <div class="flex items-center px-4 py-3 bg-white-80 backdrop-blur-xl sticky top-0 z-10 shadow-sm">
                        <button
                            class="mr-3 -ml-2 w-10 h-10 rounded-full border-0 bg-transparent flex items-center justify-center text-gray-700 hover:text-gray-900 hover:bg-gray-100 transition-colors"
                            on:click=move |_| {
                                show_scan.set(false);
                                on_remove_preview();
                            }
                            aria-label="返回"
                        >
                            <IconArrowLeft class="w-5 h-5"/>
                        </button>
                        <h1 class="text-lg font-bold text-gray-900 m-0">
                            "拍摄配料表"
                        </h1>
                    </div>

                    <div class="flex-1 px-5 py-3 flex flex-col min-h-0">
                        <div class="p-3 shadow-2xl border bg-white-95 backdrop-blur-xl h-full flex flex-col rounded-3xl">
                            <input
                                node_ref=camera_input_ref
                                style="position:absolute;width:1px;height:1px;opacity:0;pointer-events:none;"
                                type="file"
                                accept="image/*"
                                capture="environment"
                                on:change=on_file_change
                            />
                            <Show when=move || preview_url.get().is_some()>
                                <div class="space-y-3 flex-1 flex flex-col min-h-0">
                                    <div class="relative flex-1 rounded-2xl overflow-hidden min-h-0">
                                        <ImagePreview
                                            preview_url=preview_url.into()
                                            on_remove=on_remove_preview
                                        />
                                    </div>

                                    <div class="bg-gradient-to-r from-emerald-50 to-teal-50 rounded-2xl p-3 text-center border-2 border-emerald-100">
                                        <p class="text-sm text-emerald-800 font-semibold flex items-center justify-center gap-2 m-0">
                                            <div class="w-2 h-2 bg-emerald-500 rounded-full animate-pulse"></div>
                                            "图片已上传，点击分析开始识别"
                                        </p>
                                    </div>

                                    <button
                                        on:click=move |ev| on_upload.with_value(|f| f(ev))
                                        disabled=move || state.loading_state.get() != LoadingState::Idle
                                        class="w-full h-12 text-base font-semibold rounded-2xl transition-all disabled:opacity-50 border-0 bg-gradient-to-br from-emerald-500 to-teal-500 text-white shadow-lg"
                                    >
                                        <span class="flex justify-center items-center">
                                            {move || {
                                                if state.loading_state.get() == LoadingState::OcrProcessing {
                                                    "AI 分析中..."
                                                } else {
                                                    "开始分析"
                                                }
                                            }}
                                        </span>
                                    </button>
                                </div>
                            </Show>

                            <Show when=move || preview_url.get().is_none()>
                                <div class="text-center flex-1 flex flex-col justify-center px-4">
                                    <div class="mb-4">
                                        <div class="relative inline-block">
                                            <div class="w-20 h-20 mx-auto bg-gradient-to-br from-emerald-100 via-teal-50 to-emerald-50 rounded-3xl flex items-center justify-center mb-3 shadow-xl">
                                                <IconCamera class="w-10 h-10 text-emerald-600" />
                                            </div>
                                            <div class="absolute inset-0 bg-gradient-to-br from-emerald-400 to-teal-400 rounded-3xl blur-2xl opacity-20 -z-10"></div>
                                        </div>
                                        <h2 class="text-xl font-bold text-gray-900 mb-2 mt-0">
                                            "上传配料表照片"
                                        </h2>
                                        <p class="text-sm text-gray-600 m-0">
                                            "请确保配料表文字清晰可见"
                                        </p>
                                    </div>

                                    <div class="flex gap-3">
                                        <button
                                            class="flex-1 h-12 text-base font-semibold flex justify-center items-center rounded-2xl cursor-pointer m-0 border-0 bg-gradient-to-br from-emerald-500 to-teal-500 text-white shadow-lg"
                                            on:click=on_select_camera
                                        >
                                            <IconCamera class="w-5 h-5 mr-2" />
                                            "拍照"
                                        </button>
                                        <div class="relative flex-1">
                                            <label
                                                for="album-file-input"
                                                class="w-full h-12 text-base font-semibold transition-all flex justify-center items-center rounded-2xl cursor-pointer m-0 border-2 border-emerald-200 bg-white text-emerald-700 hover:bg-emerald-50"
                                            >
                                                <IconUpload class="w-5 h-5 mr-2" />
                                                "从相册选择"
                                            </label>
                                            <input
                                                id="album-file-input"
                                                node_ref=album_input_ref
                                                type="file"
                                                accept="image/*"
                                                on:change=on_file_change
                                                aria-label="从相册选择"
                                                style="position:absolute;width:1px;height:1px;padding:0;margin:-1px;overflow:hidden;clip:rect(0,0,0,0);clip-path:inset(50%);white-space:nowrap;border:0;"
                                            />
                                        </div>
                                    </div>
                                </div>
                            </Show>
                        </div>
                    </div>

                    <div class="px-5 pb-5">
                        <div class="bg-white-80 backdrop-blur-xl rounded-2xl px-4 py-4 border border-emerald-100 shadow-lg text-left">
                            <h3 class="font-bold text-gray-900 mb-3 text-sm flex items-center gap-2 mt-0">
                                <div class="w-1 h-4 bg-gradient-to-b from-emerald-500 to-teal-600 rounded-full"></div>
                                "拍摄小贴士"
                            </h3>
                            <ul class="space-y-2 text-xs text-gray-600 pl-0 m-0 list-none">
                                <li class="flex items-start gap-2.5">
                                    <div class="w-1.5 h-1.5 bg-emerald-500 rounded-full mt-1.5 flex-shrink-0"></div>
                                    <span>"确保配料表文字清晰，光线充足"</span>
                                </li>
                                <li class="flex items-start gap-2.5">
                                    <div class="w-1.5 h-1.5 bg-emerald-500 rounded-full mt-1.5 flex-shrink-0"></div>
                                    <span>"尽量平行拍摄，避免文字倾斜"</span>
                                </li>
                                <li class="flex items-start gap-2.5">
                                    <div class="w-1.5 h-1.5 bg-emerald-500 rounded-full mt-1.5 flex-shrink-0"></div>
                                    <span>"避免反光和阴影遮挡文字"</span>
                                </li>
                            </ul>
                        </div>
                    </div>
                </Show>
            </div>
        </section>
    }
}
