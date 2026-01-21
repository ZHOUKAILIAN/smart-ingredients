# 008-安卓布局适配技术方案

## 元数据

| 字段     | 值                       |
| -------- | ------------------------ |
| 文档编号 | 008-android-layout-fix   |
| 标题     | 安卓设备布局适配技术方案 |
| 版本     | 1.0                      |
| 状态     | 草稿                     |
| 创建日期 | 2026-01-21               |
| 更新日期 | 2026-01-21               |
| 作者     | Claude Code              |
| 关联需求 | 008-android-layout-fix   |

## 概述

### 目的

本技术方案旨在解决安卓设备上应用显示的两个核心问题：
1. 左右空白问题：内容区域未充分利用屏幕宽度
2. 顶部安全区域问题：内容与系统状态栏的间距处理

### 范围

本设计涵盖：
- CSS 样式调整（`app.css` 文件）
- HTML viewport 配置（`index.html` 文件）
- 安全区域适配方案
- 跨平台兼容性保证

不涉及：
- Rust 代码修改
- 组件逻辑变更
- 新功能添加

### 假设

1. Tauri WebView 支持 CSS 安全区域 API (`env(safe-area-inset-*)`)
2. 目标安卓版本为 8.0+ (API Level 26+)
3. 现有的 Figma UI 设计可以在全宽布局下正常显示
4. 用户设备的 WebView 内核版本足够新（支持 CSS env() 函数）

## 架构设计

### 高层架构

```
┌─────────────────────────────────────────────────┐
│           Android Device Screen                 │
│  ┌───────────────────────────────────────────┐  │
│  │     System Status Bar (时间/电量/信号)     │  │
│  ├───────────────────────────────────────────┤  │
│  │                                           │  │
│  │         Tauri WebView Container          │  │
│  │  ┌─────────────────────────────────────┐ │  │
│  │  │        .app-shell (修改后)          │ │  │
│  │  │  - padding-top: safe-area + 24px    │ │  │
│  │  │  - padding-left: safe-area (0)      │ │  │
│  │  │  - padding-right: safe-area (0)     │ │  │
│  │  │  ┌───────────────────────────────┐  │ │  │
│  │  │  │        .page (内容区域)        │  │ │  │
│  │  │  │  - max-width: 420px            │  │ │  │
│  │  │  │  - 居中显示                    │  │ │  │
│  │  │  └───────────────────────────────┘  │ │  │
│  │  └─────────────────────────────────────┘ │  │
│  │                                           │  │
│  └───────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────┐  │
│  │     System Navigation Bar (可选)          │  │
│  └───────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

### 组件层次

```
index.html (viewport 配置)
  └── body
      └── .app-shell (容器 - 需要修改)
          └── .page (内容卡片 - 保持不变)
              └── 页面内容
```

### 技术栈

| 组件          | 技术                | 选择理由                              |
| ------------- | ------------------- | ------------------------------------- |
| 布局容器      | CSS Flexbox         | 现有方案，无需更改                    |
| 安全区域适配  | CSS env() 函数      | W3C 标准，广泛支持                    |
| Viewport 配置 | HTML meta 标签      | 标准方案，简单有效                    |
| 回退方案      | CSS max() 函数      | 确保在不支持 env() 的环境下有默认值   |

## 修改方案

### 1. CSS 样式修改

**文件**: `frontend/src/styles/app.css`

#### 修改点 1：`.app-shell` 样式

**当前代码** (第 41-46 行):
```css
.app-shell {
  min-height: 100vh;
  display: flex;
  justify-content: center;
  padding: 24px 16px;
}
```

**修改后代码**:
```css
.app-shell {
  min-height: 100vh;
  display: flex;
  justify-content: center;
  /* 使用安全区域 API 适配不同设备 */
  padding-top: max(24px, env(safe-area-inset-top, 24px));
  padding-right: env(safe-area-inset-right, 0);
  padding-bottom: max(24px, env(safe-area-inset-bottom, 24px));
  padding-left: env(safe-area-inset-left, 0);
}
```

**修改说明**:
- `padding-top`: 使用 `max(24px, env(safe-area-inset-top, 24px))` 确保至少有 24px 的顶部间距，同时适配有刘海/打孔的设备
- `padding-left/right`: 使用 `env(safe-area-inset-*)` 适配左右安全区域（通常为 0）
- `padding-bottom`: 使用 `max()` 确保底部有足够间距，同时适配有虚拟导航栏的设备
- 第二个参数（如 `24px`）是回退值，在不支持 `env()` 的环境下使用

#### 可选修改：针对 Figma 页面的特殊处理

如果 Figma 设计的页面（`.page.figma`）需要特殊处理，可以添加：

```css
/* Figma 页面已经是全屏设计，可能需要不同的 padding */
.page.page-capture.figma,
.page.page-result.figma,
.page.page-ocr.figma,
.page.page-analyzing.figma,
.page.page-summary.figma,
.page.page-detail.figma,
.page.page-confirm.figma {
  /* 这些页面的 .app-shell 可能需要 0 padding */
}
```

**注意**: 需要测试后确定是否需要此修改。

### 2. HTML Viewport 配置

**文件**: `frontend/index.html`

**当前代码** (第 5 行):
```html
<meta name="viewport" content="width=device-width, initial-scale=1" />
```

**修改后代码**:
```html
<meta name="viewport" content="width=device-width, initial-scale=1.0, viewport-fit=cover" />
```

**修改说明**:
- `viewport-fit=cover`: 允许页面内容延伸到安全区域之外，配合 CSS 的 `env()` 函数使用
- 这对于全面屏、刘海屏设备是必需的

### 3. 可选：添加 CSS 变量（便于后续调整）

在 `:root` 中添加 CSS 变量，方便统一管理间距：

```css
:root {
  /* ... 现有变量 ... */

  /* 安全区域间距 */
  --safe-padding-top: max(24px, env(safe-area-inset-top, 24px));
  --safe-padding-right: env(safe-area-inset-right, 0);
  --safe-padding-bottom: max(24px, env(safe-area-inset-bottom, 24px));
  --safe-padding-left: env(safe-area-inset-left, 0);
}

