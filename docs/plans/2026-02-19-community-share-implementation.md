# Community Share Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a Community tab that supports anonymous sharing of analysis results, public list/detail browsing, and share/delete controls from result + history detail.

**Architecture:** Shared crate adds community request/response types; backend adds `community_posts` table and CRUD endpoints (multipart create with optional card image) plus token-hash deletion; frontend adds Community list/detail pages, a share-to-community button, and local share record storage for delete.

**Tech Stack:** Rust (Axum + SQLx + Leptos/Tauri), Postgres, shared crate, web-sys FormData.

---

### Task 1: Shared community types + serde tests (@superpowers:test-driven-development)

**Files:**
- Create: `shared/src/community.rs`
- Modify: `shared/src/lib.rs`
- Test: `shared/src/community.rs` (unit tests)

**Step 1: Write the failing test**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn author_type_serializes_as_lowercase() {
        let json = serde_json::to_string(&CommunityAuthorType::Anonymous).unwrap();
        assert_eq!(json, "\"anonymous\"");
    }

    #[test]
    fn create_payload_roundtrip() {
        let payload = CommunityCreatePayload {
            author_type: CommunityAuthorType::Anonymous,
            share_token: Some("token-123".to_string()),
            source_analysis_id: None,
            summary_text: "summary".to_string(),
            health_score: 85,
            ingredients_raw: "水、乌龙茶".to_string(),
            card_payload: CommunityCardPayload {
                health_score: 85,
                summary: "summary".to_string(),
                recommendation: "ok".to_string(),
                ingredients: vec![],
                warnings: vec![],
                preference_label: None,
            },
        };
        let json = serde_json::to_string(&payload).unwrap();
        let back: CommunityCreatePayload = serde_json::from_str(&json).unwrap();
        assert_eq!(back.summary_text, "summary");
        assert_eq!(back.health_score, 85);
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p shared community::tests::author_type_serializes_as_lowercase`  
Expected: FAIL with "cannot find module `community`"

**Step 3: Write minimal implementation**

```rust
// shared/src/community.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CommunityAuthorType {
    Anonymous,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityCardIngredient {
    pub name: String,
    pub risk_level: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub is_focus: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityCardPayload {
    pub health_score: i32,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub recommendation: String,
    #[serde(default)]
    pub ingredients: Vec<CommunityCardIngredient>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub preference_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityCreatePayload {
    pub author_type: CommunityAuthorType,
    #[serde(default)]
    pub share_token: Option<String>,
    #[serde(default)]
    pub source_analysis_id: Option<Uuid>,
    pub summary_text: String,
    pub health_score: i32,
    pub ingredients_raw: String,
    pub card_payload: CommunityCardPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityPostCreated {
    pub id: Uuid,
    pub created_at: String,
    #[serde(default)]
    pub card_image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityPostListItem {
    pub id: Uuid,
    pub summary_text: String,
    pub health_score: i32,
    #[serde(default)]
    pub card_image_url: Option<String>,
    pub author_label: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityPostDetail {
    pub id: Uuid,
    pub summary_text: String,
    pub health_score: i32,
    pub ingredients_raw: String,
    pub card_payload: CommunityCardPayload,
    #[serde(default)]
    pub card_image_url: Option<String>,
    pub author_label: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityPostListResponse {
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub items: Vec<CommunityPostListItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityDeleteRequest {
    #[serde(default)]
    pub share_token: Option<String>,
}
```

Update `shared/src/lib.rs`:

```rust
mod community;
pub use community::*;
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p shared community::tests::author_type_serializes_as_lowercase`  
Expected: PASS

**Step 5: Commit**

```bash
git add shared/src/community.rs shared/src/lib.rs
git commit -m "feat(shared): add community types"
```

---

### Task 2: Backend community validation + token hashing tests (@superpowers:test-driven-development)

**Files:**
- Create: `backend/src/services/community.rs`
- Modify: `backend/src/services/mod.rs`
- Test: `backend/src/services/community.rs`

**Step 1: Write the failing test**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use shared::{CommunityAuthorType, CommunityCardPayload, CommunityCreatePayload};

    #[test]
    fn validate_requires_summary() {
        let payload = CommunityCreatePayload {
            author_type: CommunityAuthorType::Anonymous,
            share_token: Some("token".to_string()),
            source_analysis_id: None,
            summary_text: "".to_string(),
            health_score: 85,
            ingredients_raw: "水".to_string(),
            card_payload: CommunityCardPayload {
                health_score: 85,
                summary: "s".to_string(),
                recommendation: "r".to_string(),
                ingredients: vec![],
                warnings: vec![],
                preference_label: None,
            },
        };
        let err = validate_create_payload(&payload, None, "key").unwrap_err();
        assert!(matches!(err, AppError::BadRequest(_)));
    }

    #[test]
    fn anonymous_requires_token() {
        let payload = CommunityCreatePayload {
            author_type: CommunityAuthorType::Anonymous,
            share_token: None,
            source_analysis_id: None,
            summary_text: "ok".to_string(),
            health_score: 85,
            ingredients_raw: "水".to_string(),
            card_payload: CommunityCardPayload {
                health_score: 85,
                summary: "s".to_string(),
                recommendation: "r".to_string(),
                ingredients: vec![],
                warnings: vec![],
                preference_label: None,
            },
        };
        let err = validate_create_payload(&payload, None, "key").unwrap_err();
        assert!(matches!(err, AppError::BadRequest(_)));
    }

    #[test]
    fn hash_share_token_is_deterministic() {
        let a = hash_share_token("token", "key").unwrap();
        let b = hash_share_token("token", "key").unwrap();
        assert_eq!(a, b);
        assert_eq!(a.len(), 64);
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p backend community::tests::validate_requires_summary`  
Expected: FAIL with "cannot find function `validate_create_payload`"

**Step 3: Write minimal implementation**

```rust
// backend/src/services/community.rs
use hmac::{Hmac, Mac};
use sha2::Sha256;
use uuid::Uuid;

use crate::errors::AppError;
use shared::{CommunityAuthorType, CommunityCreatePayload};

type HmacSha256 = Hmac<Sha256>;

pub fn hash_share_token(token: &str, key: &str) -> Result<String, AppError> {
    let mut mac = <HmacSha256 as hmac::digest::KeyInit>::new_from_slice(key.as_bytes())
        .map_err(|_| AppError::Internal("分享令牌哈希失败".to_string()))?;
    mac.update(token.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

pub struct ValidatedCreate {
    pub author_type: CommunityAuthorType,
    pub user_id: Option<Uuid>,
    pub share_token_hash: Option<String>,
}

pub fn validate_create_payload(
    payload: &CommunityCreatePayload,
    auth_user: Option<Uuid>,
    hash_key: &str,
) -> Result<ValidatedCreate, AppError> {
    if payload.summary_text.trim().is_empty() {
        return Err(AppError::BadRequest("摘要不能为空".to_string()));
    }
    if payload.ingredients_raw.trim().is_empty() {
        return Err(AppError::BadRequest("配料表不能为空".to_string()));
    }
    if !(0..=100).contains(&payload.health_score) {
        return Err(AppError::BadRequest("健康评分超出范围".to_string()));
    }

    match payload.author_type {
        CommunityAuthorType::Anonymous => {
            let token = payload
                .share_token
                .as_ref()
                .filter(|v| !v.trim().is_empty())
                .ok_or_else(|| AppError::BadRequest("缺少分享令牌".to_string()))?;
            let hash = hash_share_token(token, hash_key)?;
            Ok(ValidatedCreate {
                author_type: CommunityAuthorType::Anonymous,
                user_id: None,
                share_token_hash: Some(hash),
            })
        }
        CommunityAuthorType::User => {
            let user_id = auth_user.ok_or_else(|| AppError::Unauthorized("请先登录".to_string()))?;
            Ok(ValidatedCreate {
                author_type: CommunityAuthorType::User,
                user_id: Some(user_id),
                share_token_hash: None,
            })
        }
    }
}
```

Update `backend/src/services/mod.rs`:

```rust
pub mod community;
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p backend community::tests::validate_requires_summary`  
Expected: PASS

**Step 5: Commit**

```bash
git add backend/src/services/community.rs backend/src/services/mod.rs
git commit -m "feat(backend): add community payload validation"
```

---

### Task 3: Database migration for community posts

**Files:**
- Create: `backend/migrations/008_create_community_posts.sql`

**Step 1: Write the failing test**

Not applicable (SQL migration). We’ll validate by compile + later integration run.

**Step 2: Implement migration**

```sql
CREATE TABLE IF NOT EXISTS community_posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_type VARCHAR(20) NOT NULL CHECK (author_type IN ('anonymous','user')),
    user_id UUID,
    share_token_hash VARCHAR(128),
    summary_text TEXT NOT NULL,
    health_score INTEGER CHECK (health_score >= 0 AND health_score <= 100),
    ingredients_raw TEXT NOT NULL,
    card_payload JSONB NOT NULL,
    card_image_url VARCHAR(512),
    source_analysis_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_community_posts_created_at ON community_posts(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_community_posts_user_id ON community_posts(user_id);
```

**Step 3: Run tests**

Run: `cargo test -p backend community::tests::hash_share_token_is_deterministic`  
Expected: PASS

**Step 4: Commit**

```bash
git add backend/migrations/008_create_community_posts.sql
git commit -m "feat(db): add community_posts table"
```

---

### Task 4: Backend DB access + handlers + routes

**Files:**
- Modify: `backend/src/db.rs`
- Create: `backend/src/handlers/community.rs`
- Modify: `backend/src/handlers/mod.rs`
- Modify: `backend/src/routes.rs`

**Step 1: Write the failing test**

Not applicable (handler wiring). We’ll rely on compile + integration verification.

**Step 2: Implement DB helpers**

Add `CommunityPostRow` structs and functions in `backend/src/db.rs`:
- `insert_community_post(...) -> CommunityPostCreated`
- `list_community_posts(page, limit) -> (total, items)`
- `get_community_post(id) -> Option<CommunityPostDetail>`
- `delete_community_post(id, user_id, share_token_hash) -> u64`

**Step 3: Implement handlers**

`backend/src/handlers/community.rs`:
- `POST /posts`: parse multipart (`payload` JSON + optional `card_image`), validate via `services::community::validate_create_payload`, store image (optional), insert DB, return `CommunityPostCreated`
- `GET /posts`: `page/limit` query (default `limit=20`), list and return `CommunityPostListResponse`
- `GET /posts/:id`: return detail or 404
- `DELETE /posts/:id`: accept `Json<CommunityDeleteRequest>`; if logged in → delete by user_id, else delete by share_token

Use Chinese error messages per `docs/standards/error-handling-standards.md`.

**Step 4: Wire routes**

Update `backend/src/handlers/mod.rs`:
```rust
pub mod community;
```

Update `backend/src/routes.rs`:
```rust
.nest("/api/v1/community", community::routes())
```

**Step 5: Run tests**

Run: `cargo test -p backend community::tests::validate_requires_summary`  
Expected: PASS

**Step 6: Commit**

```bash
git add backend/src/db.rs backend/src/handlers/community.rs backend/src/handlers/mod.rs backend/src/routes.rs
git commit -m "feat(backend): add community CRUD endpoints"
```

---

### Task 5: API reference update

**Files:**
- Modify: `docs/api/api-reference.md`

**Step 1: Update docs**

Add endpoints:
- `POST /api/v1/community/posts` (multipart: `payload` + optional `card_image`)
- `GET /api/v1/community/posts?page=&limit=`
- `GET /api/v1/community/posts/{id}`
- `DELETE /api/v1/community/posts/{id}` (optional `share_token`)

**Step 2: Commit**

```bash
git add docs/api/api-reference.md
git commit -m "docs: add community api endpoints"
```

---

### Task 6: Frontend share payload builder + tests (@superpowers:test-driven-development)

**Files:**
- Create: `frontend/src/utils/community_share.rs`
- Modify: `frontend/src/utils/mod.rs`
- Test: `frontend/src/utils/community_share.rs`

**Step 1: Write the failing test**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use shared::{AnalysisResult, IngredientInfo, Warning};

    #[test]
    fn build_summary_prefers_focus_summary() {
        let result = AnalysisResult {
            health_score: 80,
            summary: "normal".to_string(),
            focus_summary: Some("focus".to_string()),
            recommendation: "ok".to_string(),
            table: vec![],
            ingredients: vec![],
            warnings: vec![],
            overall_assessment: None,
            focus_ingredients: None,
            score_breakdown: None,
            rule_hits: vec![],
            confidence: None,
        };
        let summary = build_summary_text(&result);
        assert_eq!(summary, "focus");
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p smart-ingredients-app community_share::tests::build_summary_prefers_focus_summary`  
Expected: FAIL with "cannot find module `community_share`"

**Step 3: Write minimal implementation**

```rust
// frontend/src/utils/community_share.rs
use shared::{AnalysisResult, CommunityCardIngredient, CommunityCardPayload};

pub fn build_summary_text(result: &AnalysisResult) -> String {
    result
        .focus_summary
        .clone()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| result.summary.clone())
}

pub fn build_card_payload(result: &AnalysisResult, preference_label: Option<String>) -> CommunityCardPayload {
    CommunityCardPayload {
        health_score: result.health_score,
        summary: build_summary_text(result),
        recommendation: result.recommendation.clone(),
        ingredients: result
            .ingredients
            .iter()
            .map(|item| CommunityCardIngredient {
                name: item.name.clone(),
                risk_level: item.risk_level.clone(),
                description: item.description.clone().unwrap_or_default(),
                is_focus: false,
            })
            .collect(),
        warnings: result.warnings.iter().map(|w| w.message.clone()).collect(),
        preference_label,
    }
}
```

Update `frontend/src/utils/mod.rs`:

```rust
pub mod community_share;
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p smart-ingredients-app community_share::tests::build_summary_prefers_focus_summary`  
Expected: PASS

**Step 5: Commit**

```bash
git add frontend/src/utils/community_share.rs frontend/src/utils/mod.rs
git commit -m "feat(frontend): add community share payload builder"
```

---

### Task 7: Frontend local share record storage

**Files:**
- Create: `frontend/src/utils/community_share_storage.rs`
- Modify: `frontend/src/utils/mod.rs`

**Step 1: Implement storage helpers**

Functions:
- `load_share_records() -> Vec<CommunityShareRecord>`
- `get_share_record(analysis_id: &str) -> Option<CommunityShareRecord>`
- `upsert_share_record(record: CommunityShareRecord)`
- `remove_share_record(analysis_id: &str)`

`CommunityShareRecord` fields: `analysis_id`, `post_id`, `author_type`, `share_token`.

**Step 2: Commit**

```bash
git add frontend/src/utils/community_share_storage.rs frontend/src/utils/mod.rs
git commit -m "feat(frontend): add community share storage"
```

---

### Task 8: Frontend community API services

**Files:**
- Modify: `frontend/src/services/mod.rs`
- Modify: `frontend/src/utils/export_image.rs` (add data_url -> Blob helper)

**Step 1: Implement data_url -> Blob helper**

Add:
```rust
pub fn data_url_to_blob(data_url: &str) -> Result<web_sys::Blob, String> { ... }
```

**Step 2: Implement API functions**

Add:
- `create_community_post(payload: &CommunityCreatePayload, card_image: Option<web_sys::Blob>)`
- `fetch_community_posts(page: i64, limit: i64)`
- `fetch_community_post(id: Uuid)`
- `delete_community_post(id: Uuid, share_token: Option<String>)`

**Step 3: Commit**

```bash
git add frontend/src/services/mod.rs frontend/src/utils/export_image.rs
git commit -m "feat(frontend): add community api services"
```

---

### Task 9: Community pages + bottom nav integration

**Files:**
- Create: `frontend/src/pages/community.rs`
- Create: `frontend/src/pages/community_detail.rs`
- Modify: `frontend/src/pages/mod.rs`
- Modify: `frontend/src/lib.rs`
- Modify: `frontend/src/stores/mod.rs`
- Modify: `frontend/src/components/bottom_nav.rs`
- Modify: `frontend/src/components/tab_icons.rs`
- Modify: `frontend/src/components/mod.rs`
- Modify: `frontend/src/styles/app.css`

**Step 1: Implement list page**
- Fetch list via `services::fetch_community_posts`
- Render cards with image (if available) + score + summary + author + time
- Navigate to `/community/:id`

**Step 2: Implement detail page**
- Fetch detail by id
- Render image if present, else render summary + score from `card_payload`
- Show original `ingredients_raw`

**Step 3: Update navigation**
- Add `TabRoute::Community` (label “社区”, path `/community`)
- Track `last_community_path` in `AppState`
- Add community icon and tab in `BottomNav`
- Add routes in `frontend/src/lib.rs`

**Step 4: Add minimal styles**
- `.page-community`, `.community-card`, `.community-detail`

**Step 5: Commit**

```bash
git add frontend/src/pages/community.rs frontend/src/pages/community_detail.rs frontend/src/pages/mod.rs frontend/src/lib.rs frontend/src/stores/mod.rs frontend/src/components/bottom_nav.rs frontend/src/components/tab_icons.rs frontend/src/components/mod.rs frontend/src/styles/app.css
git commit -m "feat(frontend): add community pages and nav"
```

---

### Task 10: Share-to-community controls in result + history detail

**Files:**
- Create: `frontend/src/components/community_share_button.rs`
- Modify: `frontend/src/components/mod.rs`
- Modify: `frontend/src/pages/result.rs`
- Modify: `frontend/src/pages/summary.rs`

**Step 1: Implement component**
- Build `CommunityCreatePayload` from `AnalysisResponse` + `utils::community_share`
- Generate `share_token` (UUID) for anonymous
- Optionally generate card image via `export_to_data_url` → `data_url_to_blob`
- Call `services::create_community_post`
- On success: store `CommunityShareRecord`, toast “已分享到社区”

**Step 2: Add delete control**
- If share record exists, show “已分享 / 取消分享” in summary page
- Call `services::delete_community_post` with stored `share_token` (if anonymous)
- On success: remove share record and toast

**Step 3: Wire buttons**
- Result page: add “分享到社区” button in actions
- Summary page: add “分享到社区” + share status (history detail)

**Step 4: Commit**

```bash
git add frontend/src/components/community_share_button.rs frontend/src/components/mod.rs frontend/src/pages/result.rs frontend/src/pages/summary.rs
git commit -m "feat(frontend): add share to community controls"
```

---

### Task 11: Verification (小周验证)

**Step 1: Start local services and confirm healthy**

Run: `docker compose up -d`  
Expected: postgres/redis/ocr containers healthy

**Step 2: Full API flow E2E**
- Upload image → analyze → result
- Share to community → list shows new item → open detail → delete share

**Step 3: Frontend compile check**

Run: `cargo check -p smart-ingredients-app`  
Expected: PASS (no compile errors)

**Step 4: Report results**

Summarize verification outcomes + any warnings.

---

Plan complete and saved to `docs/plans/2026-02-19-community-share-implementation.md`.
Two execution options:

1. Subagent-Driven (this session) — I dispatch fresh subagent per task, review between tasks
2. Parallel Session (separate) — New session with `executing-plans`, batch execution with checkpoints

Which approach?
