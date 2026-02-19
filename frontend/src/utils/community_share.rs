use shared::{AnalysisResult, CommunityCardIngredient, CommunityCardPayload};

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

#[cfg(test)]
mod tests {
    use super::*;
    use shared::AnalysisResult;

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
}
