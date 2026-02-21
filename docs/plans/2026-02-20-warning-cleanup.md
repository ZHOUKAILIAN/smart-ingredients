# Warning Cleanup Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Remove all unused/deprecated warnings in backend and smart-ingredients-app without changing runtime behavior.

**Architecture:** No architecture changes. Apply safe, localized refactors (remove unused items, replace deprecated APIs) and keep behavior identical. Use compile warnings as red/green gates (TDD exception approved by user on 2026-02-20).

**Tech Stack:** Rust (Axum, Leptos), SQLx, web-sys.

---

## Task 1: Establish warning baseline

**Files:**
- None

**Step 1: Run checks to capture baseline warnings**
Run: `cargo check -p backend`  
Expected: warns on unused/dead_code in `backend/src/config.rs`, `backend/src/services/image_converter.rs`, `backend/src/services/rules.rs`

Run: `cargo check -p smart-ingredients-app`  
Expected: warns on unused imports/vars/mut, dead_code, deprecated Leptos APIs

**Step 2: Record scope constraints**
- `cargo check --workspace` currently fails in `smart-ingredients-tauri` due to `OUT_DIR` missing; keep out of scope.

**Step 3: Commit**
- No commit (baseline only)

---

## Task 2: Backend unused/dead_code cleanup

**Files:**
- Modify: `backend/src/config.rs`
- Modify: `backend/src/services/image_converter.rs`
- Modify: `backend/src/services/rules.rs`

**Step 1: Make OcrConfig.lang explicitly intentional**
Change OcrConfig field to suppress dead_code while keeping behavior:

```rust
#[derive(Debug, Clone)]
pub struct OcrConfig {
    #[allow(dead_code)]
    pub lang: String,
    pub timeout: Duration,
    pub paddle_url: String,
}
```

**Step 2: Silence optional image helper dead_code**
Keep optional helpers but mark as intentional:

```rust
impl SupportedFormat {
    #[allow(dead_code)]
    pub fn extension(&self) -> &'static str { /* unchanged body */ }
}

#[allow(dead_code)]
pub fn convert_heic_to_jpeg(bytes: &[u8]) -> Result<(Vec<u8>, &'static str)> { /* unchanged body */ }
```

**Step 3: Remove unused RuleEngine.items**
Delete the field and adjust constructor:

```rust
pub struct RuleEngine {
    lookup: HashMap<String, RuleItem>,
    load_error: Option<String>,
}

fn build(items: Vec<RuleItem>) -> Self {
    let mut lookup = HashMap::new();
    /* unchanged */
    Self { lookup, load_error: None }
}
```

**Step 4: Run check to verify warnings removed**
Run: `cargo check -p backend`  
Expected: no unused/dead_code warnings

**Step 5: Commit**
Run:
```bash
git add backend/src/config.rs backend/src/services/image_converter.rs backend/src/services/rules.rs
git commit -m "chore(backend): clean unused warnings"
```

---

## Task 3: Frontend unused import/variable cleanup (pages/components)

**Files:**
- Modify: `frontend/src/components/mod.rs`
- Modify: `frontend/src/pages/analyzing.rs`
- Modify: `frontend/src/pages/capture.rs`
- Modify: `frontend/src/pages/confirm.rs`
- Modify: `frontend/src/pages/detail.rs`
- Modify: `frontend/src/pages/ocr.rs`
- Modify: `frontend/src/pages/result.rs`
- Modify: `frontend/src/pages/summary.rs`

**Step 1: Remove unused imports**
- Delete unused `wasm_bindgen::JsCast` imports in pages
- Remove unused `ExampleImages` import in `capture.rs`
- Remove unused `get_preference_label` import in `confirm.rs`

**Step 2: Fix unused variables**
Rename unused variables to underscore-prefixed to silence warnings:

```rust
let _navigate_for_back = navigate.clone();
let _navigate = use_navigate();
```

**Step 3: Fix unused re-exports**
For `components/mod.rs`, remove unused `pub use` items if not referenced anywhere in crate.  
If a re-export is intentionally kept, add:

```rust
#[allow(unused_imports)]
pub use error_display::ErrorDisplay;
```

Apply this only when removing the re-export would break imports.

**Step 4: Run check to verify warnings reduced**
Run: `cargo check -p smart-ingredients-app`  
Expected: unused import/variable warnings removed in targeted files

**Step 5: Commit**
Run:
```bash
git add frontend/src/components/mod.rs frontend/src/pages/analyzing.rs frontend/src/pages/capture.rs frontend/src/pages/confirm.rs frontend/src/pages/detail.rs frontend/src/pages/ocr.rs frontend/src/pages/result.rs frontend/src/pages/summary.rs
git commit -m "chore(frontend): clean unused imports and variables"
```

---

## Task 4: Frontend unused mut cleanup (services)

**Files:**
- Modify: `frontend/src/services/mod.rs`

**Step 1: Remove unnecessary mut**
Replace patterns like:

