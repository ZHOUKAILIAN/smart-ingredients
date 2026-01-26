# 012-用户体系技术方案

## 元数据

| 字段     | 值                     |
| -------- | ---------------------- |
| 文档编号 | 012-001                |
| 标题     | 用户体系技术方案       |
| 版本     | 1.0                    |
| 状态     | 草稿                   |
| 创建日期 | 2026-01-25             |
| 更新日期 | 2026-01-25             |
| 作者     | Codex                  |
| 关联需求 | 012-用户体系需求文档   |

## 概述

### 目的

为 Smart Ingredients 引入登录体系、偏好云端同步与分析历史管理能力，同时保留未登录用户的核心使用体验。

### 范围

包含：
- 手机号验证码登录与 JWT 鉴权
- 用户偏好云端存储与同步
- 登录用户分析历史列表与管理
- 个人中心基础信息展示与注销
- 登录态持久化与自动刷新

不包含：
- 社交登录
- 会员/付费体系
- 用户关系与社交功能

### 假设

- Redis 可用于验证码与频控缓存
- 短信服务提供商已确定（通过配置切换）
- 前端使用 LocalStorage 保存 token 与偏好缓存
- 现有分析表用于承载历史记录（新增用户关联字段）

## 架构设计

### 高层架构

前端通过手机号验证码完成登录，后端签发 JWT 并提供用户偏好、历史记录与个人中心接口。Redis 负责验证码与登录频控，PostgreSQL 存储用户与偏好数据。

### 组件图

- 前端（Leptos + Tauri）：登录页、偏好设置、历史列表、个人中心
- 后端（Axum）：认证服务、用户服务、历史记录服务
- 数据层：PostgreSQL（用户/偏好/分析记录）、Redis（验证码/频控）
- 共享层：shared crate 定义通用类型

### 技术栈

| 组件   | 技术               | 选择理由                         |
| ------ | ------------------ | -------------------------------- |
| 认证   | Axum + JWT         | 现有服务栈，便于统一中间件        |
| 缓存   | Redis              | 适合验证码 TTL 与限流            |
| 数据库 | PostgreSQL         | 已使用，便于扩展用户相关表        |
| 加密   | AES-256 + SHA256   | 满足手机号加密与索引检索需求      |
| 共享   | Rust shared crate  | 前后端共享类型，减少契约偏差      |

## 数据模型

### 实体

- User：用户账号（手机号）
- UserPreference：用户偏好配置（可扩展）
- Analysis：分析记录（新增 user_id 关联）

### 模式

users:
- id (uuid, pk)
- phone_encrypted (text, not null)
- phone_hash (text, unique, not null)
- created_at, updated_at, last_login_at

user_preferences:
- user_id (uuid, pk, fk users.id)
- preferences (jsonb, not null)
- updated_at

analyses:
- 新增 user_id (uuid, nullable, index)

缓存键（Redis）：
- sms:code:{phone_hash} -> code (ttl 5 min)
- sms:cooldown:{phone_hash} -> ttl 60s
- sms:attempts:{phone_hash} -> ttl 15 min
- sms:lock:{phone_hash} -> ttl 15 min
- auth:refresh:{session_id} -> user_id (ttl 60 days)

### 数据流

1. 前端请求验证码 -> 后端生成并发送 -> Redis 存 code + cooldown
2. 前端提交验证码 -> 校验 code/attempts -> 创建或更新用户 -> 签发 JWT
3. 前端保存 token -> 请求偏好/历史接口
4. 登录时对比本地偏好版本 -> 若本地更新则同步到云端
5. 分析完成写入 analyses，若已登录则附带 user_id

## API 设计

### 接口列表

| 方法 | 路径 | 描述 | 请求 | 响应 |
| ---- | ---- | ---- | ---- | ---- |
| POST | `/api/v1/auth/sms/send` | 发送验证码 | `SendSmsRequest` | `SendSmsResponse` |
| POST | `/api/v1/auth/sms/verify` | 验证码登录 | `VerifySmsRequest` | `AuthResponse` |
| POST | `/api/v1/auth/refresh` | 刷新 token | `RefreshRequest` | `AuthResponse` |
| POST | `/api/v1/auth/logout` | 退出登录 | `LogoutRequest` | `Empty` |
| GET  | `/api/v1/users/me` | 获取用户信息 | - | `UserProfile` |
| GET  | `/api/v1/users/preferences` | 获取偏好 | - | `UserPreferences` |
| PUT  | `/api/v1/users/preferences` | 更新偏好 | `UserPreferences` | `UserPreferences` |
| GET  | `/api/v1/users/history` | 历史记录列表 | `page/limit` | `HistoryResponse` |
| DELETE | `/api/v1/users/history/{id}` | 删除单条历史 | - | `Empty` |
| DELETE | `/api/v1/users/history` | 批量删除 | `BatchDeleteRequest` | `Empty` |
| DELETE | `/api/v1/users/me` | 注销账号 | - | `Empty` |

