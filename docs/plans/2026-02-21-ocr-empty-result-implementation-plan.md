# OCR Empty Result Handling Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Return HTTP 422 with a friendly message when OCR yields no text, and propagate that message to analysis `error_message` for front-end display.

**Architecture:** Add a pure OCR-result parsing helper in the OCR service, use it in the FastAPI endpoint to decide empty results and return 422. Update backend OCR client to parse 422 bodies (message/detail) and store the user-friendly error. Other non-2xx remain dependency errors.

**Tech Stack:** FastAPI + PaddleOCR (OCR service), Rust + Axum + reqwest (backend)

---

### Task 1: Add OCR result parser + unit tests (pure Python)

**Files:**
- Create: `ocr_service/ocr_utils.py`
- Create: `ocr_service/tests/test_ocr_utils.py`

**Step 1: Write the failing test**

Create `ocr_service/tests/test_ocr_utils.py`:

```python
import sys
from pathlib import Path
import unittest

sys.path.append(str(Path(__file__).resolve().parents[1]))

from ocr_utils import parse_ocr_result

class TestParseOcrResult(unittest.TestCase):
    def test_none_result_returns_empty(self):
        text, lines = parse_ocr_result([None], min_text_len=2)
        self.assertEqual(text, "")
        self.assertEqual(lines, [])

    def test_empty_list_returns_empty(self):
        text, lines = parse_ocr_result([], min_text_len=2)
        self.assertEqual(text, "")
        self.assertEqual(lines, [])

    def test_collects_text_and_lines(self):
        sample = [
            [
                [[0, 0], [1, 0], [1, 1], [0, 1]], ("配料: 水", 0.98)],
                [[0, 0], [1, 0], [1, 1], [0, 1]], ("糖", 0.95)],
            ]
        ]
        text, lines = parse_ocr_result(sample, min_text_len=2)
        self.assertEqual(text, "配料: 水\n糖")
        self.assertEqual(len(lines), 2)
        self.assertEqual(lines[0]["text"], "配料: 水")

if __name__ == "__main__":
    unittest.main()
```

**Step 2: Run test to verify it fails**

Run: `python -m unittest ocr_service/tests/test_ocr_utils.py`
Expected: FAIL (module/function not found).

**Step 3: Write minimal implementation**

Create `ocr_service/ocr_utils.py`:

```python
from typing import Any, List, Tuple


def parse_ocr_result(result: Any, min_text_len: int) -> Tuple[str, List[dict]]:
    lines: List[dict] = []
    texts: List[str] = []

    if not result:
        return "", lines

    for block in result:
        if not block:
            continue
        for item in block:
            if not item or len(item) < 2:
                continue
            text, score = item[1]
            if text:
                lines.append({"text": text, "score": score})
                texts.append(text)

    joined = "\n".join(texts).strip()
    if len(joined) < min_text_len:
        return "", lines

    return joined, lines
```

**Step 4: Run test to verify it passes**

Run: `python -m unittest ocr_service/tests/test_ocr_utils.py`
Expected: PASS.

**Step 5: Commit**

```bash
git add ocr_service/ocr_utils.py ocr_service/tests/test_ocr_utils.py
git commit -m "test(ocr): add parser coverage for empty results"
```

---

### Task 2: Update OCR endpoint to return 422 on empty text

**Files:**
- Modify: `ocr_service/app.py`
- Modify: `ocr_service/Dockerfile`

**Step 1: Write the failing test**

No direct endpoint test in repo; rely on Task 1 parser tests + manual OCR validation in Task 4. (TDD already covers core parsing rules.)

**Step 2: Write minimal implementation**

Update `ocr_service/app.py`:

