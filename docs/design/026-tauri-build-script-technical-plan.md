# 026-Tauri 构建脚本 技术方案

## 元数据

| 字段     | 值                       |
| -------- | ------------------------ |
| 文档编号 | 026-tauri-build-script   |
| 标题     | Tauri 构建脚本 技术方案   |
| 版本     | 1.0                      |
| 状态     | 草稿                     |
| 创建日期 | 2026-02-21               |
| 更新日期 | 2026-02-21               |
| 作者     | 小周                     |
| 关联需求 | 026-tauri-build-script   |

## 概述

### 目的
补齐 `frontend/src-tauri` 的 build script，确保 `tauri::generate_context!()` 在测试与构建时可用。

### 范围
仅增加 `build.rs` 与必要的重建触发配置，不调整运行时逻辑。

### 假设
- 使用 Tauri 2.x 模板推荐的 `tauri_build::build()` 方式生成构建上下文。

## 架构设计

### 高层架构
在 `frontend/src-tauri` 增加 build script：编译前由 `tauri-build` 生成 context 文件并写入 `OUT_DIR`。同时确保 `tauri.conf.json` 引用的图标文件存在。
并调整 `.gitignore`，确保 build script 与图标资源可被版本控制追踪。

### 组件图
- `build.rs` → `tauri-build` → 生成 context → `tauri::generate_context!()` 编译读取
- `icons/*.png` → 供 `tauri.conf.json` 在生成 context 时读取
- `.gitignore` → 允许 `build.rs` 与 `icons/*.png` 被纳入版本控制

### 技术栈

| 组件 | 技术 | 选择理由 |
| ---- | ---- | -------- |
| Build Script | Rust + tauri-build 2.x | 官方推荐、最小改动 |

## 数据模型
无新增数据模型。

## API 设计
无新增 API。

## 错误处理
- 如果生成失败，构建阶段直接报错，阻止进入运行时。

## 测试策略

### 单元测试
- 不新增单元测试（配置类变更）。

### 集成测试
- 运行 `cargo test` 作为验证入口，确认不再出现 `OUT_DIR` 错误。
 - 运行 `cargo test -p smart-ingredients-tauri`，确认 icon 缺失不再导致失败。

## 部署
无需部署变更。

## 实施阶段

### 阶段 1：补齐 build script
- [ ] 添加 `frontend/src-tauri/build.rs`

### 阶段 2：补齐图标文件
- [ ] 添加 `frontend/src-tauri/icons/32x32.png`
- [ ] 添加 `frontend/src-tauri/icons/128x128.png`
- [ ] 添加 `frontend/src-tauri/icons/128x128@2x.png`

### 阶段 3：更新忽略规则
- [ ] 调整 `.gitignore`，允许 `frontend/src-tauri/build.rs` 与上述图标文件被 git 跟踪

### 阶段 4：验证
- [ ] 运行 `cargo test -p smart-ingredients-tauri`
- [ ] 运行 `cargo test`

## 风险与缓解

| 风险 | 影响 | 可能性 | 缓解措施 |
| ---- | ---- | ------ | -------- |
| build script 不触发 | 中 | 低 | 添加 `rerun-if-changed` 监听配置文件 |

## 待解决问题

| 问题 | 影响 | 负责人 | 状态 |
| ---- | ---- | ------ | ---- |
| 是否需要补充 CI 文档 | 低 | 小周 | 开放 |

---

## 变更记录

| 版本 | 日期 | 作者 | 描述 |
| ---- | ---- | ---- | ---- |
| 1.0 | 2026-02-21 | 小周 | 初始版本 |
