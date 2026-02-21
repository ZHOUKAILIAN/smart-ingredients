# 026-编译告警清理需求文档

## 元数据

| 字段 | 值 |
|------|-----|
| 文档编号 | 026-warning-cleanup |
| 标题 | 编译告警清理需求文档 |
| 版本 | 1.0 |
| 状态 | 草稿 |
| 创建日期 | 2026-02-20 |
| 更新日期 | 2026-02-20 |
| 作者 | Claude Code |

## 概述

### 目的
清理 Rust 项目中的 unused / deprecated 类编译告警，提升代码可维护性与编译质量，同时不改变现有业务行为。

### 范围
**包含**：
- backend 与 smart-ingredients-app 的所有 unused/deprecated 警告清理
- 相关代码结构调整（仅限消除告警所需）

**不包含**：
- 新功能开发
- 行为调整或 UI 交互改动
- 与告警无关的重构
- smart-ingredients-tauri 的构建问题修复（OUT_DIR 缺失导致的编译错误）

### 利益相关者
- 开发团队（减少噪音告警）
- 测试团队（更清晰的告警基线）
- 产品团队（保证功能不变）

## 功能需求

### 需求 1：消除 unused 告警
- **编号**: FR-001
- **优先级**: 高
- **描述**: 移除或修正 unused_imports、unused_variables、unused_mut、dead_code 等告警。
- **验收标准**:
  - [ ] cargo check -p backend 输出无 unused 类警告
  - [ ] cargo check -p smart-ingredients-app 输出无 unused 类警告

### 需求 2：消除 deprecated 告警
- **编号**: FR-002
- **优先级**: 高
- **描述**: 替换已弃用的 Leptos API（如 create_effect、create_memo、create_signal、MaybeSignal）为推荐写法。
- **验收标准**:
  - [ ] cargo check -p smart-ingredients-app 输出无 deprecated 类警告

### 需求 3：行为不变
- **编号**: FR-003
- **优先级**: 高
- **描述**: 清理告警不引入任何可见行为变更。
- **验收标准**:
  - [ ] 现有核心流程（分析 → 分享 → 社区浏览/删除）无行为差异

## 非功能需求

### 性能
- 编译时间不显著变慢

### 安全性
- 不引入新的权限或数据通道

### 可靠性
- 清理后构建稳定，可重复通过 cargo check

### 可扩展性
- 保持现有模块结构可持续扩展

## 用户故事

### 故事 1
- **作为** 开发者
- **我想要** 编译时不被告警干扰
- **以便** 更专注于真实问题

### 故事 2
- **作为** 维护者
- **我想要** 使用最新 API
- **以便** 降低未来升级成本

## 用例

### 用例 1：清理 unused/deprecated 告警
- **参与者**: 开发者
- **前置条件**: 可编译 backend 与 smart-ingredients-app
- **主流程**: 运行 cargo check → 逐类修复告警 → 再次检查无告警
- **后置条件**: 告警清零且行为不变

## 约束条件

### 技术约束
- 遵循现有 Rust 代码规范与项目约定
- 不新增第三方依赖

### 业务约束
- 不更改功能行为

### 法规约束
- 无

## 依赖关系

### 内部依赖
- Leptos 0.7.x API

### 外部依赖
- 无

## 成功指标

- cargo check -p backend 与 cargo check -p smart-ingredients-app 输出 0 个 unused/deprecated 告警

## 待解决问题

| 问题 | 影响 | 负责人 | 状态 |
|------|------|--------|------|
| smart-ingredients-tauri 在 cargo check --workspace 下报 OUT_DIR 错误 | 中 | 待定 | 开放 |

## 参考资料

- docs/standards/coding-standards.md
- docs/standards/requirements-template.md

---

## 变更记录

| 版本 | 日期 | 作者 | 描述 |
|------|------|------|------|
| 1.0 | 2026-02-20 | Claude Code | 初始版本 |
