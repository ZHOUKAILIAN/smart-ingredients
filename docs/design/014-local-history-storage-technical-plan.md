# 014-本地历史记录存储技术方案

## 元数据

| 字段     | 值                               |
| -------- | -------------------------------- |
| 文档编号 | 014-local-history-storage        |
| 标题     | 本地历史记录存储技术方案         |
| 版本     | 1.0                              |
| 状态     | 草稿                             |
| 创建日期 | 2026-01-26                       |
| 更新日期 | 2026-01-26                       |
| 作者     | Codex                            |
| 关联需求 | 014-local-history-storage        |

## 概述

### 目的

实现未登录用户的本地历史记录保存与查看，并在登录后支持历史记录迁移和云端同步，提升即用性和跨设备体验。

### 范围

- 本地历史记录读写、展示、删除、容量限制。
- 登录态历史记录数据源切换。
- 登录后本地记录迁移到云端。
- 云端历史记录分页查询与删除能力沿用现有接口。

### 假设

- 未登录用户的分析请求仍会在后端创建分析记录（`analyses` 表），仅 `user_id` 为 NULL。
- 分析结果可从后端 API 获取，前端可缓存到本地历史记录。
- 已有登录、分析、历史 API 基础可复用。

## 架构设计

### 高层架构

前端负责本地历史记录缓存与展示，后端负责云端历史记录归属与查询。登录后由前端触发迁移，后端执行批量绑定 `user_id`。

### 组件图

- 前端
  - `local_history` 工具模块：localStorage 读写与容量控制
  - `HistoryPage`：根据登录态读取本地或云端
  - `LoginPage`：登录成功后触发迁移提示与操作
- 后端
  - `users/history` 路由：已有列表与删除
  - `users/history/batch` 路由：新增迁移绑定接口

### 技术栈

| 组件   | 技术          | 选择理由                 |
| ------ | ------------- | ------------------------ |
| 前端   | Leptos + Web API | 现有框架，适合 localStorage 操作 |
| 后端   | Axum + SQLx    | 现有接口与数据库层可复用 |
| 存储   | localStorage + PostgreSQL | 本地快速访问 + 云端持久化 |

## 数据模型

### 实体

- 本地历史记录：用于未登录用户的本地缓存。
- 云端历史记录：`analyses` 表中 `user_id` 归属的记录。

### 模式

```typescript
interface LocalHistoryItem {
  id: string;              // 分析ID
  timestamp: number;       // 时间戳
  health_score: number;    // 健康评分
  summary: string;         // 分析摘要
  result: AnalysisResult;  // 完整分析结果
}
```

```sql
-- 既有 analyses 表，仅新增使用 user_id 归属
ALTER TABLE analyses
  ADD COLUMN IF NOT EXISTS user_id UUID REFERENCES users(id) ON DELETE SET NULL;
```

### 数据流

1) 分析完成 → 前端拿到 `AnalysisResponse` → 未登录则写入 localStorage。
2) 历史页加载 → 根据登录态切换数据源（本地 or 云端）。
3) 登录成功 → 前端检测本地历史 → 用户确认迁移 → 后端绑定 `user_id` → 清空本地记录。

## API 设计

### 接口列表

| 方法   | 路径                          | 描述                 | 请求                          | 响应                         |
| ------ | ----------------------------- | -------------------- | ----------------------------- | ---------------------------- |
| GET    | `/api/v1/users/history`       | 获取用户历史记录     | `page`, `limit`               | `HistoryResponse`            |
| DELETE | `/api/v1/users/history/:id`   | 删除单条历史记录     | -                             | `{ success: true }`          |
| DELETE | `/api/v1/users/history`       | 批量删除历史记录     | `BatchDeleteRequest`          | `{ success: true }`          |
| POST   | `/api/v1/users/history/batch` | 迁移本地历史到云端   | `LocalHistoryMigrateRequest`  | `LocalHistoryMigrateResponse`|

### 数据结构

