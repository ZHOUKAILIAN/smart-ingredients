# 035-小程序开发者工具渲染进程稳定性需求文档

## 元数据

| 字段 | 值 |
|------|-----|
| 文档编号 | 035-miniapp-devtools-renderer-stability |
| 标题 | 小程序开发者工具渲染进程稳定性需求文档 |
| 版本 | 1.0 |
| 状态 | 草稿 |
| 创建日期 | 2026-02-26 |
| 更新日期 | 2026-02-26 |
| 作者 | 小周 |

## 概述

### 目的
修复 `miniapp` 在微信开发者工具中打开后渲染进程 CPU/内存异常升高导致卡顿的问题，确保可持续调试与代码修改。

### 范围
**包含**：
- 调整微信开发者工具项目配置（`project.config.json`、`project.private.config.json`）
- 将默认 AppID 调整为测试态可用配置
- 降低开发者工具热重载触发频率

**不包含**：
- 小程序业务页面逻辑改动
- 后端接口逻辑改动
- Taro 源码或微信开发者工具版本升级

## 功能需求

### 需求 1：避免登录态异常导致的循环请求
- **编号**: FR-001
- **优先级**: 高
- **描述**: 将 `miniapp/project.config.json` 的默认 `appid` 调整为 `touristappid`，避免因本地未登录正式账号触发高频 41001 错误。
- **验收标准**:
  - [ ] `project.config.json` 默认 `appid` 为 `touristappid`
  - [ ] DevTools 日志中 `41001 需要重新登录` 明显减少或消失

### 需求 2：降低热更新风暴
- **编号**: FR-002
- **优先级**: 高
- **描述**: 关闭 DevTools 编译热重载，改为手动编译/受控刷新，避免渲染进程因频繁文件事件持续占用高资源。
- **验收标准**:
  - [ ] `project.private.config.json` 中 `compileHotReLoad` 为 `false`
  - [ ] 连续调试 2 分钟内无渲染进程持续 90%+ CPU 场景

### 需求 3：减少无关文件监听
- **编号**: FR-003
- **优先级**: 中
- **描述**: 调整 `ignoreDevUnusedFiles` 为 `false`，减少 DevTools 内部重复索引/过滤抖动。
- **验收标准**:
  - [ ] `project.private.config.json` 中 `ignoreDevUnusedFiles` 为 `false`
  - [ ] DevTools 文件监听初始化时长下降或稳定

### 需求 4：忽略 Monorepo 无关目录
- **编号**: FR-004
- **优先级**: 高
- **描述**: 在 `project.config.json` 的 `packOptions.ignore` 中显式忽略后端/前端/文档/构建产物目录，避免 DevTools 扫描整个仓库时触发大规模文件索引。
- **验收标准**:
  - [ ] `packOptions.ignore` 包含 `../backend`、`../frontend`、`../shared`、`../docs`、`../ocr_service`、`../scripts`、`../target`、`../node_modules`
  - [ ] DevTools 日志中的 `all ready` 文件数量显著下降

### 需求 5：提供非软链接代理目录启动方式
- **编号**: FR-005
- **优先级**: 高
- **描述**: 提供一键脚本，将 `miniapp` 必要文件同步到独立代理目录（非软链接），并从该目录启动 DevTools，规避软链接路径兼容问题与 `app.json` 误报。
- **验收标准**:
  - [ ] 存在可执行脚本用于同步并打开 DevTools
  - [ ] 代理目录内目标文件为真实文件/目录而非软链接
  - [ ] 使用代理目录导入后不再出现“app.json 未找到”的软链接相关误报

## 非功能需求

### 稳定性
- DevTools 可连续打开并调试，渲染进程不持续高占用。

### 可维护性
- 仅修改配置文件，不引入业务逻辑耦合。

## 成功指标

- 打开项目后 2 分钟内，`wechatwebdevtools Helper (Renderer)` 不持续占用 CPU > 90%。
- 调试过程无“打开即卡死”体验。

## 变更记录

| 版本 | 日期 | 作者 | 描述 |
|------|------|------|------|
| 1.0 | 2026-02-26 | 小周 | 初始版本 |
