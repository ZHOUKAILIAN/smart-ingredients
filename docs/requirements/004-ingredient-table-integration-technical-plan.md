# 004-配料表表格数据联调技术方案

## 元数据

| 字段     | 值                         |
| -------- | -------------------------- |
| 文档编号 | 004-002                    |
| 标题     | 配料表表格数据联调技术方案 |
| 版本     | 1.0                        |
| 状态     | 草稿                       |
| 创建日期 | 2026-01-17                 |
| 更新日期 | 2026-01-17                 |
| 作者     | Codex                      |

## 目标

- 输出稳定的 `summary + table` 结构，供前端渲染
- 保持现有 `result` 结构兼容
- 打通“上传 -> OCR -> LLM -> 返回 -> 前端展示”的联调链路

## 方案概览

- 后端：在分析结果结构中新增 `summary` 与 `table`
- Prompt：让 LLM 输出结构化 JSON，包含表格数据
- 解析与兜底：校验字段，补齐 `unknown`，保证前端稳定渲染
- 前端：基于 `summary` 和 `table` 渲染

## 数据结构

### Result 新增字段

```json
{
  "summary": "string",
  "table": [
    {
      "name": "string",
      "category": "string",
      "function": "string",
      "risk_level": "low | medium | high | unknown",
      "note": "string"
    }
  ]
}
```

### 字段规则

- `summary`：1-3 句概括配料表特点
- `table`：按 OCR 文本顺序输出
- 同名配料去重：取风险更高条目
- 无法解析字段填充 `unknown` / 空字符串

## 后端改动

### 1) LLM Prompt 调整

- 在系统或用户 prompt 中要求输出 `summary` 与 `table`
- 强制 JSON 结构，明确字段枚举与示例
- 失败时返回空数组并写入错误信息

### 2) 解析与校验

- 为 `table` 行定义结构体或校验逻辑
- `risk_level` 非法值统一映射为 `unknown`
- `summary` 为空时提供默认值（例如“未识别到足够信息”）

### 3) API 响应

- 保持现有 `ingredients / warnings / recommendation` 字段不变
- 在 `result` 中新增 `summary` 与 `table`

## 前端改动

- 展示顺序：`summary` 文本 -> 表格
- 表格列：`name` / `category` / `function` / `risk_level` / `note`
- 空表格时展示提示文案

## 联调链路

1. 前端上传图片：`POST /api/v1/analysis/upload`
2. 后端 OCR + LLM：`POST /api/v1/analysis/{id}/analyze`
3. 前端读取 `result.summary` 与 `result.table`
4. 前端渲染摘要与表格

## 联调验收

- 上传图片后能成功获取 `summary + table`
- 表格可在前端正常渲染
- OCR 或 LLM 失败时前端仍可显示可读错误信息

## 风险与应对

- LLM 输出不稳定：增加 JSON Schema 指令与示例
- OCR 质量低：输出 `unknown` 并提示用户重试
