use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CommunityAuthorType {
    Anonymous,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityCardIngredient {
    pub name: String,
    pub risk_level: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub is_focus: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityCardPayload {
    pub health_score: i32,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub recommendation: String,
    #[serde(default)]
    pub ingredients: Vec<CommunityCardIngredient>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub preference_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityCreatePayload {
    pub author_type: CommunityAuthorType,
    #[serde(default)]
    pub share_token: Option<String>,
    #[serde(default)]
    pub source_analysis_id: Option<Uuid>,
    pub summary_text: String,
    pub health_score: i32,
    pub ingredients_raw: String,
    pub card_payload: CommunityCardPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityPostCreated {
    pub id: Uuid,
    pub created_at: String,
    #[serde(default)]
    pub card_image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityPostListItem {
    pub id: Uuid,
    pub summary_text: String,
    pub health_score: i32,
    #[serde(default)]
    pub card_image_url: Option<String>,
    pub author_label: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityPostDetail {
    pub id: Uuid,
    pub summary_text: String,
    pub health_score: i32,
    pub ingredients_raw: String,
    pub card_payload: CommunityCardPayload,
    #[serde(default)]
    pub card_image_url: Option<String>,
    pub author_label: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityPostListResponse {
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub items: Vec<CommunityPostListItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityDeleteRequest {
    #[serde(default)]
    pub share_token: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn author_type_serializes_as_lowercase() {
        let json = serde_json::to_string(&CommunityAuthorType::Anonymous).unwrap();
        assert_eq!(json, "\"anonymous\"");
    }

    #[test]
    fn create_payload_roundtrip() {
        let payload = CommunityCreatePayload {
            author_type: CommunityAuthorType::Anonymous,
            share_token: Some("token-123".to_string()),
            source_analysis_id: None,
            summary_text: "summary".to_string(),
            health_score: 85,
            ingredients_raw: "水、乌龙茶".to_string(),
            card_payload: CommunityCardPayload {
                health_score: 85,
                summary: "summary".to_string(),
                recommendation: "ok".to_string(),
                ingredients: vec![],
                warnings: vec![],
                preference_label: None,
            },
        };
        let json = serde_json::to_string(&payload).unwrap();
        let back: CommunityCreatePayload = serde_json::from_str(&json).unwrap();
        assert_eq!(back.summary_text, "summary");
        assert_eq!(back.health_score, 85);
    }
}
