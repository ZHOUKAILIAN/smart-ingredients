# UI Guidelines Fixes Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix Web Interface Guidelines findings with minimal CSS-only changes while preserving the current Figma-derived style.

**Architecture:** Small, targeted updates in `frontend/src/styles/app.css` only. No structural markup changes. Keep hover/focus/typography adjustments consistent with existing tokens and glassmorphism palette.

**Tech Stack:** Rust + Leptos frontend, CSS in `frontend/src/styles/app.css`.

---

### Task 1: Update CSS for hover, focus, long text, numeric alignment

**Files:**
- Modify: `frontend/src/styles/app.css`

**Step 1: Reproduce the failing guideline review**
- Run the web-design-guidelines review on:
  - `frontend/src/styles/app.css`
  - `frontend/src/styles/bottom-nav.css`
  - `frontend/src/components/bottom_nav.rs`
  - `frontend/src/pages/community.rs`
  - `frontend/src/pages/community_detail.rs`
  - `frontend/src/pages/history.rs`
- Expected: findings for focus/hover/long text/numeric alignment.

**Step 2: Apply minimal CSS fixes**
- Update `.skip-link:focus` to `:focus-visible` and keep a visible ring.
- Add hover feedback for `.community-card-main`, `.community-delete-button`, `.icon-button`.
- Clamp `.community-summary` to 2 lines.
- Add `overflow-wrap: break-word` (or `word-break`) to `.community-detail-summary` and `.community-detail-ingredients p`.
- Add `text-wrap: balance` (or `text-wrap: pretty`) to `.page.page-community .page-title`.
- Add `font-variant-numeric: tabular-nums` to `.community-score-value` and `.history-score-value`.

**Step 3: Re-run the guideline review**
- Run the same review as Step 1.
- Expected: no findings.

**Step 4: Visual spot-check**
- Open Community list/detail and History pages.
- Confirm no layout shifts; hover/focus are subtle and aligned with Figma palette.

**Step 5: Commit**
- `git add frontend/src/styles/app.css`
- `git commit -m "fix(ui): address web UI guideline findings"`

---

### Task 2: Verification (Execution Finish Checklist)

**Step 1: Start local services**
- Run: `docker compose up -d`
- Verify containers are healthy.

**Step 2: Run the full API flow end-to-end**
- Upload image, confirm OCR, fetch analysis result.
- Expected: analysis status reaches `completed` with non-null `result`.

**Step 3: Run frontend check**
- Run: `cargo check -p smart-ingredients-app`
- Expected: no compile errors.

**Step 4: Final guideline review**
- Re-run web-design-guidelines review on the same files.
- Expected: no findings.
