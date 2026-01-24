# 010-移除 Tesseract OCR 技术方案

## 元数据

| 字段     | 值                                   |
| -------- | ------------------------------------ |
| 文档编号 | 010-remove-tesseract-ocr             |
| 标题     | 移除 Tesseract OCR 技术方案          |
| 版本     | 1.0                                  |
| 状态     | 草稿                                 |
| 创建日期 | 2026-01-24                           |
| 更新日期 | 2026-01-24                           |
| 作者     | Claude Code                          |
| 关联需求 | 010-remove-tesseract-ocr-requirements |

## 概述

### 目的

本技术方案详细说明如何安全、彻底地移除项目中的 Tesseract OCR 实现，简化 OCR 架构，统一使用 PaddleOCR 作为唯一的文字识别方案。

### 范围

本方案涵盖：
- 代码层面的删除和重构
- 配置系统的简化
- 依赖项的清理
- 文档的更新
- 测试验证策略

### 假设

- PaddleOCR 服务稳定可用，满足所有识别精度要求
- 不存在必须使用 Tesseract OCR 的离线部署场景
- 移除 OpenCV 依赖不会影响其他模块

## 架构设计

### 当前架构（移除前）

```
┌─────────────────────────────────────────────────────────────┐
│ Backend API Layer                                           │
│  ├─ handlers/analysis.rs                                    │
│  └─ extract_text(image_path, config)                        │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ OCR Service Layer (services/ocr.rs)                         │
│  ├─ extract_text() ← 根据 config.provider 路由              │
│  │   ├─ OcrProvider::Tesseract → extract_text_tesseract()  │
│  │   └─ OcrProvider::Paddle → extract_text_paddle()        │
│  │                                                           │
│  ├─ extract_text_tesseract() [条件编译: ocr-tesseract]      │
│  │   ├─ 调用 ocr_preprocess::preprocess_image()            │
│  │   └─ 调用 tesseract CLI 命令                             │
│  │                                                           │
│  └─ extract_text_paddle()                                   │
│      └─ HTTP 请求到 PaddleOCR 服务                          │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Preprocessing Module (services/ocr_preprocess.rs)           │
│  [条件编译: ocr-tesseract]                                   │
│  ├─ preprocess_image()                                      │
│  │   ├─ 灰度化 (OpenCV)                                     │
│  │   ├─ CLAHE 对比度增强                                    │
│  │   ├─ 高斯降噪                                            │
│  │   ├─ 锐化                                                │
│  │   ├─ 自适应二值化                                        │
│  │   ├─ 形态学闭运算                                        │
│  │   └─ 倾斜校正 (deskew)                                   │
│  │                                                           │
│  ├─ resize_if_needed()                                      │
│  └─ deskew()                                                │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Config Layer (config.rs)                                    │
│  ├─ OcrConfig                                               │
│  │   ├─ provider: OcrProvider                               │
│  │   ├─ lang: String                                        │
│  │   ├─ paddle_url: String                                  │
│  │   ├─ psm: Option<u8>        [Tesseract 参数]            │
│  │   ├─ oem: Option<u8>        [Tesseract 参数]            │
│  │   └─ preprocess: OcrPreprocessConfig                     │
│  │                                                           │
│  ├─ OcrProvider                                             │
│  │   ├─ Tesseract [条件编译: ocr-tesseract]                │
│  │   └─ Paddle                                              │
│  │                                                           │
│  └─ OcrPreprocessConfig [完整的预处理配置]                  │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Dependencies (Cargo.toml)                                   │
│  ├─ Feature: ocr-tesseract = ["dep:opencv"]                │
│  ├─ Feature: full = ["paddle", "ocr-tesseract", "heic"]    │
│  └─ opencv = { version = "0.92", optional = true }         │
└─────────────────────────────────────────────────────────────┘
```

### 目标架构（移除后）

