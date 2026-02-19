# 024-人群定位交互与导航修复技术方案

## 元数据

| 字段     | 值                         |
| -------- | -------------------------- |
| 文档编号 | 024-preference-interaction |
| 标题     | 人群定位交互与导航修复技术方案 |
| 版本     | 1.0                        |
| 状态     | 草稿                       |
| 创建日期 | 2026-02-16                 |
| 更新日期 | 2026-02-16                 |
| 作者     | 小周                       |
| 关联需求 | 024-preference-interaction |

## 概述

### 目的
落地人群定位可跳过、默认偏好持久化、导航修复与页面紧凑化，确保首页/个人中心可达并避免 404。

### 范围
- 前端：人群定位页、首页（Capture）、底部导航、全局偏好初始化、样式调整
- 不涉及后端分析逻辑调整，仅在已登录用户时调用现有偏好更新接口

### 假设
- `services::update_preferences` 已可更新 `selection`
- `location.search` 可能不包含 `?`，需要显式拼接

## 架构设计

### 高层架构
仅前端改动，利用已有 AppState + LocalStorage 完成默认偏好与导航状态维护。

### 组件图
- CapturePage：移除强制跳转，保留 `view=scan` 逻辑
- OnboardingPage：保存/跳过后回首页
- BottomNav：修复 query 拼接与 last_home_path 记录
- App 初始化：补齐默认偏好与服务端同步

### 技术栈

| 组件 | 技术 | 选择理由 |
| ---- | ---- | -------- |
| 前端状态 | Leptos signals | 现有架构一致 |
| 偏好持久化 | LocalStorage + Preferences API | 复用现有能力 |

## 数据模型

### 实体
- `analysis_preference: Option<String>`（AppState）
- LocalStorage：`analysis_preference`

### 数据流
1. App 启动：`ensure_session` → `fetch_preferences`
2. 若偏好缺失：写入本地 `analysis_preference = normal`，并在已登录场景调用 `update_preferences`
3. Onboarding 保存/跳过：写入本地与（已登录）服务端 → 跳转首页

## API 设计

### 接口列表
| 方法 | 路径 | 描述 | 请求 | 响应 |
| ---- | ---- | ---- | ---- | ---- |
| PUT | `/api/v1/users/preferences` | 更新偏好 | `{ preferences: { selection: "normal" } }` | 200 |

### 数据结构
复用 `shared::UpdatePreferencesRequest` 与 `shared::UserPreferences`。

## 安全设计

### 认证
- 未登录：仅写入本地偏好，不调用 API
- 已登录：调用偏好更新接口，沿用现有认证

## 错误处理

- `update_preferences` 失败时展示 toast，不影响继续使用
- 默认偏好写入不应阻断页面渲染

## 性能考虑

- 仅增加一次轻量级偏好检查与本地写入
- 不新增重计算或频繁渲染

## 测试策略

### 手工验证
- 首次进入首页，不再强制跳转
- 未选择人群直接拍照分析，结果页显示“普通人群”
- Onboarding 保存/跳过后回首页
- 个人中心 → 首页不出现 `/view=scan` 404
- 个人中心在未设置人群时可进入

### 自动化
- 本次不新增自动化测试（保持最小改动）

## 部署

- 仅前端代码变更，无新增配置

## 实施阶段

### 阶段 1：偏好默认化与导航修复
- [ ] App 初始化补齐默认偏好
- [ ] CapturePage 移除强制跳转
- [ ] BottomNav 修复 query 拼接
- [ ] Onboarding 保存/跳过回首页

### 阶段 2：人群定位页紧凑化样式
- [ ] Onboarding 步骤区增加紧凑样式
- [ ] 样式验证与微调

## 风险与缓解

| 风险 | 影响 | 可能性 | 缓解措施 |
| ---- | ---- | ------ | -------- |
| 默认偏好写入触发多次请求 | 中 | 低 | 在 App 初始化处集中处理且仅在缺失时写入 |
| query 拼接变更影响其他路径 | 低 | 低 | 仅在 `search` 非空时加前缀 `?` |

## 待解决问题

| 问题 | 影响 | 负责人 | 状态 |
| ---- | ---- | ------ | ---- |
| `location.search` 是否含 `?` | 低 | 小周 | 已确认需兼容 |

## 参考资料

- `docs/requirements/024-preference-interaction-requirements.md`
- `docs/requirements/013-tab-navigation-and-onboarding-requirements.md`

---

## 变更记录

| 版本 | 日期 | 作者 | 描述 |
| ---- | ---- | ---- | ---- |
| 1.0 | 2026-02-16 | 小周 | 初始版本 |
