# 028-移除遗留 Result 页面需求文档

## 元数据

| 字段 | 值 |
|------|-----|
| 文档编号 | 028-remove-result-page |
| 标题 | 移除遗留 Result 页面需求文档 |
| 版本 | 1.0 |
| 状态 | 草稿 |
| 创建日期 | 2026-02-21 |
| 更新日期 | 2026-02-21 |
| 作者 | Codex |

## 概述

### 目的
移除不再使用的 `/result` 页面与路由，避免与当前分析流程（`/summary` + `/detail`）重复或造成入口混乱。

### 范围
**包含**：
- 删除 `/result` 路由与页面模块
- 移除相关导出/引用

**不包含**：
- 其他分析流程页面改动
- UI 风格调整

### 利益相关者
- 终端用户：避免进入遗留页面
- 前端开发：简化路由与维护成本

## 功能需求

### 需求 1：移除 /result 路由
- **编号**: FR-028-001
- **优先级**: 高
- **描述**: 从路由中移除 `/result`，不再提供该页面入口。
- **验收标准**:
  - [ ] 路由配置不包含 `/result`

### 需求 2：移除 Result 页面模块
- **编号**: FR-028-002
- **优先级**: 高
- **描述**: 删除 `ResultPage` 页面文件与导出。
- **验收标准**:
  - [ ] `frontend/src/pages/result.rs` 删除
  - [ ] `frontend/src/pages/mod.rs` 不再引用 `result`
  - [ ] `frontend/src/lib.rs` 不再导入或使用 `ResultPage`

### 需求 3：构建通过
- **编号**: FR-028-003
- **优先级**: 高
- **描述**: 删除后项目构建与测试通过。
- **验收标准**:
  - [ ] `cargo test -p smart-ingredients-app` 通过
  - [ ] `cargo check -p smart-ingredients-app` 通过

## 非功能需求

### 兼容性
- 不引入新依赖
- 仅移除遗留页面

## 依赖关系

### 内部依赖
- `docs/requirements/005-ui-optimization-requirements.md`
- `docs/requirements/007-figma-ui-redesign-requirements.md`

## 成功指标

- `/result` 不再可用且无引用残留
- 构建与测试通过

---

## 变更记录

| 版本 | 日期 | 作者 | 描述 |
|------|------|------|------|
| 1.0 | 2026-02-21 | Codex | 初始版本 |
