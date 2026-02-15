# UI 展示优化技术方案

## 方案概述
在不改变现有 API 结构的前提下，通过前端分层布局与可视化组件提升结果页体验。后端仅在必要时补充字段（如风险维度汇总），优先复用已有规则命中与可信度因子数据。

## 关键设计
1) 信息分层布局
- 新增“结论卡”组件：基于 `overall_assessment` + `health_score` 生成结论等级与一句话理由。
- “关键风险标签”取 `rule_hits` 中 `risk_level=high/medium` 的前 3–5 条。
- 规则命中细节保留折叠（已实现），默认收起。

2) 可视化模块
- 可信度条：使用现有 `confidence.level` + `factors` 生成进度条与因子列表。
- 风险维度图：
  - 基于 `score_breakdown` 生成条形/雷达图数据。
  - 若 `score_breakdown` 不存在，展示占位提示。
- 组件实现优先纯 CSS + SVG，避免引入重型图表依赖。

3) 决策引导卡片
- 人群建议卡：基于 `preference` 与 `warnings`/`rule_hits` 生成专属提示文案。
- 替代建议卡：根据高风险命中类型给出方向性建议（如“选择无过敏原/低糖/少添加剂”）。
- 文案规则写在前端本地表，避免增加后端复杂度。

## 数据结构与接口
- 优先不改后端结构；必要时可新增 `risk_tags` 或 `risk_summary`（由后端聚合）作为可选字段。
- 若前端可直接从 `rule_hits`、`score_breakdown` 推导，则不新增字段。

## 修改点
- `frontend/src/pages/result.rs`：新增结论卡、关键风险标签、可视化模块、建议卡。
- 新增/调整前端组件（如 `components/summary_card.rs`, `components/risk_chart.rs`）。
- 可能新增 `frontend/src/utils/presentation.rs` 用于文案/聚合逻辑。

## 风险与回退
- 若可视化实现复杂或引入依赖，先用条形/列表版本保底。
- 若缺失数据，所有模块需优雅降级（不阻塞主结论展示）。

## 验证
- 前端 `cargo check` 通过。
- 本地服务启动后跑完整 API 流程，结果页展示新模块且不报错。
