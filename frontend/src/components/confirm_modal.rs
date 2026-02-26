use leptos::ev;
use leptos::prelude::*;

#[component]
pub fn ConfirmModal(
    /// 是否显示模态框
    show: Signal<bool>,
    /// 标题
    #[prop(default = "确认操作".to_string())]
    title: String,
    /// 消息内容（支持动态内容）
    #[prop(into)]
    message: MaybeSignal<String>,
    /// 确认按钮文字
    #[prop(default = "确定".to_string())]
    confirm_text: String,
    /// 取消按钮文字
    #[prop(default = "取消".to_string())]
    cancel_text: String,
    /// 确认回调
    on_confirm: Callback<()>,
    /// 取消回调
    on_cancel: Callback<()>,
) -> impl IntoView {
    let on_confirm_click = move |_| {
        on_confirm.run(());
    };

    let on_cancel_click = move |_| {
        on_cancel.run(());
    };

    let on_backdrop_click = move |_| {
        on_cancel.run(());
    };

    let on_modal_click = move |ev: ev::MouseEvent| {
        // 阻止事件冒泡，避免点击模态框内容时关闭
        ev.stop_propagation();
    };

    // Escape 键关闭模态框
    let on_keydown = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Escape" {
            on_cancel.run(());
        }
    };

    view! {
        <Show when=move || show.get()>
            <div
                class="fixed inset-0 z-[1000] bg-slate-900/45 flex items-center justify-center px-5"
                on:click=on_backdrop_click
                on:keydown=on_keydown
                role="dialog"
                aria-modal="true"
                aria-labelledby="confirm-modal-title"
            >
                <div class="w-full max-w-[340px] rounded-2xl border border-emerald-100 bg-white-95 shadow-2xl overflow-hidden" on:click=on_modal_click>
                    <div class="px-5 pt-5">
                        <h3 id="confirm-modal-title" class="m-0 text-lg font-semibold text-gray-900">{title.clone()}</h3>
                    </div>
                    <div class="px-5 pt-3">
                        <p class="m-0 text-sm text-gray-600 leading-relaxed">{message.get()}</p>
                    </div>
                    <div class="px-5 py-5 mt-1 flex gap-3">
                        <button class="flex-1 h-10 rounded-xl border border-gray-200 bg-white text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50" on:click=on_cancel_click>
                            {cancel_text.clone()}
                        </button>
                        <button class="flex-1 h-10 rounded-xl border-0 bg-red-500 text-sm font-semibold text-white transition-colors hover:bg-red-600" on:click=on_confirm_click>
                            {confirm_text.clone()}
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
