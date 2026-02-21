# 026-Tauri 构建脚本 需求文档

## 元数据

| 字段 | 值 |
|------|-----|
| 文档编号 | 026-tauri-build-script |
| 标题 | Tauri 构建脚本 需求文档 |
| 版本 | 1.0 |
| 状态 | 草稿 |
| 创建日期 | 2026-02-21 |
| 更新日期 | 2026-02-21 |
| 作者 | 小周 |

## 概述

### 目的
补齐 Tauri build script，确保在本地和 CI 执行 `cargo test` 时可以正确生成 `tauri::generate_context!()` 所需的构建产物。

### 范围
仅涉及 `frontend/src-tauri` 的 build script 与构建配置；不改动运行时功能与 UI 行为。

### 利益相关者
- 前端/桌面端开发者
- CI 维护者

## 功能需求

### 需求 1：提供 Tauri build script
- **编号**: FR-001
- **优先级**: 高
- **描述**: 在 `frontend/src-tauri` 增加 `build.rs`，使用 `tauri-build` 生成构建上下文。
- **验收标准**:
  - [ ] `cargo test` 不再因 `OUT_DIR` 缺失而失败
  - [ ] `tauri::generate_context!()` 可以正常编译

### 需求 2：配置变更可触发重建
- **编号**: FR-002
- **优先级**: 中
- **描述**: `tauri.conf.json` 变更时能触发 build script 重新运行。
- **验收标准**:
  - [ ] 修改 `tauri.conf.json` 后构建会重新执行 build script

### 需求 3：Bundle 图标文件存在
- **编号**: FR-003
- **优先级**: 高
- **描述**: `tauri.conf.json` 中声明的图标文件必须存在，避免 build script 执行时失败。
- **验收标准**:
  - [ ] `frontend/src-tauri/icons/32x32.png`、`128x128.png`、`128x128@2x.png` 存在
  - [ ] `cargo test -p smart-ingredients-tauri` 不再因 icon 缺失报错

### 需求 4：构建资源可被版本控制
- **编号**: FR-004
- **优先级**: 中
- **描述**: `build.rs` 与 `tauri.conf.json` 依赖的图标文件需要纳入版本控制，避免被 `.gitignore` 过滤。
- **验收标准**:
  - [ ] `frontend/src-tauri/build.rs` 不被 `.gitignore` 忽略
  - [ ] `frontend/src-tauri/icons/32x32.png`、`128x128.png`、`128x128@2x.png` 可被 git 跟踪

## 非功能需求

### 可靠性
- 构建流程稳定，避免因缺失 build script 导致的编译错误。

## 用户故事

### 故事 1
- **作为** 开发者
- **我想要** 运行 `cargo test` 不报 `OUT_DIR` 错误
- **以便** 在本地/CI 顺利验证代码

## 用例

### 用例 1：运行测试
- **参与者**: 开发者或 CI
- **前置条件**: 代码可编译
- **主流程**: 运行 `cargo test`
- **后置条件**: 测试通过且无 `OUT_DIR` 相关构建错误

## 约束条件

### 技术约束
- 使用 Tauri 2.x 与 `tauri-build` 2.x

## 依赖关系

### 内部依赖
- `frontend/src-tauri` crate

### 外部依赖
- `tauri-build` build-dependency

## 成功指标

- `cargo test` 在本仓库可通过（不再因 `OUT_DIR` 失败）

## 待解决问题

| 问题 | 影响 | 负责人 | 状态 |
|------|------|--------|------|
| 是否需要补充 CI 文档说明 | 低 | 小周 | 开放 |

---

## 变更记录

| 版本 | 日期 | 作者 | 描述 |
|------|------|------|------|
| 1.0 | 2026-02-21 | 小周 | 初始版本 |