```
┌─────────────────────────────────────────────────────────────┐
│ Backend API Layer                                           │
│  ├─ handlers/analysis.rs                                    │
│  └─ extract_text(image_path, config)                        │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ OCR Service Layer (services/ocr.rs) [简化版]                │
│  └─ extract_text()                                          │
│      └─ 直接调用 extract_text_paddle()                      │
│                                                              │
│  └─ extract_text_paddle()                                   │
│      └─ HTTP 请求到 PaddleOCR 服务                          │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Config Layer (config.rs) [简化版]                           │
│  └─ OcrConfig                                               │
│      ├─ lang: String                                        │
│      ├─ paddle_url: String                                  │
│      └─ timeout: Duration                                   │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ Dependencies (Cargo.toml) [简化版]                          │
│  ├─ Feature: default = ["paddle"]                           │
│  ├─ Feature: full = ["paddle", "heic"]                      │
│  └─ [移除 opencv 依赖]                                      │
└─────────────────────────────────────────────────────────────┘
```

### 架构变化总结

| 变化点 | 移除前 | 移除后 | 收益 |
|--------|--------|--------|------|
| OCR 实现 | 双实现（Tesseract + Paddle） | 单实现（Paddle） | 减少 150+ 行代码 |
| 预处理模块 | 完整的 OpenCV 预处理管道 | 无 | 减少 200+ 行代码 |
| 配置结构 | 复杂（支持两种 provider） | 简单（单一 provider） | 减少 50+ 行代码 |
| 条件编译 | 多处 `#[cfg(feature = "ocr-tesseract")]` | 无 | 降低代码复杂度 |
| 依赖项 | opencv (约 50MB) | 无 | 编译更快，二进制更小 |

## 数据模型

### 配置结构变化

#### 移除前

```rust
#[derive(Debug, Clone)]
pub struct OcrConfig {
    pub provider: OcrProvider,        // ← 移除
    pub lang: String,
    pub timeout: Duration,
    pub paddle_url: String,
    pub psm: Option<u8>,              // ← 移除（Tesseract 专用）
    pub oem: Option<u8>,              // ← 移除（Tesseract 专用）
    pub preprocess: OcrPreprocessConfig, // ← 移除
}

#[derive(Debug, Clone)]
pub enum OcrProvider {
    #[cfg(feature = "ocr-tesseract")]
    Tesseract,                        // ← 移除
    Paddle,
}

#[derive(Debug, Clone)]
pub struct OcrPreprocessConfig {      // ← 整个结构体移除
    pub enabled: bool,
    pub min_width: i32,
    pub max_width: i32,
    pub deskew: bool,
    pub binary: bool,
    pub denoise: bool,
    pub clahe: bool,
    pub sharpen: bool,
    pub morph_close: bool,
}
```

#### 移除后

```rust
#[derive(Debug, Clone)]
pub struct OcrConfig {
    pub lang: String,
    pub timeout: Duration,
    pub paddle_url: String,
}
```

### 环境变量变化

#### 移除的环境变量

```bash
# OCR Provider 选择
OCR_PROVIDER=tesseract|paddle        # ← 移除，固定使用 Paddle

# Tesseract 专用参数
OCR_PSM=3                            # ← 移除
OCR_OEM=3                            # ← 移除

# 预处理配置（共 8 个）
OCR_PREPROCESS_ENABLE=true           # ← 移除
OCR_PREPROCESS_MIN_WIDTH=1600        # ← 移除
OCR_PREPROCESS_MAX_WIDTH=2000        # ← 移除
OCR_PREPROCESS_DESKEW=true           # ← 移除
OCR_PREPROCESS_BINARY=true           # ← 移除
OCR_PREPROCESS_DENOISE=true          # ← 移除
OCR_PREPROCESS_CLAHE=true            # ← 移除
OCR_PREPROCESS_SHARPEN=true          # ← 移除
OCR_PREPROCESS_MORPH_CLOSE=false     # ← 移除
```

#### 保留的环境变量

```bash
# 核心 OCR 配置
OCR_LANG=chi_sim+eng                 # 保留
OCR_TIMEOUT=30                       # 保留
OCR_PADDLE_URL=http://ocr:8000/ocr   # 保留
```

## API 设计

### 内部 API 变化

#### `services/ocr.rs`

**移除前**：

