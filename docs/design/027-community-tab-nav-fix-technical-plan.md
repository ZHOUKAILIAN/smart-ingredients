# 027-社区 Tab 导航补全技术方案

## 元数据

| 字段     | 值 |
| -------- | --- |
| 文档编号 | 027-community-tab-nav-fix |
| 标题     | 社区 Tab 导航补全技术方案 |
| 版本     | 1.0 |
| 状态     | 草稿 |
| 创建日期 | 2026-02-21 |
| 更新日期 | 2026-02-21 |
| 作者     | Codex |
| 关联需求 | 027-community-tab-nav-fix |

## 概述

### 目的
补全底部导航对社区 Tab 的路由识别与渲染逻辑，确保编译通过并与社区需求一致。

### 范围
- 前端 `BottomNav` 组件逻辑补全
- 不涉及样式、页面结构与后端变更

### 假设
- `TabRoute::Community` 已存在并配置路径与文案
- 社区页面路由为 `/community` 与 `/community/*`

## 架构设计

### 高层架构
仅修改前端导航逻辑，不改变现有页面结构。

### 组件图
- `BottomNav`：Tab 列表与状态
- `TabRoute`：路径、文案、图标

## 实施方案

### 代码改动点
文件：`frontend/src/components/bottom_nav.rs`

1) 路由识别
- `tab_for_path` 增加 `/community` 与 `/community/*` 分支

2) 路径记忆
- 记录与读取 `state.last_community_path`

3) Tab 列表
- `For` 列表加入 `TabRoute::Community`
- 顺序为“首页 / 历史 / 社区 / 我的”

4) 图标渲染
- 引入 `IconCommunity`
- `match tab` 增加 `TabRoute::Community`

## 测试策略

### 单元测试
- 若能执行测试，新增针对 `tab_for_path` 的路径映射测试（覆盖 `/community` 与 `/community/{id}`）。

### 构建验证
- 运行 `cargo build` 确认 `frontend` 编译通过。

## 风险与缓解

| 风险 | 影响 | 可能性 | 缓解措施 |
| ---- | ---- | ------ | -------- |
| 仅修复编译但遗漏社区路径记忆 | 中 | 低 | 覆盖 last_community_path 读写 |

## 参考资料

- `docs/requirements/025-community-share-requirements.md`

---

## 变更记录

| 版本 | 日期 | 作者 | 描述 |
| ---- | ---- | ---- | ---- |
| 1.0 | 2026-02-21 | Codex | 初始版本 |
