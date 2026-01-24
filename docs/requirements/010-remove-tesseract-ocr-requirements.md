# 010-移除 Tesseract OCR 需求文档

## 元数据

| 字段 | 值 |
|------|-----|
| 文档编号 | 010-remove-tesseract-ocr |
| 标题 | 移除 Tesseract OCR 实现需求文档 |
| 版本 | 1.0 |
| 状态 | 草稿 |
| 创建日期 | 2026-01-24 |
| 更新日期 | 2026-01-24 |
| 作者 | Claude Code |

## 概述

### 目的

简化 OCR 实现架构，移除 Tesseract OCR 相关代码，统一使用 PaddleOCR 作为唯一的 OCR 解决方案。

**背景**：
- 当前项目维护两套 OCR 实现（PaddleOCR + Tesseract OCR）
- PaddleOCR 已作为默认方案稳定运行
- Tesseract OCR 实现增加了代码复杂度和维护成本
- OpenCV 依赖增加了编译难度和二进制体积

### 范围

**包含内容**：
- 移除 Tesseract OCR 相关代码
- 移除 OpenCV 图像预处理模块
- 移除 `ocr-tesseract` feature flag
- 清理相关配置项和环境变量
- 更新文档和配置示例

**不包含内容**：
- 不修改 PaddleOCR 实现逻辑
- 不影响现有 OCR 功能和性能
- 不涉及 OCR 服务部署变更

### 利益相关者

- **开发者**：减少代码维护负担，简化编译流程
- **运维人员**：减少依赖项，降低部署复杂度
- **用户**：无感知变更，不影响使用体验

## 功能需求

### 需求 1：移除 Tesseract OCR 实现代码

- **编号**: FR-001
- **优先级**: 高
- **描述**: 删除 `backend/src/services/ocr.rs` 中的 Tesseract OCR 实现函数及相关条件编译代码
- **验收标准**:
  - [ ] 删除 `extract_text_tesseract()` 函数
  - [ ] 删除 `build_tesseract_args()` 函数
  - [ ] 移除 `#[cfg(feature = "ocr-tesseract")]` 条件编译
  - [ ] `extract_text()` 函数仅保留 PaddleOCR 调用逻辑

### 需求 2：移除 OpenCV 图像预处理模块

- **编号**: FR-002
- **优先级**: 高
- **描述**: 删除 `backend/src/services/ocr_preprocess.rs` 文件及所有图像预处理代码
- **验收标准**:
  - [ ] 删除 `ocr_preprocess.rs` 文件
  - [ ] 从 `backend/src/services/mod.rs` 中移除 `ocr_preprocess` 模块声明
  - [ ] 移除 `ocr.rs` 中对 `ocr_preprocess` 的引用

### 需求 3：移除 Feature Flag 和依赖

- **编号**: FR-003
- **优先级**: 高
- **描述**: 清理 `backend/Cargo.toml` 中的 `ocr-tesseract` feature 和 OpenCV 依赖
- **验收标准**:
  - [ ] 删除 `ocr-tesseract = ["dep:opencv"]` feature 定义
  - [ ] 从 `full` feature 中移除 `ocr-tesseract`
  - [ ] 删除 `opencv` 依赖项
  - [ ] 确保 `default = ["paddle"]` 仍然有效

### 需求 4：清理配置代码

- **编号**: FR-004
- **优先级**: 高
- **描述**: 移除 `backend/src/config.rs` 中的 Tesseract 和预处理配置
- **验收标准**:
  - [ ] 从 `OcrProvider` 枚举中移除 `Tesseract` 变体
  - [ ] 删除 `OcrPreprocessConfig` 结构体
  - [ ] 从 `OcrConfig` 中移除 `preprocess` 字段
  - [ ] 从 `OcrConfig` 中移除 `psm` 和 `oem` 字段（Tesseract 特有参数）
  - [ ] 简化 `parse_ocr_provider()` 函数，仅保留 `paddle` 分支
  - [ ] 删除 `parse_optional_u8()` 函数（如果仅用于 OCR）
  - [ ] 移除所有 `OCR_PREPROCESS_*` 环境变量解析代码

### 需求 5：更新环境变量和配置文件