```rust
pub async fn extract_text(image_path: &Path, config: &OcrConfig) -> Result<String> {
    match config.provider {
        #[cfg(feature = "ocr-tesseract")]
        OcrProvider::Tesseract => extract_text_tesseract(image_path, config).await,
        OcrProvider::Paddle => extract_text_paddle(image_path, config).await,
    }
}
```

**移除后**：

```rust
pub async fn extract_text(image_path: &Path, config: &OcrConfig) -> Result<String> {
    extract_text_paddle(image_path, config).await
}
```

或者直接内联：

```rust
pub async fn extract_text(image_path: &Path, config: &OcrConfig) -> Result<String> {
    let bytes = tokio::fs::read(image_path).await?;
    let part = reqwest::multipart::Part::bytes(bytes)
        .file_name("image.jpg")
        .mime_str("image/jpeg")?;
    let form = reqwest::multipart::Form::new().part("file", part);

    let client = reqwest::Client::builder()
        .timeout(config.timeout)
        .build()?;

    let response = client
        .post(&config.paddle_url)
        .multipart(form)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!(
            "paddle OCR failed: status {} body {}",
            status,
            body
        ));
    }

    let result: PaddleOcrResponse = response.json().await?;
    Ok(result.text.trim().to_string())
}

#[derive(serde::Deserialize)]
struct PaddleOcrResponse {
    text: String,
}
```

### 外部 API 影响

**无影响**：此次变更仅涉及内部实现，不影响对外 HTTP API 接口。

## 实施阶段

### 阶段 1：代码清理（核心删除）

#### 任务 1.1：移除 Tesseract OCR 实现

- [ ] 删除 `backend/src/services/ocr.rs` 中的函数：
  - `extract_text_tesseract()`
  - `build_tesseract_args()`
- [ ] 简化 `extract_text()` 函数，移除 `match config.provider` 逻辑
- [ ] 移除所有 `#[cfg(feature = "ocr-tesseract")]` 条件编译标记

**文件位置**: `backend/src/services/ocr.rs:22-100`

#### 任务 1.2：删除预处理模块

- [ ] 删除整个文件 `backend/src/services/ocr_preprocess.rs`
- [ ] 从 `backend/src/services/mod.rs` 中移除：
  ```rust
  #[cfg(feature = "ocr-tesseract")]
  pub mod ocr_preprocess;
  ```

**文件位置**:
- `backend/src/services/ocr_preprocess.rs` (整个文件)
- `backend/src/services/mod.rs`

#### 任务 1.3：简化配置结构

- [ ] 编辑 `backend/src/config.rs`：
  - 删除 `OcrProvider` 枚举（第 42-47 行）
  - 删除 `OcrPreprocessConfig` 结构体（第 24-35 行）
  - 从 `OcrConfig` 中移除字段：
    - `provider: OcrProvider`
    - `psm: Option<u8>`
    - `oem: Option<u8>`
    - `preprocess: OcrPreprocessConfig`
  - 删除 `parse_ocr_provider()` 函数（第 153-170 行）
  - 删除 `parse_optional_u8()` 函数（第 172-182 行）
  - 简化 `AppConfig::from_env()` 中的 OCR 配置解析（第 87-135 行）

**简化后的 OCR 配置解析**：

```rust
let ocr = OcrConfig {
    lang: env::var("OCR_LANG").unwrap_or_else(|_| "chi_sim+eng".to_string()),
    timeout: Duration::from_secs(
        env::var("OCR_TIMEOUT")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(30),
    ),
    paddle_url: env::var("OCR_PADDLE_URL")
        .unwrap_or_else(|_| "http://ocr:8000/ocr".to_string()),
};
```

**文件位置**: `backend/src/config.rs:14-135`

### 阶段 2：依赖清理

#### 任务 2.1：清理 Cargo.toml

- [ ] 编辑 `backend/Cargo.toml`：
  - 删除 feature 定义（第 12 行）：
    ```toml
    ocr-tesseract = ["dep:opencv"]
    ```
  - 修改 `full` feature（第 16 行）：
    ```toml
    # 修改前
    full = ["paddle", "ocr-tesseract", "heic"]
    # 修改后
    full = ["paddle", "heic"]
    ```
  - 删除 opencv 依赖（第 31 行）：
    ```toml
    opencv = { version = "0.92", default-features = false, features = ["imgcodecs", "imgproc"], optional = true }
    ```