```rust
let mut init = RequestInit::new();
```

with:

```rust
let init = RequestInit::new();
```

**Step 2: Run check to verify warnings removed**
Run: `cargo check -p smart-ingredients-app`  
Expected: no unused_mut warnings in `services/mod.rs`

**Step 3: Commit**
Run:
```bash
git add frontend/src/services/mod.rs
git commit -m "chore(frontend): remove unused mut"
```

---

## Task 5: Frontend dead_code cleanup (components/utils/stores)

**Files:**
- Modify: `frontend/src/components/error_display.rs`
- Modify: `frontend/src/components/ingredient_table.rs`
- Modify: `frontend/src/components/loading_spinner.rs`
- Modify: `frontend/src/components/preference_selector.rs`
- Modify: `frontend/src/stores/mod.rs`
- Modify: `frontend/src/utils/export_image.rs`
- Modify: `frontend/src/utils/local_storage.rs`
- Modify: `frontend/src/utils/auth_storage.rs`

**Step 1: Decide per item: use, remove, or allow**
Rules:
- If the item is part of active UI, use it in view or logic.
- If the item is not used anywhere, remove it (and update call sites).
- If item is intentionally reserved, annotate with allow and comment.

Example for intentional reserved fields:

```rust
#[allow(dead_code)]
pub warnings: Vec<String>,
```

Example for unused enum variant:

```rust
#[allow(dead_code)]
Detail,
```

**Step 2: Run check to verify warnings removed**
Run: `cargo check -p smart-ingredients-app`  
Expected: dead_code warnings removed

**Step 3: Commit**
Run:
```bash
git add frontend/src/components/error_display.rs frontend/src/components/ingredient_table.rs frontend/src/components/loading_spinner.rs frontend/src/components/preference_selector.rs frontend/src/stores/mod.rs frontend/src/utils/export_image.rs frontend/src/utils/local_storage.rs frontend/src/utils/auth_storage.rs
git commit -m "chore(frontend): clean dead_code warnings"
```

---

## Task 6: Frontend deprecated API replacement (Leptos + web-sys)

**Files:**
- Modify: `frontend/src/components/bottom_nav.rs`
- Modify: `frontend/src/components/community_share_button.rs`
- Modify: `frontend/src/components/confirm_modal.rs`
- Modify: `frontend/src/components/image_preview.rs`
- Modify: `frontend/src/components/toast.rs`
- Modify: `frontend/src/pages/analyzing.rs`
- Modify: `frontend/src/pages/capture.rs`
- Modify: `frontend/src/pages/confirm.rs`
- Modify: `frontend/src/pages/community.rs`
- Modify: `frontend/src/pages/community_detail.rs`
- Modify: `frontend/src/pages/history.rs`
- Modify: `frontend/src/pages/ocr.rs`
- Modify: `frontend/src/pages/result.rs`
- Modify: `frontend/src/pages/summary.rs`
- Modify: `frontend/src/utils/export_image.rs`
- Modify: `frontend/src/utils/mod.rs`

**Step 1: Replace deprecated Leptos APIs**
- create_effect → Effect::new
```rust
Effect::new(move |_| { /* same body */ });
```
- create_memo → Memo::new
```rust
let memo = Memo::new(move |_| { /* same body */ });
```
- create_signal → signal
```rust
let (value, set_value) = signal(initial_value);
```
- MaybeSignal → Signal  
Update component props signatures to `Signal<T>` where safe.

**Step 2: Replace deprecated web_sys APIs**
- CanvasRenderingContext2d::set_fill_style → use the current recommended setter
- CustomEventInit::detail → `set_detail`

**Step 3: Run check to verify deprecations removed**
Run: `cargo check -p smart-ingredients-app`  
Expected: no deprecated warnings

**Step 4: Commit**
Run:
```bash
git add frontend/src/components/bottom_nav.rs frontend/src/components/community_share_button.rs frontend/src/components/confirm_modal.rs frontend/src/components/image_preview.rs frontend/src/components/toast.rs frontend/src/pages/analyzing.rs frontend/src/pages/capture.rs frontend/src/pages/confirm.rs frontend/src/pages/community.rs frontend/src/pages/community_detail.rs frontend/src/pages/history.rs frontend/src/pages/ocr.rs frontend/src/pages/result.rs frontend/src/pages/summary.rs frontend/src/utils/export_image.rs frontend/src/utils/mod.rs
git commit -m "chore(frontend): replace deprecated APIs"
```

---

## Task 7: Final verification (per checklist)

**Step 1: Start local services**
Run: `docker compose up -d`  
Expected: all services healthy

**Step 2: Run full API flow**
- upload → OCR → confirm → LLM → community create → list → detail → delete  
Expected: successful responses

**Step 3: Run frontend compile check**
Run: `cargo check -p smart-ingredients-app`  
Expected: no warnings

**Step 4: Summarize changes**
Prepare brief report of files touched and verification results.
