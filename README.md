# 🔍 食品配料表分析助手 (Smart Ingredients)

<div align="center">

**智能食品配料表分析工具 | OCR + AI 驱动的健康评分与风险提示**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.x-24C8DB.svg)](https://tauri.app/)

[功能特性](#-功能特性) • [技术栈](#-技术栈) • [快速开始](#-快速开始) • [文档](#-文档) • [贡献指南](#-贡献指南)

</div>

---

## 📖 项目简介

Smart Ingredients 是一个跨平台的智能食品配料表分析工具，帮助用户快速了解食品的健康程度。

**核心能力：**
- 📷 **拍照识别**：拍照或上传配料表图片
- 🔤 **OCR 提取**：自动识别配料文字信息
- 🤖 **AI 分析**：结合大模型智能分析成分与风险
- 📊 **健康评分**：输出健康评分与风险提示
- 💾 **历史记录**：保存分析历史，随时查看

---

## ✨ 功能特性

- ✅ **跨平台支持**：基于 Tauri，支持 Windows、macOS、Linux
- ✅ **高性能 OCR**：集成 PaddleOCR，准确识别配料表文字
- ✅ **智能 AI 分析**：接入 DeepSeek / 智谱 AI，深度解析成分风险
- ✅ **实时健康评分**：即时反馈食品健康度与潜在风险
- ✅ **数据本地化**：支持本地存储分析历史（PostgreSQL + Redis）
- ✅ **现代化 UI**：基于 Leptos 构建，响应式设计，流畅体验

---

## 🛠️ 技术栈

### 前端
- **框架**：[Tauri 2.x](https://tauri.app/) + [Leptos 0.7.x](https://leptos.dev/)
- **语言**：Rust + WebAssembly
- **样式**：Tailwind CSS

### 后端
- **框架**：[Axum 0.7.x](https://github.com/tokio-rs/axum)
- **语言**：Rust
- **数据库**：PostgreSQL 16.x + [SQLx](https://github.com/launchbadge/sqlx)
- **缓存**：Redis 7.x

### AI 服务
- **OCR**：PaddleOCR (FastAPI)
- **LLM**：DeepSeek / 智谱 AI

### 开发工具
- **容器化**：Docker + Docker Compose
- **代码质量**：cargo fmt + cargo clippy
- **文档驱动**：严格的文档优先开发流程

---

## 🚀 快速开始

### 方式一：Docker 一键启动（推荐）

```bash
# 克隆仓库
git clone https://github.com/ZHOUKAILIAN/smart-ingredients.git
cd smart-ingredients

# 启动所有服务
docker-compose up -d

# 访问应用
# 前端: http://localhost:1420
# 后端 API: http://localhost:3000
```

### 方式二：本地开发启动

**前置要求：**
- Rust 1.75+
- Node.js 18+
- PostgreSQL 16+
- Redis 7+
- Python 3.8+ (用于 OCR 服务)

```bash
# 1. 启动后端
cd backend
cargo run

# 2. 启动前端（新终端）
cd frontend
cargo tauri dev

# 3. 启动 OCR 服务（新终端）
cd ocr-service
pip install -r requirements.txt
python app.py
```

**详细启动文档：**
- Docker 启动：`docs/run/docker-setup.md`
- 本地开发：`docs/run/backend-startup.md`

---

## 📚 文档

### 核心文档
- [📐 技术架构](docs/design/technical-design.md) - 系统架构与设计决策
- [🎨 UI 设计](docs/design/ui-design.md) - 界面设计规范
- [🔌 API 参考](docs/api/api-reference.md) - 后端 API 文档
- [📝 编码规范](docs/standards/coding-standards.md) - Rust 编码标准

### 开发文档
- [📋 需求文档](docs/requirements/) - 功能需求与用户故事
- [🏗️ 设计文档](docs/design/) - 技术设计与架构决策
- [⚙️ 项目约定](docs/standards/project-conventions.md) - 项目规范与约定
- [🔍 项目分析](docs/analysis/project-analysis.md) - 项目结构分析

### 协作文档
- [🤖 CLAUDE.md](CLAUDE.md) - AI 助手协作规范
- [👥 AGENTS.md](AGENTS.md) - 多智能体协作流程

---

## 📁 项目结构

```
smart-ingredients/
├── frontend/           # Tauri + Leptos 前端应用
├── backend/            # Axum 后端服务
├── shared/             # 共享类型定义（workspace member）
├── ocr-service/        # PaddleOCR 服务（Python）
├── docs/               # 项目文档
│   ├── requirements/   # 需求文档
│   ├── design/         # 设计文档
│   ├── api/            # API 文档
│   ├── standards/      # 编码规范
│   ├── analysis/       # 项目分析
│   └── run/            # 运行指南
├── scripts/            # 开发脚本
├── Cargo.toml          # Workspace 配置
├── CLAUDE.md           # AI 协作规范
└── README.md           # 项目说明
```

---

## 🤝 贡献指南

我们欢迎所有形式的贡献！在提交代码前，请确保：

1. **阅读文档优先开发流程**：查看 [CLAUDE.md](CLAUDE.md) 了解文档驱动开发规范
2. **遵循编码规范**：运行 `cargo fmt` 和 `cargo clippy`
3. **编写测试**：为新功能添加单元测试或集成测试
4. **更新文档**：如有 API 或功能变更，同步更新文档

**贡献流程：**
```bash
# 1. Fork 仓库并克隆
git clone https://github.com/YOUR_USERNAME/smart-ingredients.git

# 2. 创建功能分支
git checkout -b feat/your-feature

# 3. 提交代码
git commit -m "feat: add your feature"

# 4. 推送分支
git push origin feat/your-feature

# 5. 创建 Pull Request
```

---

## 📄 License

本项目采用 [MIT License](LICENSE) 开源协议。

---

## 🙏 致谢

感谢以下开源项目：
- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [Leptos](https://leptos.dev/) - 响应式 Rust Web 框架
- [Axum](https://github.com/tokio-rs/axum) - 高性能 Web 框架
- [PaddleOCR](https://github.com/PaddlePaddle/PaddleOCR) - OCR 识别引擎
- [DeepSeek](https://www.deepseek.com/) / [智谱 AI](https://www.zhipuai.cn/) - 大语言模型服务

---

<div align="center">

**如果这个项目对你有帮助，请给一个 ⭐️ Star！**

Made with ❤️ by [ZHOUKAILIAN](https://github.com/ZHOUKAILIAN)

</div>
