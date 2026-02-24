# 029-Figma 风格统一技术方案

## 元数据

| 字段     | 值 |
| -------- | --- |
| 文档编号 | 029-figma-style-unification |
| 标题     | Figma 风格统一技术方案 |
| 版本     | 1.0 |
| 状态     | 草稿 |
| 创建日期 | 2026-02-21 |
| 更新日期 | 2026-02-21 |
| 作者     | Codex |
| 关联需求 | 029-figma-style-unification |

## 概述

### 目的
以 Scoped 主题方式将前端页面视觉风格统一到 Figma 代码包参考，保持页面结构与业务流程不变。

### 范围
- 前端样式与页面标记类名调整
- 底部导航样式与交互视觉统一
- 不涉及后端、路由与流程变更

### 假设
- 保持现有字体不变
- Figma 代码包作为视觉基准（Home/Scan/Results/Processing）

## 架构设计

### 高层设计
在 `frontend/src/styles/app.css` 与 `frontend/src/styles/bottom-nav.css` 中新增/调整 Figma 主题样式，并在页面根节点添加 `.figma` 类实现 scoped 覆盖。

### 组件映射

| Figma 视觉元素 | 对应实现 |
| ------------- | -------- |
| 渐变背景 | `.page.*.figma` 背景渐变 |
| 主 CTA | `.primary-cta`/`.primary-button` |
| 次 CTA | `.secondary-cta`/`.link-button` |
| 卡片 | `.surface-card`/`.community-card`/历史卡片 |
| 顶部栏 | `.page-topbar`/`.figma-header` |
| TabBar | `.bottom-nav` + active 状态 |

## 实施方案

### 1) 主题 Token 与公共样式
文件：`frontend/src/styles/app.css`
- 新增 Figma 主题变量（emerald/teal/amber）并限制在 `.page.*.figma` 作用域
- 更新卡片、按钮、标签、标题、输入框等通用样式
- 保持字体不变，仅调整字号/颜色/阴影/圆角

### 2) 底部导航统一
文件：`frontend/src/styles/bottom-nav.css`
- 参考 Figma BottomNav：半透明背景、圆角高亮、微发光
- Tab 顺序使用现有逻辑（Home/History/Community/Profile）

### 3) 页面级样式应用
文件（标记类）：`frontend/src/pages/*.rs`
- 为以下页面根节点增加 `.figma` 类或对齐已有 `.figma`：
  - Capture（首页）、OCR、Confirm、Analyzing、Summary、Detail
  - History、Community、CommunityDetail、Profile
  - Login、Register、Onboarding
- 保持结构与文案不变，仅调整 class 与布局容器样式

### 4) 组件细节对齐
- 结果卡片/评分卡/列表项对齐 Figma 阴影与圆角
- 输入框/表单按钮对齐 Figma 风格
- Loading/等待页使用 ProcessingPage 视觉要素（进度条/图标/卡片）

## 测试策略

### 手动检查
- 逐页检查视觉一致性（与 Figma 参考）
- Tab 切换、页面滚动与按钮状态

### 自动化
- `cargo test -p smart-ingredients-app`
- `cargo check -p smart-ingredients-app`

## 风险与缓解

| 风险 | 影响 | 可能性 | 缓解措施 |
| ---- | ---- | ------ | -------- |
| 样式覆盖过宽导致非目标页受影响 | 中 | 中 | 仅在 `.figma` 作用域内覆盖 |
| 组件间样式冲突 | 中 | 中 | 先建立通用 class，再逐页对齐 |

## 参考资料

- `docs/requirements/007-figma-ui-redesign-requirements.md`
- Figma 代码包（本地）：`/Users/zhoukailian/Downloads/优化食品配料助手UI/`

---

## 变更记录

| 版本 | 日期 | 作者 | 描述 |
| ---- | ---- | ---- | ---- |
| 1.0 | 2026-02-21 | Codex | 初始版本 |
