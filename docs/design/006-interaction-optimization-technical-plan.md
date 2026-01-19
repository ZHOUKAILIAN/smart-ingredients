# 006-交互优化技术方案

## 元数据

| 字段     | 值                         |
| -------- | -------------------------- |
| 文档编号 | 006-interaction-optimization |
| 标题     | 交互体验优化技术方案       |
| 版本     | 1.0                        |
| 状态     | 草稿                       |
| 创建日期 | 2026-01-19                 |
| 更新日期 | 2026-01-19                 |
| 作者     | Smart Ingredients Team     |
| 关联需求 | 006-interaction-optimization-requirements |

## 概述

### 目的

本技术方案文档详细描述如何实现交互体验优化需求，包括：
1. 添加加载状态反馈机制
2. 重构分析结果页面为概要+详情两页结构
3. 增强错误处理和恢复能力
4. 实现拍照功能（移动端调用相机，桌面端文件选择器）

### 范围

**包含**：
- 前端状态管理扩展（加载状态、页面状态）
- UI 组件开发（Loading、概要页、详情页、错误页）
- 页面路由调整
- Tauri API 集成（相机/文件选择器）
- 动画和过渡效果

**不包含**：
- 后端 API 修改
- OCR 或 LLM 服务优化
- 性能监控系统

### 假设

- 现有的 OCR 和 LLM API 接口保持不变
- Tauri 2.x 提供足够的相机和文件系统 API
- Leptos 0.7.x 的响应式系统能够支持所需的状态管理
- 移动端平台（iOS/Android）能够正常调用系统相机

## 架构设计

### 高层架构

```
┌─────────────────────────────────────────────────────────┐
│                    Frontend (Leptos)                     │
├─────────────────────────────────────────────────────────┤
│  Pages:                                                  │
│  - CapturePage (拍照/选择图片)                           │
│  - OcrPage (OCR识别 + Loading)                          │
│  - ConfirmPage (确认文本)                                │
│  - AnalyzingPage (LLM分析 + Loading)                     │
│  - SummaryPage (概要页 - 新增)                           │
│  - DetailPage (详情页 - 新增)                            │
│  - ErrorPage (错误页 - 新增)                             │
├─────────────────────────────────────────────────────────┤
│  Components:                                             │
│  - LoadingSpinner (加载动画)                             │
│  - SummaryCard (概要卡片)                                │
│  - IngredientList (配料列表)                             │
│  - ErrorDisplay (错误提示)                               │
├─────────────────────────────────────────────────────────┤
│  State Management:                                       │
│  - AppState (全局状态)                                   │
│    - loading_state: LoadingState                         │
│    - current_page: PageState                             │
│    - error: Option<ErrorInfo>                            │
├─────────────────────────────────────────────────────────┤
│  Tauri APIs:                                             │
│  - Camera API (移动端)                                   │
│  - File Dialog API (桌面端)                              │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│              Backend (Axum) - 不修改                     │
│  - POST /api/ocr                                         │
│  - POST /api/analyze                                     │
└─────────────────────────────────────────────────────────┘
```

### 组件图

```
App
├── Router
│   ├── CapturePage
│   │   ├── CameraButton (调用 Tauri Camera/File API)
│   │   └── ExampleImages
│   ├── OcrPage
│   │   ├── ImagePreview
│   │   ├── LoadingSpinner (新增)
│   │   └── ErrorDisplay (新增)
│   ├── ConfirmPage
│   │   └── TextEditor
│   ├── AnalyzingPage
│   │   └── LoadingSpinner (新增)
│   ├── SummaryPage (新增)
│   │   ├── HealthScoreCard
│   │   ├── RiskSummary
│   │   ├── KeyStats
│   │   └── ViewDetailButton
│   ├── DetailPage (新增)
│   │   ├── BackButton
│   │   └── IngredientList
│   └── ErrorPage (新增)
│       ├── ErrorDisplay
│       └── BackToHomeButton
└── AppState (Context)
```

### 技术栈

| 组件           | 技术                  | 选择理由                                   |
| -------------- | --------------------- | ------------------------------------------ |
| 前端框架       | Leptos 0.7.x          | 现有技术栈，响应式状态管理                 |
| 桌面应用框架   | Tauri 2.x             | 现有技术栈，提供系统 API 访问              |
| 路由           | leptos_router         | Leptos 官方路由库                          |
| 状态管理       | Leptos Signals        | 细粒度响应式更新                           |
| 动画           | CSS Transitions       | 轻量级，性能好                             |
| 相机 API       | Tauri Camera Plugin   | 跨平台相机访问                             |
| 文件选择       | Tauri Dialog API      | 桌面端文件选择器                           |

## 数据模型

### 状态类型定义

