# 错误处理规范 (Error Handling Standards)

## 核心原则

1. **类型安全**: 使用 Rust 类型系统确保错误处理的正确性
2. **可追踪性**: 所有错误都应记录日志，关键错误包含 request_id
3. **用户友好**: 前端展示清晰、可操作的错误信息
4. **一致性**: 统一的错误码、响应格式和处理流程
5. **安全第一**: 不泄露敏感信息（数据库结构、内部路径）

---

## 强制规则

### Backend 规则

#### 必须遵守 (MUST)

- [ ] Handler 返回类型：`Result<impl IntoResponse, AppError>`
- [ ] Service 层使用 `anyhow::Result<T>`
- [ ] 所有错误必须记录日志（使用 `tracing::error!`）
- [ ] 5xx 错误必须包含完整错误堆栈
- [ ] 外部错误转换必须隐藏内部细节
- [ ] 错误消息必须使用中文（面向中文用户）
- [ ] 使用 `?` 运算符传播错误
- [ ] 外部服务错误使用 `map_err` 转换为业务错误

#### 禁止事项 (MUST NOT)

- [ ] 禁止在生产代码中使用 `unwrap()` 或 `expect()`
- [ ] 禁止暴露数据库错误细节给用户
- [ ] 禁止忽略错误（使用 `let _ = ...`）
- [ ] 禁止使用英文错误消息
- [ ] 禁止在 Handler 中使用 `panic!`
- [ ] 禁止在响应中包含内部路径或堆栈信息

### Frontend 规则

#### 必须遵守 (MUST)

- [ ] API 调用必须返回 `Result<T, ErrorInfo>`
- [ ] 错误状态必须展示给用户
- [ ] 必须提供错误恢复机制（返回首页/重试按钮）
- [ ] 解析 API 错误响应时处理格式异常

#### 禁止事项 (MUST NOT)

- [ ] 禁止忽略 API 调用错误
- [ ] 禁止展示原始错误对象给用户

### Shared Types 规则

#### 必须遵守 (MUST)

- [ ] 所有错误类型必须实现 `Serialize` + `Deserialize`
- [ ] 错误码使用 `UPPER_SNAKE_CASE`
- [ ] 保持 Frontend 和 Backend 类型同步

---

## 快速决策指南

### 何时使用哪种错误类型？

| 场景 | Backend 错误类型 | HTTP 状态码 | 错误码常量 |
|------|-----------------|------------|----------|
| 参数验证失败、格式错误 | `AppError::BadRequest` | 400 | `BAD_REQUEST` |
| 缺少认证信息、Token 过期 | `AppError::Unauthorized` | 401 | `UNAUTHORIZED` |
| 已认证但无权限 | `AppError::Forbidden` | 403 | `FORBIDDEN` |
| 资源不存在 | `AppError::NotFound` | 404 | `NOT_FOUND` |
| 资源已存在、状态冲突 | `AppError::Conflict` | 409 | `CONFLICT` |
| 上传文件超过限制 | `AppError::PayloadTooLarge` | 413 | `PAYLOAD_TOO_LARGE` |
| 不支持的文件格式 | `AppError::UnsupportedMediaType` | 415 | `UNSUPPORTED_MEDIA_TYPE` |
| 数据库错误、未预期异常 | `AppError::Internal` | 500 | `INTERNAL_ERROR` |
| OCR 服务失败 | `AppError::Ocr` | 503 | `OCR_ERROR` |
| LLM 服务失败 | `AppError::Llm` | 503 | `LLM_ERROR` |
| 文件存储失败 | `AppError::Storage` | 500 | `STORAGE_ERROR` |

### 外部错误转换规则

| 外部错误类型 | 转换为 | 用户友好消息 |
|------------|-------|------------|
| `sqlx::Error::RowNotFound` | `AppError::NotFound` | "资源不存在" |
| `sqlx::Error::*` (其他) | `AppError::Internal` | "数据库错误，请稍后重试" |
| `redis::RedisError` | `AppError::Internal` | "缓存服务错误，请稍后重试" |
| `std::io::Error` | `AppError::Storage` | "文件操作失败，请稍后重试" |
| `anyhow::Error` | `AppError::Internal` | "服务器内部错误，请稍后重试" |