.app-shell {
  min-height: 100vh;
  display: flex;
  justify-content: center;
  padding-top: var(--safe-padding-top);
  padding-right: var(--safe-padding-right);
  padding-bottom: var(--safe-padding-bottom);
  padding-left: var(--safe-padding-left);
}
```

**优点**:
- 集中管理安全区域配置
- 其他组件也可以使用这些变量
- 便于后续调整

## 兼容性分析

### CSS env() 函数支持情况

| 平台      | WebView 版本      | 支持情况 | 备注                       |
| --------- | ----------------- | -------- | -------------------------- |
| iOS       | iOS 11.0+         | ✅ 支持  | 最早支持的平台             |
| Android   | Chrome 69+ (2018) | ✅ 支持  | Android 8.0+ 默认支持      |
| Desktop   | Chrome 69+        | ✅ 支持  | 通常 safe-area 值为 0      |
| Tauri     | 基于系统 WebView  | ✅ 支持  | 依赖系统 WebView 版本      |

### 回退策略

使用 `env()` 的第二个参数作为回退值：

```css
/* 如果不支持 env()，使用回退值 24px */
padding-top: max(24px, env(safe-area-inset-top, 24px));
```

即使在不支持 `env()` 的旧设备上，也会有 24px 的默认间距。

## 测试策略

### 单元测试

不适用（纯 CSS 修改，无逻辑代码）

### 视觉测试

需要在以下设备/场景进行人工测试：

#### 1. 安卓设备测试

| 设备类型       | 测试要点                                   | 预期结果                       |
| -------------- | ------------------------------------------ | ------------------------------ |
| 普通屏幕       | 内容左右是否充满屏幕                       | 左右无明显空白                 |
| 刘海屏         | 内容是否避开刘海区域                       | 顶部有足够间距，不被遮挡       |
| 打孔屏         | 内容是否避开打孔区域                       | 顶部有足够间距，不被遮挡       |
| 有虚拟导航栏   | 底部内容是否避开导航栏                     | 底部有足够间距                 |
| 小屏设备 (4.7")| 内容是否正常显示                           | 布局不变形，可正常操作         |
| 大屏设备 (6.7")| 内容是否正常显示                           | 布局不变形，可正常操作         |

#### 2. iOS 设备测试（回归测试）

| 设备类型   | 测试要点           | 预期结果           |
| ---------- | ------------------ | ------------------ |
| iPhone SE  | 布局是否正常       | 与修改前一致       |
| iPhone 14  | 刘海区域是否正确   | 内容不被遮挡       |
| iPhone 14+ | 大屏显示是否正常   | 布局不变形         |

#### 3. 桌面端测试（回归测试）

| 平台    | 测试要点       | 预期结果       |
| ------- | -------------- | -------------- |
| macOS   | 布局是否正常   | 与修改前一致   |
| Windows | 布局是否正常   | 与修改前一致   |
| Linux   | 布局是否正常   | 与修改前一致   |

### 测试检查清单

- [ ] 安卓普通屏幕设备：内容左右无空白
- [ ] 安卓刘海屏设备：顶部内容不被遮挡
- [ ] 安卓打孔屏设备：顶部内容不被遮挡
- [ ] 安卓虚拟导航栏设备：底部内容不被遮挡
- [ ] iOS 设备：显示正常，无回归问题
- [ ] 桌面端：显示正常，无回归问题
- [ ] 所有页面：导航、按钮、输入框等交互元素可正常使用
- [ ] 横屏模式：布局适配正常（如果支持）

## 实施阶段

### 阶段 1：代码修改（预计 10 分钟）

- [x] 创建需求文档
- [x] 创建技术设计文档
- [ ] 修改 `frontend/src/styles/app.css` 文件
  - [ ] 更新 `.app-shell` 的 padding 属性
  - [ ] （可选）添加 CSS 变量到 `:root`
- [ ] 修改 `frontend/index.html` 文件
  - [ ] 更新 viewport meta 标签

### 阶段 2：本地测试（预计 15 分钟）

- [ ] 在开发环境运行应用
- [ ] 使用浏览器开发者工具模拟移动设备
- [ ] 检查不同屏幕尺寸下的显示效果
- [ ] 使用 Chrome DevTools 的设备模式测试安全区域

### 阶段 3：真机测试（预计 30 分钟）

- [ ] 构建 Android APK
- [ ] 在真实安卓设备上安装测试
- [ ] 验证修复效果
- [ ] 记录任何问题或需要微调的地方

### 阶段 4：回归测试（预计 20 分钟）

- [ ] 在 iOS 设备上测试（如果有）
- [ ] 在桌面端测试
- [ ] 确认没有引入新问题

### 阶段 5：文档更新和提交（预计 10 分钟）

- [ ] 更新需求文档状态
- [ ] 更新技术设计文档（记录实际修改）
- [ ] 提交代码到 git
- [ ] 创建 PR（如果需要）

## 风险与缓解

| 风险                                   | 影响 | 可能性 | 缓解措施                                         |
| -------------------------------------- | ---- | ------ | ------------------------------------------------ |
| Tauri WebView 不支持 env() 函数        | 高   | 低     | 使用回退值，在不支持的情况下使用固定值           |
| 某些安卓设备的安全区域值不准确         | 中   | 中     | 使用 max() 确保最小间距，避免内容被完全遮挡      |
| 修改后在 iOS 或桌面端出现回归问题      | 高   | 低     | 充分的回归测试，env() 在桌面端通常返回 0         |
| Figma 设计的页面布局在全宽下显示异常   | 中   | 中     | 针对 Figma 页面添加特殊样式处理                  |
| 横屏模式下布局问题                     | 低   | 低     | 如果发现问题，后续添加媒体查询处理               |

## 性能考虑

### 渲染性能

- CSS 修改不涉及 JavaScript 计算，对性能无影响
- `env()` 和 `max()` 是原生 CSS 函数，性能开销可忽略
- 修改不会增加 DOM 复杂度或重排次数

### 包体积

- 代码量增加极少（约 50 字节）
- 对最终包体积无明显影响

## 调试方法

### Chrome DevTools 模拟安全区域

在 Chrome DevTools 中可以模拟安全区域：

1. 打开 DevTools（F12）
2. 切换到设备模式（Ctrl+Shift+M）
3. 选择设备（如 iPhone X）
4. 在 Console 中设置安全区域：
   ```javascript
   // 模拟顶部刘海（44px）
   document.documentElement.style.setProperty('--safe-area-inset-top', '44px');
   ```

### 在真机上调试

对于 Android 设备：

1. 启用 USB 调试
2. 连接设备到电脑
3. Chrome 浏览器访问 `chrome://inspect`
4. 选择 Tauri 应用进行调试
5. 在 Console 中检查安全区域值：
   ```javascript
   getComputedStyle(document.documentElement).getPropertyValue('padding-top');
   ```