```rust
// frontend/src/stores/mod.rs

/// 加载状态
#[derive(Clone, Debug, PartialEq)]
pub enum LoadingState {
    Idle,
    OcrProcessing,      // OCR 识别中
    LlmAnalyzing,       // LLM 分析中
}

/// 页面状态（用于分析结果页面）
#[derive(Clone, Debug, PartialEq)]
pub enum ResultPageState {
    Summary,  // 概要页
    Detail,   // 详情页
}

/// 错误信息
#[derive(Clone, Debug)]
pub struct ErrorInfo {
    pub title: String,
    pub message: String,
    pub recoverable: bool,  // 是否可恢复（显示返回首页按钮）
}

/// 扩展后的 AppState
#[derive(Clone)]
pub struct AppState {
    // 现有字段
    pub analysis_id: RwSignal<Option<Uuid>>,
    pub analysis_result: RwSignal<Option<AnalysisResponse>>,
    pub ocr_text: RwSignal<Option<String>>,
    pub confirmed_text: RwSignal<Option<String>>,

    // 新增字段
    pub loading_state: RwSignal<LoadingState>,
    pub result_page_state: RwSignal<ResultPageState>,
    pub error: RwSignal<Option<ErrorInfo>>,
    pub selected_image_path: RwSignal<Option<String>>,
}
```

### 数据流

```
1. 拍照/选择图片流程:
   User Click "拍照"
   → Tauri Camera/File API
   → 返回图片路径
   → 更新 selected_image_path
   → 导航到 OcrPage

2. OCR 识别流程:
   User Click "开始识别"
   → 设置 loading_state = OcrProcessing
   → 调用 POST /api/ocr
   → 成功: 更新 ocr_text, loading_state = Idle, 导航到 ConfirmPage
   → 失败: 设置 error, loading_state = Idle, 显示错误

3. LLM 分析流程:
   User Click "开始分析"
   → 设置 loading_state = LlmAnalyzing
   → 调用 POST /api/analyze
   → 成功: 更新 analysis_result, loading_state = Idle, 导航到 SummaryPage
   → 失败: 设置 error, loading_state = Idle, 显示错误

4. 查看详情流程:
   User Click "查看详细配料列表"
   → 设置 result_page_state = Detail
   → 导航到 DetailPage

5. 错误恢复流程:
   User Click "返回首页"
   → 清空所有状态
   → 导航到 CapturePage
```

## API 设计

### Tauri Commands

```rust
// src-tauri/src/main.rs

/// 调用系统相机（移动端）或文件选择器（桌面端）
#[tauri::command]
async fn pick_image() -> Result<String, String> {
    #[cfg(mobile)]
    {
        // 使用 Tauri Camera Plugin
        use tauri_plugin_camera::Camera;
        Camera::take_photo().await
            .map_err(|e| e.to_string())
    }

    #[cfg(not(mobile))]
    {
        // 使用文件选择器
        use tauri::api::dialog::FileDialogBuilder;
        FileDialogBuilder::new()
            .add_filter("Images", &["png", "jpg", "jpeg"])
            .pick_file()
            .await
            .ok_or("No file selected".to_string())
    }
}
```

### 前端服务接口（不变）

现有的 OCR 和 LLM API 调用保持不变：

```rust
// frontend/src/services/mod.rs

pub async fn call_ocr(image_path: &str) -> Result<String, String>;
pub async fn call_analyze(text: &str) -> Result<AnalysisResponse, String>;
```

## 组件设计

### 1. LoadingSpinner 组件

```rust
// frontend/src/components/loading_spinner.rs

#[component]
pub fn LoadingSpinner(
    /// 加载提示文字
    message: String,
) -> impl IntoView {
    view! {
        <div class="loading-container">
            <div class="spinner"></div>
            <p class="loading-message">{message}</p>
        </div>
    }
}
```

**样式**：
- 居中显示
- 旋转动画（CSS animation）
- 半透明背景遮罩

### 2. SummaryPage 页面

```rust
// frontend/src/pages/summary.rs

#[component]
pub fn SummaryPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let analysis = state.analysis_result.get();

    let navigate = leptos_router::use_navigate();

    let on_view_detail = move |_| {
        navigate("/detail", Default::default());
    };

    view! {
        <div class="summary-page">
            <h1>"分析结果概要"</h1>

            // 健康评分
            <HealthScoreCard score={analysis.health_score} />

            // 风险提示
            <RiskSummary risks={analysis.risks} />

            // 关键统计
            <KeyStats
                total_ingredients={analysis.ingredients.len()}
                high_risk_count={analysis.high_risk_count}
            />

            // 查看详情按钮
            <button
                class="btn-primary"
                on:click=on_view_detail
            >
                "查看详细配料列表"
            </button>
        </div>
    }
}
```

