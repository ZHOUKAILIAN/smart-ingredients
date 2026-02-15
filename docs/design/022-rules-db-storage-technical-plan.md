# 规则库 DB 存储技术方案

## 方案概述
将规则库从 `rules.json` 迁移到数据库。服务启动时加载规则到内存，并在后台定时刷新。提供一次性导入脚本，保证迁移平滑，且保持现有规则命中与可信度逻辑不变。

## 关键设计
1. **数据模型**
   - 新增表 `rules`：
     - `id` (pk, text)
     - `name` (text)
     - `aliases` (text[]) 或 JSONB
     - `category` (text)
     - `risk_level` (text)
     - `groups` (text[])
     - `description` (text)
     - `evidence` (text, nullable)
     - `source` (text, nullable)
     - `enabled` (bool, default true)
     - `created_at`, `updated_at` (timestamptz)
   - 可选扩展：`version` 字段用于未来版本控制（本次仅预留）。

2. **加载与刷新**
   - 启动时通过 SQL 拉取 `enabled=true` 规则，构建内存索引。
   - 使用后台任务 `tokio::spawn` 定时刷新（例如 5 分钟），刷新失败时保留旧缓存。
   - 规则引擎改为 `RuleEngine::load_from_db(pool)` 与 `RuleEngine::refresh()`。

3. **初始化导入**
   - 新增 `cargo run --bin rules_import` 或 `sqlx` 脚本：读取 `rules.json` 写入 DB。
   - 导入逻辑为 upsert（以 `id` 作为唯一键）。

4. **配置**
   - 新增 `RULES_REFRESH_SECONDS` 环境变量（默认 300）。
   - 保留 `RULES_PATH` 作为导入路径，不再作为主加载来源。

5. **兼容与回退**
   - DB 加载失败：规则引擎进入降级模式（空命中 + 可信度降低）。
   - 可选：允许启动时检测 DB 空表时从 `rules.json` 自动导入（默认关闭）。

## 修改点
- `backend/src/services/rules.rs`: 增加 DB 加载与刷新逻辑。
- `backend/src/state.rs`: 初始化 `RuleEngine` 时使用 DB。
- `backend/src/main.rs`: 启动刷新任务。
- 新增 migration：`rules` 表。
- 新增导入脚本/二进制。
- `.env.example`、`docker-compose*.yml` 增加刷新参数。

## 迁移步骤
1. 添加 migration 创建 `rules` 表。
2. 实现导入脚本并将现有 `rules.json` 导入 DB。
3. 替换加载逻辑为 DB 缓存 + 定时刷新。
4. 验证 E2E 流程与前端 `cargo check`。

## 验证
- 启动后日志显示规则加载成功与数量。
- 修改 DB 规则并等待刷新，命中结果更新。
- 完整接口流程可用，前端 `cargo check` 通过。
