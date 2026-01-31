# 018-账号密码登录技术方案

## 元数据

| 字段     | 值                           |
| -------- | ---------------------------- |
| 文档编号 | 018-001                      |
| 标题     | 账号密码登录技术方案         |
| 版本     | 1.0                          |
| 状态     | 草稿                         |
| 创建日期 | 2026-01-31                   |
| 更新日期 | 2026-01-31                   |
| 作者     | Codex                        |
| 关联需求 | 018-账号密码登录需求文档     |

## 概述

### 目的

使用“账号 + 密码”的本地登录体系替代短信/邮箱登录，降低外部资质依赖，并保持现有 token、偏好同步与历史迁移流程不变。

### 范围

包含：
- 账号注册/登录接口
- 密码安全哈希与校验
- 登录失败限流与锁定
- 前端登录/注册界面

不包含：
- 密码找回
- 第三方登录
- 账号绑定

### 假设

- Redis 可用于登录失败限流
- PostgreSQL 作为用户数据存储
- 现有 token 机制（access/refresh）保持不变

## 架构设计

### 高层架构

前端提供账号注册与登录入口。后端新增注册/登录接口，使用安全密码哈希存储账号密码，登录成功后签发 JWT 与 refresh token。Redis 用于失败次数与锁定。数据库新增 username 与 password_hash。

### 组件

- 前端（Leptos + Tauri）：登录页、注册入口、个人中心展示
- 后端（Axum）：认证服务、用户服务
- 数据层：PostgreSQL（users）、Redis（限流）
- 共享层：shared crate 类型定义

## 数据模型

### 用户表扩展

新增字段（建议）：
- `username` (text, not null)
- `username_normalized` (text, unique, not null)
- `password_hash` (text, not null)
- `password_updated_at` (timestamptz, nullable)

说明：
- `username_normalized` 统一小写，用于唯一性判断
- `password_hash` 存 Argon2id/Bcrypt 的标准编码字符串

### 历史记录关联

- `analyses` 表保留 `user_id`（nullable）
- 登录用户产生的新分析记录应写入 `user_id`
- 历史记录查询接口仅返回 `user_id = 当前用户` 的记录

### Redis 键设计

- `auth:login:attempts:{username_hash}` -> attempts (ttl 15 min)
- `auth:login:lock:{username_hash}` -> 1 (ttl 15 min)
- `auth:refresh:{refresh_token}` -> user_id (ttl 60 days)

`username_hash` 建议使用 HMAC-SHA256( normalized_username, LOGIN_HASH_KEY )，避免明文写入 Redis 键。

## API 设计

### 接口列表

| 方法 | 路径 | 描述 | 请求 | 响应 |
| ---- | ---- | ---- | ---- | ---- |
| POST | `/api/v1/auth/register` | 账号注册 | `{ username, password }` | `AuthResponse` |
| POST | `/api/v1/auth/login` | 账号登录 | `{ username, password }` | `AuthResponse` |
| POST | `/api/v1/auth/refresh` | 刷新 token | `RefreshRequest` | `AuthResponse` |
| POST | `/api/v1/auth/logout` | 退出登录 | `LogoutRequest` | `Empty` |

### 数据结构调整

`UserProfile` 统一为：

```
struct UserProfile {
  id: Uuid,
  login_id: String,
  created_at: String,
  analysis_count: i64,
}
```

前端展示 `login_id`，不再展示手机号。

## 密码安全设计

- 密码哈希：优先 Argon2id（可选 bcrypt）
- 每个用户独立随机盐，存储在 `password_hash` 中
- 禁止记录明文密码或哈希原文日志
- 密码强度：长度 >= 6（不要求复杂度）

建议配置项：
- `LOGIN_HASH_KEY`（用于 username_hash）
- `AUTH_PASSWORD_ALGO`（argon2id/bcrypt）
- `AUTH_PASSWORD_COST`（用于调整哈希成本）

## 业务流程

### 注册流程

1. 前端提交 `username + password`
2. 后端规范化 username（trim + lowercase）
3. 校验格式与唯一性
4. 生成密码哈希，写入 users 表
5. 签发 token 并返回 `AuthResponse`

### 登录流程

1. 前端提交 `username + password`
2. 规范化 username，并校验是否被锁定
3. 校验密码哈希
4. 登录失败：记录 attempts，并在阈值后锁定
5. 登录成功：重置 attempts，签发 token

### 历史关联流程

1. 用户登录后，新分析写入 `analyses.user_id`
2. 历史列表查询仅返回当前用户记录
3. 退出登录后仍可查看本地历史（不影响已登录历史）
4. 登录成功后触发“本地历史迁移到云端”的提示与迁移流程（沿用现有逻辑）

## 前端改造

- 登录页改为“账号 + 密码”输入
- 提供“注册账号”入口
- 登录/注册成功后流程保持一致
- 移除手机号/邮箱登录入口与文案

## 账号格式规范

- 允许字符：`a-z`、`A-Z`、`0-9`、`_`
- 长度范围：4-20
- 大小写不敏感（统一转小写后判重）
- 不允许空格与特殊符号

## 迁移策略

- 移除短信登录接口与手机号相关字段
- 用户数据不做强制迁移（历史与偏好保持）
- 账号注册后即可使用全部功能

## 测试策略

- 单元测试：密码哈希与校验
- 集成测试：注册/登录/刷新 token
- 限流测试：多次失败触发锁定
- 前端测试：登录/注册 UI 与错误提示

## 风险与对策

- 忘记密码无法找回
  - 登录页与注册页明确提示
- 弱密码风险
  - 最低长度限制 + 登录失败限流

## 里程碑

1. 数据模型与配置更新
2. 后端注册/登录接口
3. 前端登录/注册界面
4. 联调与验收
