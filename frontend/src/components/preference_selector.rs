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
        value: "normal",
        label: "普通人群",
        description: "适合大多数人，综合查看风险与建议",
        icon: "🙂",
    },
    PreferenceOption {
        value: "allergy",
        label: "过敏体质",
        description: "重点关注过敏原与交叉污染提示",
        icon: "⚠️",
    },
    PreferenceOption {
        value: "kids",
        label: "儿童/婴幼儿",
        description: "关注高糖、刺激性与儿童敏感成分",
        icon: "👶",
    },
    PreferenceOption {
        value: "pregnancy",
        label: "孕期/哺乳",
        description: "关注刺激性成分与不明确添加剂",
        icon: "🤰",
    },
    PreferenceOption {
        value: "weight_loss",
        label: "控糖/控重",
        description: "关注糖分、脂肪与热量负担",
        icon: "🍬",
    },
    PreferenceOption {
        value: "low_sodium",
        label: "低钠/心血管关注",
        description: "关注钠盐、调味剂与血压负担",
        icon: "🫀",
    },
    PreferenceOption {
        value: "fitness",
        label: "健身增肌",
        description: "关注蛋白质与整体营养结构",
        icon: "💪",
    },
    PreferenceOption {
        value: "gut_sensitive",
        label: "肠胃敏感",
        description: "关注刺激性成分与肠胃负担",
        icon: "🫧",
    },
    PreferenceOption {
        value: "lactose_intolerant",
        label: "乳糖不耐/乳制品敏感",
        description: "关注乳制品相关成分",
        icon: "🥛",
    },
];

pub fn get_preference_label(value: &str) -> &'static str {
    PREFERENCE_OPTIONS
        .iter()
        .find(|opt| opt.value == value)
        .map(|opt| opt.label)
        .unwrap_or("普通人群")
}

pub fn get_preference_icon(value: &str) -> &'static str {
    PREFERENCE_OPTIONS
        .iter()
        .find(|opt| opt.value == value)
        .map(|opt| opt.icon)
        .unwrap_or("🙂")
}

pub fn get_preference_description(value: &str) -> &'static str {
    PREFERENCE_OPTIONS
        .iter()
        .find(|opt| opt.value == value)
        .map(|opt| opt.description)
        .unwrap_or("适合大多数人，综合查看风险与建议")
}

#[component]
pub fn PreferenceSelector(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] show_description: bool,
) -> impl IntoView {
    let label_text = label.unwrap_or("人群定位");

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
