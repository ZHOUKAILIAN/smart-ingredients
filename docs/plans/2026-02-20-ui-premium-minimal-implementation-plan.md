# Premium Minimal UI Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 在不改变核心业务流程的前提下，将首页/流程页/结果页统一为“先锋极简/高端质感”视觉系统，采用 rem + clamp 的响应式规则与高端衬线字体。

**Architecture:** 以 CSS tokens 驱动全局视觉风格，新增字体资源并通过 Trunk 静态拷贝；主要通过 `app.css`/`bottom-nav.css` 与页面结构的小幅调整落地。必要的展示逻辑抽离为前端工具函数以便测试。

**Tech Stack:** Rust (Leptos), CSS, Trunk

---

## Preflight / 已知限制

- 基线 `cargo build` 在 `smart-ingredients-tauri` 处因 `OUT_DIR` 缺失失败（用户选择记录失败并继续）。
- 后续验证以 `cargo check -p smart-ingredients-app` 为前端编译基线。

---

### Task 1: 抽离结论标签工具函数（含单测）

**Files:**
- Create: `frontend/src/utils/presentation.rs`
- Modify: `frontend/src/utils/mod.rs`
- Modify: `frontend/src/pages/result.rs`

**Step 1: Write the failing test**

在 `frontend/src/utils/presentation.rs` 创建最小测试：

```rust
#[cfg(test)]
mod tests {
    use super::conclusion_label;

    #[test]
    fn conclusion_label_ranges() {
        assert_eq!(conclusion_label(80), "可吃");
        assert_eq!(conclusion_label(60), "谨慎");
        assert_eq!(conclusion_label(20), "不推荐");
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p smart-ingredients-app conclusion_label_ranges`

Expected: FAIL with “cannot find function `conclusion_label`”.

**Step 3: Write minimal implementation**

在 `frontend/src/utils/presentation.rs` 增加函数：

```rust
pub fn conclusion_label(score: i32) -> &'static str {
    match score {
        75..=100 => "可吃",
        50..=74 => "谨慎",
        _ => "不推荐",
    }
}
```

并在 `frontend/src/utils/mod.rs` 中新增模块导出：

```rust
pub mod presentation;
```

在 `frontend/src/pages/result.rs` 替换本地 `conclusion_label` 为：

```rust
use crate::utils::presentation::conclusion_label;
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p smart-ingredients-app conclusion_label_ranges`

Expected: PASS

**Step 5: Commit**

```bash
git add frontend/src/utils/presentation.rs frontend/src/utils/mod.rs frontend/src/pages/result.rs
git commit -m "test(ui): add conclusion label helper"
```

---

### Task 2: 引入高端字体资源并注册（Trunk + @font-face）

**Files:**
- Create: `frontend/assets/fonts/SourceHanSerifSC-Regular.woff2`
- Create: `frontend/assets/fonts/SourceHanSerifSC-SemiBold.woff2`
- Create: `frontend/assets/fonts/CormorantGaramond-Regular.woff2`
- Create: `frontend/assets/fonts/CormorantGaramond-SemiBold.woff2`
- Modify: `frontend/index.html`
- Modify: `frontend/src/styles/app.css`

**Step 1: Add font files**

将上述字体文件放入 `frontend/assets/fonts/`（OFL 许可）。

**Step 2: Register assets with Trunk**

在 `frontend/index.html` `<head>` 内添加：

```html
<link data-trunk rel="copy-dir" href="assets" />
```

**Step 3: Add @font-face + font tokens**

在 `frontend/src/styles/app.css` 顶部加入：

```css
@font-face {
  font-family: "Source Han Serif SC";
  src: url("/assets/fonts/SourceHanSerifSC-Regular.woff2") format("woff2");
  font-weight: 400;
  font-style: normal;
  font-display: swap;
}
@font-face {
  font-family: "Source Han Serif SC";
  src: url("/assets/fonts/SourceHanSerifSC-SemiBold.woff2") format("woff2");
  font-weight: 600;
  font-style: normal;
  font-display: swap;
}
@font-face {
  font-family: "Cormorant Garamond";
  src: url("/assets/fonts/CormorantGaramond-Regular.woff2") format("woff2");
  font-weight: 400;
  font-style: normal;
  font-display: swap;
}
@font-face {
  font-family: "Cormorant Garamond";
  src: url("/assets/fonts/CormorantGaramond-SemiBold.woff2") format("woff2");
  font-weight: 600;
  font-style: normal;
  font-display: swap;
}
```

并在 `:root` 增加：

```css
--font-display: "Source Han Serif SC", "Cormorant Garamond", serif;
--font-body: "Source Han Serif SC", serif;
```

**Step 4: Commit**

```bash
git add frontend/assets/fonts frontend/index.html frontend/src/styles/app.css
git commit -m "feat(ui): add premium serif fonts"
```

---

### Task 3: 更新全局 Tokens（色彩/间距/字体/阴影）

**Files:**
- Modify: `frontend/src/styles/app.css`

**Step 1: Update :root tokens**

替换/新增：

