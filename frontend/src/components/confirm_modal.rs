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
                class="confirm-modal-overlay"
                on:click=on_backdrop_click
                on:keydown=on_keydown
                role="dialog"
                aria-modal="true"
                aria-labelledby="confirm-modal-title"
            >
                <div class="confirm-modal-card" on:click=on_modal_click>
                    <div class="confirm-modal-header">
                        <h3 id="confirm-modal-title" class="confirm-modal-title">{title.clone()}</h3>
                    </div>
                    <div class="confirm-modal-body">
                        <p class="confirm-modal-message">{message.get()}</p>
                    </div>
                    <div class="confirm-modal-actions">
                        <button class="confirm-modal-btn cancel" on:click=on_cancel_click>
                            {cancel_text.clone()}
                        </button>
                        <button class="confirm-modal-btn confirm" on:click=on_confirm_click>
                            {confirm_text.clone()}
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