---

## 错误消息模板

### 客户端错误（4xx）

**格式**: `"{字段/操作}{问题描述}[，{建议操作}]"`

```
"缺少文件字段"
"文件大小超过 10MB 限制"
"不支持的图片格式，仅支持 JPG、PNG、WebP"
"{字段}不能为空"
"{字段}长度必须在 {min}-{max} 字符之间"
"分析记录不存在"
```

### 服务器错误（5xx）

**格式**: `"{服务名}错误/失败，请稍后重试"`

```
"服务器内部错误，请稍后重试"
"数据库错误，请稍后重试"
"图片识别失败，请尝试上传更清晰的图片"
"AI 分析服务暂时不可用，请稍后重试"
"文件存储失败，请稍后重试"
```

---

## 类型定义规范

### Backend 错误类型

**位置**: `backend/src/errors.rs`

**结构**:
```rust
#[derive(Error, Debug)]
pub enum AppError {
    // 客户端错误 (4xx)
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Not found: {0}")]
    NotFound(String),

    // 服务器错误 (5xx)
    #[error("Internal error: {0}")]
    Internal(String),

    // 业务错误 (5xx)
    #[error("OCR error: {0}")]
    Ocr(String),
}
```

**要求**:
- 使用 `thiserror::Error` 派生宏
- 每个变体包含一个 `String` 消息（面向用户）
- 使用 `#[error("...")]` 定义 Display 格式
- 按错误类型分组

### Frontend 错误类型

**位置**: `frontend/src/stores/mod.rs` 或 `frontend/src/error.rs`

**结构**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub title: String,
    pub message: String,
    pub recoverable: bool,
}
```

### Shared 错误类型

**位置**: `shared/src/error.rs`

**API 错误响应格式**:
```json
{
  "code": "ERROR_CODE",
  "message": "Human-readable error message",
  "details": { "field": "additional context (optional)" },
  "request_id": "uuid-for-tracing (optional)"
}
```

**Rust 结构**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub request_id: Option<String>,
}
```

**错误码常量** (`shared/src/error.rs::error_codes`):
```rust
pub const BAD_REQUEST: &str = "BAD_REQUEST";
pub const UNAUTHORIZED: &str = "UNAUTHORIZED";
pub const NOT_FOUND: &str = "NOT_FOUND";
pub const INTERNAL_ERROR: &str = "INTERNAL_ERROR";
pub const OCR_ERROR: &str = "OCR_ERROR";
pub const LLM_ERROR: &str = "LLM_ERROR";
// ... 更多常量
```

---

## 实现模式

### Backend Handler 模式

```rust
pub async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<Request>,
) -> Result<impl IntoResponse, AppError> {
    // 1. 输入验证
    if payload.field.is_empty() {
        return Err(AppError::BadRequest("{字段}不能为空".to_string()));
    }

    // 2. 调用 Service 层（使用 ? 传播错误）
    let result = state.service.process(payload).await?;

    // 3. 返回成功响应
    Ok(Json(result))
}
```

### Backend Service 模式

```rust
use anyhow::{Result, Context};

pub async fn process(&self, input: Input) -> Result<Output> {
    // 使用 .context() 添加错误上下文
    let data = self.fetch_data()
        .await
        .context("failed to fetch data")?;

    Ok(output)
}
```

### 外部服务错误转换模式

```rust
// Handler 层转换外部服务错误
let ocr_result = state.ocr_service
    .extract_text(&image_path)
    .await
    .map_err(|err| {
        error!("OCR service error: {:?}", err);
        AppError::Ocr("图片识别失败，请稍后重试".to_string())
    })?;
```

### Frontend API 调用模式

```rust
pub async fn fetch_analysis(id: Uuid) -> Result<AnalysisResponse, ErrorInfo> {
    let response = reqwest::get(&format!("/api/analysis/{}", id))
        .await
        .map_err(|err| ErrorInfo::new("网络错误", format!("无法连接到服务器: {}", err)))?;

    if !response.status().is_success() {
        let api_error = response.json::<ApiError>().await.ok();
        return Err(match api_error {
            Some(err) => ErrorInfo::new("请求失败", err.message),
            None => ErrorInfo::new("请求失败", format!("服务器返回错误: {}", response.status())),
        });
    }

    response.json().await
        .map_err(|err| ErrorInfo::new("数据解析错误", format!("无法解析服务器响应: {}", err)))
}
```

