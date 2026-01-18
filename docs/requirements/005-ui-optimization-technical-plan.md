# 005-UI优化与处理流程改进技术方案

## 元数据

| 字段     | 值                       |
| -------- | ------------------------ |
| 文档编号 | 005-ui-optimization      |
| 标题     | UI优化与处理流程改进技术方案 |
| 版本     | 1.0                      |
| 状态     | 草稿                     |
| 创建日期 | 2026-01-17               |
| 更新日期 | 2026-01-17               |
| 作者     | Smart Ingredients Team   |
| 关联需求 | 005-ui-optimization-requirements.md |

## 概述

### 目的

本技术方案旨在解决当前系统存在的UI体验问题和处理流程性能问题，通过前后端协同优化，提升用户体验和系统响应速度。

### 范围

本设计涵盖：
1. 前端UI组件的重构和优化
2. 后端API的调整以支持分阶段处理
3. 状态管理机制的改进
4. 分析流程的异步化处理

### 假设

- OCR服务和LLM服务可以独立调用
- 数据库支持分阶段保存分析结果
- 前端能够通过轮询或WebSocket获取实时状态更新
- 用户网络环境稳定，支持异步请求

## 架构设计

### 高层架构

```
┌─────────────────────────────────────────────────────────────────┐
│                         Frontend                                │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌──────────┐ │
│  │CapturePage │→ │ OCRPage    │→ │ConfirmPage │→ │ResultPage│ │
│  │  (精简)    │  │ (OCR识别)  │  │ (新增)     │  │ (重构)   │ │
│  └────────────┘  └────────────┘  └────────────┘  └──────────┘ │
│         │              │                │              │        │
│         └──────────────┴────────────────┴──────────────┘        │
│                              │                                   │
│                        AppState (扩展)                          │
└───────────────────────────────┬─────────────────────────────────┘
                                │
                          HTTP/REST API
                                │
┌───────────────────────────────┴─────────────────────────────────┐
│                         Backend API                             │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   Upload    │→ │  OCR Service │  │  LLM Service │          │
│  │  + Trigger  │  │   (独立)     │  │   (独立)     │          │
│  │     OCR     │  └──────────────┘  └──────────────┘          │
│  └─────────────┘          │                  ↑                  │
│         │                 │                  │                  │
│         │                 ↓                  │                  │
│         │        ┌─────────────────┐         │                  │
│         └───────→│  Analysis       │─────────┘                  │
│                  │  Workflow       │                            │
│                  │  (两阶段处理)    │                            │
│                  └─────────────────┘                            │
│                          │                                       │
│                  阶段1: OCR识别                                  │
│                  阶段2: LLM分析 (用户确认后)                     │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                      ┌─────┴─────┐
                      │ Database  │
                      │ (状态存储) │
                      └───────────┘
```

### 组件图

#### 前端组件层次

```
App
├── CapturePage (首页 - 精简布局)
│   ├── BrandSection (品牌展示 - 精简30-40%)
│   ├── FeatureCard (功能介绍 - 一句话)
│   ├── CollapsibleSteps (步骤说明 - 默认折叠) [新增]
│   ├── CollapsibleExamples (示例图片 - 默认折叠) [新增]
│   └── ImagePreview (图片预览)
│
├── OCRPage (OCR识别页面 - 保留原AnalyzingPage)
│   ├── LoadingIndicator (加载指示器)
│   └── StatusText (状态文字："正在识别配料表...")
│
├── ConfirmPage (文本确认页面 - 新增)
│   ├── OCRResultDisplay (OCR结果展示)
│   ├── TextEditor (文本编辑器 - 可编辑) [新增]
│   ├── ActionButtons (操作按钮组) [新增]
│   │   ├── RetakeButton (重新拍照)
│   │   └── ConfirmButton (确认并分析)
│   └── EditTips (编辑提示) [新增]
│
├── AnalyzingPage (LLM分析页面 - 重命名/复用)
│   ├── LoadingIndicator (加载指示器)
│   └── StatusText (状态文字："AI正在分析成分...")
│
└── ResultPage (结果页面 - UI重构)
    ├── HealthScoreCard (健康评分卡片 - 重新设计)
    ├── SummaryCard (摘要卡片 - 可展开/收起)
    ├── WarningsSection (警告信息 - 如有)
    └── IngredientCardList (配料列表 - 重构)
        └── IngredientCard (单个配料卡片 - 重新设计) [重构]
            ├── CardHeader (名称 + 风险徽章)
            ├── TagsRow (类别、功能标签 - 水平排列) [新增]
            └── Note (备注 - 仅有内容时显示)
```

#### 后端服务层次