```python
from fastapi.responses import JSONResponse
from ocr_utils import parse_ocr_result

EMPTY_MESSAGE = "未识别到文字，请重新拍摄或上传更清晰的图片"

# inside ocr_endpoint
min_text_len = int(os.getenv("OCR_MIN_TEXT_LEN", "2"))
result = OCR_ENGINE.ocr(np.array(image), cls=True)
text, lines = parse_ocr_result(result, min_text_len)
if not text:
    return JSONResponse(status_code=422, content={"message": EMPTY_MESSAGE})

return {"text": text, "lines": lines}
```

Update `ocr_service/Dockerfile` to copy the new helper:

```dockerfile
COPY ocr_service/ocr_utils.py /app/ocr_utils.py
```

**Step 3: Manual check**

Run inside OCR container (or via API flow in Task 4) and confirm empty image returns 422 with message.

**Step 4: Commit**

```bash
git add ocr_service/app.py ocr_service/Dockerfile
git commit -m "fix(ocr): return 422 on empty OCR result"
```

---

### Task 3: Backend parse 422 message and store friendly error

**Files:**
- Modify: `backend/src/services/ocr.rs`

**Step 1: Write the failing test**

Add tests in `backend/src/services/ocr.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::parse_ocr_error_message;

    #[test]
    fn parse_message_field() {
        let body = r#"{\"message\":\"empty\"}"#;
        assert_eq!(parse_ocr_error_message(body), Some("empty".to_string()));
    }

    #[test]
    fn parse_detail_field() {
        let body = r#"{\"detail\":\"empty\"}"#;
        assert_eq!(parse_ocr_error_message(body), Some("empty".to_string()));
    }

    #[test]
    fn returns_none_on_invalid_json() {
        let body = "not-json";
        assert_eq!(parse_ocr_error_message(body), None);
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p backend parse_ocr_error_message`
Expected: FAIL (function not found).

**Step 3: Write minimal implementation**

Add helper + 422 handling in `backend/src/services/ocr.rs`:

```rust
#[derive(serde::Deserialize)]
struct OcrErrorBody {
    message: Option<String>,
    detail: Option<String>,
}

fn parse_ocr_error_message(body: &str) -> Option<String> {
    let parsed: OcrErrorBody = serde_json::from_str(body).ok()?;
    parsed.message.or(parsed.detail)
}

// in extract_text():
if !response.status().is_success() {
    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    if status == reqwest::StatusCode::UNPROCESSABLE_ENTITY {
        let fallback = "未识别到文字，请重新拍摄或上传更清晰的图片".to_string();
        let message = parse_ocr_error_message(&body).unwrap_or(fallback);
        return Err(anyhow::anyhow!(message));
    }
    return Err(anyhow::anyhow!(
        "paddle OCR failed: status {} body {}",
        status,
        body
    ));
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p backend parse_ocr_error_message`
Expected: PASS.

**Step 5: Commit**

```bash
git add backend/src/services/ocr.rs
git commit -m "fix(backend): surface OCR 422 message"
```

---

### Task 4: Verification (required checklist)

**Step 1: Start local services**

Run: `docker compose up -d`
Expected: services running; resolve conflicts if already running.

**Step 2: Full API flow**

Run:
```bash
curl -s -X POST http://127.0.0.1:3000/api/v1/analysis/upload -F file=@frontend/src-tauri/icons/128x128.png
curl -s -X POST http://127.0.0.1:3000/api/v1/analysis/<id>/analyze
curl -s http://127.0.0.1:3000/api/v1/analysis/<id>
```
Expected: OCR fails with `ocr_failed` and `error_message` set to friendly message when image has no text.

**Step 3: Frontend compile check**

Run: `cargo check -p smart-ingredients-app`
Expected: success (warnings acceptable if pre-existing).

**Step 4: Commit (if needed)**

If any uncommitted changes remain for this feature, stage only related files:
```bash
git add ocr_service/app.py ocr_service/ocr_utils.py ocr_service/tests/test_ocr_utils.py ocr_service/Dockerfile backend/src/services/ocr.rs
git commit -m "fix: handle empty OCR results"
```
