# 033-Frontend Tailwind 迁移（一期）技术方案

## 元数据

| 字段 | 值 |
|------|-----|
| 文档编号 | 033 |
| 标题 | Frontend Tailwind 迁移（一期）技术方案 |
| 版本 | 1.0 |
| 状态 | 实施中 |
| 创建日期 | 2026-02-26 |
| 更新日期 | 2026-02-26 |
| 作者 | Codex |
| 关联需求 | `docs/requirements/033-frontend-tailwind-migration-phase1-requirements.md` |

## 目的

在一期范围内，将 `frontend/` 样式迁移为 Tailwind 主导，保留必要全局基础样式，确保行为不变、风险可控、可分批回滚。

## 现状调研结论

1. 已存在大量 Tailwind class（尤其在 `capture.rs`、`community.rs` 等页面）。
2. 同时存在较多语义类依赖：`app.css`、`bottom-nav.css`、`figma-util.css`。
3. 项目通过 Trunk 构建，当前具备 CSS 引入链路（`frontend/index.html`）。
4. 当前主要风险不是技术不可行，而是迁移过程中的视觉回归和类名膨胀。

## 设计原则

1. Tailwind 优先：新增/改造样式默认使用 utility class。
2. 全局样式最小化：仅保留基础规则，不保留页面私有视觉逻辑。
3. 渐进迁移：按批次迁移并每批验证，避免一次性大爆炸改动。
4. 等价迁移：优先保持视觉与交互等价，必要调整需记录。

## 技术设计

### 1) Tailwind 配置与入口

- 检查并补齐以下内容（如已存在则复用）：
  - `tailwind.config.js`/`tailwind.config.cjs`：覆盖 `frontend/src/**/*.rs`、`frontend/index.html` 扫描路径；
  - Tailwind 输入样式入口（如 `src/styles/tailwind.css`）；
  - `@tailwind base; @tailwind components; @tailwind utilities;`；
  - Trunk 对编译后 CSS 的引用关系。
- 若当前已通过其他方式引入 Tailwind，则统一到单一入口并文档化。

### 2) 样式分层策略

保留全局样式（允许）：
- `:root` 基础变量；
- 全局 reset/base；
- 通用关键帧动画（如 shimmer、float）；
- 少量跨页面工具类（必须可解释且复用明显）；
- 第三方兼容样式（若有）。

迁移到 Tailwind（必须）：
- 页面布局（容器、栅格、间距）；
- 卡片、按钮、文本、边框、阴影等视觉样式；
- 页面私有类（如 `page-*`、`*-card`、`*-state`）优先消除。

### 3) 批次执行计划

#### 批次 A：基础设施与 token 对齐
- 目标：配置稳定、类名可扫描、theme token 可用。
- 文件：Tailwind 配置文件、样式入口、`frontend/index.html`（如需）。
- 验证：`trunk build`、`cargo check -p smart-ingredients-app`。

#### 批次 B：核心流程页面迁移
- 页面：`capture`、`ocr`、`confirm`、`summary`、`detail`。
- 方法：先布局后组件，先容器后细节，逐页迁移并对照。
- 验证：关键流程手工回归（拍摄 -> OCR -> 结果）。

#### 批次 C：社区与历史页面迁移
- 页面：`community`、`community_detail`、`history`。
- 方法：替换页面私有类与骨架屏样式，保持数据状态展示不变。
- 验证：列表、空态、加载态、详情跳转回归。

#### 批次 D：公共组件与 CSS 清理
- 组件：`components/` 下通用组件。
- 清理：删除无引用 CSS 规则，收敛到少量基础全局样式。
- 验证：全局巡检 + 编译检查。

## 风险与缓解

1. 风险：视觉细节回归（间距/字号/层级变化）。
- 缓解：逐页对照，单批提交，保留必要截图比对。

2. 风险：类名过长导致可读性下降。
- 缓解：使用可复用片段（必要时 `@layer components`）而不是重复长串类。

3. 风险：Trunk/Tailwind 构建链路不稳定。
- 缓解：优先使用项目已有链路；仅做最小配置增补。

## 验证计划

每批次执行：
1. `cargo check -p smart-ingredients-app`
2. `NO_COLOR=false trunk build`
3. 手工页面验证（本批涉及页面）

收尾执行（按仓库清单）：
1. 启动本地服务并检查健康（受 Docker 环境可用性影响）
2. 跑完整 API 流程
3. 再次执行 frontend `cargo check`

## 回滚方案

- 采用分批提交策略，每批出现回归可单独回滚。
- 全局样式删除前先确认无引用，必要时保留过渡期兼容层。

## 待确认事项

1. 是否接受在迁移过程中对个别页面做“等价微调”（1-2px/字号微调）以匹配 Tailwind 标准刻度。
2. 是否需要在一期结束时输出一份“保留全局样式白名单”文档附录。

## 实施进展（2026-02-26）

已完成：
1. 批次 A（基础设施）
 - Tailwind 配置与构建链路接入（Trunk pre_build hook + Tailwind 产物接入）。
 - 移除 `figma-util.css` 依赖，改为真实 Tailwind 生成样式。
2. 批次 B（核心流程页）
 - `capture/ocr/analyzing/confirm/summary/detail` 主结构样式逐步切换为 Tailwind class。
3. 批次 C（部分）
 - `login/register/profile/onboarding/community_detail` 主结构切换为 Tailwind class。
 - `community/history` 骨架屏结构切换为 Tailwind class。

新增完成：
1. 批次 D（组件级迁移）
 - `export_preview_modal / confirm_modal / share_button / community_share_button / toast / preference_selector / error_display / loading_spinner / image_preview / usage_tips / example_images / ingredient_table` 已迁移为 Tailwind utility class。
 - `capture.rs`、`community.rs` 中剩余语义类（如 `figma-btn-*`、`empty-state-*`）已替换或移除。
2. 全局样式收敛
 - `frontend/src/styles/app.css` 已收敛为基础全局样式（字体、reset、`app-shell`、`page`、`page-scrollable-content`、`skip-link`），删除大量页面私有规则。

验证结果：
1. `cargo check -p smart-ingredients-app` 通过（warnings only）。
2. `NO_COLOR=false trunk build` 可通过；存在一次 wasm-bindgen stage 目录偶发错误，重跑后成功。
3. `docker compose up -d postgres redis backend` 受本机 Docker daemon 不可用阻塞（`Cannot connect to .../docker.sock`），因此后端健康检查与完整 API 流程验证待环境恢复后执行。