```
API Layer
├── POST /api/v1/analysis/upload (上传并启动OCR)
├── GET  /api/v1/analysis/:id (获取分析状态和OCR结果)
├── POST /api/v1/analysis/:id/confirm (确认文本并启动LLM分析) [新增]
└── POST /api/v1/analysis/:id/retry (重试失败的步骤)

Service Layer
├── AnalysisService (分析服务 - 重构为两阶段)
│   ├── create_analysis() (创建分析记录)
│   ├── trigger_ocr() (触发OCR识别) [修改]
│   ├── save_ocr_result() (保存OCR结果)
│   ├── trigger_llm() (触发LLM分析) [新增]
│   ├── save_llm_result() (保存LLM结果)
│   └── get_analysis() (获取分析状态和结果)
│
├── OCRService (OCR服务 - 保持不变)
└── LLMService (LLM服务 - 保持不变)

Background Tasks (可选，根据实现方式)
├── OCRTask (OCR后台任务)
└── LLMTask (LLM后台任务)
```

### 技术栈

| 组件   | 技术   | 选择理由   |
| ------ | ------ | ---------- |
| 前端框架 | Leptos 0.7 | 保持现有技术栈，利用响应式特性 |
| 状态管理 | RwSignal + Context | Leptos原生支持，性能好 |
| UI样式 | CSS + CSS Variables | 灵活性高，易于主题化 |
| 后端框架 | Axum 0.7 | 保持现有技术栈 |
| 异步任务 | Tokio + spawn | Rust原生异步支持 |
| 状态存储 | PostgreSQL | 现有数据库，支持事务 |

## 数据模型

### 实体

#### Analysis (分析记录 - 扩展)

```rust
pub struct Analysis {
    pub id: Uuid,
    pub image_url: String,
    pub status: AnalysisStatus,
    pub ocr_status: OcrStatus,        // 新增：OCR状态
    pub ocr_text: Option<String>,     // 新增：OCR识别文本
    pub ocr_completed_at: Option<DateTime<Utc>>, // 新增：OCR完成时间
    pub llm_status: LlmStatus,        // 新增：LLM状态
    pub result: Option<AnalysisResult>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

#### 状态枚举 (新增)

```rust
// OCR处理状态
pub enum OcrStatus {
    Pending,      // 等待处理
    Processing,   // 处理中
    Completed,    // 完成
    Failed,       // 失败
}

// LLM处理状态
pub enum LlmStatus {
    Pending,      // 等待处理
    Processing,   // 处理中
    Completed,    // 完成
    Failed,       // 失败
}

// 整体分析状态 (保持兼容)
pub enum AnalysisStatus {
    Pending,      // 等待处理（OCR未完成）
    Processing,   // 处理中（OCR完成，LLM处理中）
    Completed,    // 完成（OCR和LLM都完成）
    Failed,       // 失败
}
```

### 数据库模式

#### 新增字段到 analyses 表

```sql
ALTER TABLE analyses
ADD COLUMN ocr_status VARCHAR(20) DEFAULT 'pending',
ADD COLUMN ocr_text TEXT,
ADD COLUMN ocr_completed_at TIMESTAMP,
ADD COLUMN llm_status VARCHAR(20) DEFAULT 'pending';

-- 创建索引以优化查询
CREATE INDEX idx_analyses_ocr_status ON analyses(ocr_status);
CREATE INDEX idx_analyses_llm_status ON analyses(llm_status);
```

### 数据流（新流程）

```
阶段1: OCR识别
──────────────
1. 用户上传图片
   ↓
2. 后端创建Analysis记录
   (status=OcrPending, ocr_status=Pending, llm_status=NotStarted)
   ↓
3. 触发OCR识别任务
   ↓
4. OCR处理中 (ocr_status=Processing)
   ↓
5. OCR完成 (ocr_status=Completed, ocr_text=识别结果, status=OcrCompleted)
   ↓
6. 前端跳转到文本确认页面
   ↓
7. 用户查看和编辑文本
   ↓
【用户确认环节 - 关键分隔点】

阶段2: LLM分析
──────────────
8. 用户点击"确认并分析"
   ↓
9. 前端提交确认的文本到后端
   ↓
10. 后端更新Analysis记录
    (confirmed_text=用户确认的文本, llm_status=Pending, status=LlmPending)
    ↓
11. 触发LLM分析任务
    ↓
12. LLM处理中 (llm_status=Processing, status=LlmProcessing)
    ↓
13. LLM完成 (llm_status=Completed, result=分析结果, status=Completed)
    ↓
14. 前端跳转到结果页面
```

## API 设计

### 接口列表

| 方法 | 路径 | 描述 | 请求 | 响应 |
| ---- | ---- | ---- | ---- | ---- |
| POST | `/api/v1/analysis/upload` | 上传图片并启动OCR识别 | FormData (file) | UploadResponse |
| GET | `/api/v1/analysis/:id` | 获取分析状态和OCR结果 | - | AnalysisResponse |
| POST | `/api/v1/analysis/:id/confirm` | 确认文本并启动LLM分析 | ConfirmRequest | AnalysisResponse |
| POST | `/api/v1/analysis/:id/retry-ocr` | 重试OCR识别 | - | AnalysisResponse |
| POST | `/api/v1/analysis/:id/retry-llm` | 重试LLM分析 | - | AnalysisResponse |

### 数据结构

#### UploadResponse (保持不变)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResponse {
    pub id: Uuid,
    pub image_url: String,
}
```

