# 技术选型文档

## 仓库结构

采用 **Monorepo** 方式，前后端代码在同一个仓库中，通过目录清晰分离。

```text
smart-ingredients/
├── frontend/                 # Rust 前端应用 (独立 workspace 成员)
│   ├── src/
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── backend/                  # Rust 后端服务 (独立 workspace 成员)
│   ├── src/
│   ├── Cargo.toml
│   └── migrations/           # 数据库迁移文件
│
├── shared/                   # 共享类型定义 (workspace 成员)
│   ├── src/
│   └── Cargo.toml
│
├── docs/                     # 项目文档
│   ├── api/                  # API 文档
│   └── design/               # 设计文档
│
├── scripts/                  # 开发脚本
│   ├── dev.sh                # 一键启动前后端
│   └── build.sh              # 构建脚本
│
├── Cargo.toml                # Workspace 配置
├── docker-compose.yml        # 本地开发环境
├── .gitignore
├── README.md
└── TECH-STACK.md
```

### 为什么选择 Monorepo？

1. **简化管理** - 一个仓库，一套 CI/CD
2. **版本同步** - 前后端 API 变更更容易追踪
3. **开发效率** - 不需要频繁切换仓库
4. **类型共享** - 前后端共享类型定义，避免不一致
5. **适合本场景** - 前后端紧密耦合，独立发版需求不强

### Workspace 配置

```toml
# Cargo.toml (根目录)
[workspace]
members = ["frontend", "backend", "shared"]
resolver = "2"
```

---

## 前端技术栈 (Rust)

### 核心框架

| 技术 | 版本 | 说明 |
|------|------|------|
| **Tauri** | v2.x | 跨平台桌面应用框架，体积小、性能高 |
| **Leptos** | v0.7.x | 响应式 Web 框架，支持 SSR/CSR |
| **Bevy** | v0.14.x | 游戏引擎（可选，用于 3D 交互） |

### 为什么选择 Rust 前端？

1. **性能优势**
   - 编译型语言，运行速度极快
   - 内存安全，无 GC 暂停
   - 适合处理大量图片数据

2. **Tauri vs Electron**
   - Electron 包含 Chromium，体积 100MB+
   - Tauri 使用系统 WebView，体积 < 10MB
   - 内存占用显著降低

3. **Leptos 特性**
   - 细粒度响应式系统
   - 类 React 开发体验
   - 支持 Server-Side Rendering

4. **与后端统一**
   - 共享类型定义，API 调用更安全
   - 统一的开发工具链

### 依赖库

```toml
[dependencies]
tauri = { version = "2", features = ["shell-open"] }
leptos = { version = "0.7", features = ["csr"] }
leptos-use = "0.15"           # 组合式 API 工具库
reqwest = { version = "0.12", features = ["multipart"] }
image = "0.25"                 # 图片处理
serde = { version = "1", features = ["derive"] }
serde_json = "1"
shared = { path = "../shared" } # 共享类型
```

---

## 后端技术栈 (Rust)

### 核心框架

| 技术 | 版本 | 说明 |
|------|------|------|
| **Axum** | v0.7.x | 高性能异步 Web 框架 |
| **SQLx** | v0.8.x | 类型安全的 SQL 工具 |
| **PostgreSQL** | v16.x | 主数据库 |
| **Redis** | v7.x | 缓存 + 队列 |

### 为什么选择 Rust 后端？

1. **性能极致**
   - 零成本抽象，编译优化
   - 无 GC，内存占用可控
   - 适合高并发场景

2. **类型安全**
   - 编译时检查 SQL 查询 (SQLx)
   - 与前端共享类型定义
   - 减少运行时错误

3. **并发模型**
   - Tokio 异步运行时
   - 适合处理大量 I/O 密集型任务
   - OCR/LLM 调用异步处理

4. **生态成熟**
   - Axum 基于 Tokio 和 Tower
   - SQLx 提供编译时 SQL 验证
   - 丰富的中间件支持

### 服务架构

```text
┌─────────────────────────────────────────────────────────┐
│                      API Gateway                         │
│                        (Axum)                             │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┼────────────┬────────────┐
        │            │            │            │
   ┌────▼────┐  ┌───▼────┐  ┌───▼────┐  ┌────▼────┐
   │  上传   │  │  查询  │  │  分析  │  │  用户  │
   │  服务   │  │  服务  │  │  服务  │  │  服务  │
   └────┬────┘  └───┬────┘  └───┬────┘  └────┬────┘
        │            │            │            │
   ┌────▼────────────▼────────────▼────────────▼────┐
   │              PostgreSQL + Redis                 │
   └───────────────────────────────────────────────────┘
```