**文件位置**: `backend/Cargo.toml:12, 16, 31`

#### 任务 2.2：验证编译

- [ ] 运行 `cargo clean` 清理旧的构建产物
- [ ] 运行 `cargo build` 确保编译成功
- [ ] 运行 `cargo clippy` 确保无警告
- [ ] 检查编译时间和二进制大小的改善

### 阶段 3：配置文件更新

#### 任务 3.1：更新环境变量配置

- [ ] 编辑 `.env.example`：
  - 移除 `OCR_PROVIDER=paddle`（已固定为 Paddle）
  - 移除所有 `OCR_PREPROCESS_*` 变量
  - 移除 `OCR_PSM` 和 `OCR_OEM`
  - 保留：
    ```bash
    OCR_LANG=chi_sim+eng
    OCR_TIMEOUT=30
    OCR_PADDLE_URL=http://ocr:8000/ocr
    ```

**文件位置**: `.env.example`

#### 任务 3.2：更新 Docker Compose 配置

- [ ] 编辑 `docker-compose.yml`：
  - 移除 `OCR_PROVIDER: paddle`（不再需要）

- [ ] 编辑 `docker-compose.prod.yml`：
  - 移除 `OCR_PROVIDER: ${OCR_PROVIDER:-tesseract}`
  - 或改为注释说明已固定使用 Paddle

**文件位置**:
- `docker-compose.yml`
- `docker-compose.prod.yml`

### 阶段 4：文档更新

#### 任务 4.1：更新项目文档

- [ ] 编辑 `CLAUDE.md`：
  - 移除 "OCR: PaddleOCR / Tesseract" 描述
  - 改为 "OCR: PaddleOCR"
  - 移除 Tesseract 相关的技术栈说明

- [ ] 编辑 `docs/standards/project-conventions.md`：
  - 移除 `OCR_PROVIDER=paddle  # tesseract | paddle` 说明
  - 更新为 "OCR 固定使用 PaddleOCR 服务"

- [ ] 编辑 `docs/design/003-ocr-quality-technical-plan.md`：
  - 移除 Tesseract 相关内容
  - 更新为仅支持 PaddleOCR 的说明

**文件位置**:
- `CLAUDE.md`
- `docs/standards/project-conventions.md`
- `docs/design/003-ocr-quality-technical-plan.md`

#### 任务 4.2：更新 API 文档（如果有）

- [ ] 检查 `docs/api/api-reference.md` 是否有 OCR 相关描述
- [ ] 如有，更新为仅支持 PaddleOCR 的说明

### 阶段 5：测试验证

#### 任务 5.1：单元测试

- [ ] 运行 `cargo test` 确保所有测试通过
- [ ] 检查是否有测试依赖 Tesseract 功能，如有则更新或删除

#### 任务 5.2：集成测试

- [ ] 启动完整的开发环境（backend + OCR service）
- [ ] 测试图片上传和 OCR 识别功能
- [ ] 验证 PaddleOCR 服务正常工作
- [ ] 测试各种图片格式（JPEG, PNG, WebP 等）

#### 任务 5.3：性能验证

- [ ] 测量编译时间（移除前 vs 移除后）
- [ ] 测量二进制文件大小（移除前 vs 移除后）
- [ ] 验证 OCR 识别精度未下降

### 阶段 6：代码审查和提交

#### 任务 6.1：代码审查

- [ ] 使用 `git diff` 检查所有变更
- [ ] 确认没有遗漏的 Tesseract 相关代码
- [ ] 确认没有破坏性变更

#### 任务 6.2：Git 提交

- [ ] 创建功能分支：`git checkout -b feat/remove-tesseract-ocr`
- [ ] 分阶段提交：
  1. `git commit -m "refactor: remove Tesseract OCR implementation"`
  2. `git commit -m "refactor: remove OpenCV preprocessing module"`
  3. `git commit -m "refactor: simplify OCR config structure"`
  4. `git commit -m "chore: remove opencv dependency from Cargo.toml"`
  5. `git commit -m "docs: update documentation to reflect Paddle-only OCR"`
  6. `git commit -m "chore: clean up environment variable examples"`