### 3. DetailPage 页面

```rust
// frontend/src/pages/detail.rs

#[component]
pub fn DetailPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let analysis = state.analysis_result.get();

    let navigate = leptos_router::use_navigate();

    let on_back = move |_| {
        navigate("/summary", Default::default());
    };

    view! {
        <div class="detail-page">
            <header>
                <button class="btn-back" on:click=on_back>
                    "← 返回概要"
                </button>
                <h1>"详细配料列表"</h1>
            </header>

            <IngredientList ingredients={analysis.ingredients} />
        </div>
    }
}
```

### 4. ErrorDisplay 组件

```rust
// frontend/src/components/error_display.rs

#[component]
pub fn ErrorDisplay(
    /// 错误信息
    error: ErrorInfo,
    /// 返回首页回调
    on_back_home: impl Fn() + 'static,
) -> impl IntoView {
    view! {
        <div class="error-container">
            <div class="error-icon">⚠️</div>
            <h2>{error.title}</h2>
            <p>{error.message}</p>

            {move || if error.recoverable {
                view! {
                    <button
                        class="btn-primary"
                        on:click=move |_| on_back_home()
                    >
                        "返回首页"
                    </button>
                }.into_view()
            } else {
                view! { <></> }.into_view()
            }}
        </div>
    }
}
```

## 错误处理

### 错误类型

| 场景             | 错误标题       | 错误消息                                   | 可恢复 |
| ---------------- | -------------- | ------------------------------------------ | ------ |
| OCR 识别失败     | "识别失败"     | "无法识别图片中的文字，请确保图片清晰可读" | 是     |
| LLM 分析失败     | "分析失败"     | "无法分析配料表，请稍后重试"               | 是     |
| 网络错误         | "网络错误"     | "网络连接失败，请检查网络设置"             | 是     |
| 相机权限被拒绝   | "需要相机权限" | "请在设置中开启相机权限"                   | 是     |
| 图片格式不支持   | "格式错误"     | "不支持的图片格式，请选择 JPG 或 PNG"      | 是     |

### 错误处理流程

```rust
// 在每个异步操作中统一处理错误

async fn handle_ocr(image_path: String) {
    let state = use_context::<AppState>().expect("AppState not found");

    state.loading_state.set(LoadingState::OcrProcessing);

    match call_ocr(&image_path).await {
        Ok(text) => {
            state.ocr_text.set(Some(text));
            state.loading_state.set(LoadingState::Idle);
            // 导航到确认页
        }
        Err(e) => {
            state.error.set(Some(ErrorInfo {
                title: "识别失败".to_string(),
                message: format!("无法识别图片中的文字: {}", e),
                recoverable: true,
            }));
            state.loading_state.set(LoadingState::Idle);
            // 显示错误提示
        }
    }
}
```

## 性能考虑

### 动画性能

- 使用 CSS `transform` 和 `opacity` 属性（GPU 加速）
- 避免使用 `width`、`height` 等触发 layout 的属性
- 动画时长控制在 200-300ms

```css
/* 页面切换动画 */
.page-transition {
    transition: transform 0.3s ease-out, opacity 0.3s ease-out;
}

.page-enter {
    transform: translateX(100%);
    opacity: 0;
}

.page-enter-active {
    transform: translateX(0);
    opacity: 1;
}
```

### 加载状态优化

- Loading 组件使用 CSS animation，不依赖 JavaScript
- 防抖按钮点击，避免重复请求

```rust
let is_loading = create_memo(move |_| {
    state.loading_state.get() != LoadingState::Idle
});

view! {
    <button
        disabled={is_loading}
        on:click=on_submit
    >
        "开始识别"
    </button>
}
```

### 图片处理

- 图片预览使用缩略图（如果 Tauri 支持）
- 限制上传图片大小（< 10MB）

## 测试策略

### 单元测试

- [ ] `LoadingState` 状态转换逻辑
- [ ] `ErrorInfo` 创建和验证
- [ ] 路由导航逻辑

### 集成测试

- [ ] OCR 识别流程（mock API）
- [ ] LLM 分析流程（mock API）
- [ ] 错误恢复流程
- [ ] 页面切换流程

### E2E 测试

- [ ] 完整的用户流程：拍照 → OCR → 确认 → 分析 → 查看概要 → 查看详情
- [ ] 错误场景：OCR 失败 → 返回首页 → 重试
- [ ] 相机权限拒绝场景

### 平台兼容性测试

- [ ] iOS 相机调用
- [ ] Android 相机调用
- [ ] macOS 文件选择器
- [ ] Windows 文件选择器
- [ ] Web 版本（如果支持）

## 部署

### 环境要求

