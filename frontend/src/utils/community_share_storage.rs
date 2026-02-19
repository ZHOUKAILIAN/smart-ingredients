use serde::{Deserialize, Serialize};
use web_sys::window;

const COMMUNITY_SHARE_KEY: &str = "smart-ingredients-community-shares";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityShareRecord {
    pub analysis_id: String,
    pub post_id: String,
    pub author_type: String,
    pub share_token: Option<String>,
}

pub fn load_share_records() -> Vec<CommunityShareRecord> {
    let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) else {
        return Vec::new();
    };
    let Some(raw) = storage.get_item(COMMUNITY_SHARE_KEY).ok().flatten() else {
        return Vec::new();
    };
    serde_json::from_str(&raw).unwrap_or_default()
}

pub fn save_share_records(records: &[CommunityShareRecord]) -> Result<(), String> {
    let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) else {
        return Err("localStorage 不可用".to_string());
    };
    let payload =
        serde_json::to_string(records).map_err(|_| "分享记录序列化失败".to_string())?;
    storage
        .set_item(COMMUNITY_SHARE_KEY, &payload)
        .map_err(|_| "分享记录写入失败".to_string())?;
    Ok(())
}

pub fn get_share_record(analysis_id: &str) -> Option<CommunityShareRecord> {
    load_share_records()
        .into_iter()
        .find(|item| item.analysis_id == analysis_id)
}

pub fn upsert_share_record(record: CommunityShareRecord) -> Result<(), String> {
    let mut records = load_share_records();
    records.retain(|item| item.analysis_id != record.analysis_id);
    records.insert(0, record);
    save_share_records(&records)
}

pub fn remove_share_record(analysis_id: &str) -> Result<(), String> {
    let mut records = load_share_records();
    let before = records.len();
    records.retain(|item| item.analysis_id != analysis_id);
    if records.len() == before {
        return Ok(());
    }
    save_share_records(&records)
}