## 错误处理

### 错误场景处理

| 场景 | 当前行为 | 移除后行为 | 处理方式 |
|------|---------|-----------|---------|
| PaddleOCR 服务不可用 | 返回错误 | 返回错误（不变） | 保持现有错误处理 |
| 图片格式不支持 | 返回错误 | 返回错误（不变） | 保持现有错误处理 |
| 环境变量 `OCR_PROVIDER=tesseract` | 尝试使用 Tesseract | **配置解析失败** | 移除该环境变量 |
| 编译时启用 `ocr-tesseract` feature | 编译成功 | **编译失败（feature 不存在）** | 移除 feature flag |

### 错误信息更新

移除前（如果配置了 Tesseract 但未编译该 feature）：
```rust
Err(anyhow::anyhow!(
    "OCR_PROVIDER=tesseract is not supported in this build (enable feature: ocr-tesseract)"
))
```

移除后：
- 不再需要此错误信息，因为 `OCR_PROVIDER` 环境变量本身已被移除

## 性能考虑

### 编译性能改善

| 指标 | 移除前（估算） | 移除后（估算） | 改善 |
|------|---------------|---------------|------|
| 首次编译时间 | ~5-8 分钟 | ~3-5 分钟 | **减少 30-40%** |
| 增量编译时间 | ~30-60 秒 | ~20-40 秒 | **减少 30%** |
| 二进制大小 (release) | ~50-60 MB | ~30-40 MB | **减少 30-40%** |
| 依赖项数量 | ~200+ | ~180+ | **减少 10%** |

### 运行时性能

- **无影响**：OCR 识别性能取决于 PaddleOCR 服务，移除 Tesseract 不影响运行时性能
- **内存占用**：减少约 20-30 MB（移除 OpenCV 相关库）

### 监控建议

- 监控 PaddleOCR 服务可用性（已有）
- 监控 OCR 识别成功率（已有）
- 监控 OCR 响应时间（已有）

## 测试策略

### 单元测试

**现有测试**：
- 检查 `backend/src/services/ocr.rs` 是否有单元测试
- 如有 Tesseract 相关测试，移除或更新

**新增测试**（可选）：
```rust
#[tokio::test]
async fn test_extract_text_paddle_success() {
    // 测试 PaddleOCR 正常识别
}

#[tokio::test]
async fn test_extract_text_paddle_service_unavailable() {
    // 测试 PaddleOCR 服务不可用时的错误处理
}
```

### 集成测试

**测试清单**：
- [ ] 上传 JPEG 图片，验证 OCR 识别成功
- [ ] 上传 PNG 图片，验证 OCR 识别成功
- [ ] 上传 WebP 图片，验证 OCR 识别成功
- [ ] 上传包含中文的配料表图片，验证识别准确
- [ ] 上传包含英文的配料表图片，验证识别准确
- [ ] 模拟 PaddleOCR 服务不可用，验证错误处理

### 回归测试

**关键功能验证**：
- [ ] 完整的配料表分析流程（拍照 → OCR → LLM 分析 → 结果展示）
- [ ] 多张图片连续识别
- [ ] 错误处理和用户提示

## 部署

### 环境要求

**无变化**：
- 仍然需要 PaddleOCR 服务（Docker 容器）
- 仍然需要 PostgreSQL 和 Redis

**移除的要求**：
- ~~不再需要本地安装 Tesseract CLI~~
- ~~不再需要 OpenCV 运行时库~~

### 配置

**生产环境配置变更**：

```bash
# 移除的环境变量
# OCR_PROVIDER=paddle          # ← 移除（固定使用 Paddle）
# OCR_PSM=3                    # ← 移除
# OCR_OEM=3                    # ← 移除
# OCR_PREPROCESS_*=...         # ← 移除（共 8 个）

# 保留的环境变量
OCR_LANG=chi_sim+eng
OCR_TIMEOUT=30
OCR_PADDLE_URL=http://ocr:8000/ocr
```

### 回滚计划

**如果需要回滚**：

1. **代码回滚**：
   ```bash
   git revert <commit-hash>
   ```