- Tauri 2.x CLI
- Rust 1.70+
- Node.js 18+ (构建前端)

### 配置

**Tauri 配置**（`src-tauri/tauri.conf.json`）：

```json
{
  "plugins": {
    "camera": {
      "enabled": true
    }
  },
  "permissions": [
    "camera",
    "fs:read-file",
    "fs:write-file"
  ]
}
```

**移动端权限**：

- iOS: `Info.plist` 添加 `NSCameraUsageDescription`
- Android: `AndroidManifest.xml` 添加 `CAMERA` 权限

### 回滚计划

如果新版本出现严重问题：
1. 恢复到上一个稳定版本
2. 禁用新增页面的路由
3. 使用 feature flag 关闭新功能

## 实施阶段

### 阶段 1：状态管理和基础组件（2天）

- [ ] 扩展 `AppState` 添加新字段
- [ ] 实现 `LoadingSpinner` 组件
- [ ] 实现 `ErrorDisplay` 组件
- [ ] 编写单元测试

### 阶段 2：加载状态集成（1天）

- [ ] 在 `OcrPage` 集成 Loading 状态
- [ ] 在 `AnalyzingPage` 集成 Loading 状态
- [ ] 测试加载状态显示和隐藏

### 阶段 3：概要和详情页面（3天）

- [ ] 设计概要页面布局
- [ ] 实现 `SummaryPage` 组件
- [ ] 实现 `DetailPage` 组件
- [ ] 添加页面切换动画
- [ ] 更新路由配置
- [ ] 测试页面切换流程

### 阶段 4：错误处理（2天）

- [ ] 统一错误处理逻辑
- [ ] 在所有异步操作中添加错误处理
- [ ] 实现"返回首页"功能
- [ ] 测试各种错误场景

### 阶段 5：拍照功能（3天）

- [ ] 集成 Tauri Camera Plugin
- [ ] 实现 `pick_image` Tauri command
- [ ] 更新 `CapturePage` 调用相机 API
- [ ] 处理相机权限请求
- [ ] 桌面端文件选择器降级
- [ ] 测试各平台相机功能

### 阶段 6：集成测试和优化（2天）

- [ ] E2E 测试完整流程
- [ ] 性能优化（动画流畅度）
- [ ] UI 细节调整
- [ ] 跨平台兼容性测试

### 阶段 7：文档和发布（1天）

- [ ] 更新用户文档
- [ ] 更新开发文档
- [ ] 准备发布说明
- [ ] 打包和发布

**总计：约 14 天**

## 风险与缓解

| 风险                         | 影响 | 可能性 | 缓解措施                                           |
| ---------------------------- | ---- | ------ | -------------------------------------------------- |
| Tauri Camera Plugin 不稳定   | 高   | 中     | 提前测试，准备备选方案（纯文件选择器）             |
| 移动端权限处理复杂           | 中   | 高     | 参考官方文档，提供清晰的权限请求说明               |
| 动画性能问题                 | 中   | 低     | 使用 CSS GPU 加速，避免复杂动画                    |
| 页面状态管理复杂             | 中   | 中     | 使用清晰的状态机模式，充分测试状态转换             |
| 不同平台行为不一致           | 高   | 中     | 在所有目标平台上测试，提供平台特定的降级方案       |
| 概要页面内容设计不合理       | 中   | 中     | 与产品团队充分沟通，进行用户测试                   |

## 待解决问题

| 问题                                  | 影响 | 负责人   | 状态 |
| ------------------------------------- | ---- | -------- | ---- |
| 概要页面具体显示哪些统计信息？        | 高   | 产品     | 开放 |
| Loading 动画使用哪种风格？            | 低   | UI 设计  | 开放 |
| Web 版本是否支持相机调用？            | 中   | 前端开发 | 开放 |
| 是否需要图片压缩功能？                | 中   | 前端开发 | 开放 |
| 错误提示文案是否需要国际化？          | 低   | 产品     | 开放 |

## 参考资料

- [Tauri 2.x Documentation](https://tauri.app/v2/)
- [Tauri Camera Plugin](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/camera)
- [Leptos Router](https://docs.rs/leptos_router/)
- [Leptos Signals](https://leptos.dev/guide/reactivity/)
- [CSS Transitions Best Practices](https://web.dev/animations/)
- 相关需求文档：
  - `006-interaction-optimization-requirements.md`
- 相关设计文档：
  - `001-mobile-ui-technical-plan.md`
  - `005-ui-optimization-technical-plan.md`

---

## 变更记录

| 版本 | 日期       | 作者                       | 描述     |
| ---- | ---------- | -------------------------- | -------- |
| 1.0  | 2026-01-19 | Smart Ingredients Team     | 初始版本 |
