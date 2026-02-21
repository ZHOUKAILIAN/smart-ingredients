# Community Share UI Fixes Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix community list/detail infinite requests, hide share button once shared, and move delete actions to community list + detail.

**Architecture:** Keep backend unchanged. Add a small frontend utility module for UI decision helpers (test-first). Update community list/detail pages to use safe fetch logic and show delete actions only for locally shared posts. Update share button component to show a shared status label instead of a delete action.

**Tech Stack:** Rust (Leptos/Tauri), shared crate types, web-sys for localStorage.

---

## Status

- [x] Task 1: Add UI helper module with tests
- [x] Task 2: Fix community list fetch loop + add delete action
- [x] Task 3: Fix community detail fetch loop + add delete action
- [x] Task 4: Hide share button after sharing (show status only)
- [x] Task 5: Documentation update (API reference alignment - verified, no change needed)

---

### Task 1: Add UI helper module with tests (@superpowers:test-driven-development)

**Files:**
- Create: `frontend/src/utils/community_ui.rs`
- Modify: `frontend/src/utils/mod.rs`
- Test: `frontend/src/utils/community_ui.rs`

**Step 1: Write the failing tests**

```rust
// frontend/src/utils/community_ui.rs
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
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p smart-ingredients-app community_ui::tests::should_fetch_page_skips_when_same_page`  
Expected: FAIL with "cannot find module `community_ui`" or missing items.

**Step 3: Write minimal implementation**

```rust
// frontend/src/utils/community_ui.rs
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
```

Update `frontend/src/utils/mod.rs`:

```rust
pub mod community_ui;
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p smart-ingredients-app community_ui::tests::should_fetch_page_skips_when_same_page`  
Expected: PASS

**Step 5: Commit**

```bash
git add frontend/src/utils/community_ui.rs frontend/src/utils/mod.rs
git commit -m "feat(frontend): add community ui helpers"
```

---

### Task 2: Fix community list fetch loop + add delete action

**Files:**
- Modify: `frontend/src/pages/community.rs`
- Modify: `frontend/src/styles/app.css`

**Step 1: Write a failing test**

Re-run the Task 1 test first (serves as guard for helper usage).  
Run: `cargo test -p smart-ingredients-app community_ui::tests::should_fetch_page_skips_when_same_page`  
Expected: PASS (existing test used for red/green here is not needed because code change is UI wiring).

**Step 2: Write minimal implementation**

- Track `last_requested_page` and use `community_ui::should_fetch_page`.
- Remove `loading.get()` from reactive dependency (no more loop).
- Load share records once into a signal and refresh after delete.
- Add delete button inside each card when a local share record exists.
- Stop propagation on delete click.
- On delete success: remove share record, refresh local share records, and remove the list item.

**Step 3: Verify behavior manually**

- Open community list page.
- Confirm it loads once (no repeated network requests).
- For a locally shared post, delete button appears; clicking deletes and removes the item.

**Step 4: Commit**

```bash
git add frontend/src/pages/community.rs frontend/src/styles/app.css
git commit -m "fix(frontend): stop community list request loop and add delete action"
```

---

### Task 3: Fix community detail fetch loop + add delete action

**Files:**
- Modify: `frontend/src/pages/community_detail.rs`
- Modify: `frontend/src/styles/app.css`

**Step 1: Write a failing test**

Re-run the Task 1 test first (guards helper behavior).  
Run: `cargo test -p smart-ingredients-app community_ui::tests::share_button_state_hides_when_shared`  
Expected: PASS

**Step 2: Write minimal implementation**

- Remove `loading.get()` from reactive dependency (no more loop).
- Compute local share record from `detail.id` + local storage using `find_share_record_by_post_id`.
- When record exists, show delete button.
- On delete success: remove local share record, show toast, navigate back to `/community`.

**Step 3: Verify behavior manually**

- Open a community detail that was shared from this device.
- Delete button appears and works; returns to list.

**Step 4: Commit**

```bash
git add frontend/src/pages/community_detail.rs frontend/src/styles/app.css
git commit -m "fix(frontend): stop community detail request loop and add delete action"
```

---

### Task 4: Hide share button after sharing (show status only)

**Files:**
- Modify: `frontend/src/components/community_share_button.rs`
- Modify: `frontend/src/styles/app.css`

**Step 1: Write a failing test**

Re-run the Task 1 test first (guards share button state).  
Run: `cargo test -p smart-ingredients-app community_ui::tests::share_button_state_hides_when_shared`  
Expected: PASS

**Step 2: Write minimal implementation**

- Use `community_ui::share_button_state` to choose view.
- When shared: show a status text like “已分享”，不显示按钮。
- Remove delete action from this component.

**Step 3: Verify behavior manually**

- Share once from result/summary; button disappears and shows status.

**Step 4: Commit**

```bash
git add frontend/src/components/community_share_button.rs frontend/src/styles/app.css
git commit -m "fix(frontend): hide share button after sharing"
```

---

### Task 5: Documentation update

**Files:**
- Modify: `docs/api/api-reference.md` (if any UI interaction text needs alignment)

**Step 1: Verify docs are aligned**

Check that community delete entry points are documented; update if missing.

**Step 2: Commit**

```bash
git add docs/api/api-reference.md
git commit -m "docs: clarify community delete entry points"
```

---

Plan complete and saved to `docs/plans/2026-02-19-community-share-ui-fixes.md`.