#### ConfirmRequest (新增)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmRequest {
    /// 用户确认/编辑后的文本
    pub confirmed_text: String,
}
```

#### AnalysisResponse (扩展)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResponse {
    pub id: Uuid,
    pub status: AnalysisStatus,

    // OCR相关字段
    pub ocr_status: OcrStatus,
    pub ocr_text: Option<String>,          // OCR识别的原始文本
    pub confirmed_text: Option<String>,    // 用户确认/编辑后的文本 [新增]
    pub ocr_completed_at: Option<String>,

    // LLM相关字段
    pub llm_status: LlmStatus,

    // 分析结果
    pub result: Option<AnalysisResult>,
    pub error_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
```

#### AnalysisStatus (扩展枚举)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnalysisStatus {
    OcrPending,       // OCR等待处理 [新增]
    OcrProcessing,    // OCR处理中 [新增]
    OcrCompleted,     // OCR完成，等待用户确认 [新增]
    OcrFailed,        // OCR失败 [新增]
    LlmPending,       // LLM等待处理 [新增]
    LlmProcessing,    // LLM处理中 [新增]
    Completed,        // 全部完成
    Failed,           // 失败
}
```

#### AnalysisResult (保持不变)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub health_score: i32,
    pub recommendation: String,
    pub summary: String,
    pub warnings: Vec<String>,
    pub ingredients: Vec<IngredientInfo>,
    pub table: Vec<TableRow>,
}
```

#### IngredientInfo (保持不变，但前端会过滤显示)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngredientInfo {
    pub name: String,
    pub category: String,        // 前端：为"未知"时不显示
    pub description: Option<String>, // 前端：为None或"未知"时不显示
    pub risk_level: String,
}
```

## 前端实现方案

### 状态管理扩展

```rust
#[derive(Clone)]
pub struct AppState {
    pub analysis_id: RwSignal<Option<Uuid>>,
    pub analysis_result: RwSignal<Option<AnalysisResponse>>,
    pub error_message: RwSignal<Option<String>>,

    // OCR相关状态
    pub ocr_text: RwSignal<Option<String>>,           // OCR识别的原始文本
    pub confirmed_text: RwSignal<Option<String>>,     // 用户确认/编辑后的文本 [新增]
    pub ocr_completed: RwSignal<bool>,

