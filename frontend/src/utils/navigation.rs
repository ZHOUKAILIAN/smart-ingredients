pub fn normalize_search(search: &str) -> String {
    if search.is_empty() {
        String::new()
    } else if search.starts_with('?') {
        search.to_string()
    } else {
        format!("?{}", search)
    }
}

pub fn build_full_path(path: &str, search: &str) -> String {
    let normalized = normalize_search(search);
    if normalized.is_empty() {
        path.to_string()
    } else {
        format!("{}{}", path, normalized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_full_path_adds_question_mark_when_missing() {
        assert_eq!(build_full_path("/", "view=scan"), "/?view=scan");
    }

    #[test]
    fn build_full_path_keeps_existing_question_mark() {
        assert_eq!(build_full_path("/", "?view=scan"), "/?view=scan");
    }

    #[test]
    fn build_full_path_handles_empty_search() {
        assert_eq!(build_full_path("/profile", ""), "/profile");
    }
}
