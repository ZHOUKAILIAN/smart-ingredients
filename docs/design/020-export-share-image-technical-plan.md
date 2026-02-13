# 020 - 导出分析结果为图片并分享 - 技术方案

## 技术架构

本功能完全在前端实现，无需后端改动。

## 实现方案

### 核心技术选型

使用浏览器原生 **Canvas 2D API** 绘制图片。

在 WASM (Leptos + wasm-bindgen) 环境中：

- 使用 `web_sys::HtmlCanvasElement` 和 `web_sys::CanvasRenderingContext2d`
- 将分析数据结构化绘制到 Canvas
- 导出为 PNG Blob，触发下载或调用 Web Share API

### 文件结构

```
frontend/src/
├── utils/
│   └── export_image.rs      # Canvas 绘图 + 下载/分享逻辑
├── components/
│   └── share_button.rs      # 分享/导出按钮组件
└── pages/
    ├── result.rs             # 修改：添加导出按钮
    └── summary.rs            # 修改：添加导出按钮
```

### CSS 新增

在 `app.css` 中添加：

- `.export-btn` - 导出按钮样式
- `.export-loading` - 导出加载状态
- `.share-modal` - 分享弹窗（如需要）

## 详细设计

### 1. export_image.rs

#### 数据结构

```rust
pub struct ExportData {
    pub health_score: i32,
    pub recommendation: String,
    pub ingredients: Vec<ExportIngredient>,
    pub warnings: Vec<String>,
    pub summary: String,
    pub preference: String,
}

pub struct ExportIngredient {
    pub name: String,
    pub risk_level: String,
    pub description: String,
}
```

#### Canvas 绘图流程

1. 创建隐藏的 `<canvas>` 元素
2. 设置宽度 750px
3. 预计算总高度（根据内容）
4. 按区域绘制：
   - 头部区域（品牌 + 渐变背景）
   - 健康评分区域（圆环 + 分数）
   - 配料分析列表（卡片样式）
   - 健康建议区域
   - 总结区域
   - 底部水印
5. 调用 `canvas.toBlob()` 导出 PNG

#### 下载逻辑

```rust
// 创建 Blob URL -> 创建 <a> 元素 -> 点击下载 -> 清理
```

#### 分享逻辑

```rust
// 检查 navigator.share 支持
// 支持：调用 navigator.share({ files: [file] })
// 不支持：降级为下载
```

### 2. share_button.rs

```rust
#[component]
pub fn ShareButton(export_data: ExportData) -> impl IntoView {
    // 点击事件：调用 export_image 逻辑
    // 加载状态管理
    // Toast 提示
}
```

### 3. 页面集成

在 `result.rs` 和 `summary.rs` 的 action 区域添加导出按钮。

## web-sys Features 依赖

需要在 `Cargo.toml` 中添加以下 web-sys features：

```toml
web-sys = { version = "0.3", features = [
    # 现有 features...
    "CanvasRenderingContext2d",
    "Document",
    "HtmlCanvasElement",
    "Blob",
    "BlobPropertyBag",
    "Url",
    "HtmlAnchorElement",
    "Navigator",
] }
```

## 风险与对策

| 风险                 | 对策                       |
| -------------------- | -------------------------- |
| Canvas 中文字体渲染  | 使用系统中文字体 fallback  |
| Web Share API 兼容性 | 不支持时降级为下载         |
| 长图内容过多         | 分页或限制最大高度         |
| WASM 中操作 Canvas   | 通过 web-sys bindings 实现 |
