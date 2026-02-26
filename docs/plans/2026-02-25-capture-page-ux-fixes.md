# Capture Page UX Fixes Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 修复首页/拍摄页 5 个 UI 与交互问题，保证流程可用且一屏观感更紧凑。

**Architecture:** 仅改前端页面与样式，核心落点在 `capture.rs`、`history.rs`、`community.rs`。通过最小结构调整与 spacing 缩减解决首屏问题；通过隐藏 input 策略修复相册选择失效；通过统一骨架样式优化加载态观感。

**Tech Stack:** Rust, Leptos, Tauri, CSS

---

### Task 1: 首页品牌区与步骤区修复

**Files:**
- Modify: `frontend/src/pages/capture.rs`

1. 删除重复步骤，保留固定 3 步。
2. 调整品牌区顶部留白与 icon 容器间距，避免浮动触顶。
3. 减少步骤卡和 CTA 的垂直间隔，移除过大占位空白。

### Task 2: 拍摄页空态/上传态紧凑化

**Files:**
- Modify: `frontend/src/pages/capture.rs`

1. 下调空态图标与区块尺寸（标题区、按钮区、提示区）。
2. 收紧上传态 `space-y` 与卡片 padding。
3. 统一相关边框与内边距。

### Task 3: 相册选择可用性修复

**Files:**
- Modify: `frontend/src/pages/capture.rs`
- Modify: `frontend/src/styles/app.css`

1. 保持 `album_input_ref.click()`。
2. 将 `.file-input-hidden` 改为“视觉隐藏但可触发”样式，避免 `display:none` 触发兼容性问题。
3. 验证 `on:change` 后可进入预览态。

### Task 4: 编译与流程验证

**Files:**
- Verify only

1. `cd frontend && cargo check`
2. `docker compose up -d` 并确认健康状态。
3. 按 `docs/run/integration-testing.md` 跑完整 API 流程。

### Task 5: 社区/历史骨架屏重设计

**Files:**
- Modify: `frontend/src/pages/history.rs`
- Modify: `frontend/src/pages/community.rs`
- Modify: `frontend/src/styles/app.css`

1. 历史页加载中渲染结构化骨架卡片（替换单一 spinner）。
2. 社区页新增加载骨架并与空态分离，避免加载态误用空态卡片。
3. 统一骨架 shimmer、圆角、边框和内边距，确保两页一致。
