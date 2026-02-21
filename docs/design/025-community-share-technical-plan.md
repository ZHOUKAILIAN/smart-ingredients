# 025-社区分享与浏览技术方案

## 元数据

| 字段     | 值                       |
| -------- | ------------------------ |
| 文档编号 | 025-community-share      |
| 标题     | 社区分享与浏览技术方案   |
| 版本     | 1.2                      |
| 状态     | 草稿                     |
| 创建日期 | 2026-02-19               |
| 更新日期 | 2026-02-20               |
| 作者     | Claude Code              |
| 关联需求 | 025-community-share      |

## 概述

### 目的
在不引入复杂互动能力的前提下，实现社区分享与浏览的 MVP，支持匿名发布、公开浏览、可删除，并与现有分析与分享图片逻辑复用。

### 范围
- 前端：社区 Tab、列表/详情页面、分享入口、分享状态标记
- 后端：社区帖子增删查接口、存储结构化数据与可选图片
- 数据库：新增社区帖子表与索引

不包含评论、点赞、内容审核与推荐排序。

### 假设
- 分析结果中已有健康评分与摘要文本
- 前端已具备导出分享图片能力（020-export-share-image）
- 用户系统存在但非必需，匿名发布可通过 token 控制删除权限

## 架构设计

### 高层架构

```
Frontend (Leptos/Tauri)
  ├─ 结果页/历史详情页：分享入口
  ├─ 社区 Tab：列表/详情
  └─ 结果卡片渲染组件
        ↓
Backend (Axum)
  ├─ /api/v1/community/posts
  ├─ 图片存储(复用 uploads)
  └─ SQLx + Postgres
        ↓
Postgres: community_posts
```

### 组件图
- 前端
  - CommunityListPage / CommunityDetailPage
  - ShareToCommunityButton
  - ResultCard (支持图片或结构化渲染)
- 后端
  - community::handlers
  - community::service
  - community::repo
  - storage::store_image (复用或封装为 store_share_image)

### 技术栈

| 组件   | 技术         | 选择理由                 |
| ------ | ------------ | ------------------------ |
| 前端   | Leptos/Tauri | 现有技术栈复用           |
| 后端   | Axum + SQLx  | 现有 API 与数据库规范    |
| 存储   | 本地 uploads | MVP 简化，后续可替换 OSS |

## 数据模型

### 实体

#### CommunityPost（社区帖子）

| 字段 | 类型 | 说明 | 约束 |
|------|------|------|------|
| id | UUID | 主键 | PRIMARY KEY, DEFAULT gen_random_uuid() |
| author_type | VARCHAR(20) | 作者类型 | NOT NULL, CHECK IN ('anonymous','user') |
| user_id | UUID | 用户 ID | NULLABLE |
| share_token_hash | VARCHAR(128) | 匿名删除 token 哈希 | NULLABLE |
| summary_text | TEXT | 摘要 | NOT NULL |
| health_score | INTEGER | 健康评分 | CHECK (0-100) |
| ingredients_raw | TEXT | 配料表原文 | NOT NULL |
| card_payload | JSONB | 结构化卡片数据 | NOT NULL |
| card_image_url | VARCHAR(512) | 卡片图片 URL | NULLABLE |
| source_analysis_id | UUID | 关联分析记录 | NULLABLE |
| created_at | TIMESTAMP | 创建时间 | DEFAULT NOW() |
| updated_at | TIMESTAMP | 更新时间 | DEFAULT NOW() |

### 模式

```sql
CREATE TABLE community_posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_type VARCHAR(20) NOT NULL CHECK (author_type IN ('anonymous','user')),
    user_id UUID,
    share_token_hash VARCHAR(128),
    summary_text TEXT NOT NULL,
    health_score INTEGER CHECK (health_score >= 0 AND health_score <= 100),
    ingredients_raw TEXT NOT NULL,
    card_payload JSONB NOT NULL,
    card_image_url VARCHAR(512),
    source_analysis_id UUID,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_community_posts_created_at ON community_posts(created_at DESC);
CREATE INDEX idx_community_posts_user_id ON community_posts(user_id);
```

### 数据流

1) 发布
- Frontend 组装 card_payload + 摘要 + 配料表原文
- 可选生成卡片图片并上传
- Backend 持久化结构化数据 + 可选图片 URL

2) 列表
- Frontend 调用列表接口获取分页数据
- 若无图片，则用 summary_text + health_score 渲染简化卡片

3) 详情
- Frontend 获取完整详情数据
- 优先显示 card_image_url，否则用 card_payload 渲染

4) 删除
- 匿名：携带 share_token 校验其哈希
- 登录：校验 user_id

### 交互规则补充
- 已分享的分析结果在结果页/历史详情页不显示分享按钮（可显示“已分享”文本）
- 删除入口仅在社区列表与社区详情页展示
- 社区列表请求只在分页参数变化时触发，避免因 UI 状态变更重复请求
- 社区列表与社区详情页的时间统一格式化为 `YYYY-MM-DD HH:mm:ss`（与历史页一致）
- 社区页面标题不显示“社区”字样

## API 设计

### 接口列表

| 方法 | 路径 | 描述 | 请求 | 响应 |
| ---- | ---- | ---- | ---- | ---- |
| POST | /api/v1/community/posts | 创建帖子 | multipart/form-data | CommunityPostCreated |
| GET | /api/v1/community/posts | 列表 | query | CommunityPostList |
| GET | /api/v1/community/posts/{id} | 详情 | - | CommunityPostDetail |
| DELETE | /api/v1/community/posts/{id} | 删除 | header/body | DeleteResult |

