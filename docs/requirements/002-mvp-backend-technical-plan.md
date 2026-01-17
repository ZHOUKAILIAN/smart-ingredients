# 002-MVP 后端技术方案

## 元数据

| 字段     | 值                       |
| -------- | ------------------------ |
| 文档编号 | 002-002                  |
| 标题     | MVP 后端技术方案 |
| 版本     | 1.0                      |
| 状态     | 草稿   |
| 创建日期 | 2026-01-17               |
| 更新日期 | 2026-01-17               |
| 作者     | Claude               |
| 关联需求 | 002-002 (MVP 后端需求文档)                  |

## 概述

### 目的

设计并实现一个简化的 MVP 后端服务，支持图片上传、OCR 文本提取和 DeepSeek LLM 分析功能。

### 范围

本设计涵盖：
- Axum Web 服务器配置
- 图片上传处理
- OCR 文本提取（Tesseract）
- PostgreSQL 数据库集成
- LLM Provider 抽象（可切换 DeepSeek / 其他厂商）
- RESTful API 设计
- 错误处理机制

### 假设

- 默认使用 DeepSeek，后续可切换其他 LLM 提供方
- PostgreSQL 数据库已安装并运行
- 后端负责 OCR，前端仅提供图片
- OCR 结果需要人工核对，但 MVP 阶段不实现校对流程
- MVP 阶段不需要用户认证

## 架构设计

### 高层架构

```text
┌─────────────────────────────────────────────────────┐
│              Frontend (Leptos + Tauri)           │
│                                                  │
└────────────────────┬─────────────────────────────┘
                     │ HTTP/REST
                     ▼
┌─────────────────────────────────────────────────────┐
│            Backend (Axum + Tokio)                │
│                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌─────────┐  │
│  │   Router     │  │   Handlers   │  │  State  │  │
│  │  (Routes)    │→│  (Business)  │→│ (Shared)│  │
│  └──────────────┘  └──────────────┘  └─────────┘  │
│         │                  │                │      │
└─────────┼──────────────────┼────────────────┼──────┘
          │                  │                │
          ▼                  ▼                ▼
  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐
  │  File System │  │  OCR (Tess.) │  │ PostgreSQL   │
  │  (uploads/)  │  └──────────────┘  │   Database   │
  └──────────────┘          │         └──────────────┘
                             ▼
                      ┌──────────────┐
                      │  LLM Provider│
                      │ (DeepSeek/..)│
                      └──────────────┘
```

### 组件图

```text
backend/
├── src/
│   ├── main.rs              # 入口，服务器启动
│   ├── config.rs            # 配置加载（环境变量）
│   ├── state.rs             # 应用状态（DB 连接池等）
│   ├── error.rs             # 错误类型定义
│   ├── routes/
│   │   ├── mod.rs           # 路由聚合
│   │   ├── health.rs        # 健康检查
│   │   └── analysis.rs      # 分析相关路由
│   ├── handlers/
│   │   ├── mod.rs           # 处理器聚合
│   │   ├── upload.rs        # 图片上传处理
│   │   ├── analyze.rs       # OCR + 分析处理
│   │   └── query.rs         # 结果查询处理
│   ├── services/
│   │   ├── mod.rs           # 服务聚合
│   │   ├── ocr.rs           # OCR 服务
│   │   ├── llm.rs           # LLM Provider 抽象与选择
│   │   ├── llm_deepseek.rs  # DeepSeek Provider 实现
│   │   └── storage.rs       # 文件存储逻辑
│   └── models/
│       ├── mod.rs           # 模型聚合
│       ├── analysis.rs      # 分析记录模型
│       └── response.rs      # API 响应模型
└── migrations/
    └── 001_create_analyses.sql  # 数据库迁移
```

### 技术栈

| 组件   | 技术   | 版本 | 选择理由   |
| ------ | ------ | ---- | ---------- |
| Web 框架 | Axum | 0.7.x | 类型安全、性能高、生态好 |
| 异步运行时 | Tokio | 1.x | Rust 异步标准 |
| 数据库 | PostgreSQL | 16.x | 成熟稳定、JSONB 支持 |
| 数据库驱动 | SQLx | 0.7.x | 编译时查询检查 |
| HTTP 客户端 | reqwest | 0.12.x | 异步、易用 |
| 序列化 | serde | 1.x | Rust 标准序列化库 |
| OCR | Tesseract | 5.x | 本地开源、可离线 |
| 日志 | tracing | 0.1.x | 结构化日志 |
| 错误处理 | anyhow | 1.x | 灵活的错误处理 |