    // LLM相关状态
    pub llm_completed: RwSignal<bool>,
}
```

### OCRPage (OCR识别页面)

```rust
#[component]
pub fn OCRPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();

    // 轮询OCR状态
    create_effect(move |_| {
        let analysis_id = state.analysis_id.get();
        if let Some(id) = analysis_id {
            spawn_local(async move {
                match services::fetch_analysis(id).await {
                    Ok(response) => {
                        state.analysis_result.set(Some(response.clone()));

                        // OCR完成，跳转到确认页面
                        if response.status == AnalysisStatus::OcrCompleted {
                            state.ocr_text.set(response.ocr_text.clone());
                            navigate("/confirm", Default::default());
                        }

                        // OCR失败
                        if response.status == AnalysisStatus::OcrFailed {
                            state.error_message.set(response.error_message);
                        }
                    }
                    Err(err) => state.error_message.set(Some(err)),
                }
            });
        }
    });

    view! {
        <section class="page page-ocr">
            <div class="loading-container">
                <div class="loading-spinner"></div>
                <p class="loading-text">"正在识别配料表..."</p>
                <p class="loading-hint">"请稍候，通常需要3-5秒"</p>
            </div>

            // 错误提示
            <Show when=move || state.error_message.get().is_some()>
                <div class="error-message">
                    {move || state.error_message.get().unwrap_or_default()}
                </div>
                <button class="btn-retry" on:click=/* 重试 */>
                    "重试"
                </button>
            </Show>
        </section>
    }
}
```

### ConfirmPage (文本确认页面 - 新增)

```rust
#[component]
pub fn ConfirmPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();

    // 本地可编辑文本
    let (edited_text, set_edited_text) = create_signal(
        state.ocr_text.get().unwrap_or_default()
    );

    let on_confirm = move |_| {
        let text = edited_text.get();
        let analysis_id = state.analysis_id.get();

        if let Some(id) = analysis_id {
            spawn_local(async move {
                match services::confirm_and_analyze(id, text).await {
                    Ok(_) => {
                        state.confirmed_text.set(Some(edited_text.get()));
                        navigate("/analyzing", Default::default());
                    }
                    Err(err) => state.error_message.set(Some(err)),
                }
            });
        }
    };

    let on_retake = move |_| {
        navigate("/", Default::default());
    };

    view! {
        <section class="page page-confirm">
            <header class="page-header">
                <h1>"识别结果确认"</h1>
                <p class="subtitle">"请确认识别文本是否正确，可以编辑修改"</p>
            </header>

            // 文本编辑器
            <div class="text-editor-container">
                <textarea
                    class="text-editor"
                    rows="10"
                    placeholder="OCR识别的文本..."
                    prop:value=move || edited_text.get()
                    on:input=move |ev| {
                        set_edited_text.set(event_target_value(&ev));
                    }
                />
                <p class="edit-tips">
                    "💡 提示：您可以修改识别错误的文字，以提高分析准确性"
                </p>
            </div>

            // 操作按钮
            <div class="action-buttons">
                <button class="btn-secondary" on:click=on_retake>
                    "重新拍照"
                </button>
                <button class="btn-primary" on:click=on_confirm>
                    "确认并分析"
                </button>
            </div>
        </section>
    }
}
```

### AnalyzingPage (LLM分析页面 - 重命名)

```rust
#[component]
pub fn AnalyzingPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let navigate = use_navigate();

    // 轮询LLM分析状态
    create_effect(move |_| {
        let analysis_id = state.analysis_id.get();
        if let Some(id) = analysis_id {
            spawn_local(async move {
                match services::fetch_analysis(id).await {
                    Ok(response) => {
                        state.analysis_result.set(Some(response.clone()));

                        // LLM分析完成，跳转到结果页面
                        if response.status == AnalysisStatus::Completed {
                            navigate("/result", Default::default());
                        }

                        // 分析失败
                        if response.status == AnalysisStatus::Failed {
                            state.error_message.set(response.error_message);
                        }
                    }
                    Err(err) => state.error_message.set(Some(err)),
                }
            });
        }
    });

    view! {
        <section class="page page-analyzing">
            <div class="loading-container">
                <div class="loading-spinner"></div>
                <p class="loading-text">"AI正在分析成分..."</p>
                <p class="loading-hint">"请稍候，通常需要5-10秒"</p>
            </div>

            // 错误提示
            <Show when=move || state.error_message.get().is_some()>
                <div class="error-message">
                    {move || state.error_message.get().unwrap_or_default()}
                </div>
                <button class="btn-retry" on:click=/* 重试 */>
                    "重试"
                </button>
            </Show>
        </section>
    }
}
```

### IngredientCard 优化 (配料卡片重构)

```rust
#[component]
pub fn IngredientCard(
    name: String,
    category: String,
    function: String,
    risk_level: String,
    note: String,
) -> impl IntoView {
    // 过滤"未知"值的辅助函数
    let is_valid = |s: &str| !s.is_empty() && s != "未知" && s != "暂无";

    // 仅在有有效值时显示类别和功能标签
    let show_category = is_valid(&category);
    let show_function = is_valid(&function);
    let show_note = is_valid(&note);

    view! {
        <div class="ingredient-card-compact">
            // 卡片头部：名称 + 风险徽章
            <div class="card-header">
                <h3 class="ingredient-name">{name}</h3>
                <RiskBadge level={risk_level} />
            </div>

            // 标签行：类别和功能以标签形式水平排列
            <Show when=move || show_category || show_function>
                <div class="tags-row">
                    <Show when=move || show_category>
                        <span class="tag tag-category">{category.clone()}</span>
                    </Show>
                    <Show when=move || show_function>
                        <span class="tag tag-function">{function.clone()}</span>
                    </Show>
                </div>
            </Show>

            // 备注（仅在有内容时显示）
            <Show when=move || show_note>
                <p class="ingredient-note">{note.clone()}</p>
            </Show>
        </div>
    }
}
```

### CapturePage 优化 (首页精简)

```rust
#[component]
pub fn CapturePage() -> impl IntoView {
    // ... 保持现有逻辑 ...

    view! {
        <section class="page page-capture compact">
            // 品牌区域（精简30-40%）
            <div class="brand-section-compact">
                <div class="brand-icon-small">"🥗"</div>
                <h1 class="brand-name-small">"Smart Ingredients"</h1>
                <p class="brand-tagline-small">"AI智能配料表分析"</p>
            </div>

            // 功能卡片（一句话说明）
            <div class="feature-card-compact">
                <p>"拍照识别配料表，AI分析健康风险"</p>
            </div>

            // 步骤说明（默认折叠）
            <details class="collapsible-section">
                <summary class="section-toggle">"使用步骤 ▼"</summary>
                <div class="steps-content">
                    <div class="step-item-compact">
                        <span class="step-number">"1"</span>
                        <span>"拍摄配料表"</span>
                    </div>
                    <div class="step-item-compact">
                        <span class="step-number">"2"</span>
                        <span>"确认识别文本"</span>
                    </div>
                    <div class="step-item-compact">
                        <span class="step-number">"3"</span>
                        <span>"查看健康报告"</span>
                    </div>
                </div>
            </details>

            // 示例图片（默认折叠）
            <details class="collapsible-section">
                <summary class="section-toggle">"查看示例 ▼"</summary>
                <ExampleImages />
            </details>

            // 隐藏的文件输入
            <input
                node_ref=file_input_ref
                class="file-input-hidden"
                type="file"
                accept="image/*"
                on:change=on_file_change
            />

            // 主操作按钮（首屏可见）
            <Show when=move || preview_url.get().is_none()>
                <div class="main-action-compact">
                    <button class="btn-start-large" on:click=on_select_image>
                        <span class="icon">"📷"</span>
                        <span>"开始分析"</span>
                    </button>
                </div>
            </Show>

            // 图片预览
            <ImagePreview
                preview_url=preview_url.into()
                on_remove=on_remove_preview
            />

            // 确认上传按钮
            <Show when=move || preview_url.get().is_some()>
                <button class="btn-confirm" on:click=move |ev| on_upload.with_value(|f| f(ev))>
                    "确认上传"
                </button>
            </Show>
        </section>
    }
}
```

### ResultPage UI重构

```rust
#[component]
pub fn ResultPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");

    view! {
        <section class="page page-result">
            // 页面头部
            <header class="result-header">
                <h1>"分析完成"</h1>
            </header>

            // 健康评分卡片（重新设计）
            <HealthScoreCard
                score=move || state.analysis_result.get()
                    .and_then(|r| r.result)
                    .map(|r| r.health_score)
                recommendation=move || state.analysis_result.get()
                    .and_then(|r| r.result)
                    .map(|r| r.recommendation)
            />

            // 警告信息（如果有）
            <Show when=move || {
                state.analysis_result.get()
                    .and_then(|r| r.result)
                    .map(|r| !r.warnings.is_empty())
                    .unwrap_or(false)
            }>
                <WarningsSection
                    warnings=move || state.analysis_result.get()
                        .and_then(|r| r.result)
                        .map(|r| r.warnings)
                        .unwrap_or_default()
                />
            </Show>

            // 摘要卡片
            <SummaryCard
                summary=move || state.analysis_result.get()
                    .and_then(|r| r.result)
                    .map(|r| r.summary)
                    .unwrap_or_default()
            />

            // 配料详情
            <section class="ingredients-section">
                <h2>"配料详情"</h2>
                <IngredientCardList
                    items=move || /* ... */
                />
            </section>

            // 操作按钮
            <div class="action-buttons">
                <button class="btn-secondary" on:click=/* 重新分析 */>
                    "重新分析"
                </button>
                <a class="btn-primary" href="/">
                    "返回首页"
                </a>
            </div>
        </section>
    }
}
```

### CapturePage 优化

```rust
#[component]
pub fn CapturePage() -> impl IntoView {
    // ... 保持现有逻辑

    view! {
        <section class="page page-capture">
            // 品牌区域（精简）
            <div class="brand-section compact">
                <div class="brand-icon">"🥗"</div>
                <h1 class="brand-name">"Smart Ingredients"</h1>
                <p class="brand-tagline">"AI智能配料表分析"</p>
            </div>

            // 功能卡片（精简）
            <div class="feature-card compact">
                <p>"拍照识别配料表，AI分析健康风险"</p>
            </div>

            // 步骤说明（可折叠）
            <details class="steps-card">
                <summary>"使用步骤"</summary>
                <div class="steps-content">
                    // ... 步骤内容
                </div>
            </details>

            // 示例图片（可折叠）
            <details class="examples-section">
                <summary>"查看示例"</summary>
                <ExampleImages />
            </details>

            // 主操作按钮（保持在首屏）
            <div class="main-action">
                <button class="btn-start" on:click=on_select_image>
                    <span class="icon">"📷"</span>
                    <span>"开始分析"</span>
                </button>
            </div>

            // 图片预览（选择后显示）
            <ImagePreview /* ... */ />
        </section>
    }
}
```

## 后端实现方案

### AnalysisService 重构

```rust
impl AnalysisService {
    /// 创建分析记录并启动异步处理
    pub async fn create_and_process(
        &self,
        image_url: String,
    ) -> Result<Uuid> {
        // 1. 创建分析记录
        let analysis_id = self.create_analysis(&image_url).await?;

        // 2. 启动OCR后台任务
        let ocr_service = self.ocr_service.clone();
        let db = self.db.clone();
        let analysis_id_clone = analysis_id;

        tokio::spawn(async move {
            Self::process_ocr_task(
                analysis_id_clone,
                image_url,
                ocr_service,
                db,
            ).await
        });

        Ok(analysis_id)
    }