2. **重新编译**：
   ```bash
   cargo clean
   cargo build --release
   ```

3. **恢复环境变量**：
   - 恢复 `OCR_PROVIDER=paddle`
   - 恢复其他 Tesseract 相关配置

4. **验证**：
   - 运行集成测试
   - 验证 OCR 功能正常

**回滚风险**：低（仅涉及内部实现，不影响 API 接口）

## 风险与缓解

| 风险 | 影响 | 可能性 | 缓解措施 |
|------|------|--------|---------|
| PaddleOCR 服务不稳定 | 高 | 低 | 1. 提前验证 PaddleOCR 稳定性<br>2. 保留回滚能力<br>3. 监控服务可用性 |
| 识别精度下降 | 高 | 低 | 1. 对比测试 Tesseract 和 Paddle 的识别结果<br>2. 保留测试数据集 |
| 离线部署需求 | 中 | 低 | 1. 确认无离线部署场景<br>2. 如有需求，考虑 Paddle 本地化部署 |
| 编译失败 | 中 | 低 | 1. 分阶段提交，逐步验证<br>2. 使用 CI/CD 自动化测试 |
| 遗漏清理 | 低 | 中 | 1. 使用 `grep -r "tesseract"` 全局搜索<br>2. 代码审查 |
| 文档不同步 | 低 | 中 | 1. 系统性更新所有文档<br>2. 文档审查 |

## 待解决问题

| 问题 | 影响 | 负责人 | 状态 |
|------|------|--------|------|
| 是否存在离线部署场景需要 Tesseract | 中 | 产品负责人 | 待确认 |
| PaddleOCR 识别精度是否满足所有场景 | 高 | 技术负责人 | 待验证 |
| 是否需要保留 Tesseract 作为备用方案 | 低 | 技术负责人 | 待讨论 |

## 验证清单

### 代码层面

- [ ] 所有 Tesseract 相关代码已删除
- [ ] 所有 OpenCV 相关代码已删除
- [ ] 所有条件编译标记已移除
- [ ] `cargo build` 编译成功
- [ ] `cargo clippy` 无警告
- [ ] `cargo test` 所有测试通过

### 配置层面

- [ ] `OcrConfig` 结构体已简化
- [ ] `OcrProvider` 枚举已删除
- [ ] `OcrPreprocessConfig` 结构体已删除
- [ ] 环境变量解析代码已简化
- [ ] `.env.example` 已更新
- [ ] `docker-compose.yml` 已更新
- [ ] `docker-compose.prod.yml` 已更新

### 依赖层面

- [ ] `ocr-tesseract` feature 已删除
- [ ] `opencv` 依赖已删除
- [ ] `full` feature 已更新
- [ ] `Cargo.lock` 已更新（运行 `cargo build` 后自动更新）

### 文档层面

- [ ] `CLAUDE.md` 已更新
- [ ] `docs/standards/project-conventions.md` 已更新
- [ ] `docs/design/003-ocr-quality-technical-plan.md` 已更新
- [ ] 其他相关文档已检查并更新

### 功能验证

- [ ] OCR 识别功能正常工作
- [ ] 支持多种图片格式（JPEG, PNG, WebP）
- [ ] 中文识别准确
- [ ] 英文识别准确
- [ ] 错误处理正常
- [ ] 性能无明显下降

### 性能验证

- [ ] 编译时间减少 30% 以上
- [ ] 二进制大小减少 30% 以上
- [ ] OCR 识别速度无明显变化
- [ ] 内存占用减少

## 参考资料

- [010-移除 Tesseract OCR 需求文档](../requirements/010-remove-tesseract-ocr-requirements.md)
- [003-OCR 质量优化技术方案](./003-ocr-quality-technical-plan.md)
- [项目编码规范](../standards/coding-standards.md)
- [错误处理规范](../standards/error-handling-standards.md)
- [项目约定规范](../standards/project-conventions.md)
- [PaddleOCR 官方文档](https://github.com/PaddlePaddle/PaddleOCR)

---

## 变更记录

| 版本 | 日期       | 作者        | 描述     |
| ---- | ---------- | ----------- | -------- |
| 1.0  | 2026-01-24 | Claude Code | 初始版本 |