## 待解决问题

| 问题                                     | 影响 | 负责人   | 状态 |
| ---------------------------------------- | ---- | -------- | ---- |
| 是否需要针对 Figma 页面做特殊处理       | 中   | 开发团队 | 开放 |
| 横屏模式是否需要特殊处理                 | 低   | 产品团队 | 开放 |
| 是否需要为超大屏设备（平板）做适配       | 低   | 产品团队 | 开放 |

## 参考资料

- [MDN - env()](https://developer.mozilla.org/en-US/docs/Web/CSS/env)
- [MDN - max()](https://developer.mozilla.org/en-US/docs/Web/CSS/max)
- [MDN - Viewport meta tag](https://developer.mozilla.org/en-US/docs/Web/HTML/Viewport_meta_tag)
- [WebKit - Designing Websites for iPhone X](https://webkit.org/blog/7929/designing-websites-for-iphone-x/)
- [W3C - CSS Environment Variables Module Level 1](https://drafts.csswg.org/css-env-1/)
- [Tauri - WebView](https://tauri.app/v1/guides/building/webview/)
- 项目文档：
  - `docs/requirements/008-android-layout-fix-requirements.md`
  - `docs/standards/coding-standards.md`

---

## 变更记录

| 版本 | 日期       | 作者        | 描述                                     |
| ---- | ---------- | ----------- | ---------------------------------------- |
| 1.0  | 2026-01-21 | Claude Code | 初始版本，包含 CSS 和 viewport 修改方案 |
