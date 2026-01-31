use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=API_BASE");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default());
    let mut candidates = Vec::new();
    if !manifest_dir.as_os_str().is_empty() {
        candidates.push(manifest_dir.join(".env"));
        candidates.push(manifest_dir.join("..").join(".env"));
    } else {
        candidates.push(PathBuf::from(".env"));
        candidates.push(PathBuf::from("../.env"));
    }

    for path in &candidates {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    if let Some(value) = normalized_env_var("API_BASE") {
        set_api_base(value);
        return;
    }

    for path in candidates {
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Some(value) = parse_env_value(&contents, "API_BASE") {
                set_api_base(value);
                return;
            }
        }
    }

    panic!(
        "API_BASE is required. Set API_BASE in environment or in .env (repo root or frontend/.env)."
    );
}

fn normalized_env_var(key: &str) -> Option<String> {
    env::var(key).ok().and_then(normalize_value)
}

fn normalize_value(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    let normalized = trimmed.trim_end_matches('/').to_string();
    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

fn set_api_base(value: String) {
    println!("cargo:rustc-env=API_BASE={}", value);
}

fn parse_env_value(contents: &str, key: &str) -> Option<String> {
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let line = line.strip_prefix("export ").unwrap_or(line);
        let mut parts = line.splitn(2, '=');
        let k = parts.next()?.trim();
        if k != key {
            continue;
        }
        let raw = parts.next().unwrap_or("").trim();
        if raw.is_empty() {
            continue;
        }
        let value = strip_inline_comment(raw);
        let value = strip_quotes(value);
        if value.is_empty() {
            continue;
        }
        return normalize_value(value.to_string());
    }
    None
}

fn strip_inline_comment(value: &str) -> &str {
    if value.starts_with('"') || value.starts_with('\'') {
        return value;
    }
    value.split('#').next().unwrap_or(value).trim()
}

fn strip_quotes(value: &str) -> &str {
    let bytes = value.as_bytes();
    if bytes.len() >= 2 {
        let first = bytes[0];
        let last = bytes[bytes.len() - 1];
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return &value[1..value.len() - 1];
        }
    }
    value
}