### 依赖库

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.8", features = ["postgres", "runtime-tokio", "chrono"] }
redis = { version = "0.26", features = ["tokio-comp", "connection-manager"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"                  # 错误处理
thiserror = "1"               # 错误类型定义
tracing = "0.1"               # 日志
tracing-subscriber = "0.3"
jsonwebtoken = "9"           # JWT
reqwest = { version = "0.12", features = ["json"] }
shared = { path = "../shared" } # 共享类型
```

### OCR 集成

#### Rust 原生库（开发/测试）

```toml
# 开发环境快速验证
tesseract = "0.15"
leptonica = "0.13"
```

```rust
use tesseract::Tesseract;

let mut tesseract = Tesseract::new(None, Some("chi_sim"))?;
tesseract.set_image_path("image.png")?;
let text = tesseract.get_text()?;
```

#### 独立 OCR 服务（推荐生产）

```text
┌─────────────┐      ┌─────────────┐
│  Rust 后端  │ ───> │  PaddleOCR  │
│   (Axum)    │      │  (Python)   │
└─────────────┘      └─────────────┘
```

后端通过 HTTP 调用独立 OCR 服务，支持 PaddleOCR、Tesseract 等多种引擎。

---

## 共享类型 (shared)

### 作用

前后端共享类型定义，确保 API 契约一致。

```rust
// shared/src/types.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisRequest {
    pub image_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResponse {
    pub id: String,
    pub status: AnalysisStatus,
    pub result: Option<AnalysisResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}
```

---

## 第三方服务

### OCR 服务

#### 开源方案

| 服务 | 语言 | 中文支持 | 优势 | 劣势 |
|------|------|----------|------|------|
| **PaddleOCR** | Python | 非常好 | 百度开源，准确率高，模型丰富 | 依赖 PaddlePaddle |
| **Tesseract** | C++ | 一般 | 老牌开源，生态成熟 | 中文效果一般 |
| **EasyOCR** | Python | 较好 | 易用，支持多语言 | 准确率略低于 PaddleOCR |

#### Rust 原生库

| 库 | 说明 | 中文支持 |
|------|------|----------|
| **tesseract-rs** | Tesseract 的 Rust 绑定 | 一般 |
| **tesseract** | 另一个 Tesseract 绑定 | 一般 |

#### 云服务

| 服务 | 优势 | 劣势 |
|------|------|------|
| **百度 OCR** | 中文识别准确率高 | 收费 |
| **腾讯 OCR** | 多场景适配 | API 限制 |

**推荐方案**:
- 开发/测试：tesseract-rs（快速验证）
- 生产环境：PaddleOCR 独立服务（准确率保证）

### LLM 服务

| 服务 | 优势 | 劣势 |
|------|------|------|
| **DeepSeek** | 性价比高 | 新兴服务 |
| **智谱 AI** | 中文优化好 | API 限制 |
| **OpenAI** | 能力最强 | 国内访问困难 |

**推荐**: DeepSeek 或 智谱 AI

### 对象存储

| 服务 | 优势 | 劣势 |
|------|------|------|
| **阿里云 OSS** | 国内稳定 | 收费 |
| **腾讯云 COS** | CDN 加速好 | 收费 |
| **MinIO** | 自建免费 | 需要运维 |

**推荐**: 阿里云 OSS（生产）/ MinIO（开发）

---

## 开发工具

- **Rust**: rustup 管理
- **IDE**: VS Code + rust-analyzer
- **构建**: cargo
- **数据库**: psql / pgAdmin
- **Redis**: redis-cli

---

## 部署方案

### 开发环境

```bash
# Docker Compose
docker-compose up -d
```

### 生产环境

- **前端**: Tauri 打包成安装包
- **后端**: Docker 镜像 + K8s 部署
- **数据库**: RDS PostgreSQL
- **缓存**: Redis Cluster

---

## 待定事项

- [ ] 确定具体 OCR 服务商
- [ ] 确定 LLM 模型选择
- [ ] 确定对象存储方案
- [ ] 数据库表结构设计