    /// OCR处理任务
    async fn process_ocr_task(
        analysis_id: Uuid,
        image_url: String,
        ocr_service: Arc<dyn OcrProvider>,
        db: PgPool,
    ) {
        // 更新状态为Processing
        let _ = Self::update_ocr_status(
            &db,
            analysis_id,
            OcrStatus::Processing,
        ).await;

        // 执行OCR识别
        match ocr_service.recognize(&image_url).await {
            Ok(text) => {
                // 保存OCR结果
                let _ = Self::save_ocr_result(
                    &db,
                    analysis_id,
                    &text,
                ).await;

                // 启动LLM分析任务
                Self::start_llm_task(
                    analysis_id,
                    text,
                    db.clone(),
                ).await;
            }
            Err(err) => {
                // 保存错误信息
                let _ = Self::update_ocr_status(
                    &db,
                    analysis_id,
                    OcrStatus::Failed,
                ).await;

                let _ = Self::save_error(
                    &db,
                    analysis_id,
                    &format!("OCR失败: {}", err),
                ).await;
            }
        }
    }

    /// LLM分析任务
    async fn start_llm_task(
        analysis_id: Uuid,
        ocr_text: String,
        db: PgPool,
    ) {
        tokio::spawn(async move {
            Self::process_llm_task(
                analysis_id,
                ocr_text,
                db,
            ).await
        });
    }

