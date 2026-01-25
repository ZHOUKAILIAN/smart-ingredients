use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PreferenceOption {
    pub value: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
}

pub const PREFERENCE_OPTIONS: &[PreferenceOption] = &[
    PreferenceOption {
        value: "none",
        label: "通用分析",
        description: "全面分析配料表,适合大多数人",
        icon: "🔍",
    },
    PreferenceOption {
        value: "weight_loss",
        label: "减肥",
        description: "关注热量、脂肪、糖分等",
        icon: "⚖️",
    },
    PreferenceOption {
        value: "health",
        label: "健康",
        description: "关注添加剂、防腐剂等人工成分",
        icon: "💚",
    },
    PreferenceOption {
        value: "fitness",
        label: "健身",
        description: "关注蛋白质、碳水化合物等营养",
        icon: "💪",
    },
    PreferenceOption {
        value: "allergy",
        label: "过敏",
        description: "关注常见过敏原成分",
        icon: "⚠️",
    },
    PreferenceOption {
        value: "kids",
        label: "儿童",
        description: "关注色素、香精等儿童敏感成分",
        icon: "👶",
    },
];

pub fn get_preference_label(value: &str) -> &'static str {
    PREFERENCE_OPTIONS
        .iter()
        .find(|opt| opt.value == value)
        .map(|opt| opt.label)
        .unwrap_or("未知")
}

pub fn get_preference_icon(value: &str) -> &'static str {
    PREFERENCE_OPTIONS
        .iter()
        .find(|opt| opt.value == value)
        .map(|opt| opt.icon)
        .unwrap_or("🔍")
}

#[component]
pub fn PreferenceSelector(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] show_description: bool,
) -> impl IntoView {
    let label_text = label.unwrap_or("分析偏好");

    view! {
        <div class="preference-selector">
            <label class="preference-label">{label_text}</label>
            <select
                class="preference-select"
                prop:value=move || value.get()
                on:change=move |ev| {
                    let new_value = event_target_value(&ev);
                    on_change.run(new_value);
                }
            >
                {PREFERENCE_OPTIONS
                    .iter()
                    .map(|opt| {
                        let opt_value = opt.value;
                        view! {
                            <option
                                value=opt.value
                                prop:selected=move || value.get() == opt_value
                            >
                                {opt.icon} " " {opt.label}
                            </option>
                        }
                    })
                    .collect_view()}
            </select>
            {show_description.then(|| {
                view! {
                    <p class="preference-description">
                        {move || {
                            let current_value = value.get();
                            PREFERENCE_OPTIONS
                                .iter()
                                .find(|opt| opt.value == current_value.as_str())
                                .map(|opt| opt.description)
                                .unwrap_or("")
                        }}
                    </p>
                }
            })}
        </div>
    }
}

#[component]
pub fn PreferenceCard(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
) -> impl IntoView {
    view! {
        <div class="preference-cards">
            {PREFERENCE_OPTIONS
                .iter()
                .map(|opt| {
                    let opt_value = opt.value.to_string();
                    let opt_value_for_selected = opt_value.clone();
                    let is_selected = move || value.get() == opt_value_for_selected;
                    view! {
                        <button
                            class="preference-card"
                            class:selected=is_selected
                            on:click=move |_| {
                                on_change.run(opt_value.clone());
                            }
                        >
                            <div class="preference-card-icon">{opt.icon}</div>
                            <div class="preference-card-label">{opt.label}</div>
                            <div class="preference-card-description">{opt.description}</div>
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}