---

## 日志记录规范

### 日志级别

| 级别 | 使用场景 | 示例 |
|------|---------|------|
| `error!` | 5xx 错误、外部服务失败 | 数据库错误、OCR 服务失败 |
| `warn!` | 4xx 错误、降级处理 | 参数验证失败、资源不存在 |
| `info!` | 重要的业务事件 | 用户上传、分析完成 |
| `debug!` | 调试信息 | 函数调用、中间状态 |

### 日志格式

**必须包含**:
- 错误详情（使用 `error = %err`）
- 关键上下文（ID、用户信息、操作类型）

**禁止记录**:
- 敏感信息（密码、Token、个人隐私）

**示例**:
```rust
use tracing::{error, warn, info};

// ✅ 好的日志
error!(
    error = %err,
    analysis_id = %id,
    "failed to run OCR task"
);

warn!(
    user_id = %user_id,
    content_type = ?content_type,
    "unsupported media type uploaded"
);

// ❌ 不好的日志
error!("error occurred");
error!("{:?}", err);
```

---

## 实现检查清单

在实现新功能时，必须检查以下项目:

### Backend 检查清单

- [ ] Handler 返回类型是 `Result<impl IntoResponse, AppError>`
- [ ] 所有输入验证都有清晰的错误消息（中文）
- [ ] 外部服务调用有错误处理和日志记录
- [ ] 数据库操作有错误转换（不暴露内部细节）
- [ ] 5xx 错误不暴露内部实现细节
- [ ] 关键操作有 `request_id` 追踪
- [ ] 错误日志包含足够的上下文

### Frontend 检查清单

- [ ] API 调用返回 `Result<T, ErrorInfo>`
- [ ] 错误状态正确展示给用户
- [ ] 提供错误恢复机制（返回首页/重试）
- [ ] 处理 API 响应解析异常

### Shared Types 检查清单

- [ ] 错误类型实现 `Serialize` + `Deserialize`
- [ ] 错误码使用 `UPPER_SNAKE_CASE`
- [ ] Frontend 和 Backend 类型保持同步

### 测试检查清单

- [ ] 编写了错误场景的测试用例
- [ ] 测试覆盖所有错误类型
- [ ] 验证错误消息的正确性

---

## 常见错误示例

### ❌ 错误做法

```rust
// 1. 暴露内部错误细节
AppError::Internal(format!("database error: {}", err))

// 2. 使用泛泛的错误消息
AppError::BadRequest("invalid input".to_string())

// 3. 在 Handler 中使用 unwrap
let user = get_user(id).await.unwrap();

// 4. 忽略错误
let _ = update_status(id).await;

// 5. 使用英文错误消息
AppError::BadRequest("file too large".to_string())
```

### ✅ 正确做法

```rust
// 1. 隐藏内部错误细节
error!("Database error: {:?}", err);
AppError::Internal("服务器内部错误，请稍后重试".to_string())

// 2. 提供清晰的错误消息
AppError::BadRequest("文件名长度不能超过 255 字符".to_string())

// 3. 使用 ? 传播错误
let user = get_user(id).await?;

// 4. 记录并处理错误
update_status(id).await.map_err(|err| {
    error!("Failed to update status: {:?}", err);
    err
})?;

// 5. 使用中文错误消息
AppError::BadRequest("文件大小超过限制".to_string())
```

---

## 参考文档

- [Rust Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [thiserror crate](https://docs.rs/thiserror/)
- [anyhow crate](https://docs.rs/anyhow/)
- [Axum Error Handling](https://docs.rs/axum/latest/axum/error_handling/index.html)
- [HTTP Status Codes](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status)

---

## 版本历史

| 版本 | 日期 | 变更说明 |
|------|------|---------|
| 2.0.0 | 2026-01-22 | 重构为简洁的规范格式，移除实现示例 |
| 1.0.0 | 2026-01-22 | 初始版本，定义 HTTP 错误处理标准 |
