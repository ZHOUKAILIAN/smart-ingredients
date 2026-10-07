# Smart Ingredients · 食品配料表分析助手

拍摄或上传食品配料表，经过 OCR、文字确认、规则与模型分析，展示健康评分、风险依据及历史记录。优先保证结论可解释。

## 项目结构

| 路径 | 职责 |
| --- | --- |
| [frontend/](frontend/) | Rust + Leptos 界面，Tauri 原生壳位于 `frontend/src-tauri/` |
| [miniapp/](miniapp/) | Taro + React 微信小程序 |
| [backend/](backend/) | Axum API、分析流程、账号、规则与社区服务 |
| [shared/](shared/) | Rust 前后端共享类型与序列化契约 |
| [ocr_service/](ocr_service/) | Python + FastAPI + PaddleOCR 服务 |
| [scripts/](scripts/) | 构建、部署及辅助工具 |
| [docs/](docs/README.md) | 维护规范、模板及保留的部署配置 |

具体依赖以 [Cargo workspace](Cargo.toml)、[前端工具链](package.json) 和 [小程序包清单](miniapp/package.json) 为准。后端使用 PostgreSQL、Redis；当前模型提供方实现见 [后端配置](backend/src/config.rs)。

## 运行入口

以下是配置入口，不是已经重新验收的一键启动方案。首次运行前检查依赖、环境变量和端口，再按 [验证要求](AGENTS.md#验证与完成) 实际验收。

| 对象 | 配置与入口 |
| --- | --- |
| 环境变量 | [.env.example](.env.example)；仅在本地创建 `.env`，替换 API 密钥及认证密钥占位值，不提交真实值 |
| 后端与依赖服务 | [docker-compose.yml](docker-compose.yml)：PostgreSQL、Redis、OCR、后端及可选 MinIO；**不包含前端** |
| 生产部署配置 | [docker-compose.prod.yml](docker-compose.prod.yml)；使用前核对部署环境，不直接沿用开发默认凭据 |
| Web 前端 | [Trunk.toml](Trunk.toml)；在仓库根安装 npm 依赖，准备 Rust WASM target 与 Trunk，构建前配置 `API_BASE` |
| Tauri | [tauri.conf.json](frontend/src-tauri/tauri.conf.json)；需要对应平台工具链 |
| 微信小程序 | [miniapp/package.json](miniapp/package.json) 中的构建命令，以及 [project.config.json](miniapp/project.config.json) |
| OCR 独立服务 | [ocr_service/Dockerfile](ocr_service/Dockerfile) 与 [requirements.txt](ocr_service/requirements.txt) |
| 监控与反向代理 | [监控 Compose](docs/deployment/monitoring/docker-compose.monitoring.yml)、[Nginx 配置](docs/deployment/nginx/smart-ingredients.conf) |

注意：

- 容器内地址与宿主机地址不同；本地独立启动后端时应核对 `DATABASE_URL`、`REDIS_URL`、`OCR_PADDLE_URL`，配置读取规则见 [config.rs](backend/src/config.rs)。
- 前端 `API_BASE` 在构建时读取，规则见 [frontend/build.rs](frontend/build.rs)。不要把个人或生产环境地址写死到共享配置。
- 当前 Trunk 服务端口为 `8085`，Tauri `devUrl` 为 `8080`，两者尚未对齐；不能把 Tauri 开发启动视为已验证可用。本轮文档治理不修改运行配置。
- 部署脚本、种子 SQL、签名工具会修改外部环境或生成敏感产物；执行前确认目标与影响。

## 开发与协作

1. 先读 [AGENTS.md](AGENTS.md)：调研、计划、确认、实施和验证的统一规则。
2. 再读 [docs/README.md](docs/README.md)：规范入口及文档归属规则。
3. 修改已有功能时更新原需求和设计；没有合理归属时才按本次范围补齐必要文档，不为每个修复另建文档对。
4. 验证后说明结果、未执行项和风险。代码实现完成不等于测试通过或已部署。

[CLAUDE.md](CLAUDE.md) 仅为工具入口，不维护第二套规则。旧功能文档已清理，无需额外归档或批量恢复。

## License

仓库目前缺少 `LICENSE` 文件。项目许可证与第三方资源授权需由维护者确认并补齐，不能仅凭此前的 MIT 标识视为授权材料完整。
