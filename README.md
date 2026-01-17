# 食品配料表分析助手

## 项目简介

一个智能食品配料表分析工具，帮助用户快速了解食品的健康程度。

## 核心功能

1. **拍照识别**：用户通过 App/小程序拍摄食品配料表
2. **OCR 解析**：自动识别图片中的文字内容
3. **智能分析**：基于 LLM 分析配料健康程度
4. **结果展示**：返回详细的健康评估报告

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
├── frontend/                 # Rust 前端应用
├── backend/                  # Rust 后端服务
├── shared/                   # 共享类型定义
├── docs/                     # 项目文档
├── scripts/                  # 开发脚本
└── docker-compose.yml        # 本地开发环境
```

## 快速开始

### 前置要求

- Rust 1.80+
- Docker & Docker Compose

### 安装

```bash
# 克隆仓库
git clone git@github.com:ZHOUKAILIAN/smart-ingredients.git
cd smart-ingredients

# 启动后端
cd backend
cargo run

# 启动前端
cd frontend
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
| OCR    | OCR                   |
| LLM    | DeepSeek / 智谱 AI    |

## 开发计划

- [ ] 前端框架搭建
- [ ] 后端 API 设计
- [ ] OCR 服务集成
- [ ] LLM 接入
- [ ] 数据库设计

## License

MIT
