//! Preference storage helpers

const PREFERENCE_KEY: &str = "analysis_preference";
pub fn load_preference() -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok().flatten()?;
    let value = storage.get_item(PREFERENCE_KEY).ok().flatten()?;
    let trimmed = value.trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_lowercase())
    }
}

pub fn save_preference(value: &str) {
    let Some(window) = web_sys::window() else { return };
    let Ok(Some(storage)) = window.local_storage() else { return };
    let _ = storage.set_item(PREFERENCE_KEY, value);
}
