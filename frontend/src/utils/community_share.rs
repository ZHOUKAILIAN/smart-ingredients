use shared::{
    AnalysisResult, CommunityAuthorType, CommunityCardIngredient, CommunityCardPayload,
    CommunityCreatePayload,
};
use uuid::Uuid;

pub fn build_summary_text(result: &AnalysisResult) -> String {
    result
        .focus_summary
        .clone()
        .filter(|summary| !summary.trim().is_empty())
        .unwrap_or_else(|| result.summary.clone())
}

pub fn build_card_payload(
    result: &AnalysisResult,
    preference_label: Option<String>,
) -> CommunityCardPayload {
    CommunityCardPayload {
        health_score: result.health_score,
        summary: build_summary_text(result),
        recommendation: result.recommendation.clone(),
        ingredients: result
            .ingredients
            .iter()
            .map(|item| CommunityCardIngredient {
                name: item.name.clone(),
                risk_level: item.risk_level.clone(),
                description: item.description.clone().unwrap_or_default(),
                is_focus: false,
            })
            .collect(),
        warnings: result.warnings.iter().map(|w| w.message.clone()).collect(),
        preference_label,
    }
}

pub fn select_ingredients_raw(
    confirmed_text: Option<&str>,
    ocr_text: Option<&str>,
    fallback: &str,
) -> String {
    confirmed_text
        .filter(|text| !text.trim().is_empty())
        .or_else(|| ocr_text.filter(|text| !text.trim().is_empty()))
        .map(|text| text.to_string())
        .unwrap_or_else(|| fallback.to_string())
}

pub fn build_create_payload(
    result: &AnalysisResult,
    share_token: Option<String>,
    analysis_id: Option<Uuid>,
    confirmed_text: Option<&str>,
    ocr_text: Option<&str>,
    preference_label: Option<String>,
) -> CommunityCreatePayload {
    let summary_text = build_summary_text(result);
    let ingredients_raw = select_ingredients_raw(confirmed_text, ocr_text, &summary_text);
    CommunityCreatePayload {
        author_type: CommunityAuthorType::Anonymous,
        share_token,
        source_analysis_id: analysis_id,
        summary_text,
        health_score: result.health_score,
        ingredients_raw,
        card_payload: build_card_payload(result, preference_label),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::AnalysisResult;
    use uuid::Uuid;

    #[test]
    fn build_summary_prefers_focus_summary() {
        let result = AnalysisResult {
            health_score: 80,
            summary: "normal".to_string(),
            table: vec![],
            ingredients: vec![],
            warnings: vec![],
            recommendation: "ok".to_string(),
            overall_assessment: None,
            focus_summary: Some("focus".to_string()),
            focus_ingredients: None,
            score_breakdown: None,
            rule_hits: vec![],
            confidence: None,
        };
        let summary = build_summary_text(&result);
        assert_eq!(summary, "focus");
    }

    #[test]
    fn select_ingredients_raw_prefers_confirmed_text() {
        let selected = select_ingredients_raw(Some("确认文本"), Some("ocr文本"), "fallback");
        assert_eq!(selected, "确认文本");
    }

    #[test]
    fn select_ingredients_raw_falls_back_to_ocr() {
        let selected = select_ingredients_raw(Some("   "), Some("ocr文本"), "fallback");
        assert_eq!(selected, "ocr文本");
    }

    #[test]
    fn select_ingredients_raw_uses_fallback_when_empty() {
        let selected = select_ingredients_raw(Some(""), Some("  "), "fallback");
        assert_eq!(selected, "fallback");
    }

    #[test]
    fn build_create_payload_sets_required_fields() {
        let result = AnalysisResult {
            health_score: 92,
            summary: "summary".to_string(),
            table: vec![],
            ingredients: vec![],
            warnings: vec![],
            recommendation: "ok".to_string(),
            overall_assessment: None,
            focus_summary: Some("focus".to_string()),
            focus_ingredients: None,
            score_breakdown: None,
            rule_hits: vec![],
            confidence: None,
        };
        let analysis_id = Uuid::new_v4();
        let payload = build_create_payload(
            &result,
            Some("token".to_string()),
            Some(analysis_id),
            Some("confirmed"),
            Some("ocr"),
            Some("偏好".to_string()),
        );

        assert_eq!(payload.author_type, shared::CommunityAuthorType::Anonymous);
        assert_eq!(payload.share_token.as_deref(), Some("token"));
        assert_eq!(payload.source_analysis_id, Some(analysis_id));
        assert_eq!(payload.summary_text, "focus");
        assert_eq!(payload.ingredients_raw, "confirmed");
        assert_eq!(payload.health_score, 92);
        assert_eq!(payload.card_payload.preference_label.as_deref(), Some("偏好"));
    }
}