## 数据模型

### 实体

#### Analysis（分析记录）

| 字段 | 类型 | 说明 | 约束 |
|------|------|------|------|
| id | UUID | 主键 | PRIMARY KEY, DEFAULT gen_random_uuid() |
| image_url | VARCHAR(512) | 图片 URL | NOT NULL |
| text | TEXT | 配料表文本 | NULLABLE |
| health_score | INTEGER | 健康评分 (0-100) | NULLABLE, CHECK (0-100) |
| result | JSONB | LLM 分析结果 | NULLABLE |
| status | VARCHAR(50) | 状态 | NOT NULL, DEFAULT 'pending' |
| error_message | TEXT | 错误信息 | NULLABLE |
| created_at | TIMESTAMP | 创建时间 | DEFAULT NOW() |
| updated_at | TIMESTAMP | 更新时间 | DEFAULT NOW() |

### 模式

```sql
CREATE TABLE analyses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    image_url VARCHAR(512) NOT NULL,
    text TEXT,
    health_score INTEGER CHECK (health_score >= 0 AND health_score <= 100),
    result JSONB,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    error_message TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_analyses_created_at ON analyses(created_at DESC);
CREATE INDEX idx_analyses_status ON analyses(status);
```

### 数据流

```text
1. 图片上传流程:
   Frontend → POST /api/v1/analysis/upload
   → Save file to uploads/
   → Insert record to DB (status: pending)
   → Return { id, image_url }

2. OCR + 分析流程:
   Frontend → POST /api/v1/analysis/{id}/analyze
   → Load image from uploads/
   → OCR (Tesseract) extract text
   → Update record with text
   → Call DeepSeek API
   → Parse LLM response
   → Update record with result (status: completed)
   → Return { id, status, result }

3. 查询结果流程:
   Frontend → GET /api/v1/analysis/{id}
   → Query DB by id
   → Return full analysis record
```

## API 设计

### 接口列表

| 方法 | 路径 | 描述 | 请求 | 响应 |
| ---- | ---- | ---- | ---- | ---- |
| POST | `/api/v1/analysis/upload` | 上传图片 | multipart/form-data | AnalysisCreated |
| POST | `/api/v1/analysis/{id}/analyze` | OCR + 分析 | - | AnalysisResult |
| GET | `/api/v1/analysis/{id}` | 查询分析结果 | - | AnalysisDetail |
| GET | `/health` | 健康检查 | - | HealthResponse |

### 数据结构

#### AnalysisCreated (响应)
```rust
#[derive(Serialize)]
pub struct AnalysisCreated {
    pub id: Uuid,
    pub image_url: String,
}
```

#### AnalysisResult (响应)
```rust
#[derive(Serialize)]
pub struct AnalysisResult {
    pub id: Uuid,
    pub status: String,
    pub health_score: Option<i32>,
    pub ingredients: Option<Vec<Ingredient>>,
    pub warnings: Option<Vec<Warning>>,
    pub recommendation: Option<String>,
}
```

#### Ingredient (配料项)
```rust
#[derive(Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub category: String,      // "additive" | "allergen" | "nutrition"
    pub risk_level: String,    // "low" | "medium" | "high"
    pub description: String,
}
```

#### Warning (警告项)
```rust
#[derive(Serialize, Deserialize)]
pub struct Warning {
    pub warning_type: String,
    pub ingredients: Vec<String>,
    pub message: String,
}
```

#### AnalysisDetail (响应)
```rust
#[derive(Serialize)]
pub struct AnalysisDetail {
    pub id: Uuid,
    pub image_url: String,
    pub text: Option<String>,
    pub result: Option<AnalysisResult>,
    pub created_at: DateTime<Utc>,
}
```

#### HealthResponse (响应)
```rust
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub database: String,
    pub version: String,
}
```

## OCR 设计

### OCR 配置

```rust
// config.rs
pub struct OcrConfig {
    pub lang: String, // e.g. "chi_sim+eng"
    pub timeout: Duration,
}
```

### OCR 调用流程