### 数据结构

```
struct AuthResponse {
  access_token: String,
  refresh_token: String,
  expires_in: i64,
  user: UserProfile,
}

struct UserProfile {
  id: String,
  phone_masked: String,
  created_at: String,
  analysis_count: i64,
}

struct UserPreferences {
  preferences: serde_json::Value,
  updated_at: String,
}
```

## 安全设计

### 认证

- JWT access token（30 天）
- refresh token（60 天，存 Redis，可撤销）
- token 刷新窗口：到期前 3 天自动刷新

### 授权

- `/users/*` 接口需要 JWT
- 历史记录查询与删除需校验 user_id

### 数据保护

- 手机号使用 AES-256 加密后存储
- phone_hash 用于唯一索引与查询
- 敏感操作（注销账号）二次确认 + 再次校验 token

## 错误处理

### 错误码

| 错误码 | 消息 | 描述 |
| ------ | ---- | ---- |
| SmsCooldown | 请稍后再试 | 60 秒内重复发送 |
| SmsCodeInvalid | 验证码错误 | 验证失败 |
| SmsCodeExpired | 验证码过期 | 超过 5 分钟 |
| SmsLocked | 验证码错误次数过多 | 15 分钟锁定 |
| Unauthorized | 未登录 | 缺少或无效 token |
| Forbidden | 无权限 | 访问非本人资源 |

### 错误响应格式

沿用统一错误响应结构（status + message + code）。

## 性能考虑

### 缓存策略

- Redis 存验证码与频控，避免频繁 DB 写入
- 前端偏好 LocalStorage 作为缓存层

### 优化

- analyses 增加 user_id 索引，历史记录分页查询
- 批量删除使用 `WHERE id = ANY($1)` 避免多次 round-trip

### 监控

- 监控验证码发送成功率与登录失败率
- 统计 refresh token 使用情况

## 测试策略

### 单元测试

- 验证码生成与校验逻辑
- token 生成与过期判断
- 偏好同步冲突处理

### 集成测试

- 登录流程（发送 -> 校验 -> token）
- 历史记录分页/删除
- 偏好读写接口

### E2E 测试

- 未登录用户访问历史 -> 登录引导
- 登录后同步偏好并展示历史

## 部署

### 环境要求

- PostgreSQL 16+
- Redis 7+
- 短信服务凭证

### 配置

- `JWT_SECRET`
- `JWT_ISSUER`
- `JWT_ACCESS_TTL_DAYS` (30)
- `JWT_REFRESH_TTL_DAYS` (60)
- `PHONE_ENC_KEY`
- `REDIS_URL`
- `SMS_PROVIDER_*`

### 回滚计划

- 保持旧接口兼容（历史接口仍可返回空列表）
- 可通过配置关闭登录入口以回退到匿名模式

## 实施阶段

### 阶段 1：基础认证与数据模型

- [ ] 新增 users / user_preferences 表
- [ ] analyses 增加 user_id 字段与索引
- [ ] 验证码发送与校验服务
- [ ] JWT 签发与中间件

### 阶段 2：偏好与历史

- [ ] 偏好读写接口与同步逻辑
- [ ] 历史记录列表/删除接口
- [ ] 前端历史页登录态控制

### 阶段 3：个人中心与注销

- [ ] 个人中心数据统计
- [ ] 退出登录/注销账号流程
- [ ] token 刷新与失效处理

## 风险与缓解

| 风险 | 影响 | 可能性 | 缓解措施 |
| ---- | ---- | ------ | -------- |
| 短信服务不稳定 | 高 | 中 | 预留多供应商切换与重试 |
| Token 泄漏 | 高 | 低 | refresh token 可撤销、短期 access token |
| 偏好同步冲突 | 中 | 中 | 使用 updated_at 比较，最近写入覆盖 |
| 历史数据量膨胀 | 中 | 中 | 分页 + 索引，必要时归档 |

## 待解决问题

| 问题 | 影响 | 负责人 | 状态 |
| ---- | ---- | ------ | ---- |
| 短信服务供应商选择 | 高 | 产品/后端 | 开放 |
| 手机号加密库选型 | 中 | 后端 | 开放 |
| 刷新 token 存储策略 | 中 | 后端 | 开放 |

## 参考资料

- `docs/requirements/012-user-system-requirements.md`
- `TECH-STACK.md`

---

## 变更记录

| 版本 | 日期 | 作者 | 描述 |
| ---- | ---- | ---- | ---- |
| 1.0 | 2026-01-25 | Codex | 初始版本 |
