use crate::utils::community_share_storage::CommunityShareRecord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShareButtonState {
    ShowShare,
    Shared,
}

pub fn should_fetch_page(requested_page: i64, last_requested: Option<i64>) -> bool {
    Some(requested_page) != last_requested
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
}
