# 016-服务端稳定性与可观测性技术方案

## 元数据

| 字段     | 值                                         |
| -------- | ------------------------------------------ |
| 文档编号 | 016-server-stability-observability         |
| 标题     | 服务端稳定性与可观测性技术方案             |
| 版本     | 1.0                                        |
| 状态     | 草稿                                       |
| 创建日期 | 2026-01-28                                 |
| 更新日期 | 2026-01-28                                 |
| 作者     | Codex                                      |
| 关联需求 | 016-server-stability-observability         |

## 概述

### 目的

为服务端建立结构化日志、失败率指标与告警能力，并接入阿里云日志服务（SLS），让异常可见、可追踪、可统计。

### 范围

- 后端日志结构化输出（JSON）
- 关键请求与依赖调用埋点
- 接入 SLS 采集、检索、看板与告警
- 失败率（接口级与端到端）统计口径

### 假设

- 服务以 Docker 方式部署，可在宿主机安装 Logtail。
- 后端已使用 `tracing`，可扩展为 JSON 日志。
- 日志通过 stdout 输出，便于容器采集。

## 架构设计

### 高层架构

应用日志（stdout JSON） → Docker 日志文件 → Logtail 采集 → SLS Logstore → 可视化/告警

### 组件图

- 后端
  - 请求追踪与结构化日志（`tracing`）
  - 错误日志与依赖调用日志
- 采集
  - Logtail（宿主机代理）
- 平台
  - SLS Logstore（索引、检索、可视化、告警）

## 日志规范

### 字段定义

| 字段 | 说明 | 必填 |
| --- | --- | --- |
| `timestamp` | 日志时间 | 是 |
| `level` | 日志级别 | 是 |
| `service` | 服务名（如 `backend`） | 是 |
| `env` | 环境（prod/staging/dev） | 是 |
| `request_id` | 请求唯一标识 | 是 |
| `route` | 路由模板（如 `/api/v1/analysis`） | 是 |
| `method` | HTTP 方法 | 是 |
| `status` | HTTP 状态码 | 是 |
| `latency_ms` | 请求耗时（毫秒） | 是 |
| `success` | 请求是否成功 | 是 |
| `error_code` | 业务错误码（如 `OCR_ERROR`） | 否 |
| `error_type` | 错误类型（internal/validation/dependency） | 否 |
| `dependency` | 依赖名称（ocr/llm/redis/db） | 否 |
| `user_id_hash` | 仅保存哈希或脱敏用户标识 | 否 |

### 示例

```json
{
  "timestamp": "2026-01-28T12:00:00Z",
  "level": "INFO",
  "service": "backend",
  "env": "prod",
  "request_id": "6c1f7f6c-35e7-4c6a-8ce3-8c1e3f0dcb7a",
  "route": "/api/v1/analysis",
  "method": "POST",
  "status": 200,
  "latency_ms": 842,
  "success": true
}
```

## 埋点方案

### 请求入口与结束

- 位置：`backend/src/routes.rs`、`backend/src/middleware.rs`
- 方案：定制 `TraceLayer`，在请求开始/结束记录统一日志
- 输出字段：`request_id`、`route`、`method`、`status`、`latency_ms`、`success`
- `request_id` 同时写入响应头便于前后端联调

### 错误处理

- 位置：`backend/src/errors.rs`
- 方案：统一记录 `error_code` 与错误类型（系统/依赖/业务）
- 输出字段：`error_code`、`error_type`、`status`、`request_id`

### 依赖调用埋点

- 位置：`backend/src/services/*`
- 方案：为 OCR、LLM、存储、数据库调用增加开始/结束日志，记录依赖名与耗时
- 输出字段：`dependency`、`latency_ms`、`success`、`error_code`

## 失败率口径

### 接口失败率

定义：`(status >= 500 或 success = false) / total`

### 端到端失败率

定义：一次请求中，只要发生依赖失败（OCR/LLM/DB/Redis）或最终状态为失败，则计为端到端失败。

## SLS 接入方案

### 方案选择

- **推荐**：非阿里云主机采用 **LoongCollector 容器采集 Docker stdout**（Docker 29+ 需 3.2.4+）
- 备选：宿主机手动安装 Logtail（需匹配支持的 Linux 版本）

### 1. 创建资源

- 创建 SLS Project 与 Logstore
- 设置日志保留时间与索引字段

### 2. 采集配置

- 方案 A：LoongCollector 容器（推荐）
  - 拉取 LoongCollector 镜像（按 SLS 区域选择 `${region_id}`）
  - 启动 LoongCollector 容器并绑定 Machine Group
  - 选择模板 “Docker Stdout and Stderr - New Version”
- 方案 B：宿主机 Logtail
  - 手动安装 Logtail（Linux x86-64 版本匹配）
  - 采集 Docker stdout（默认日志路径 `/var/lib/docker/containers/<id>/<id>-json.log`）

**注意**：Logtail 只采集新增日志，配置下发前的旧日志不会被采集

### 3. 索引与查询

- 建议索引字段：`service`、`env`、`route`、`method`、`status`、`success`、`latency_ms`、`error_code`
- 用于看板与告警的核心查询：
  - 接口失败率（按 route 分组）
  - 端到端失败率（按时间聚合）
  - P50/P95/P99 延迟趋势

### 4. 告警配置

- 失败率 > 阈值（如 1%）触发
- 5xx 错误数量激增触发
- 依赖服务失败率异常触发

## 安全与合规

- 禁止记录密码、验证码、Token、完整手机号等敏感信息
- 用户标识使用哈希或脱敏字段

## 测试策略

### 本地验证

- 请求触发后确认 stdout 输出 JSON 日志
- 故意触发错误验证 `error_code` 与 `success=false`

### 线上验证

- SLS 能检索到日志字段
- 看板与告警触发符合预期

## 部署

### 环境配置

- `RUST_LOG` 控制日志级别
- `SERVICE_NAME`、`DEPLOY_ENV` 标识服务与环境

### 回滚方案

- 禁用 Logtail 采集或恢复到文本日志格式
- 保留现有业务功能不受影响

## 实施阶段

### 阶段 1：日志结构化与请求埋点

- [ ] 定制 `TraceLayer` 输出 JSON
- [ ] `request_id` 写入响应头与日志字段
- [ ] `AppError` 输出 `error_code` 与错误类型

### 阶段 2：依赖埋点与失败率口径

- [ ] OCR/LLM/DB/Redis 调用日志补齐
- [ ] 明确成功/失败字段与端到端口径

### 阶段 3：SLS 接入与看板告警

- [ ] 创建 Project/Logstore/索引
- [ ] 安装 Logtail 并采集 Docker 日志
- [ ] 配置看板与告警规则
