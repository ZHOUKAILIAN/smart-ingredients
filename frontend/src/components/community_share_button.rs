//! Share analysis result to community.

use leptos::prelude::*;
use leptos::task::spawn_local;
use uuid::Uuid;

use crate::services;
use crate::stores::ToastLevel;
use crate::utils::{community_share, community_share_storage, emit_toast};
use crate::utils::export_image::{data_url_to_blob, ExportData, ExportIngredient};

#[component]
pub fn CommunityShareButton(
    analysis_id: Option<uuid::Uuid>,
    analysis_result: shared::AnalysisResult,
    confirmed_text: Option<String>,
    ocr_text: Option<String>,
    preference_label: Option<String>,
) -> impl IntoView {
    let publishing = RwSignal::new(false);
    let share_state = RwSignal::new(None::<community_share_storage::CommunityShareRecord>);

    create_effect(move |_| {
        let Some(id) = analysis_id else {
            share_state.set(None);
            return;
        };
        share_state.set(community_share_storage::get_share_record(&id.to_string()));
    });

    let on_publish = Callback::new(move |_: web_sys::MouseEvent| {
        if publishing.get() {
            return;
        }
        let Some(id) = analysis_id else {
            emit_toast(ToastLevel::Error, "分享失败", "缺少分析记录编号");
            return;
        };

        publishing.set(true);
        let result = analysis_result.clone();
        let confirmed = confirmed_text.clone();
        let ocr = ocr_text.clone();
        let preference = preference_label.clone().filter(|label| !label.trim().is_empty());
        spawn_local(async move {
            let share_token = Uuid::new_v4().to_string();
            let create_payload = community_share::build_create_payload(
                &result,
                Some(share_token.clone()),
                Some(id),
                confirmed.as_deref(),
                ocr.as_deref(),
                preference.clone(),
            );
            let card_payload = create_payload.card_payload.clone();

            let export_data = ExportData {
                health_score: result.health_score,
                recommendation: result.recommendation.clone(),
                ingredients: result
                    .ingredients
                    .iter()
                    .map(|item| ExportIngredient {
                        name: item.name.clone(),
                        risk_level: item.risk_level.clone(),
                        description: item.description.clone().unwrap_or_default(),
                        is_focus: false,
                    })
                    .collect(),
                warnings: result.warnings.iter().map(|w| w.message.clone()).collect(),
                summary: card_payload.summary.clone(),
                preference_label: preference.clone().unwrap_or_default(),
            };

            let image_blob = crate::utils::export_image::export_to_data_url(&export_data)
                .ok()
                .and_then(|url| data_url_to_blob(&url).ok());

            match services::create_community_post(&create_payload, image_blob).await {
                Ok(created) => {
                    let record = community_share_storage::CommunityShareRecord {
                        analysis_id: id.to_string(),
                        post_id: created.id.to_string(),
                        author_type: "anonymous".to_string(),
                        share_token: Some(share_token.clone()),
                    };
                    if let Err(err) = community_share_storage::upsert_share_record(record.clone()) {
                        emit_toast(ToastLevel::Warning, "分享成功", &format!("记录保存失败: {err}"));
                    } else {
                        emit_toast(ToastLevel::Success, "已分享到社区", "分享成功");
                    }
                    share_state.set(Some(record));
                }
                Err(err) => {
                    emit_toast(ToastLevel::Error, "分享失败", &err);
                }
            }
            publishing.set(false);
        });
    });

    let on_delete = Callback::new(move |_: web_sys::MouseEvent| {
        if publishing.get() {
            return;
        }
        let Some(record) = share_state.get() else {
            return;
        };
        let Ok(post_id) = Uuid::parse_str(&record.post_id) else {
            emit_toast(ToastLevel::Error, "取消失败", "无效的分享记录");
            return;
        };
        publishing.set(true);
        spawn_local(async move {
            let share_token = record.share_token.clone();
            match services::delete_community_post(post_id, share_token).await {
                Ok(()) => {
                    if let Err(err) =
                        community_share_storage::remove_share_record(&record.analysis_id)
                    {
                        emit_toast(ToastLevel::Warning, "已取消分享", &err);
                    } else {
                        emit_toast(ToastLevel::Success, "已取消分享", "分享已删除");
                    }
                    share_state.set(None);
                }
                Err(err) => {
                    emit_toast(ToastLevel::Error, "取消失败", &err);
                }
            }
            publishing.set(false);
        });
    });

    view! {
        <Show
            when=move || share_state.get().is_some()
            fallback=move || {
                let on_publish = on_publish;
                view! {
                    <button class="secondary-cta" on:click=move |ev| on_publish.run(ev) disabled=move || publishing.get()>
                        <span>{move || if publishing.get() { "分享中…" } else { "分享到社区" }}</span>
                    </button>
                }
            }
        >
            {move || {
                let on_delete = on_delete;
                view! {
                    <button class="secondary-cta" on:click=move |ev| on_delete.run(ev) disabled=move || publishing.get()>
                        <span>{move || if publishing.get() { "处理中…" } else { "取消分享" }}</span>
                    </button>
                }
            }}
        </Show>
    }
}
