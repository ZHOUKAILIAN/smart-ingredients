# 026-编译告警清理技术方案

## 元数据

| 字段     | 值                       |
| -------- | ------------------------ |
| 文档编号 | 026-warning-cleanup      |
| 标题     | 编译告警清理技术方案      |
| 版本     | 1.0                      |
| 状态     | 草稿                     |
| 创建日期 | 2026-02-20               |
| 更新日期 | 2026-02-20               |
| 作者     | Claude Code              |
| 关联需求 | 026-warning-cleanup      |

## 概述

### 目的
消除 backend 与 smart-ingredients-app 中的 unused/deprecated 编译告警，保持行为不变，降低后续升级与维护成本。

### 范围
- backend：unused 与 dead_code 类警告
- smart-ingredients-app：unused、dead_code、deprecated、unused_mut 等警告

不包含 smart-ingredients-tauri 的构建问题处理（OUT_DIR 缺失）。

### 假设
- 本次为“告警清理”任务，用户已同意 TDD 例外（不新增测试用例）。
- 现有功能行为必须保持不变。

## 架构设计

### 高层架构
不调整架构，仅做局部代码清理与 API 替换。

### 组件图
无新增组件。

### 技术栈

| 组件   | 技术        | 选择理由 |
| ------ | ----------- | -------- |
| 后端   | Rust + Axum | 现有栈   |
| 前端   | Rust + Leptos | 现有栈 |

## 数据模型

不涉及数据模型变更。

## API 设计

不涉及 API 变更。

## 安全设计

不新增认证/授权逻辑。

## 错误处理

不新增错误码或错误响应结构。

## 性能考虑

清理告警不引入额外性能消耗。

## 测试策略

- 本次为告警清理，用户已同意 TDD 例外，不新增单元测试。
- 以编译告警作为 red/green 判定：
  - cargo check -p backend
  - cargo check -p smart-ingredients-app
- 执行 cargo fmt 保持格式一致。
- 说明：cargo check --workspace 目前因 smart-ingredients-tauri 缺少 build.rs 报 OUT_DIR 错误，作为已知限制。

## 实施阶段

### 阶段 1：Backend 告警清理

- [ ] 处理 backend/src/config.rs 的 dead_code 字段
- [ ] 处理 backend/src/services/image_converter.rs 未使用方法
- [ ] 处理 backend/src/services/rules.rs 未使用字段

### 阶段 2：Frontend unused 类告警

- [ ] 清理 frontend/src/components/mod.rs 未使用 re-export
- [ ] 清理 frontend/src/pages/* 未使用 import/变量
- [ ] 清理 frontend/src/services/mod.rs 未使用 mut
- [ ] 清理 frontend/src/utils/* 未使用变量/函数
- [ ] 清理 frontend/src/stores/mod.rs 未使用枚举/字段

### 阶段 3：Frontend deprecated API 替换

- [ ] create_effect → Effect::new
- [ ] create_memo → Memo::new
- [ ] create_signal → signal
- [ ] MaybeSignal → Signal
- [ ] web_sys deprecated API 替换为推荐方法

## 风险与缓解

| 风险 | 影响 | 可能性 | 缓解措施 |
| ---- | ---- | ------ | -------- |
| 删除未使用代码引发潜在功能差异 | 中 | 中 | 优先改为显式使用或改名为下划线变量，必要时保留并加解释性注释 |
| API 替换导致行为变化 | 中 | 中 | 严格对照原逻辑，逐文件小步替换并持续 cargo check |

## 待解决问题

| 问题 | 影响 | 负责人 | 状态 |
| ---- | ---- | ------ | ---- |
| smart-ingredients-tauri 的 OUT_DIR 错误如何处理 | 中 | 待定 | 开放 |

## 参考资料

- docs/standards/coding-standards.md
- docs/standards/technical-design-template.md

---

## 变更记录

| 版本 | 日期       | 作者   | 描述     |
| ---- | ---------- | ------ | -------- |
| 1.0  | 2026-02-20 | Claude Code | 初始版本 |
