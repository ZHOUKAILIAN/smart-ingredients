# 高端极简 UI 视觉系统与页面落地设计

## 元数据

| 字段 | 值 |
| --- | --- |
| 标题 | 高端极简 UI 视觉系统与页面落地设计 |
| 日期 | 2026-02-20 |
| 版本 | 1.0 |
| 状态 | 已确认 |
| 关联需求 | docs/requirements/005-ui-optimization-requirements.md, docs/requirements/023-ui-ux-presentation-requirements.md |
| 关联方案 | docs/design/005-ui-optimization-technical-plan.md, docs/design/023-ui-ux-presentation-technical-plan.md |

## 目标

- 统一“先锋极简/高端质感”的视觉语言，确保首页、流程页、结果页风格一致。
- 用响应式规则替代固定像素，适配 C 端不同机型。
- 先结论后证据，强化信息层级与可信感。

## 设计方向（已确认）

- **视觉风格**：先锋极简 / 高端质感。
- **色彩方向**：亮色为主，象牙白背景 + 深炭黑文字 + 少量品牌绿点缀。
- **字体方向**：高端极简，中文采用 Source Han Serif SC，英文采用 Cormorant Garamond（OFL）。
- **卡片质感**：轻奢面板（细线边框 + 微弱内高光 + 轻外阴影）。
- **动效风格**：明显仪式感（分段入场 + 轻微漂浮感）。
- **响应式策略**：全量 rem + clamp，避免写死 px。

## 视觉系统与 Tokens

### 颜色（示例值，落地时写入 tokens）

- 背景（BG）：象牙白，如 #f7f5f2
- 表面（Surface）：纯白 #ffffff
- 文字（Primary）：深炭黑 #1a1a1a
- 次级文字（Muted）：#5c5c5c
- 细线边框（Hairline）：暖灰 #e8e4df
- 主色（Primary）：精致绿 #1f8a59（并提供浅色填充）
- 风险色：
  - 低风险：#1f8a59
  - 中风险：#b76b0b
  - 高风险：#a13333

### 字体

- `--font-display`: "Source Han Serif SC", "Cormorant Garamond", serif
- `--font-body`: "Source Han Serif SC", serif
- 数字/评分可优先使用 `--font-display` 以强化高级质感

### 组件质感

- 卡片：1px 细线边框 + 轻微内高光 + 极浅外阴影
- 圆角：卡片 14px，按钮 10px，标签 pill 999px
- 阴影：极低强度，用于区分层级，不追求明显浮起

## 响应式规则（rem + clamp）

- **字体**：
  - `--fs-display: clamp(1.75rem, 6vw, 2.75rem)`
  - `--fs-h1: clamp(1.25rem, 4vw, 1.75rem)`
  - `--fs-body: clamp(0.875rem, 2.6vw, 1rem)`
- **间距**（示例变量）：
  - `--space-3: clamp(0.5rem, 2vw, 0.75rem)`
  - `--space-5: clamp(1rem, 3.5vw, 1.5rem)`
- **容器宽度**：`width: min(92vw, 36rem)`
- **触控面积**：按钮 `min-height: clamp(2.5rem, 8vw, 3rem)`
- **图像/图表**：`width: 100%` + `aspect-ratio`，避免固定高度

## 页面落地规范

### 首页（CapturePage）

- 首屏只保留：品牌区 + 一句话说明 + 主按钮。
- 示例图与步骤说明默认折叠（`details/summary` 或自定义折叠组件）。
- 品牌区图标尺寸使用 clamp；标题用 `--font-display`。
- 主按钮为深绿实心，次要操作使用线框。

### 流程页（OCR/确认/分析）

- OCR 页与 LLM 分析页使用统一“仪式感加载模板”。
- 进度条：极细线条 + 微光动效。
- 确认页：
  - 顶部 OCR 文本编辑区（细线边框 + 轻内阴影）。
  - 底部双按钮：主按钮“确认并分析”，次按钮“重新拍照”。

### 结果页（ResultPage）

- 首屏结构：结论卡 → 关键风险标签 → 可信度条 + 因子。
- 次级区块：风险维度图、摘要卡（折叠）、人群建议卡、替代建议卡。
- 配料卡片：名称 + 风险徽章 + 标签行 + 可选备注；未知字段不展示。

## 动效规范

- 页面进入分段淡入：标题 → 主卡片 → 次级信息。
- hover/press：轻微位移 + 高光变化，保持克制。
- 动效时长建议 180–240ms，避免过长影响效率。

## 实现路径（不写代码）

1) **设计 Tokens 与字体**：更新 `frontend/src/styles/app.css`，引入字体文件并声明变量。
2) **基础组件语法**：统一卡片/按钮/标签样式，全部切换为 rem + clamp。
3) **页面落地**：按“先结论后证据”重排结果页；首页首屏精简；流程页统一加载模板。
4) **降级策略**：缺字段时隐藏或展示占位，不阻塞主结论。

## 验证计划（实现后执行）

- 启动本地服务（docker compose）并确认健康。
- 跑完整 API 流程：上传 → OCR → 确认 → LLM → 结果。
- 前端 `cargo check` 通过（允许 warnings）。

## 变更记录

- 2026-02-20：首次确认高端极简视觉系统与页面落地规范。
