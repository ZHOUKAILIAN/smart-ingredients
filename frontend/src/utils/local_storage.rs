use web_sys::window;

pub const KEY_LAST_TAB: &str = "lastTab";

pub fn get_last_tab() -> Option<String> {
    window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item(KEY_LAST_TAB).ok().flatten())
}

pub fn set_last_tab(tab: &str) {
    if let Some(storage) = window()
        .and_then(|w| w.local_storage().ok().flatten())
    {
        let _ = storage.set_item(KEY_LAST_TAB, tab);
    }
}
