# Figma Style Unification Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Apply the downloaded Figma visual style across all pages (Home/Community/History/Profile + Login/Register/Onboarding + OCR/Confirm/Analyzing + Summary/Detail + Community Detail) without changing structure, copy, or flows.

**Architecture:** Scoped theme approach. Add/adjust Figma theme tokens and component classes in CSS, then apply `.figma`/page-specific classes on page roots. Keep current layout/logic; adjust visual classes only.

**Tech Stack:** Rust + Leptos (frontend), CSS (app.css + bottom-nav.css)

---

### Task 1: Add Figma theme tokens and shared component classes

**Files:**
- Modify: `frontend/src/styles/app.css`

**Step 1: Write the failing test**
- No automated test exists for CSS tokens. **Skip (approved).**

**Step 2: Write minimal implementation**
- Add Figma color tokens (emerald/teal/amber) and shared utility classes for:
  - Gradient background
  - Glass card (bg + blur + shadow + radius)
  - Primary/secondary CTA styles
  - Topbar/title styles
  - Form input styles
- Keep existing fonts unchanged.

**Step 3: Manual check**
- Ensure tokens align with Figma code bundle colors.

**Step 4: Commit**
```
git add frontend/src/styles/app.css
git commit -m "style: add figma theme tokens"
```

---

### Task 2: Bottom navigation Figma styling (4 tabs)

**Files:**
- Modify: `frontend/src/styles/bottom-nav.css`
- (Optional) Modify: `frontend/src/components/bottom_nav.rs` (class hooks if needed)

**Step 1: Write the failing test**
- No automated test exists for CSS. **Skip (approved).**

**Step 2: Write minimal implementation**
- Apply Figma bottom bar styles:
  - Semi-transparent background + blur
  - Rounded active highlight with glow
  - Active/inactive color states
  - Maintain 4-tab layout order (already in logic)

**Step 3: Manual check**
- Visually inspect active and inactive states.

**Step 4: Commit**
```
git add frontend/src/styles/bottom-nav.css frontend/src/components/bottom_nav.rs
git commit -m "style: align bottom nav with figma"
```

---

### Task 3: Home (Capture) page styling alignment

**Files:**
- Modify: `frontend/src/pages/capture.rs`
- Modify: `frontend/src/styles/app.css`

**Step 1: Write the failing test**
- No automated test exists for styling. **Skip (approved).**

**Step 2: Write minimal implementation**
- Keep structure (steps + examples fold) but update classes to Figma tokens:
  - Hero brand block
  - Primary CTA
  - Collapsible cards
  - Scan section panel

**Step 3: Manual check**
- Ensure layout remains intact and feels like Figma Home.

**Step 4: Commit**
```
git add frontend/src/pages/capture.rs frontend/src/styles/app.css
git commit -m "style: align capture page with figma"
```

---

### Task 4: OCR / Confirm / Analyzing pages styling alignment

**Files:**
- Modify: `frontend/src/pages/ocr.rs`
- Modify: `frontend/src/pages/confirm.rs`
- Modify: `frontend/src/pages/analyzing.rs`
- Modify: `frontend/src/styles/app.css`

**Step 1: Write the failing test**
- No automated test exists for styling. **Skip (approved).**

**Step 2: Write minimal implementation**
- Replace card/background/CTA styles with Figma tokens
- Use ProcessingPage visual cues (progress/gradient/AI badge)
- Preserve existing flow & messages

**Step 3: Manual check**
- Ensure OCR/Confirm/Analyzing are visually consistent.

**Step 4: Commit**
```
git add frontend/src/pages/ocr.rs frontend/src/pages/confirm.rs frontend/src/pages/analyzing.rs frontend/src/styles/app.css
git commit -m "style: align ocr/confirm/analyzing pages"
```

---

### Task 5: Summary + Detail result pages styling alignment

**Files:**
- Modify: `frontend/src/pages/summary.rs`
- Modify: `frontend/src/pages/detail.rs`
- Modify: `frontend/src/styles/app.css`

