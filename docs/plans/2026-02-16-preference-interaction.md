# 人群定位交互与导航修复 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 修复人群定位强制选择与导航异常，默认“普通人群”可直接体验，并压缩定位页步骤区视觉占比。

**Architecture:** 仅前端改动。通过本地偏好初始化 + 已登录时偏好同步，移除强制跳转；新增纯函数修复 query 拼接；调整 Onboarding 页样式与跳转目标。

**Tech Stack:** Rust (Leptos + Tauri), LocalStorage, existing preferences API

---

### Task 1: Add query normalization helper with unit tests

**Files:**
- Create: `frontend/src/utils/navigation.rs`
- Modify: `frontend/src/utils/mod.rs`

**Step 1: Write the failing test**

```rust
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
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p smart-ingredients-app navigation::tests::build_full_path_adds_question_mark_when_missing`

Expected: FAIL with "cannot find function `build_full_path`"

**Step 3: Write minimal implementation**

```rust
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
```

Add to `frontend/src/utils/mod.rs`:

```rust
pub mod navigation;
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p smart-ingredients-app navigation::tests::build_full_path_adds_question_mark_when_missing`

Expected: PASS

**Step 5: Commit**

```bash
git add frontend/src/utils/navigation.rs frontend/src/utils/mod.rs
git commit -m "feat: add query normalization helper"
```

---

### Task 2: Fix BottomNav query concatenation

**Files:**
- Modify: `frontend/src/components/bottom_nav.rs`

**Step 1: Write the failing test**

Reuse Task 1 tests (they cover the logic used by BottomNav).

**Step 2: Run test to verify it fails (if helper not wired yet)**

Run: `cargo test -p smart-ingredients-app navigation::tests::build_full_path_adds_question_mark_when_missing`

Expected: PASS (helper already added). This step validates the helper before wiring.

**Step 3: Write minimal implementation**

Replace the `full_path` build with the helper:

```rust
use crate::utils::navigation::build_full_path;

let search = location.search.get();
let full_path = build_full_path(path.as_str(), search.as_str());
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p smart-ingredients-app navigation::tests::build_full_path_adds_question_mark_when_missing`

Expected: PASS

**Step 5: Commit**

```bash
git add frontend/src/components/bottom_nav.rs
git commit -m "feat: fix bottom nav query concatenation"
```

---

### Task 3: Initialize default preference on app start + server sync

**Files:**
- Modify: `frontend/src/lib.rs`

**Step 1: Write the failing test**

No new unit test (logic depends on web storage + async API). Add a manual verification note in Task 7.

**Step 2: Implement minimal changes**

- Import `load_preference` and `emit_toast` + `ToastLevel`.
- Initialize `analysis_preference` from local storage; if missing, save and set "normal".
- After `fetch_preferences`, if server has no `selection`, push local value to server.

Pseudo-code:

```rust
let local_pref = load_preference();
let resolved_pref = local_pref.clone().unwrap_or_else(|| "normal".to_string());
if local_pref.is_none() {
    save_preference(&resolved_pref);
}
analysis_preference.set(Some(resolved_pref.clone()));

// ... inside spawn_local, after fetch_preferences
if let Some(value) = prefs.preferences.get("selection").and_then(|v| v.as_str()) {
    save_preference(value);
    auth_state.analysis_preference.set(Some(value.to_string()));
} else if let Some(local_value) = auth_state.analysis_preference.get() {
    if let Err(err) = services::update_preferences(json!({ "selection": local_value })).await {
        emit_toast(ToastLevel::Error, "同步失败", &err);
    }
}
```

**Step 3: Run tests**

Run: `cargo test -p smart-ingredients-app navigation::tests::build_full_path_adds_question_mark_when_missing`

Expected: PASS (ensures no unrelated breakage)

**Step 4: Commit**

```bash
git add frontend/src/lib.rs
git commit -m "feat: default preference initialization and sync"
```

---

### Task 4: Remove forced onboarding redirect + adjust onboarding navigation

**Files:**
- Modify: `frontend/src/pages/capture.rs`
- Modify: `frontend/src/pages/onboarding.rs`

**Step 1: Write the failing test**

No unit test (UI navigation). Add manual verification in Task 7.

**Step 2: Implement minimal changes**

- Remove the `nav("/onboarding")` redirect in `CapturePage` when no preference.
- Update onboarding save/skip navigation target from `/?view=scan` to `/`.

**Step 3: Run tests**

Run: `cargo test -p smart-ingredients-app navigation::tests::build_full_path_adds_question_mark_when_missing`

Expected: PASS

**Step 4: Commit**

```bash
git add frontend/src/pages/capture.rs frontend/src/pages/onboarding.rs
git commit -m "feat: allow skip preference and return home"
```

---

### Task 5: Update login/register post-auth routing (no forced onboarding)

**Files:**
- Modify: `frontend/src/pages/login.rs`
- Modify: `frontend/src/pages/register.rs`

**Step 1: Write the failing test**

No unit test (UI flow). Add manual verification in Task 7.

**Step 2: Implement minimal changes**

- Always route to `/` after login/register.
- If local preference exists, attempt `update_preferences`; on error show toast.
- If no local preference, use default "normal" and save before updating.

**Step 3: Run tests**

Run: `cargo test -p smart-ingredients-app navigation::tests::build_full_path_adds_question_mark_when_missing`

Expected: PASS

**Step 4: Commit**

```bash
git add frontend/src/pages/login.rs frontend/src/pages/register.rs
git commit -m "feat: avoid forced onboarding after auth"
```

---

### Task 6: Compact onboarding step styles

**Files:**
- Modify: `frontend/src/pages/onboarding.rs`
- Modify: `frontend/src/styles/app.css`

**Step 1: Write the failing test**

No unit test (CSS change). Add manual verification in Task 7.

**Step 2: Implement minimal changes**

- Add compact classes in onboarding step list, e.g. `analysis-list compact`, `analysis-item compact`.
- Add scoped CSS (only affects `analysis-list.compact`). Reduce padding/gap/font sizes.

Example CSS:

```css
.analysis-list.compact {
  gap: 8px;
}

.analysis-list.compact .analysis-item {
  padding: 10px 12px;
}

.analysis-list.compact .analysis-summary {
  font-size: 12px;
  font-weight: 600;
}

.analysis-list.compact .analysis-desc {
  font-size: 11px;
  margin-top: 4px;
}
```

**Step 3: Run tests**

Run: `cargo test -p smart-ingredients-app navigation::tests::build_full_path_adds_question_mark_when_missing`

Expected: PASS

**Step 4: Commit**

```bash
git add frontend/src/pages/onboarding.rs frontend/src/styles/app.css
git commit -m "feat: compact onboarding steps"
```

---

### Task 7: Verification (per project checklist)

**Step 1: Start local services**

Run: `docker compose up -d`

Expected: all services healthy

**Step 2: Run full API flow end-to-end**

Follow `docs/run/integration-testing.md` (or latest guide). Expected: full upload → OCR → analyze → result works.

**Step 3: Frontend compile check**

Run: `cargo check -p smart-ingredients-app`

Expected: PASS

**Step 4: Manual UI verification**

- 首页不再强制跳转人群定位页
- 未选择人群可进入个人中心
- Onboarding 保存/跳过后回首页
- 个人中心 → 首页不出现 `/view=scan` 404
- 步骤区字体更小、间距更紧凑

---

### Task 8: Git workflow

**Step 1: Create branch**

```bash
git checkout -b feat/preference-interaction
```

**Step 2: Push and open PR**

```bash
git push -u origin feat/preference-interaction
```

Open PR following repo template.
