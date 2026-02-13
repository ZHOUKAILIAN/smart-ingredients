# 019-规则兜底与可信度提升 技术方案

## 元数据

| 字段     | 值                       |
| -------- | ------------------------ |
| 文档编号 | 019-rule-based-analysis  |
| 标题     | 规则兜底与可信度提升 技术方案 |
| 版本     | 1.0                      |
| 状态     | 草稿                     |
| 创建日期 | 2026-02-13               |
| 更新日期 | 2026-02-13               |
| 作者     | 小周                     |
| 关联需求 | 019-rule-based-analysis  |

## 概述

### 目的
为“规则兜底 + LLM 解释”提供后端规则引擎、数据结构与前端展示方案，确保分析结果可解释、可追溯。

### 范围
- 后端：规则库加载、规则命中与输出合成
- 共享类型：规则命中与可信度字段
- 前端：结果页展示可信度与规则说明

### 假设
- OCR 已完成，用户已确认文本
- 现有 LLM 输出可接入解释字段

## 架构设计

### 高层架构

1. 解析确认文本 -> 规则引擎匹配成分 -> 输出规则命中结果
2. LLM 继续输出解释与建议
3. 合并规则结果 + LLM 解释 -> 返回给前端

### 组件图

- **Rule Engine**: 负责规则匹配、风险等级与偏好加权
- **LLM Service**: 生成解释与建议
- **Result Composer**: 合成最终结果、计算可信度

### 技术栈

| 组件 | 技术 | 选择理由 |
| --- | --- | --- |
| 规则库 | JSON 配置文件 | 易扩展，便于迭代 |
| 规则引擎 | Rust 模块（backend/services） | 与后端统一，易测试 |
| 类型共享 | shared crate | 前后端一致数据结构 |

## 数据模型

### 实体

**RuleItem**
- `id`: String
- `name`: String
- `aliases`: Vec<String>
- `category`: String
- `risk_level`: String (low/medium/high)
- `groups`: Vec<String> (kids/allergy/fitness/...)
- `description`: String

**RuleHit**
- `name`: String
- `category`: String
- `risk_level`: String
- `description`: String
- `group_tags`: Vec<String>

**ConfidenceInfo**
- `level`: String (high/medium/low)
- `reasons`: Vec<String>

### 数据流

1. 用户确认文本 -> token 化成分列表
2. 规则引擎按名称/别名匹配
3. 根据偏好提升风险等级
4. 规则命中输出 + LLM 输出合成结果

## API 设计

### 接口列表

保持现有接口，扩展 `AnalysisResult` 返回结构。

### 数据结构

`AnalysisResult` 增加字段：
- `rule_hits: Vec<RuleHit>`
- `confidence: ConfidenceInfo`

## 安全设计

### 认证
- 复用现有鉴权逻辑

### 授权
- 无新增权限

### 数据保护
- 规则库只读，不接受外部输入覆盖

## 错误处理

### 错误码

| 错误码 | 消息 | 描述 |
| ------ | ---- | ---- |
| RULES_LOAD_FAILED | 规则库加载失败 | 降级为 LLM-only 并标记低可信度 |

### 错误响应格式

沿用现有 `AppError` 结构。

## 性能考虑

### 缓存策略
- 规则库进程启动时加载到内存

### 优化
- 规则匹配使用小写归一化 + HashSet

### 监控
- 记录规则命中次数与耗时指标（tracing）

## 测试策略

### 单元测试
- 规则匹配与别名命中
- 偏好加权后风险等级调整

### 集成测试
- 上传->确认->分析返回包含 `rule_hits` 与 `confidence`

### E2E 测试
- 选取包含过敏原的样本图，确认结果有强提醒

## 部署

### 环境要求
- 无新增依赖

### 配置
- 新增 `rules.json` 文件（后端读取路径）

### 回滚计划
- 回滚代码并移除 `rule_hits`/`confidence` 字段

## 实施阶段

### 阶段 1：规则库与引擎
- [ ] 添加 `rules.json` 与加载逻辑
- [ ] 实现规则匹配与风险等级调整

### 阶段 2：结果合成与展示
- [ ] 扩展 `AnalysisResult` 数据结构
- [ ] 前端结果页展示规则命中与可信度

## 风险与缓解

| 风险 | 影响 | 可能性 | 缓解措施 |
| ---- | ---- | ---- | ---- |
| 规则库覆盖不足 | 中 | 中 | 先覆盖高频/高风险成分 |
| 规则错误导致误导 | 高 | 低 | 规则条目与说明人工审核 |

## 待解决问题

| 问题 | 影响 | 负责人 | 状态 |
| ---- | ---- | ---- | ---- |
| 规则库最小集合列表确认 | 中 | 待定 | 开放 |
| 可信度等级规则 | 中 | 待定 | 开放 |

## 参考资料

- docs/requirements/019-rule-based-analysis-requirements.md
- docs/standards/technical-design-template.md

---

## 变更记录

| 版本 | 日期 | 作者 | 描述 |
| ---- | ---- | ---- | ---- |
| 1.0 | 2026-02-13 | 小周 | 初始版本 |
