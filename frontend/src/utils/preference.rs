//! Preference storage helpers

use serde_json::{Map, Value};

const PREFERENCE_KEY: &str = "analysis_preference";
pub fn load_preference() -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok().flatten()?;
    let value = storage.get_item(PREFERENCE_KEY).ok().flatten()?;
    let trimmed = value.trim().to_lowercase();
    if trimmed.is_empty() || trimmed == "none" {
        Some("normal".to_string())
    } else {
        Some(trimmed)
    }
}

pub fn save_preference(value: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(Some(storage)) = window.local_storage() else {
        return;
    };
    let _ = storage.set_item(PREFERENCE_KEY, value);
}

pub fn merge_preferences(
    base: Value,
    selection: Option<&str>,
    has_seen_onboarding: Option<bool>,
) -> Value {
    let mut map = match base {
        Value::Object(value) => value,
        _ => Map::new(),
    };
    if let Some(value) = selection {
        if !value.trim().is_empty() {
            map.insert("selection".to_string(), Value::String(value.to_string()));
        }
    }
    if let Some(value) = has_seen_onboarding {
        map.insert("has_seen_onboarding".to_string(), Value::Bool(value));
    }
    Value::Object(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn merge_preferences_preserves_existing_fields() {
        let base = json!({"foo": 1});
        let merged = merge_preferences(base, Some("normal"), Some(true));
        assert_eq!(merged.get("foo").and_then(|v| v.as_i64()), Some(1));
        assert_eq!(
            merged.get("selection").and_then(|v| v.as_str()),
            Some("normal")
        );
        assert_eq!(
            merged.get("has_seen_onboarding").and_then(|v| v.as_bool()),
            Some(true)
        );
    }

    #[test]
    fn merge_preferences_handles_non_object_base() {
        let base = json!(null);
        let merged = merge_preferences(base, Some("normal"), None);
        assert_eq!(
            merged.get("selection").and_then(|v| v.as_str()),
            Some("normal")
        );
        assert!(merged.get("has_seen_onboarding").is_none());
    }

    #[test]
    fn merge_preferences_keeps_existing_selection_when_none() {
        let base = json!({"selection": "elderly"});
        let merged = merge_preferences(base, None, Some(true));
        assert_eq!(
            merged.get("selection").and_then(|v| v.as_str()),
            Some("elderly")
        );
        assert_eq!(
            merged.get("has_seen_onboarding").and_then(|v| v.as_bool()),
            Some(true)
        );
    }
}
