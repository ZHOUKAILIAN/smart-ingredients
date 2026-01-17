# 003-OCR 质量提升技术方案

## 元数据

| 字段     | 值                         |
| -------- | -------------------------- |
| 文档编号 | 003-002                    |
| 标题     | OCR 质量提升技术方案       |
| 版本     | 1.1                        |
| 状态     | 草稿                       |
| 创建日期 | 2026-01-17                 |
| 更新日期 | 2026-01-17                 |
| 作者     | Codex                      |
| 关联需求 | 003-001                    |

## 概述

### 目的
引入 PaddleOCR 服务作为主要 OCR 引擎，显著提升中文配料表识别质量；保留 Tesseract + OpenCV 作为备选路径。

### 范围
- 新增 PaddleOCR 本地服务（Docker）
- 后端 OCR Provider 配置化切换（tesseract / paddle）
- 保留 OpenCV 预处理能力（仅 Tesseract 路径使用）

### 假设
- Docker 环境可拉起 PaddleOCR 依赖
- PaddleOCR 模型可在首次启动下载

## 架构设计

### 高层架构
上传图片 → 保存到 `uploads/` → PaddleOCR 服务识别 → LLM 分析 → 数据库存储。
（可选）Tesseract 路径：OpenCV 预处理 → Tesseract OCR → LLM 分析

### 组件图
- `services/storage`: 图片保存
- `ocr` 服务（新增）: PaddleOCR HTTP 服务
- `services/ocr_preprocess`（可选）: OpenCV 预处理（Tesseract 路径）
- `services/ocr`: 根据 `OCR_PROVIDER` 调用 PaddleOCR / Tesseract
- `services/llm`: 调用 LLM

### 技术栈

| 组件 | 技术 | 选择理由 |
| --- | --- | --- |
| OCR 服务 | PaddleOCR (FastAPI) | 中文识别效果更优 |
| 图像处理 | OpenCV (Rust crate `opencv`) | Tesseract 预处理能力保留 |
| OCR 备选 | Tesseract | 本地开源、可降级 |
| 后端 | Rust + Axum | 已有框架 |

## 数据模型

### 实体
沿用 `analyses` 表，新增 OCR 质量评分字段（可选）：
- `ocr_score`（float, 0-1）

### 数据流
图像文件 → PaddleOCR 文本 → LLM 输入。
（可选）Tesseract：图像 → OpenCV 预处理 → OCR 文本 → LLM 输入。

## API 设计

### 接口列表
不新增接口，复用现有：

| 方法 | 路径 | 描述 |
| --- | --- | --- |
| POST | `/api/v1/analysis/{id}/analyze` | 触发 OCR + LLM |

### 数据结构
可选在响应中增加 `ocr_score`：
- `ocr_text: string`
- `ocr_score: number`

## 错误处理

| 错误码 | 消息 | 描述 |
| --- | --- | --- |
| OCR_FAILED | OCR failed | PaddleOCR 或 Tesseract 识别失败 |

## 性能考虑

- PaddleOCR 首次启动需要下载模型（一次性成本）
- OCR 请求耗时依赖图片大小与模型推理时间

## 测试策略

### 单元测试
- 预处理函数输入输出存在性检查

### 集成测试
- 固定样例图片：对比 OCR 文本长度提升

## 部署

### 环境要求
- Docker 需可拉起 PaddleOCR 服务
- 后端保留 OpenCV 依赖用于 Tesseract 路径

### 配置
新增/调整环境变量（默认值在 `.env.example`）：
- `OCR_PROVIDER=paddle`
- `OCR_PADDLE_URL=http://ocr:8000/ocr`
- `OCR_TIMEOUT=30`
- `OCR_LANG=chi_sim+eng`（Tesseract 使用）
- `OCR_PSM=6`
- `OCR_OEM=1`
- `OCR_PREPROCESS_ENABLE=true`
- `OCR_PREPROCESS_MIN_WIDTH=1600`
- `OCR_PREPROCESS_MAX_WIDTH=2000`
- `OCR_PREPROCESS_DESKEW=true`
- `OCR_PREPROCESS_BINARY=true`
- `OCR_PREPROCESS_DENOISE=true`
- `OCR_PREPROCESS_CLAHE=true`
- `OCR_PREPROCESS_SHARPEN=true`
- `OCR_PREPROCESS_MORPH_CLOSE=false`

## 实施阶段

### 阶段 1：PaddleOCR 服务
- [ ] 新增 `ocr` 服务（FastAPI + PaddleOCR）
- [ ] 后端接入 `OCR_PROVIDER` 切换逻辑
- [ ] 基于 `OCR_PADDLE_URL` 调用 OCR

### 阶段 2：质量策略（可选）
- [ ] 增加 OCR 质量评分计算（文本长度、字符比例）
- [ ] 低质量时降级到 Tesseract 或切换参数

## 风险与缓解

| 风险 | 影响 | 可能性 | 缓解措施 |
| --- | --- | --- | --- |
| PaddleOCR 镜像体积与启动耗时 | 中 | 中 | 复用缓存与延迟加载 |
| 预处理对部分图片反效果 | 中 | 中 | 提供开关与多策略 |

## 待解决问题

| 问题 | 影响 | 负责人 | 状态 |
| --- | --- | --- | --- |
| PaddleOCR 镜像体积与模型缓存策略 | 中 | 待定 | 开放 |

## 参考资料

- PaddleOCR 文档
- OpenCV 图像预处理实践

---

## 变更记录

| 版本 | 日期 | 作者 | 描述 |
| --- | --- | --- | --- |
| 1.0 | 2026-01-17 | Claude | 初始版本 |
| 1.1 | 2026-01-17 | Codex | 切换到 PaddleOCR 作为主 OCR 路径 |
