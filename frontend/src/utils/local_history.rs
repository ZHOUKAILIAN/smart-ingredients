use serde::{Deserialize, Serialize};
use shared::AnalysisResult;
use web_sys::window;

const LOCAL_HISTORY_KEY: &str = "smart-ingredients-history";
const LOCAL_HISTORY_LIMIT: usize = 50;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalHistoryItem {
    pub id: String,
    pub timestamp: i64,
    pub health_score: i32,
    pub summary: String,
    pub result: AnalysisResult,
    #[serde(default)]
    pub image_path: Option<String>,
}

pub fn load_local_history() -> Vec<LocalHistoryItem> {
    let Some(storage) = window()
        .and_then(|w| w.local_storage().ok().flatten()) else {
        return Vec::new();
    };
    let Some(raw) = storage.get_item(LOCAL_HISTORY_KEY).ok().flatten() else {
        return Vec::new();
    };
    serde_json::from_str(&raw).unwrap_or_default()
}

pub fn save_local_history(items: &[LocalHistoryItem]) -> Result<(), String> {
    let Some(storage) = window()
        .and_then(|w| w.local_storage().ok().flatten()) else {
        return Err("localStorage 不可用".to_string());
    };
    let payload = serde_json::to_string(items)
        .map_err(|_| "本地记录序列化失败".to_string())?;
    storage
        .set_item(LOCAL_HISTORY_KEY, &payload)
        .map_err(|_| "本地记录写入失败".to_string())?;
    Ok(())
}

pub fn add_local_history(item: LocalHistoryItem) -> Result<(), String> {
    let mut items = load_local_history();
    items.retain(|existing| existing.id != item.id);
    items.insert(0, item);
    if items.len() > LOCAL_HISTORY_LIMIT {
        items.truncate(LOCAL_HISTORY_LIMIT);
    }
    save_local_history(&items)
}

pub fn delete_local_history(id: &str) -> Result<(), String> {
    let mut items = load_local_history();
    let original_len = items.len();
    items.retain(|item| item.id != id);
    if items.len() == original_len {
        return Ok(());
    }
    save_local_history(&items)
}

pub fn clear_local_history() -> Result<(), String> {
    save_local_history(&[])
}
