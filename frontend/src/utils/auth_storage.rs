//! Auth token storage helpers

const ACCESS_TOKEN_KEY: &str = "auth_access_token";
const REFRESH_TOKEN_KEY: &str = "auth_refresh_token";

pub fn load_access_token() -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok().flatten()?;
    let value = storage.get_item(ACCESS_TOKEN_KEY).ok().flatten()?;
    let trimmed = value.trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

pub fn load_refresh_token() -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok().flatten()?;
    let value = storage.get_item(REFRESH_TOKEN_KEY).ok().flatten()?;
    let trimmed = value.trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

pub fn save_tokens(access_token: &str, refresh_token: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(Some(storage)) = window.local_storage() else {
        return;
    };
    let _ = storage.set_item(ACCESS_TOKEN_KEY, access_token);
    let _ = storage.set_item(REFRESH_TOKEN_KEY, refresh_token);
}

pub fn save_access_token(access_token: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(Some(storage)) = window.local_storage() else {
        return;
    };
    let _ = storage.set_item(ACCESS_TOKEN_KEY, access_token);
}

pub fn clear_tokens() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(Some(storage)) = window.local_storage() else {
        return;
    };
    let _ = storage.remove_item(ACCESS_TOKEN_KEY);
    let _ = storage.remove_item(REFRESH_TOKEN_KEY);
}