- **编号**: FR-005
- **优先级**: 中
- **描述**: 清理配置文件中的 Tesseract 相关环境变量
- **验收标准**:
  - [ ] 从 `.env.example` 中移除 Tesseract 相关配置
  - [ ] 更新 `docker-compose.prod.yml`，移除 `OCR_PROVIDER` 环境变量或固定为 `paddle`
  - [ ] 确保 `docker-compose.yml` 和 `.env` 保持 `OCR_PROVIDER=paddle`

### 需求 6：更新文档

- **编号**: FR-006
- **优先级**: 中
- **描述**: 更新项目文档，移除 Tesseract OCR 相关说明
- **验收标准**:
  - [ ] 更新 `CLAUDE.md`，移除 Tesseract OCR 描述
  - [ ] 更新 `docs/standards/project-conventions.md`，移除 `OCR_PROVIDER=tesseract` 选项
  - [ ] 更新 `docs/design/003-ocr-quality-technical-plan.md`，明确仅支持 PaddleOCR
  - [ ] 检查其他文档中是否有 Tesseract 相关内容并更新

### 需求 7：验证编译和运行

- **编号**: FR-007
- **优先级**: 高
- **描述**: 确保移除代码后项目可以正常编译和运行
- **验收标准**:
  - [ ] `cargo build` 成功编译
  - [ ] `cargo test` 所有测试通过
  - [ ] `cargo clippy` 无警告
  - [ ] OCR 功能正常工作（使用 PaddleOCR）

## 非功能需求

### 性能

- 移除 OpenCV 依赖后，编译时间应减少 20%-30%
- 二进制文件体积应减少（OpenCV 是较大的依赖）

### 可维护性

- 代码行数减少约 300-400 行
- 移除条件编译逻辑，降低代码复杂度
- 减少一个外部系统依赖（Tesseract CLI）

### 兼容性

- 不影响现有 PaddleOCR 功能
- 不影响 API 接口定义
- 不影响数据库结构

## 用户故事

### 故事 1
- **作为** 开发者
- **我想要** 移除冗余的 OCR 实现
- **以便** 简化代码维护和降低编译复杂度

### 故事 2
- **作为** 运维人员
- **我想要** 减少系统依赖项
- **以便** 降低部署和故障排查的难度

### 故事 3
- **作为** 用户
- **我想要** 系统保持稳定的 OCR 功能
- **以便** 继续正常使用配料表识别功能

## 用例

### 用例 1：OCR 文本提取

- **参与者**: 后端服务
- **前置条件**: 用户上传图片到系统
- **主流程**:
  1. 系统接收图片上传请求
  2. 调用 `extract_text()` 函数
  3. 函数直接调用 PaddleOCR 服务（无需判断 provider）
  4. 返回识别的文本结果
- **后置条件**: 文本成功提取并返回给调用方
- **备选流程**:
  - 如果 PaddleOCR 服务不可用，返回错误信息

## 约束条件

### 技术约束

- 必须保持 PaddleOCR 服务的可用性
- 不能破坏现有 API 接口契约

### 业务约束

- 变更需要向下兼容，不影响现有用户
- 必须在移除前确认 PaddleOCR 满足所有使用场景

## 依赖关系

### 内部依赖

- 依赖 `backend/src/services/ocr.rs`
- 依赖 `backend/src/config.rs`
- 依赖 `backend/Cargo.toml`

### 外部依赖

- 依赖 PaddleOCR 服务持续可用
- 依赖 Docker Compose 配置中的 OCR 服务定义

## 成功指标

- **代码简化**: 移除 300+ 行代码
- **编译时间**: 减少 20%-30%
- **依赖项**: 移除 1 个大型依赖（OpenCV）
- **功能完整性**: OCR 功能 100% 可用，无回归问题
- **文档更新**: 所有相关文档已更新

## 待解决问题

| 问题 | 影响 | 负责人 | 状态 |
|------|------|--------|------|
| 是否有离线环境需要使用 Tesseract OCR | 中 | 产品/技术负责人 | 待确认 |
| PaddleOCR 是否满足所有识别精度要求 | 高 | 技术负责人 | 待验证 |

## 参考资料

- [003-OCR质量优化技术方案](../design/003-ocr-quality-technical-plan.md)
- [项目约定规范](../standards/project-conventions.md)
- [Backend Cargo.toml](../../backend/Cargo.toml)

---

## 变更记录

| 版本 | 日期 | 作者 | 描述 |
|------|------|------|------|
| 1.0 | 2026-01-24 | Claude Code | 初始版本 |