```rust
// 新增请求结构（shared）
pub struct LocalHistoryMigrateRequest {
    pub ids: Vec<Uuid>,
}

pub struct LocalHistoryMigrateResponse {
    pub migrated: i64,
    pub skipped: i64,
    pub total_after: i64,
}
```

- 迁移时仅绑定 `user_id IS NULL` 的记录，已归属或不存在的记录计为 `skipped`。
- 超出云端容量限制（500）时，后端返回 `total_after`，前端可提示并引导用户删除最旧记录。

## 安全设计

### 认证

- 迁移与云端历史接口均需要用户登录 token。

### 授权

- 仅允许操作当前登录用户的历史记录。

### 数据保护

- 本地历史记录不存用户敏感信息（手机号等）。
- 云端历史记录仍按现有鉴权机制访问。

## 错误处理

### 错误码

| 错误码              | 消息                 | 描述                         |
| ------------------- | -------------------- | ---------------------------- |
| `history_migrate_fail` | 迁移失败            | 批量绑定 user_id 失败        |
| `history_limit_exceeded` | 超出历史记录上限 | 云端超出 500 条限制          |
| `local_storage_error` | 本地存储失败        | localStorage 写入或读取失败 |

### 错误响应格式

沿用现有后端错误响应结构（`AppError`）。前端使用 Toast 展示错误信息。

## 性能考虑

### 缓存策略

- localStorage 作为本地缓存，读写直接在浏览器端完成。

### 优化

- 历史列表读取时只序列化必要字段，避免大对象反复解析。
- 本地历史最多 50 条，避免过大 JSON 影响性能。

### 监控

- 迁移 API 记录成功/失败数量与耗时日志。

## 测试策略

### 单元测试

- localStorage 读写、删除、容量上限逻辑。

### 集成测试

- 登录态切换历史数据源。
- 迁移接口绑定逻辑与边界（空 ids、已归属记录）。

### E2E 测试

- 未登录分析 → 历史页展示 → 点击查看详情。
- 登录后迁移提示 → 迁移成功 → 本地清空 → 云端可见。

## 部署

### 环境要求

- 无新增外部依赖。

### 配置

- 云端历史容量上限（默认 500）可在服务端配置。

### 回滚计划

- 回滚前端本地历史功能开关。
- 后端新增接口可保持向后兼容，必要时仅停止调用。

## 实施阶段

### 阶段 1：前端本地历史

- [ ] 新增 localStorage 工具模块与数据结构
- [ ] 未登录分析完成时保存本地历史
- [ ] 历史页展示本地记录并支持删除/查看

### 阶段 2：迁移与云端绑定

- [ ] 新增迁移 API 与数据库绑定逻辑
- [ ] 登录后迁移提示与执行
- [ ] 云端容量限制提示与处理

## 风险与缓解

| 风险                     | 影响 | 可能性 | 缓解措施                                       |
| ------------------------ | ---- | ------ | ---------------------------------------------- |
| 本地记录与云端记录不一致 | 中   | 中     | 迁移仅绑定已有分析记录，失败保留本地记录       |
| localStorage 写入失败    | 低   | 中     | Toast 提示，主流程不中断                       |
| 云端容量超限             | 中   | 低     | 迁移时提示并引导删除最旧记录                   |

## 待解决问题

| 问题                     | 影响 | 负责人 | 状态 |
| ------------------------ | ---- | ------ | ---- |
| 迁移超限时是否自动清理    | 中   | 待定   | 开放 |
| 本地记录点击查看的交互细节 | 低   | 待定   | 开放 |

## 参考资料

- `docs/requirements/014-local-history-storage-requirements.md`
- `docs/requirements/012-user-system-requirements.md`
- `docs/requirements/013-tab-navigation-and-onboarding-requirements.md`

---

## 变更记录

| 版本 | 日期       | 作者  | 描述     |
| ---- | ---------- | ----- | -------- |
| 1.0  | 2026-01-26 | Codex | 初始版本 |