```css
:root {
  --bg: #f7f5f2;
  --card: #ffffff;
  --text: #1a1a1a;
  --muted: #5c5c5c;
  --border: #e8e4df;
  --primary: #1f8a59;
  --primary-dark: #166947;
  --risk-low: #1f8a59;
  --risk-low-bg: #e7f3ee;
  --risk-medium: #b76b0b;
  --risk-medium-bg: #f6ede2;
  --risk-high: #a13333;
  --risk-high-bg: #f4e6e6;

  --shadow-sm: 0 1px 0 rgba(0,0,0,0.04), 0 8px 24px rgba(0,0,0,0.04);
  --shadow-md: 0 2px 8px rgba(0,0,0,0.06);

  --fs-display: clamp(1.75rem, 6vw, 2.75rem);
  --fs-h1: clamp(1.25rem, 4vw, 1.75rem);
  --fs-body: clamp(0.875rem, 2.6vw, 1rem);

  --sp-xs: clamp(0.25rem, 1vw, 0.4rem);
  --sp-sm: clamp(0.5rem, 2vw, 0.75rem);
  --sp-md: clamp(0.75rem, 3vw, 1rem);
  --sp-lg: clamp(1rem, 3.5vw, 1.25rem);
  --sp-xl: clamp(1.25rem, 4vw, 1.5rem);
}
```

**Step 2: Update base typography**

```css
body {
  font-family: var(--font-body);
  font-size: var(--fs-body);
}
.title, .hero-title { font-family: var(--font-display); }
```

**Step 3: Commit**

```bash
git add frontend/src/styles/app.css
git commit -m "feat(ui): update global tokens for premium minimal"
```

---

### Task 4: 统一卡片/按钮/徽章/标签质感

**Files:**
- Modify: `frontend/src/styles/app.css`
- Modify: `frontend/src/styles/bottom-nav.css`

**Step 1: Card + surface style**

更新 `.card` / `.surface-card`：细线边框 + 内高光 + 轻外阴影。

**Step 2: Buttons + badges + tags**

更新 `.primary-cta`, `.secondary-cta`, `.btn-primary`, `.btn-secondary`, `.risk-badge`, `.tag` 使用新 tokens；压缩 padding，改为 `rem + clamp`。

**Step 3: Bottom nav**

统一字体与色彩：`color`/`stroke` 使用 `--text`/`--primary`，背景使用 `--card` 与 `--border`。

**Step 4: Commit**

```bash
git add frontend/src/styles/app.css frontend/src/styles/bottom-nav.css
git commit -m "feat(ui): unify card/button/badge styles"
```

---

### Task 5: 首页精简与折叠内容

**Files:**
- Modify: `frontend/src/pages/capture.rs`
- Modify: `frontend/src/styles/app.css`

**Step 1: Make steps collapsible**

将“使用步骤”卡片改为 `<details>` 折叠；仅保留首屏品牌/一句话/CTA。

**Step 2: Add example images collapsible**

将 `ExampleImages` 放入“示例图片”折叠区。

**Step 3: Update CSS**

新增 `.collapse-card`, `.collapse-summary`, `.collapse-body` 样式，减少首屏高度。

**Step 4: Commit**

```bash
git add frontend/src/pages/capture.rs frontend/src/styles/app.css
git commit -m "feat(ui): compact home hero with collapsible sections"
```

---

### Task 6: OCR/确认/分析页仪式感模板

**Files:**
- Modify: `frontend/src/styles/app.css`

**Step 1: Status card style**

统一 `.status-card`, `.status-icon`, `.progress-bar` 为极细线条 + 微光动效。

**Step 2: Confirm editor style**

更新 `.text-editor`, `.action-buttons` 使用新字体/间距/边框。

**Step 3: Commit**

```bash
git add frontend/src/styles/app.css
git commit -m "feat(ui): refine OCR/confirm/analyzing styles"
```

---

### Task 7: 结果页信息层级与视觉重排

**Files:**
- Modify: `frontend/src/pages/result.rs`
- Modify: `frontend/src/components/health_score_card.rs`
- Modify: `frontend/src/components/summary_card.rs`
- Modify: `frontend/src/components/ingredient_card.rs`
- Modify: `frontend/src/styles/app.css`

**Step 1: Conclusion + key tags style**

调整结论卡内部布局与标签行，减少冗余描述。

**Step 2: Health score card**

更新 `HealthScoreCard` 结构与样式（大数字 + 等级 + 推荐语），用 `--font-display`。

**Step 3: Summary + ingredient card**

保持摘要可折叠，压缩配料卡高度，隐藏“未知”字段。

**Step 4: Commit**

```bash
git add frontend/src/pages/result.rs frontend/src/components/health_score_card.rs frontend/src/components/summary_card.rs frontend/src/components/ingredient_card.rs frontend/src/styles/app.css
git commit -m "feat(ui): redesign result page hierarchy"
```

---

### Task 8: 前端编译验证

**Step 1: Frontend check**

Run: `cargo check -p smart-ingredients-app`

Expected: PASS (允许 warnings)

**Step 2: Commit**

如果无额外变更，可跳过。

---

### Task 9: 交付前执行清单（必须）

**Step 1: 启动本地服务**

Run: `docker compose up -d`

**Step 2: 完整 API 流程**

手动走通：上传 → OCR → 确认 → LLM → 结果页。

**Step 3: 前端 cargo check**

Run: `cargo check -p smart-ingredients-app`

---
