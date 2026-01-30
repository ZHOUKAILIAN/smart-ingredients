# 016-服务端稳定性与可观测性技术方案

## 元数据

| 字段     | 值                                         |
| -------- | ------------------------------------------ |
| 文档编号 | 016-server-stability-observability         |
| 标题     | 服务端稳定性与可观测性技术方案             |
| 版本     | 1.2                                        |
| 状态     | 草稿                                       |
| 创建日期 | 2026-01-28                                 |
| 更新日期 | 2026-01-29                                 |
| 作者     | Codex                                      |
| 关联需求 | 016-server-stability-observability         |

## 概述

### 目的

为服务端建立结构化日志、失败率指标与可视化能力，并通过自建 Grafana + Loki 实现异常可见、可追踪、可统计（前期不配置告警）。

### 范围

- 后端日志结构化输出（JSON）
- 关键请求与依赖调用埋点
- Promtail 采集日志 → Loki 存储检索 → Grafana 展示
- 失败率（接口级）统计口径（基于日志）

### 假设

- 服务以 Docker 方式单机部署，使用 Docker Compose 管理。
- 后端已使用 `tracing`，可扩展为 JSON 日志。
- 日志优先输出到 stdout，由 Promtail 从 Docker 日志文件采集。

## 架构设计

### 高层架构

应用日志（JSON） → Promtail 采集 → Loki → Grafana 可视化

### 组件图

- 后端
  - 请求追踪与结构化日志（`tracing`）
  - 错误日志与依赖调用日志
- 采集
  - Promtail（宿主机或容器）
- 平台
  - Loki（索引、检索）
  - Grafana（看板）

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
| `error_message` | 错误摘要信息 | 否 |
| `stacktrace` | 错误堆栈摘要 | 否 |
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

### 错误日志示例

```json
{
  "timestamp": "2026-01-29T09:21:17Z",
  "level": "ERROR",
  "service": "backend",
  "env": "prod",
  "request_id": "9c2f8d4e-3d7a-4a2f-9a55-1cf2f6e6bcb2",
  "route": "/api/v1/analysis",
  "method": "POST",
  "status": 500,
  "latency_ms": 1874,
  "success": false,
  "error_code": "LLM_TIMEOUT",
  "error_type": "dependency",
  "error_message": "LLM request timeout after 1500ms",
  "stacktrace": "services/llm.rs:132 -> AppError::timeout"
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

## Grafana + Loki + Promtail 接入方案（自建）

### 1. 部署方式

- 单机 Docker Compose 部署 Grafana、Loki、Promtail
- Loki/Grafana 数据目录挂载到宿主机，防止重装丢失

### 1.1 简要部署步骤（单机）

- 准备 `docker-compose.yml`、`loki-config.yml`、`promtail-config.yml`（见 `docs/deployment/monitoring/` 示例）
- Promtail 采集路径指向 Docker 日志文件（如 `/var/lib/docker/containers/*/*-json.log`）
- Promtail 容器需只读挂载 `/var/lib/docker/containers` 与 `/var/run/docker.sock`
- 执行 `docker compose -f docs/deployment/monitoring/docker-compose.monitoring.yml up -d`
- 访问 `http://<server_ip>:3001`，在 Grafana 添加 Loki 数据源（避免与后端 3000 端口冲突）

### 2. Promtail 采集配置

**stdout 采集（当前示例）**：
- 直接使用容器日志路径（由 `docker inspect` 获取）
- 容器重建后需要更新 `__path__`

**示例（当前配置）**：
```yaml
scrape_configs:
  - job_name: backend-stdout
    static_configs:
      - targets: [localhost]
        labels:
          job: smart-ingredients
          container: smart-ingredients-backend
          service: backend
          __path__: /var/lib/docker/containers/<container_id>/<container_id>-json.log
    pipeline_stages:
      - docker: {}
```

> 说明：`__path__` 需要和实际容器的日志路径一致。

**可选：自动发现容器（进阶）**：
```yaml
scrape_configs:
  - job_name: docker
    docker_sd_configs:
      - host: unix:///var/run/docker.sock
        refresh_interval: 5s
    relabel_configs:
      - source_labels: [__meta_docker_container_name]
        regex: /smart-ingredients-backend
        action: keep
      - source_labels: [__meta_docker_container_log_path]
        target_label: __path__
      - source_labels: [__meta_docker_container_name]
        target_label: container
    pipeline_stages:
      - docker: {}
```

### 3. Loki 索引与查询

- 建议索引字段：`service`、`env`、`route`、`method`、`status`、`success`、`error_code`
- 核心查询：
- 错误数统计：`sum(count_over_time({container="smart-ingredients-backend"} |= "ERROR" [5m]))`
- 5xx 统计：`sum(count_over_time({container="smart-ingredients-backend"} |~ "status=5.." [5m]))`
- 接口错误 TopN：`topk(10, sum by (route) (count_over_time({container="smart-ingredients-backend"} |= "ERROR" [1h])))`

### 4. Grafana 面板（前期）

- 面板：错误数趋势、5xx 趋势、接口错误 TopN
- 前期不配置告警规则与通知通道

### 4.1 快速查询示例

- 仅查看错误日志：`{container="smart-ingredients-backend"} |= "ERROR"`
- 统计 5xx 次数：`sum(count_over_time({container="smart-ingredients-backend"} |~ "status=5.." [5m]))`

## 安全与合规

- 禁止记录密码、验证码、Token、完整手机号等敏感信息
- 用户标识使用哈希或脱敏字段

## 测试策略

### 本地验证

- 请求触发后确认 Docker 日志输出 JSON 日志
- 故意触发错误验证 `error_code` 与 `success=false`

### 线上验证

- Grafana 可检索到日志字段
- 看板展示符合预期

## 部署

### 环境配置

- `RUST_LOG` 控制日志级别
- `SERVICE_NAME`、`DEPLOY_ENV` 标识服务与环境

### 回滚方案

- 禁用 Promtail 采集或恢复到文本日志格式
- 保留现有业务功能不受影响

## 实施阶段

### 阶段 1：日志结构化与请求埋点

- [ ] 定制 `TraceLayer` 输出 JSON
- [ ] `request_id` 写入响应头与日志字段
- [ ] `AppError` 输出 `error_code` 与错误类型

### 阶段 2：依赖埋点与失败率口径

- [ ] OCR/LLM/DB/Redis 调用日志补齐
- [ ] 明确成功/失败字段与端到端口径

### 阶段 3：自建接入与看板

- [ ] 部署 Grafana/Loki/Promtail
- [ ] 配置采集路径与标签
- [ ] 配置看板
