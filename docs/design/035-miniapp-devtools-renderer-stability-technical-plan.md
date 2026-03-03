# 035-小程序开发者工具渲染进程稳定性技术方案

## 元数据

| 字段 | 值 |
|------|-----|
| 文档编号 | 035-miniapp-devtools-renderer-stability |
| 标题 | 小程序开发者工具渲染进程稳定性技术方案 |
| 版本 | 1.0 |
| 状态 | 草稿 |
| 创建日期 | 2026-02-26 |
| 更新日期 | 2026-02-26 |
| 作者 | 小周 |
| 关联需求 | 035-miniapp-devtools-renderer-stability |

## 问题诊断

通过 DevTools 日志与进程监控确认：
1. `appid=wx1e...` 在本地未稳定登录时，频繁出现 `41001 需要重新登录`。
2. DevTools 出现 appservice 反复重载，渲染进程 CPU/内存快速上升。
3. 文件监听与构建事件叠加时，卡顿概率上升。

## 设计目标

- 保证默认开发环境“开箱可调试”。
- 不改业务代码，仅通过配置降低 DevTools 负载。

## 方案设计

### 1. 统一默认 AppID 为游客态
- 文件：`miniapp/project.config.json`
- 变更：`appid` -> `touristappid`
- 目的：规避未登录导致的远端接口高频失败与重试。

### 2. 关闭编译热重载
- 文件：`miniapp/project.private.config.json`
- 变更：`compileHotReLoad` -> `false`
- 目的：降低频繁文件事件触发的重编译链路。

### 3. 调整无用文件过滤策略
- 文件：`miniapp/project.private.config.json`
- 变更：`ignoreDevUnusedFiles` -> `false`
- 目的：减少 DevTools 内部文件索引抖动。

### 4. 显式忽略 Monorepo 目录
- 文件：`miniapp/project.config.json`
- 变更：在 `packOptions.ignore` 添加 `../backend`、`../frontend`、`../shared`、`../docs`、`../ocr_service`、`../scripts`、`../target`、`../node_modules`
- 目的：当 DevTools 误扫描到仓库根目录时，降低文件基数与索引负担。

### 5. 使用非软链接代理目录
- 文件：`scripts/open-miniapp-devtools.sh`
- 变更：由“软链接代理”改为“复制同步代理”（`rsync/cp`），每次启动前刷新代理目录内容。
- 目的：避免 DevTools 对软链接工程路径识别异常（如 `app.json` 误报），同时保持代理目录体积可控。

## 风险与缓解

| 风险 | 影响 | 缓解 |
|------|------|------|
| 关闭热重载影响实时体验 | 中 | 通过手动“编译”保证稳定优先 |
| 游客 AppID 下部分能力受限 | 低 | 需要真机能力时再切正式 AppID |

## 验证策略

1. 打开 DevTools 并加载 `miniapp/`。
2. 观察 2 分钟进程占用：Renderer 不应持续 >90% CPU。
3. 检查日志中 41001 错误是否明显减少。
4. 保留手动编译路径：`npm run build:weapp`。

## 实施步骤

1. 更新配置文件。
2. 更新 DevTools 启动脚本为非软链接代理方案。
3. 重启 DevTools。
4. 执行构建并验证资源占用。

## 变更记录

| 版本 | 日期 | 作者 | 描述 |
|------|------|------|------|
| 1.0 | 2026-02-26 | 小周 | 初始版本 |
