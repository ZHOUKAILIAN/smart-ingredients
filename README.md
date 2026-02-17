# 食品配料表分析助手

## 项目简介

本项目是一个智能食品配料表分析工具，支持拍照/上传配料表图片，结合 OCR 与大模型分析输出健康评分与风险提示，帮助用户快速理解食品成分与潜在风险。

## 依赖概览（详细配置见 docs）

- **前端**：Rust、Tauri、Leptos
- **后端**：Rust、Axum、SQLx
- **数据库/缓存**：PostgreSQL、Redis
- **OCR**：PaddleOCR（FastAPI）
- **LLM**：DeepSeek / 智谱 AI
- **运行环境**：Docker & Docker Compose（可选）

依赖的安装方式、版本与配置细节请查看 `docs/` 目录内的对应文档。

## 技术架构

- 技术架构图与详细说明见 `docs/design/technical-design.md`。

## 路由与页面

- 前端路由定义与维护方式见 `docs/design/routing.md`。

## Claude/Agents 使用说明

- 使用时机与维护规范见 `docs/standards/ai-collaboration.md`。

## 启动文档（两种方式）

- **方式一：Docker 一键启动**：`docs/run/backend-startup.md`
- **方式二：本地开发启动**：`docs/run/local-dev-startup.md`

> 具体命令、环境变量与端口说明，请以对应文档为准。

## License

MIT