**Step 1: Write the failing test**
- No automated test exists for styling. **Skip (approved).**

**Step 2: Write minimal implementation**
- Apply Figma Results visual treatment:
  - Score card, list cards, section headers
  - Gradients and shadow intensity
  - Bottom action area (if any)

**Step 3: Manual check**
- Ensure Summary/Detail appear as Figma results style while preserving content.

**Step 4: Commit**
```
git add frontend/src/pages/summary.rs frontend/src/pages/detail.rs frontend/src/styles/app.css
git commit -m "style: align summary/detail pages with figma"
```

---

### Task 6: Community list + Community detail styling alignment

**Files:**
- Modify: `frontend/src/pages/community.rs`
- Modify: `frontend/src/pages/community_detail.rs`
- Modify: `frontend/src/styles/app.css`

**Step 1: Write the failing test**
- No automated test exists for styling. **Skip (approved).**

**Step 2: Write minimal implementation**
- Apply card visuals and spacing consistent with Figma
- Keep list structure, actions, and timestamps intact

**Step 3: Manual check**
- Verify list and detail card styles align with overall Figma theme.

**Step 4: Commit**
```
git add frontend/src/pages/community.rs frontend/src/pages/community_detail.rs frontend/src/styles/app.css
git commit -m "style: align community pages with figma"
```

---

### Task 7: History page styling alignment

**Files:**
- Modify: `frontend/src/pages/history.rs`
- Modify: `frontend/src/styles/app.css`

**Step 1: Write the failing test**
- No automated test exists for styling. **Skip (approved).**

**Step 2: Write minimal implementation**
- Update list cards, pagination, empty state visuals to Figma style

**Step 3: Manual check**
- Verify history items align with Figma look.

**Step 4: Commit**
```
git add frontend/src/pages/history.rs frontend/src/styles/app.css
git commit -m "style: align history page with figma"
```

---

### Task 8: Profile page styling alignment

**Files:**
- Modify: `frontend/src/pages/profile.rs`
- Modify: `frontend/src/styles/app.css`

**Step 1: Write the failing test**
- No automated test exists for styling. **Skip (approved).**

**Step 2: Write minimal implementation**
- Update profile header, menu items, buttons to Figma look

**Step 3: Manual check**
- Verify profile page is visually consistent.

**Step 4: Commit**
```
git add frontend/src/pages/profile.rs frontend/src/styles/app.css
git commit -m "style: align profile page with figma"
```

---

### Task 9: Login / Register / Onboarding styling alignment

**Files:**
- Modify: `frontend/src/pages/login.rs`
- Modify: `frontend/src/pages/register.rs`
- Modify: `frontend/src/pages/onboarding.rs`
- Modify: `frontend/src/styles/app.css`

**Step 1: Write the failing test**
- No automated test exists for styling. **Skip (approved).**

**Step 2: Write minimal implementation**
- Update auth cards, inputs, buttons, hero block to Figma visuals

**Step 3: Manual check**
- Ensure flows intact and styling consistent with Figma.

**Step 4: Commit**
```
git add frontend/src/pages/login.rs frontend/src/pages/register.rs frontend/src/pages/onboarding.rs frontend/src/styles/app.css
git commit -m "style: align auth and onboarding pages with figma"
```

---

### Task 10: Run UI guideline review (web-design-guidelines)

**Files:**
- Review: All modified UI files (pages + CSS)

**Step 1: Fetch latest guidelines**
- Use web-design-guidelines skill to fetch rules

**Step 2: Review files**
- Apply rules to modified files

**Step 3: Report findings**
- Fix any high/medium issues if found

---

### Task 11: Verification (required checklist)

**Step 1: Start local services**
```
docker compose up -d
docker ps --filter name=smart-ingredients-
```

**Step 2: Full API flow**
- Upload image → OCR → confirm → LLM → query result

**Step 3: Frontend compile**
```
cargo check -p smart-ingredients-app
```

**Step 4: Tests**
```
cargo test -p smart-ingredients-app
```

**Step 5: Commit**
```
git add .
git commit -m "style: unify ui with figma theme"
```