    /// LLM处理任务
    async fn process_llm_task(
        analysis_id: Uuid,
        text: String,
        db: PgPool,
    ) {
        // 更新状态为Processing
        let _ = Self::update_llm_status(
            &db,
            analysis_id,
            LlmStatus::Processing,
        ).await;

        // 执行LLM分析
        // ... 实现细节
    }

    /// 获取分析状态（包含OCR和LLM状态）
    pub async fn get_analysis_status(
        &self,
        id: Uuid,
    ) -> Result<AnalysisResponse> {
        let analysis = sqlx::query_as!(
            Analysis,
            r#"
            SELECT * FROM analyses WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(AnalysisResponse::from(analysis))
    }
}
```

### API Handler 调整

```rust
/// 上传图片处理器（简化）
pub async fn upload_handler(
    State(state): State<AppState>,
    multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
    // 1. 保存图片
    let image_url = handle_upload(multipart, &state.config).await?;

    // 2. 创建分析记录并启动异步处理
    let analysis_id = state.analysis_service
        .create_and_process(image_url.clone())
        .await?;

    // 3. 立即返回
    Ok(Json(UploadResponse {
        id: analysis_id,
        image_url,
    }))
}

/// 获取分析状态处理器（扩展）
pub async fn get_analysis_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AnalysisResponse>, AppError> {
    let response = state.analysis_service
        .get_analysis_status(id)
        .await?;

    Ok(Json(response))
}
```

## CSS 样式优化

### 设计原则

1. **移动优先**：优先考虑移动端体验
2. **卡片化**：使用卡片布局，清晰分隔信息
3. **颜色语义**：使用颜色传达信息（成功、警告、错误）
4. **间距一致**：统一的间距系统
5. **动画流畅**：适度使用动画提升体验

### CSS Variables

```css
:root {
    /* 颜色系统 */
    --color-primary: #4CAF50;
    --color-secondary: #2196F3;
    --color-success: #4CAF50;
    --color-warning: #FF9800;
    --color-danger: #F44336;
    --color-info: #2196F3;

    /* 背景色 */
    --bg-primary: #FFFFFF;
    --bg-secondary: #F5F5F5;
    --bg-card: #FFFFFF;

    /* 文字颜色 */
    --text-primary: #212121;
    --text-secondary: #757575;
    --text-hint: #9E9E9E;

    /* 间距系统 */
    --spacing-xs: 4px;
    --spacing-sm: 8px;
    --spacing-md: 16px;
    --spacing-lg: 24px;
    --spacing-xl: 32px;

    /* 圆角 */
    --radius-sm: 4px;
    --radius-md: 8px;
    --radius-lg: 12px;
    --radius-xl: 16px;

    /* 阴影 */
    --shadow-sm: 0 2px 4px rgba(0,0,0,0.1);
    --shadow-md: 0 4px 8px rgba(0,0,0,0.12);
    --shadow-lg: 0 8px 16px rgba(0,0,0,0.15);
}
```

### 关键样式

```css
/* ========== 首页精简样式 ========== */
.page-capture.compact {
    padding: var(--spacing-md);
    max-width: 100%;
}

/* 品牌区域（精简30-40%） */
.brand-section-compact {
    text-align: center;
    padding: var(--spacing-md) 0;
    margin-bottom: var(--spacing-sm);
}

.brand-icon-small {
    font-size: 48px; /* 原来可能是 64px-80px */
    margin-bottom: var(--spacing-xs);
}

.brand-name-small {
    font-size: 20px; /* 精简后 */
    font-weight: 600;
    margin: var(--spacing-xs) 0;
}

.brand-tagline-small {
    font-size: 13px;
    color: var(--text-secondary);
}

/* 功能卡片（一句话） */
.feature-card-compact {
    background: var(--bg-card);
    padding: var(--spacing-sm);
    border-radius: var(--radius-md);
    margin-bottom: var(--spacing-sm);
    text-align: center;
    font-size: 14px;
}

/* 可折叠区域 */
.collapsible-section {
    margin-bottom: var(--spacing-sm);
    border: 1px solid #E0E0E0;
    border-radius: var(--radius-md);
    overflow: hidden;
}

.section-toggle {
    padding: var(--spacing-sm) var(--spacing-md);
    background: var(--bg-secondary);
    cursor: pointer;
    font-weight: 500;
    user-select: none;
}

.section-toggle:hover {
    background: #E8E8E8;
}

.steps-content {
    padding: var(--spacing-sm);
}

.step-item-compact {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-xs) 0;
}

.step-number {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    background: var(--color-primary);
    color: white;
    border-radius: 50%;
    font-size: 12px;
    font-weight: 600;
}

/* ========== 配料卡片优化样式 ========== */
.ingredient-card-compact {
    background: var(--bg-card);
    border-radius: var(--radius-md);
    padding: var(--spacing-sm) var(--spacing-md);
    margin-bottom: var(--spacing-xs); /* 减少间距 */
    box-shadow: var(--shadow-sm);
    transition: transform 0.2s, box-shadow 0.2s;
}

.ingredient-card-compact:active {
    transform: translateY(-2px);
    box-shadow: var(--shadow-md);
}

