# 032-首页/拍摄页交互与布局修复技术方案

## 关联需求
- `docs/requirements/032-capture-page-ux-fixes-requirements.md`

## 设计目标
- 仅在前端做小范围修复，解决首页与拍摄页的可用性/密度问题。
- 保持当前视觉语言，不引入新依赖，不改动路由和后端协议。

## 方案概览
1. 在 `capture.rs` 精简重复步骤、收紧结构间距、下调拍摄页空态尺寸。
2. 在样式中为品牌区提供更稳的顶部留白，避免图标浮动贴顶。
3. 调整隐藏文件输入策略，避免 `display: none` 场景下 programmatic click 失效风险。
4. 统一相关容器的边框与内边距，提升一致性。
5. 重做社区/历史加载态骨架屏，统一样式语言与结构占位。

## 详细设计

### 1) 首页图标与首屏布局
- 文件：`frontend/src/pages/capture.rs`
- 调整点：
  - 品牌区 `pt/pb` 与图标外层间距下调但保留顶部余量。
  - 减少步骤卡与 CTA 间空白，移除无意义 `flex-1` 拉伸导致的大间隔。
  - 首页“使用步骤”保持 3 条唯一步骤。

### 2) 拍摄页视觉密度
- 文件：`frontend/src/pages/capture.rs`
- 调整点：
  - 空态 icon 容器尺寸、标题/说明间距、按钮区间距适当缩小。
  - 上传态（预览 + 提示 + 按钮）区块 spacing 收紧，减少一屏内滚动压力。
  - 拍摄小贴士卡片内边距适度降低。

### 3) “从相册选择”可用性修复
- 文件：
  - `frontend/src/pages/capture.rs`
  - `frontend/src/styles/app.css`
- 调整点：
  - 保持 `NodeRef + input.click()` 逻辑。
  - 将 `.file-input-hidden` 从 `display: none` 改为可编程触发更稳定的“视觉隐藏”方案（绝对定位 + 透明 + 1px 尺寸）。
  - 保留 `accept="image/*"` 与 `on:change`，确保选择后进入统一预览流程。

### 4) 边框与内边距优化
- 文件：
  - `frontend/src/pages/capture.rs`
  - `frontend/src/styles/app.css`（仅必要时）
- 调整点：
  - 首页步骤卡、拍摄页主卡、提示卡统一轻边框和紧凑 padding。
  - 不改颜色体系，仅做密度和细节微调。

### 5) 社区/历史骨架屏重设计
- 文件：
  - `frontend/src/pages/history.rs`
  - `frontend/src/pages/community.rs`
  - `frontend/src/styles/app.css`
- 调整点：
  - 历史页：`loading=true` 时渲染列表骨架卡片，替换当前仅 `LoadingSpinner` 的表现。
  - 社区页：区分“加载中”和“无数据”两种状态；加载中渲染骨架，空数据保留独立空态卡。
  - 样式：新增一套可复用骨架样式（卡片、缩略图、文本行、底部元信息、shimmer 动画），并与现有配色保持一致。
  - 保持最小结构，不引入新依赖。

## 风险与回退
- 风险：过度压缩导致视觉拥挤。
- 缓解：仅小幅调整尺寸与间距，优先保留层级清晰。
- 回退：变更集中于 `capture.rs` 与少量样式，单提交可快速回滚。

## 验证策略
1. `cargo check`（frontend）确保无编译错误。
2. 手动验证首页：
  - 图标上浮不触顶。
  - 步骤仅 3 条且无重复。
  - 首页首屏 CTA 可见性改善。
3. 手动验证拍摄页：
  - 空态视觉更紧凑。
  - 点击“从相册选择”可打开选择器并进入预览。
4. 按项目执行清单启动本地服务并跑完整 API 流程。
5. 手动验证社区/历史在加载态和空态切换时视觉与状态正确。