```rust
// services/ocr.rs
pub async fn extract_text(image_path: &Path, config: &OcrConfig) -> Result<String, OcrError> {
    // 1. 调用本地 tesseract 进程
    // 2. 读取 stdout 作为识别结果
    // 3. 进行基本清洗与长度校验
    // 4. 返回文本
}
```

## LLM Provider 设计

### Provider 配置

```rust
// config.rs
pub enum LlmProvider {
    DeepSeek,
    // 预留：OpenAI, Anthropic 等
}

pub struct LlmConfig {
    pub provider: LlmProvider,
    pub api_key: String,
    pub api_url: String,  // DeepSeek: https://api.deepseek.com/v1/chat/completions
    pub model: String,    // DeepSeek: deepseek-chat
    pub timeout: Duration, // 30 seconds
}
```

### Prompt 设计

```rust
pub fn build_analysis_prompt(text: &str) -> String {
    format!(
        r#"你是一个专业的食品配料分析专家。请分析以下配料表，并返回 JSON 格式的健康评估。

配料表：
{}

请严格按照以下 JSON 格式返回：
{{
  "health_score": <0-100 的整数>,
  "ingredients": [
    {{
      "name": "<配料名称>",
      "category": "<additive|allergen|nutrition>",
      "risk_level": "<low|medium|high>",
      "description": "<简短说明>"
    }}
  ],
  "warnings": [
    {{
      "warning_type": "<警告类型>",
      "ingredients": ["<配料1>", "<配料2>"],
      "message": "<警告信息>"
    }}
  ],
  "recommendation": "<总体建议>"
}}

要求：
1. health_score 基于配料的整体健康程度评分
2. 识别所有添加剂、过敏原和关键营养成分
3. 对高风险配料给出明确警告
4. recommendation 提供实用的食用建议"#,
        text
    )
}
```

### Provider 调用流程

```rust
// services/llm.rs
#[async_trait]
pub trait LlmProviderClient {
    async fn analyze_ingredients(&self, text: &str) -> Result<AnalysisResult, LlmError>;
}

pub fn build_llm_client(config: &LlmConfig, http: reqwest::Client) -> Box<dyn LlmProviderClient> {
    match config.provider {
        LlmProvider::DeepSeek => Box::new(DeepSeekClient::new(config, http)),
        // 预留其他 provider
    }
}

// services/llm_deepseek.rs (示意)
pub struct DeepSeekClient { /* ... */ }
impl LlmProviderClient for DeepSeekClient { /* 调用 DeepSeek API */ }
```

### Provider 切换策略

- 默认 DeepSeek，后续新增 Provider 只需新增实现并在配置中切换
- 可通过环境变量指定 provider（如 `LLM_PROVIDER=deepseek`）

## 错误处理

### 错误码

| 错误码 | HTTP 状态 | 消息 | 描述 |
|--------|----------|------|------|
| INVALID_FILE_TYPE | 415 | Unsupported file type | 文件类型不支持 |
| FILE_TOO_LARGE | 413 | File size exceeds 10MB | 文件过大 |
| ANALYSIS_NOT_FOUND | 404 | Analysis record not found | 记录不存在 |
| INVALID_TEXT | 400 | Text length must be 1-5000 | 文本长度不合法 |
| OCR_FAILED | 500 | OCR failed | OCR 识别失败 |
| LLM_API_ERROR | 500 | LLM API call failed | LLM 调用失败 |
| DATABASE_ERROR | 500 | Database operation failed | 数据库错误 |

### 错误响应格式

```rust
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Serialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}
```

### 错误处理实现

```rust
// error.rs
#[derive(Debug)]
pub enum AppError {
    InvalidFileType,
    FileTooLarge,
    AnalysisNotFound,
    InvalidText(String),
    OcrFailed(String),
    LlmApiError(String),
    DatabaseError(sqlx::Error),
    InternalError(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::InvalidFileType => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "INVALID_FILE_TYPE",
                "Only JPEG, PNG, and WebP are supported",
            ),
            AppError::FileTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                "FILE_TOO_LARGE",
                "File size exceeds 10MB",
            ),
            // ... 其他错误类型
        };

        let body = Json(ErrorResponse {
            error: ErrorDetail {
                code: code.to_string(),
                message: message.to_string(),
                details: None,
            },
        });

        (status, body).into_response()
    }
}
```

## 性能考虑

### 文件上传优化