/* 卡片头部：名称 + 徽章 */
.card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--spacing-xs);
}

.ingredient-name {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
}

/* 标签行（水平排列） */
.tags-row {
    display: flex;
    flex-wrap: wrap;
    gap: var(--spacing-xs);
    margin-bottom: var(--spacing-xs);
}

.tag {
    display: inline-block;
    padding: 2px 8px;
    border-radius: var(--radius-sm);
    font-size: 11px;
    font-weight: 500;
}

.tag-category {
    background: #E3F2FD;
    color: #1976D2;
}

.tag-function {
    background: #F3E5F5;
    color: #7B1FA2;
}

/* 备注（精简样式） */
.ingredient-note {
    font-size: 12px;
    color: var(--text-secondary);
    margin: 0;
    line-height: 1.4;
}

/* 风险等级徽章 */
.risk-badge {
    display: inline-block;
    padding: 4px 10px;
    border-radius: var(--radius-sm);
    font-size: 11px;
    font-weight: 600;
    white-space: nowrap;
}

.risk-badge.low, .risk-badge.低 {
    background: #E8F5E9;
    color: #2E7D32;
}

.risk-badge.medium, .risk-badge.中 {
    background: #FFF3E0;
    color: #E65100;
}

.risk-badge.high, .risk-badge.高 {
    background: #FFEBEE;
    color: #C62828;
}

/* ========== 文本确认页面样式 ========== */
.page-confirm {
    padding: var(--spacing-md);
}

.text-editor-container {
    margin: var(--spacing-lg) 0;
}

.text-editor {
    width: 100%;
    padding: var(--spacing-md);
    border: 2px solid #E0E0E0;
    border-radius: var(--radius-md);
    font-size: 14px;
    font-family: inherit;
    line-height: 1.6;
    resize: vertical;
    min-height: 200px;
}

.text-editor:focus {
    outline: none;
    border-color: var(--color-primary);
}

.edit-tips {
    font-size: 12px;
    color: var(--text-secondary);
    margin-top: var(--spacing-sm);
}

/* ========== 加载页面样式 ========== */
.loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 50vh;
    text-align: center;
}

.loading-spinner {
    width: 48px;
    height: 48px;
    border: 4px solid #E0E0E0;
    border-top-color: var(--color-primary);
    border-radius: 50%;
    animation: spin 1s linear infinite;
}

@keyframes spin {
    to { transform: rotate(360deg); }
}

.loading-text {
    font-size: 16px;
    font-weight: 500;
    margin-top: var(--spacing-md);
}

.loading-hint {
    font-size: 13px;
    color: var(--text-secondary);
    margin-top: var(--spacing-xs);
}

/* ========== 健康评分卡片 ========== */
.health-score-card {
    background: linear-gradient(135deg, var(--color-primary), var(--color-secondary));
    color: white;
    padding: var(--spacing-lg);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-md);
    text-align: center;
    margin-bottom: var(--spacing-md);
}

.health-score {
    font-size: 48px;
    font-weight: bold;
    margin: var(--spacing-md) 0;
}
```

## 性能优化

### 前端优化

1. **组件懒加载**：非关键组件延迟加载
2. **图片优化**：压缩上传图片，使用WebP格式
3. **状态优化**：避免不必要的重渲染
4. **动画优化**：使用CSS transform和opacity

### 后端优化

1. **异步处理**：OCR和LLM并行执行
2. **连接池**：复用HTTP连接
3. **超时控制**：设置合理的超时时间
4. **错误重试**：失败自动重试机制

### 数据库优化

1. **索引优化**：为状态字段创建索引
2. **查询优化**：减少不必要的字段查询
3. **连接池**：使用数据库连接池

## 错误处理

### 错误码

| 错误码 | 消息 | 描述 |
| ------ | ---- | ---- |
| OCR_001 | OCR识别失败 | OCR服务返回错误 |
| OCR_002 | OCR超时 | OCR处理超过30秒 |
| OCR_003 | 图片不清晰 | OCR无法识别文字 |
| LLM_001 | LLM分析失败 | LLM服务返回错误 |
| LLM_002 | LLM超时 | LLM处理超过60秒 |
| LLM_003 | 解析失败 | 无法解析LLM返回结果 |

### 错误响应格式

```json
{
  "error": {
    "code": "OCR_001",
    "message": "OCR识别失败",
    "details": "图片质量过低，请重新拍摄",
    "retryable": true
  }
}
```

## 测试策略

### 单元测试

- **前端组件测试**：测试各UI组件的渲染和交互
- **状态管理测试**：测试状态更新逻辑
- **服务层测试**：测试异步任务逻辑
- **数据库操作测试**：测试状态更新和查询

### 集成测试

- **API端到端测试**：测试完整的上传→OCR→LLM流程
- **状态转换测试**：测试各种状态转换场景
- **错误处理测试**：测试各种失败场景

### E2E 测试

- **完整流程测试**：从上传到查看结果的完整流程
- **并发测试**：测试多用户同时使用
- **性能测试**：测试响应时间和吞吐量

## 部署

### 环境要求

- Rust 1.80+
- PostgreSQL 16+
- Redis 7+ (可选，用于缓存)
- Node.js 18+ (前端构建)

### 配置

```bash
# 新增环境变量
OCR_ASYNC=true                    # 启用异步OCR
LLM_ASYNC=true                    # 启用异步LLM
ANALYSIS_POLL_INTERVAL=2          # 轮询间隔（秒）
OCR_TIMEOUT=30                    # OCR超时（秒）
LLM_TIMEOUT=60                    # LLM超时（秒）
```

### 数据库迁移

```bash
# 运行迁移脚本
sqlx migrate run

