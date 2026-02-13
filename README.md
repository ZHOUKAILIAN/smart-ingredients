# 食品配料表分析助手

## 项目简介

一个智能食品配料表分析工具，帮助用户快速了解食品的健康程度。

## 核心功能（已实现）

1. **拍照/上传**：前端支持拍照或相册选择配料表图片
2. **OCR 解析**：后端接入 PaddleOCR 服务识别文本
3. **文本确认**：OCR 结果可在前端确认后再分析
4. **智能分析**：LLM 输出健康评分与分析报告
5. **偏好设置**：支持体重管理/健康/健身/过敏/儿童等偏好
6. **历史记录**：可查看分析历史与详情

## 技术架构

```text
┌─────────────┐      ┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│   前端 App  │ ───> │   后端 API  │ ───> │  OCR 服务   │ ───> │   LLM 分析  │
│  (Rust/Tauri) │   │  (Rust/Axum) │   │             │      │             │
└─────────────┘      └─────────────┘      └─────────────┘      └─────────────┘
```

## 项目结构

采用 Monorepo 方式，前后端代码在同一个仓库中：

```text
smart-ingredients/
├── frontend/                 # Rust 前端应用（Tauri + Leptos）
├── backend/                  # Rust 后端服务（Axum + SQLx）
├── shared/                   # 共享类型定义
├── ocr_service/              # PaddleOCR 服务（FastAPI）
├── docs/                     # 项目文档
├── scripts/                  # 开发脚本
├── docker-compose.yml        # 本地开发环境
└── docker-compose.prod.yml   # 生产环境示例
```

## 快速开始

### 前置要求

- Rust 1.80+
- Docker & Docker Compose
- Node.js（用于 Tauri 前端构建环境）

### 方式一：Docker 一键启动（推荐）

```bash
# 克隆仓库
git clone git@github.com:ZHOUKAILIAN/smart-ingredients.git
cd smart-ingredients

# 复制环境变量模板
cp .env.example .env

# 启动后端 + OCR + 数据库 + Redis

docker compose up --build
```

后端默认端口为 `http://localhost:3000`，OCR 服务为 `http://localhost:8000`。

### 方式二：本地开发启动

```bash
# 克隆仓库
git clone git@github.com:ZHOUKAILIAN/smart-ingredients.git
cd smart-ingredients
cp .env.example .env

# 启动后端
cd backend
cargo run

# 启动前端
cd ../frontend
cargo tauri dev
```

## 技术选型

详见 [TECH-STACK.md](./TECH-STACK.md)

| 模块   | 技术                  |
| ------ | --------------------- |
| 前端   | Rust + Tauri + Leptos |
| 后端   | Rust + Axum + SQLx    |
| 共享   | Rust 共享类型库       |
| 数据库 | PostgreSQL + Redis    |
| OCR    | PaddleOCR (FastAPI) + Tesseract 备用 |
| LLM    | DeepSeek / 智谱 AI    |

## 路线图（建议下一步）

- [ ] 完善配料解析与风险规则库
- [ ] 增加分享卡片/导出能力
- [ ] 丰富过敏原/偏好配置（细分项 + 强提醒）
- [ ] 提升 OCR 图像预处理与准确率

## License

MIT
