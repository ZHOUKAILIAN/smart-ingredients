use crate::utils::community_share_storage::CommunityShareRecord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShareButtonState {
    ShowShare,
    Shared,
}

pub fn should_fetch_page(requested_page: i64, last_requested: Option<i64>) -> bool {
    Some(requested_page) != last_requested
}

pub fn should_fetch_key<T: PartialEq>(requested: &T, last_requested: Option<&T>) -> bool {
    match last_requested {
        Some(last) => last != requested,
        None => true,
    }
}

pub fn share_button_state(has_record: bool) -> ShareButtonState {
    if has_record {
        ShareButtonState::Shared
    } else {
        ShareButtonState::ShowShare
    }
}

pub fn should_show_delete_button(has_record: bool) -> bool {
    has_record
}

pub fn find_share_record_by_post_id(
    records: &[CommunityShareRecord],
    post_id: &str,
) -> Option<CommunityShareRecord> {
    records.iter().find(|item| item.post_id == post_id).cloned()
}

pub fn format_community_datetime(iso_string: &str) -> String {
    let base = iso_string.split('.').next().unwrap_or(iso_string);
    let base = base.split('+').next().unwrap_or(base);
    let base = base.strip_suffix('Z').unwrap_or(base);
    base.replace('T', " ")
}

pub fn community_page_title() -> Option<&'static str> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::community_share_storage::CommunityShareRecord;

    #[test]
    fn should_fetch_page_skips_when_same_page() {
        assert!(!should_fetch_page(1, Some(1)));
    }

    #[test]
    fn should_fetch_page_allows_new_page() {
        assert!(should_fetch_page(2, Some(1)));
    }

    #[test]
    fn should_fetch_key_skips_when_same() {
        let last = Some("same".to_string());
        assert!(!should_fetch_key(&"same".to_string(), last.as_ref()));
    }

    #[test]
    fn should_fetch_key_allows_when_different() {
        let last = Some("one".to_string());
        assert!(should_fetch_key(&"two".to_string(), last.as_ref()));
    }

    #[test]
    fn share_button_state_hides_when_shared() {
        assert_eq!(share_button_state(true), ShareButtonState::Shared);
    }

    #[test]
    fn share_button_state_shows_when_not_shared() {
        assert_eq!(share_button_state(false), ShareButtonState::ShowShare);
    }

    #[test]
    fn find_share_record_by_post_id_matches() {
        let records = vec![
            CommunityShareRecord {
                analysis_id: "a1".to_string(),
                post_id: "p1".to_string(),
                author_type: "anonymous".to_string(),
                share_token: Some("t1".to_string()),
            },
            CommunityShareRecord {
                analysis_id: "a2".to_string(),
                post_id: "p2".to_string(),
                author_type: "anonymous".to_string(),
                share_token: None,
            },
        ];
        let found = find_share_record_by_post_id(&records, "p2").expect("record");
        assert_eq!(found.analysis_id, "a2");
    }

    #[test]
    fn format_community_datetime_removes_fractional_and_offset() {
        let input = "2026-02-19T12:00:00.123456+00:00";
        assert_eq!(format_community_datetime(input), "2026-02-19 12:00:00");
    }

    #[test]
    fn format_community_datetime_removes_z_suffix() {
        let input = "2026-02-19T12:00:00Z";
        assert_eq!(format_community_datetime(input), "2026-02-19 12:00:00");
    }

    #[test]
    fn community_page_title_is_hidden() {
        assert!(community_page_title().is_none());
    }
}