- 使用流式上传，避免一次性加载到内存
- 设置合理的 body size limit (10MB)
- 异步写入文件系统

### OCR 优化

- 使用本地进程池或限制并发，避免 OCR 占用过高
- 对大图做尺寸压缩或灰度化（可选）

### 数据库优化

- 使用连接池（max_connections: 20）
- 对常用查询字段建立索引
- 使用 JSONB 存储灵活数据

### LLM API 优化

- 设置合理的超时时间（30 秒）
- 使用连接复用（keep-alive）
- 错误重试机制（最多 3 次）

## 测试策略

### 单元测试

- 测试 Prompt 构建函数
- 测试错误类型转换
- 测试数据模型序列化

### 集成测试

- 测试图片上传流程
- 测试 OCR + 分析流程
- 测试数据库读写
- 模拟 LLM API 响应

### E2E 测试

- 完整的分析流程测试
- 错误场景测试（文件过大、API 失败等）

## 部署

### 环境要求

```bash
# 必需
PostgreSQL 16.x
Rust 1.75+
Tesseract OCR

# 可选
Docker (用于容器化部署)
```

### 配置

```bash
# .env
DATABASE_URL=postgresql://user:password@localhost:5432/smart_ingredients
LLM_PROVIDER=deepseek
DEEPSEEK_API_KEY=sk-xxxxxxxxxxxxx
DEEPSEEK_API_URL=https://api.deepseek.com/v1/chat/completions
DEEPSEEK_MODEL=deepseek-chat
OCR_LANG=chi_sim+eng
UPLOAD_DIR=./uploads
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
```

### 启动命令

```bash
# 开发环境
cargo run

# 生产环境
cargo build --release
./target/release/backend
```

### 数据库迁移

```bash
# 安装 sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# 运行迁移
sqlx migrate run
```

## 实施阶段

### 阶段 1：项目初始化
- [ ] 创建 backend 项目结构
- [ ] 添加依赖项到 Cargo.toml
- [ ] 配置环境变量加载
- [ ] 创建数据库迁移文件

### 阶段 2：核心功能实现
- [ ] 实现应用状态和配置
- [ ] 实现错误处理机制
- [ ] 实现健康检查端点
- [ ] 实现图片上传处理
- [ ] 实现 OCR 服务（Tesseract）
- [ ] 实现 DeepSeek API 集成
- [ ] 实现 OCR + 分析接口
- [ ] 实现结果查询

### 阶段 3：测试和优化
- [ ] 编写单元测试
- [ ] 编写集成测试
- [ ] 性能测试和优化
- [ ] 错误场景测试

### 阶段 4：文档和部署
- [ ] 编写 API 文档
- [ ] 创建 Docker 配置
- [ ] 部署到开发环境
- [ ] 前后端联调

## 风险与缓解

| 风险 | 影响 | 可能性 | 缓解措施 |
|------|------|--------|----------|
| DeepSeek API 不稳定 | 高 | 中 | 添加重试机制、错误日志、备用方案 |
| LLM 返回非 JSON 格式 | 高 | 中 | 严格的 Prompt 设计、JSON 解析容错 |
| OCR 识别准确率不足 | 中 | 中 | 图片预处理、提示前端拍摄规范、可选手工纠正 |
| 数据库连接失败 | 高 | 低 | 连接池管理、健康检查、自动重连 |
| 文件上传占用磁盘 | 中 | 中 | 定期清理、磁盘监控、文件大小限制 |

## 待解决问题

| 问题 | 影响 | 负责人 | 状态 |
|------|------|--------|------|
| DeepSeek API Rate Limit | 中 | 开发者 | 需确认配额 |
| 上传文件清理策略 | 低 | 开发者 | 待设计 |
| 日志存储位置 | 低 | 开发者 | 待确认 |

## 参考资料

- [Axum 官方文档](https://docs.rs/axum/)
- [SQLx 使用指南](https://github.com/launchbadge/sqlx)
- [DeepSeek API 文档](https://platform.deepseek.com/api-docs/)
- [Tesseract OCR](https://github.com/tesseract-ocr/tesseract)
- [Tokio 异步运行时](https://tokio.rs/)

---

## 变更记录

| 版本 | 日期 | 作者 | 描述 |
|------|------|------|------|
| 1.0 | 2026-01-17 | Claude | 初始版本 - MVP 后端技术方案 |