# 迁移文件: migrations/YYYYMMDDHHMMSS_add_ocr_llm_status.sql
```

### 回滚计划

1. **数据库回滚**：保留旧字段，新字段可为NULL
2. **API兼容**：保持旧API接口不变
3. **功能开关**：通过配置控制新旧流程

## 实施阶段

### 阶段 1：后端API重构 (2-3天)

- [ ] 扩展数据库Schema
  - 添加 `confirmed_text` 字段
  - 扩展 `AnalysisStatus` 枚举
  - 创建数据库迁移脚本
- [ ] 实现新API接口
  - `POST /api/v1/analysis/:id/confirm` (确认文本并启动LLM)
  - 修改 `POST /api/v1/analysis/upload` (仅启动OCR)
  - 修改 `GET /api/v1/analysis/:id` (返回扩展字段)
- [ ] 重构 AnalysisService
  - 分离 OCR 和 LLM 触发逻辑
  - 实现两阶段处理流程
- [ ] 编写单元测试

### 阶段 2：前端页面和组件开发 (4-5天)

- [ ] 创建 ConfirmPage (文本确认页面)
  - 文本编辑器组件
  - 操作按钮组件
  - 页面路由配置
- [ ] 优化 CapturePage (首页精简)
  - 精简品牌区域
  - 实现可折叠区域
  - 调整布局和间距
- [ ] 重构 IngredientCard (配料卡片)
  - 实现标签行布局
  - 添加字段过滤逻辑
  - 优化样式和间距
- [ ] 调整页面路由和导航
  - 添加 `/confirm` 路由
  - 调整页面跳转逻辑
- [ ] 扩展 AppState (状态管理)
- [ ] 编写组件测试

### 阶段 3：CSS样式优化 (2天)

- [ ] 实现首页精简样式
  - 品牌区域样式
  - 可折叠区域样式
  - 按钮样式优化
- [ ] 实现配料卡片优化样式
  - 紧凑布局样式
  - 标签样式
  - 风险徽章样式
- [ ] 实现文本确认页面样式
  - 文本编辑器样式
  - 提示文字样式
- [ ] 实现加载页面样式
  - 加载动画
  - 状态文字
- [ ] 响应式适配测试

### 阶段 4：集成测试和优化 (2-3天)

- [ ] 前后端集成测试
  - 完整流程测试（上传→OCR→确认→LLM→结果）
  - 错误场景测试
  - 边界情况测试
- [ ] 性能测试
  - OCR响应时间测试
  - LLM响应时间测试
  - 页面加载性能测试
- [ ] UI/UX测试
  - 不同屏幕尺寸测试
  - 交互流畅度测试
  - 用户体验测试
- [ ] Bug修复和优化

### 阶段 5：部署和监控 (1-2天)

- [ ] 部署到测试环境
- [ ] 用户验收测试
- [ ] 收集反馈并调整
- [ ] 部署到生产环境
- [ ] 监控和告警配置

**预计总工期**: 11-15天

## 风险与缓解

| 风险 | 影响 | 可能性 | 缓解措施 |
| ---- | ---- | ------ | -------- |
| 异步处理增加系统复杂度 | 高 | 中 | 充分测试，添加日志和监控 |
| 数据库迁移失败 | 高 | 低 | 提前在测试环境验证，准备回滚方案 |
| 前端轮询增加服务器负载 | 中 | 中 | 使用合理的轮询间隔，考虑WebSocket |
| UI改动用户不适应 | 中 | 低 | 保持核心交互不变，渐进式改进 |
| OCR和LLM服务不稳定 | 高 | 中 | 添加重试机制，提供降级方案 |

## 待解决问题

| 问题 | 影响 | 负责人 | 状态 |
| ---- | ---- | ------ | ---- |
| 是否使用WebSocket替代轮询 | 中 | 架构师 | 开放 |
| OCR结果是否需要用户确认 | 低 | 产品经理 | 开放 |
| 是否支持离线模式 | 低 | 技术负责人 | 开放 |
| 性能监控指标的定义 | 中 | DevOps | 开放 |

## 参考资料

- [Leptos异步处理文档](https://leptos.dev/async/)
- [Tokio异步编程指南](https://tokio.rs/tokio/tutorial)
- [PostgreSQL事务处理](https://www.postgresql.org/docs/current/tutorial-transactions.html)
- [Material Design规范](https://material.io/design)

---

## 变更记录

| 版本 | 日期 | 作者 | 描述 |
| ---- | ---- | ---- | ---- |
| 1.0 | 2026-01-17 | Smart Ingredients Team | 初始版本 |