### 数据结构

#### Create Payload（字段示例）

- Content-Type: multipart/form-data
- Field: payload (JSON string)
- Field: card_image (optional file)

```json
{
  "author_type": "anonymous",
  "share_token": "client-generated-token",
  "source_analysis_id": "uuid",
  "summary_text": "配料以茶叶提取物为主，整体风险较低。",
  "health_score": 85,
  "ingredients_raw": "水、乌龙茶、白砂糖...",
  "card_payload": {
    "health_score": 85,
    "summary": "配料以茶叶提取物为主，整体风险较低。",
    "key_ingredients": [
      {"name": "乌龙茶", "risk_level": "low"}
    ],
    "recommendation": "建议适量饮用"
  }
}
```

#### CommunityPostCreated

```json
{
  "id": "uuid",
  "created_at": "2026-02-19T12:00:00Z",
  "card_image_url": "/uploads/community/xxx.png"
}
```

#### CommunityPostList

```json
{
  "total": 120,
  "page": 1,
  "limit": 20,
  "items": [
    {
      "id": "uuid",
      "summary_text": "配料以茶叶提取物为主，整体风险较低。",
      "health_score": 85,
      "card_image_url": "/uploads/community/xxx.png",
      "author_label": "匿名用户",
      "created_at": "2026-02-19T12:00:00Z"
    }
  ]
}
```

#### CommunityPostDetail

```json
{
  "id": "uuid",
  "summary_text": "配料以茶叶提取物为主，整体风险较低。",
  "health_score": 85,
  "ingredients_raw": "水、乌龙茶、白砂糖...",
  "card_payload": {
    "health_score": 85,
    "summary": "配料以茶叶提取物为主，整体风险较低。",
    "key_ingredients": [
      {"name": "乌龙茶", "risk_level": "low"}
    ],
    "recommendation": "建议适量饮用"
  },
  "card_image_url": null,
  "author_label": "匿名用户",
  "created_at": "2026-02-19T12:00:00Z"
}
```

#### DeleteResult

```json
{ "deleted": true }
```

## 安全设计

### 认证
- 浏览与发布无需登录
- 登录用户发布时绑定 user_id（若已登录）

### 授权
- 删除时要求 share_token 或 user_id 匹配
- share_token 仅存哈希，使用固定盐或 HMAC

### 数据保护
- 限制 card_image 大小与类型
- 仅保存必要字段，避免存入敏感个人信息

## 错误处理

### 错误码

| 错误码 | 消息 | 描述 |
| ------ | ---- | ---- |
| 400 | invalid_payload | 缺少必填字段或格式不正确 |
| 401 | delete_forbidden | 删除权限不足 |
| 404 | not_found | 帖子不存在 |
| 413 | image_too_large | 图片过大 |
| 500 | internal_error | 服务器错误 |

### 错误响应格式

```json
{ "error": "invalid_payload", "message": "summary_text is required" }
```

## 性能考虑

### 缓存策略
- 暂不引入缓存，后续可对列表接口加 CDN 或 Redis 缓存

### 优化
- 列表仅返回必要字段
- 图片尺寸限制与压缩

### 监控
- 记录创建与删除的日志
- 统计列表请求响应时间

## 测试策略

### 单元测试
- share_token 哈希与校验
- payload 校验与默认值处理

### 集成测试
- 创建/列表/详情/删除接口
- 匿名删除与登录删除权限

### E2E 测试
- 结果页分享 → 社区列表可见 → 详情渲染 → 删除成功

## 部署

### 环境要求
- Postgres 需要新增迁移
- uploads 目录需可写

### 配置
- MAX_COMMUNITY_IMAGE_SIZE
- COMMUNITY_LIST_LIMIT_DEFAULT

### 回滚计划
- 迁移回滚：删除 community_posts 表
- 前端隐藏社区 Tab

## 实施阶段

### 阶段 1：数据库与后端 API
- [ ] 创建 community_posts 迁移
- [ ] 实现创建/列表/详情/删除接口
- [ ] 实现 share_token 哈希与校验

### 阶段 2：前端 UI
- [ ] 新增社区 Tab 页面
- [ ] 结果页与历史详情页分享入口
- [ ] 详情页结果卡片渲染
- [ ] 已分享隐藏分享按钮，删除入口位于社区列表/详情

### 阶段 3：联调与文档
- [ ] 补充 api-reference
- [ ] 端到端流程验证

## 风险与缓解

| 风险 | 影响 | 可能性 | 缓解措施 |
| ---- | ---- | ------ | -------- |
| 图片存储增长 | 中 | 中 | 限制尺寸与后续清理策略 |
| 匿名滥用 | 中 | 中 | 保留删除能力，后续可加审核 |
| 列表渲染性能 | 中 | 低 | 列表字段精简与分页 |

## 待解决问题

| 问题 | 影响 | 负责人 | 状态 |
| ---- | ---- | ------ | ---- |
| share_token 存储位置（LocalStorage/本地文件） | 中 | 待定 | 开放 |
| card_payload 字段最小集合 | 低 | 待定 | 开放 |
| 图片存储清理周期 | 中 | 待定 | 开放 |

## 参考资料

- docs/requirements/025-community-share-requirements.md
- docs/design/020-export-share-image-technical-plan.md
- docs/api/api-reference.md

---

## 变更记录

| 版本 | 日期 | 作者 | 描述 |
| ---- | ---- | ---- | ---- |
| 1.0 | 2026-02-19 | Claude Code | 初始版本 |
| 1.1 | 2026-02-19 | Claude Code | 调整分享/删除入口交互规则 |
