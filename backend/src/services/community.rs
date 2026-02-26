use hmac::{Hmac, Mac};
use sha2::Sha256;
use uuid::Uuid;

use crate::errors::AppError;
use shared::{CommunityAuthorType, CommunityCreatePayload};

type HmacSha256 = Hmac<Sha256>;

pub fn hash_share_token(token: &str, key: &str) -> Result<String, AppError> {
    let mut mac = <HmacSha256 as hmac::digest::KeyInit>::new_from_slice(key.as_bytes())
        .map_err(|_| AppError::Internal("分享令牌哈希失败".to_string()))?;
    mac.update(token.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

#[derive(Debug)]
pub struct ValidatedCreate {
    pub author_type: CommunityAuthorType,
    pub user_id: Option<Uuid>,
    pub share_token_hash: Option<String>,
}

pub fn validate_create_payload(
    payload: &CommunityCreatePayload,
    auth_user: Option<Uuid>,
    hash_key: &str,
) -> Result<ValidatedCreate, AppError> {
    if payload.summary_text.trim().is_empty() {
        return Err(AppError::BadRequest("摘要不能为空".to_string()));
    }
    if payload.ingredients_raw.trim().is_empty() {
        return Err(AppError::BadRequest("配料表不能为空".to_string()));
    }
    if !(0..=100).contains(&payload.health_score) {
        return Err(AppError::BadRequest("健康评分超出范围".to_string()));
    }

    match payload.author_type {
        CommunityAuthorType::Anonymous => {
            let token = payload
                .share_token
                .as_ref()
                .filter(|v| !v.trim().is_empty())
                .ok_or_else(|| AppError::BadRequest("缺少分享令牌".to_string()))?;
            let hash = hash_share_token(token, hash_key)?;
            Ok(ValidatedCreate {
                author_type: CommunityAuthorType::Anonymous,
                user_id: None,
                share_token_hash: Some(hash),
            })
        }
        CommunityAuthorType::User => {
            let user_id =
                auth_user.ok_or_else(|| AppError::Unauthorized("请先登录".to_string()))?;
            Ok(ValidatedCreate {
                author_type: CommunityAuthorType::User,
                user_id: Some(user_id),
                share_token_hash: None,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::{CommunityAuthorType, CommunityCardPayload, CommunityCreatePayload};

    #[test]
    fn validate_requires_summary() {
        let payload = CommunityCreatePayload {
            author_type: CommunityAuthorType::Anonymous,
            share_token: Some("token".to_string()),
            source_analysis_id: None,
            summary_text: "".to_string(),
            health_score: 85,
            ingredients_raw: "水".to_string(),
            card_payload: CommunityCardPayload {
                health_score: 85,
                summary: "s".to_string(),
                recommendation: "r".to_string(),
                ingredients: vec![],
                warnings: vec![],
                preference_label: None,
            },
        };
        let err = validate_create_payload(&payload, None, "key").unwrap_err();
        assert!(matches!(err, AppError::BadRequest(_)));
    }

    #[test]
    fn anonymous_requires_token() {
        let payload = CommunityCreatePayload {
            author_type: CommunityAuthorType::Anonymous,
            share_token: None,
            source_analysis_id: None,
            summary_text: "ok".to_string(),
            health_score: 85,
            ingredients_raw: "水".to_string(),
            card_payload: CommunityCardPayload {
                health_score: 85,
                summary: "s".to_string(),
                recommendation: "r".to_string(),
                ingredients: vec![],
                warnings: vec![],
                preference_label: None,
            },
        };
        let err = validate_create_payload(&payload, None, "key").unwrap_err();
        assert!(matches!(err, AppError::BadRequest(_)));
    }

    #[test]
    fn hash_share_token_is_deterministic() {
        let a = hash_share_token("token", "key").unwrap();
        let b = hash_share_token("token", "key").unwrap();
        assert_eq!(a, b);
        assert_eq!(a.len(), 64);
    }
}
