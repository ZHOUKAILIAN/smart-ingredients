# 017-移动端 UI 精细化优化技术方案

## 元数据

| 字段     | 值                                    |
| -------- | ------------------------------------- |
| 文档编号 | 017-mobile-ui-refinement              |
| 标题     | 移动端 UI 精细化优化技术方案          |
| 版本     | 1.0                                   |
| 状态     | 草稿                                  |
| 创建日期 | 2026-01-31                            |
| 更新日期 | 2026-01-31                            |
| 作者     | Claude + User                         |
| 关联需求 | 017-mobile-ui-refinement-requirements |

## 概述

### 目的

提供在 Android 模拟器/真机上进行 UI 精细化调整的技术方案,建立基于真实设备的 UI 开发和测试流程。

### 范围

- 设置 Android 模拟器开发环境
- 建立快速迭代流程(修改 → 构建 → 测试)
- 识别和修复移动端 UI 问题
- 优化 CSS 样式和组件布局
- 文档化移动端 UI 最佳实践

### 假设

- 已有 Android 开发环境(Android Studio / SDK)
- 已有 Tauri Android 构建配置
- 基础 UI 框架和组件已实现
- 使用 Leptos 0.7.x + CSS 进行 UI 开发

## 开发环境设置

### Android 模拟器配置

#### 推荐配置

```bash
# 创建 AVD (Android Virtual Device)
avdmanager create avd \
  -n "Pixel_6_API_33" \
  -k "system-images;android-33;google_apis;x86_64" \
  -d "pixel_6"

# 启动模拟器
emulator -avd Pixel_6_API_33 -gpu host
```

#### 多设备测试配置

| 设备名称    | 屏幕尺寸 | 分辨率    | 密度   | 用途           |
| ----------- | -------- | --------- | ------ | -------------- |
| Pixel 6     | 6.4"     | 1080x2400 | 420dpi | 主要测试设备   |
| Pixel 6 Pro | 6.7"     | 1440x3120 | 560dpi | 大屏高密度测试 |
| Pixel 4a    | 5.8"     | 1080x2340 | 440dpi | 中等屏幕测试   |

### 快速迭代工作流

#### 方案 1: 热重载开发(推荐)

```bash
# 终端 1: 启动 Tauri dev 模式(带热重载)
cd frontend
cargo tauri android dev

# 修改代码后自动重新加载
# 适用于: CSS 调整、小幅组件改动
```

#### 方案 2: 快速构建测试

```bash
# 构建并安装 APK
cd frontend
cargo tauri android build --apk

# 安装到模拟器/设备
adb install -r target/android/app/build/outputs/apk/debug/app-debug.apk

# 适用于: 需要完整构建的改动
```

#### 方案 3: 调试模式

```bash
# 启动带 Chrome DevTools 的调试模式
cargo tauri android dev --open-devtools

# 可以使用 Chrome DevTools 检查元素、调试 CSS
```

### 开发工具

#### Chrome DevTools 远程调试

```bash
# 1. 在 Android 设备上启动应用
# 2. 在 Chrome 浏览器打开
chrome://inspect/#devices

# 3. 点击 "inspect" 进行远程调试
```

#### ADB 工具

```bash
# 查看连接的设备
adb devices

# 查看应用日志
adb logcat | grep "smart-ingredients"

# 截图
adb exec-out screencap -p > screenshot.png

# 录屏
adb shell screenrecord /sdcard/demo.mp4
```

## UI 问题分类与解决方案

### 1. 布局问题

#### 问题类型

- 内容超出屏幕边界
- 固定定位元素位置错误
- 滚动区域不正确
- 安全区域处理缺失

#### 解决方案

**A. 视口配置**

```css
/* frontend/src/styles/app.css */

/* 确保视口配置正确 */
html,
body {
  width: 100%;
  height: 100%;
  overflow: hidden; /* 防止整体页面滚动 */
  position: fixed; /* 防止地址栏隐藏导致的布局跳动 */
}

/* 主容器 */
#app {
  width: 100%;
  height: 100%;
  overflow: hidden;
}
```

**B. 安全区域处理**

```css
/* 底部导航栏 - 处理刘海屏/圆角屏 */
.bottom-nav {
  padding-bottom: env(safe-area-inset-bottom);
  padding-left: env(safe-area-inset-left);
  padding-right: env(safe-area-inset-right);
}

/* 顶部区域 */
.header {
  padding-top: env(safe-area-inset-top);
}
```

**C. 滚动区域**

```css
/* 可滚动内容区域 */
.page-content {
  height: calc(100% - var(--bottom-nav-height));
  overflow-y: auto;
  -webkit-overflow-scrolling: touch; /* iOS 平滑滚动 */
}
```

### 2. 触摸交互问题

#### 问题类型

- 按钮太小难以点击
- 点击反馈不明显
- 误触问题
- 滑动不流畅

#### 解决方案

**A. 最小触摸目标**

```css
/* 按钮最小尺寸 */
.button {
  min-height: 48px; /* 48dp = ~44pt iOS */
  min-width: 48px;
  padding: 12px 24px;
}

/* 小图标按钮 - 增加触摸区域 */
.icon-button {
  width: 48px;
  height: 48px;
  padding: 12px; /* icon 24px + padding 12px = 48px */
}
```

**B. 点击反馈**

```css
/* 触摸反馈效果 */
.button:active,
.card:active {
  transform: scale(0.98);
  opacity: 0.8;
  transition: all 0.1s ease;
}

/* 波纹效果 */
.button {
  position: relative;
  overflow: hidden;
}

.button::after {
  content: "";
  position: absolute;
  top: 50%;
  left: 50%;
  width: 0;
  height: 0;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.3);
  transform: translate(-50%, -50%);
  transition:
    width 0.3s,
    height 0.3s;
}

.button:active::after {
  width: 200%;
  height: 200%;
}
```

**C. 防止误触**

```css
/* 相邻可点击元素间距 */
.button-group > .button {
  margin: 8px; /* 至少 8dp 间距 */
}

/* 禁用双击缩放 */
* {
  touch-action: manipulation;
}
```

### 3. 字体与文本问题

#### 问题类型

- 字体太小难以阅读
- 文本截断
- 行高不合理
- 颜色对比度不足

#### 解决方案

**A. 字体大小规范**

```css
/* 字体大小规范 (基于 16px = 1rem) */
:root {
  /* 标题 */
  --font-size-h1: 24px; /* 1.5rem */
  --font-size-h2: 20px; /* 1.25rem */
  --font-size-h3: 18px; /* 1.125rem */

  /* 正文 */
  --font-size-body: 16px; /* 1rem - 最小可读大小 */
  --font-size-body-sm: 14px; /* 0.875rem */

  /* 辅助文本 */
  --font-size-caption: 12px; /* 0.75rem - 不低于此 */

  /* 行高 */
  --line-height-tight: 1.25;
  --line-height-normal: 1.5;
  --line-height-relaxed: 1.75;
}

/* 应用 */
body {
  font-size: var(--font-size-body);
  line-height: var(--line-height-normal);
}

h1 {
  font-size: var(--font-size-h1);
}
h2 {
  font-size: var(--font-size-h2);
}
h3 {
  font-size: var(--font-size-h3);
}
```

**B. 文本换行与截断**

```css
/* 多行文本 */
.text-multiline {
  word-wrap: break-word;
  word-break: break-word;
  overflow-wrap: break-word;
}

/* 单行截断 */
.text-ellipsis {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* 多行截断 */
.text-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
```

**C. 颜色对比度**

```css
/* 确保符合 WCAG AA 标准 (4.5:1) */
:root {
  /* 深色背景 + 浅色文本 */
  --bg-dark: #1a1a1a;
  --text-light: #ffffff; /* 对比度 14.6:1 ✓ */

  /* 浅色背景 + 深色文本 */
  --bg-light: #ffffff;
  --text-dark: #333333; /* 对比度 12.6:1 ✓ */

  /* 辅助文本 */
  --text-secondary: #666666; /* 对比度 5.7:1 ✓ */
  --text-tertiary: #999999; /* 对比度 2.8:1 ✗ - 仅用于装饰 */
}
```

### 4. 间距与视觉层次问题

#### 问题类型

- 元素过于拥挤
- 视觉层次不清晰
- 页面边距不统一
- 卡片间距不合理

#### 解决方案

**A. 间距系统**

```css
/* 8dp 间距系统 */
:root {
  --spacing-xs: 4px; /* 0.25rem */
  --spacing-sm: 8px; /* 0.5rem */
  --spacing-md: 16px; /* 1rem */
  --spacing-lg: 24px; /* 1.5rem */
  --spacing-xl: 32px; /* 2rem */
  --spacing-2xl: 48px; /* 3rem */
}

/* 页面容器 */
.page {
  padding: var(--spacing-md); /* 16px 标准边距 */
}

/* 卡片 */
.card {
  padding: var(--spacing-md);
  margin-bottom: var(--spacing-md);
}

/* 列表项 */
.list-item {
  padding: var(--spacing-md) var(--spacing-md);
  gap: var(--spacing-sm);
}
```

**B. 视觉层次**

```css
/* 使用阴影和边框建立层次 */
.card {
  background: var(--bg-card);
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.card-elevated {
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
}

/* 使用间距建立分组 */
.section {
  margin-bottom: var(--spacing-xl); /* 32px 分组间距 */
}

.section > * + * {
  margin-top: var(--spacing-md); /* 16px 元素间距 */
}
```

### 5. 图片与图标问题

#### 问题类型

- 图片模糊或失真
- 图标大小不一致
- 加载状态缺失
- 图片过大导致性能问题

#### 解决方案

**A. 图标规范**

```css
/* 图标大小规范 */
:root {
  --icon-sm: 16px;
  --icon-md: 24px;
  --icon-lg: 32px;
  --icon-xl: 48px;
}

.icon {
  width: var(--icon-md);
  height: var(--icon-md);
  flex-shrink: 0;
}

/* SVG 图标 */
svg.icon {
  fill: currentColor;
  stroke: currentColor;
}
```

**B. 图片优化**

```css
/* 响应式图片 */
.image {
  width: 100%;
  height: auto;
  object-fit: cover;
}

/* 固定比例容器 */
.image-container {
  position: relative;
  width: 100%;
  padding-top: 75%; /* 4:3 比例 */
  overflow: hidden;
}

.image-container img {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  object-fit: cover;
}
```

**C. 加载状态**

```css
/* 骨架屏 */
.skeleton {
  background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
  background-size: 200% 100%;
  animation: skeleton-loading 1.5s infinite;
}

@keyframes skeleton-loading {
  0% {
    background-position: 200% 0;
  }
  100% {
    background-position: -200% 0;
  }
}
```

### 6. 输入控件问题

#### 问题类型

- 输入框太小
- 键盘弹出遮挡内容
- 输入类型不正确
- 占位符不清晰

#### 解决方案

**A. 输入框规范**

```css
/* 输入框样式 */
.input {
  width: 100%;
  height: 48px; /* 最小 48dp */
  padding: 12px 16px;
  font-size: 16px; /* 防止 iOS 自动缩放 */
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: var(--bg-input);
}

.input:focus {
  outline: none;
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px rgba(var(--primary-rgb), 0.1);
}

/* 文本域 */
.textarea {
  min-height: 120px;
  resize: vertical;
}
```

**B. 键盘处理**

```rust
// frontend/src/pages/login.rs
// 示例: 键盘弹出时滚动到输入框

use leptos::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    let input_ref = NodeRef::<Input>::new();

    let on_focus = move |_| {
        // 滚动到输入框
        if let Some(input) = input_ref.get() {
            input.scroll_into_view_with_bool(true);
        }
    };

    view! {
        <input
            node_ref=input_ref
            on:focus=on_focus
            type="tel"
            inputmode="numeric"
        />
    }
}
```

### 7. 加载与状态反馈问题

#### 问题类型

- 加载动画不明显
- Toast 位置不合理
- 错误提示不清晰
- 空状态页面缺失

#### 解决方案

**A. 加载动画**

```css
/* 加载旋转动画 */
.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid rgba(0, 0, 0, 0.1);
  border-top-color: var(--primary-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* 全屏加载遮罩 */
.loading-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}
```

**B. Toast 提示**

```css
/* Toast 位置 */
.toast-container {
  position: fixed;
  top: calc(env(safe-area-inset-top) + 16px);
  left: 16px;
  right: 16px;
  z-index: 10000;
  pointer-events: none;
}

.toast {
  padding: 12px 16px;
  background: var(--bg-toast);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  margin-bottom: 8px;
  pointer-events: auto;
  animation: toast-in 0.3s ease;
}

@keyframes toast-in {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
```

## 具体问题技术方案 (FR-017-009 至 FR-017-014)

本章节针对真实设备测试中发现的 6 个具体问题,提供详细的技术实施方案。

### 问题 1: 偏好选择 Modal 高度适配 (FR-017-009)

#### 当前实现

**文件**: `frontend/src/styles/app.css` (L2152-2188)

```css
.preference-guide-card {
  width: min(520px, 100%);
  padding: 20px;
  /* ❌ 没有高度限制 */
}
```

#### 技术方案

**方案**: 添加响应式高度限制和媒体查询

**文件修改**: `frontend/src/styles/app.css`

```css
/* 修改 .preference-guide-card */
.preference-guide-card {
  width: min(520px, 100%);
  max-height: 85vh; /* ✅ 限制最大高度 */
  padding: 16px; /* ✅ 减小内边距 */
  display: flex;
  flex-direction: column;
  gap: 12px; /* ✅ 减小间距 */
  overflow-y: auto; /* ✅ 内容过多时滚动 */
  -webkit-overflow-scrolling: touch;
  scroll-behavior: smooth;
}

/* 小屏设备优化 (iPhone SE/8 等) */
@media (max-height: 667px) {
  .preference-guide-card {
    max-height: 80vh;
    padding: 12px;
    gap: 10px;
  }

  .preference-guide-title {
    font-size: 16px; /* 从 18px 减小 */
  }

  .preference-guide-subtitle {
    font-size: 13px; /* 从 14px 减小 */
  }

  .preference-cards {
    gap: 10px;
  }

  .preference-card {
    padding: 14px 10px;
  }

  .preference-card-icon {
    font-size: 28px; /* 从 32px 减小 */
  }
}

/* 超小屏设备 (iPhone SE 1st gen) */
@media (max-height: 568px) {
  .preference-guide-card {
    max-height: 75vh;
    padding: 10px;
    gap: 8px;
  }
}
```

**实施步骤**:

1. 定位到 `app.css` L2163 `.preference-guide-card`
2. 添加 `max-height` 和 `overflow-y`
3. 添加媒体查询
4. 在模拟器上测试不同屏幕尺寸

---

### 问题 2: 滚动区域隔离 (FR-017-010)

#### 当前实现

**文件**: `frontend/src/styles/app.css` (L49-83)

```css
.app-shell {
  overflow-y: auto; /* ❌ 整个 shell 可滚动 */
}

.page {
  padding: 24px 20px; /* ❌ padding 和内容混在一起 */
  /* ❌ 没有明确的滚动容器 */
}
```

#### 技术方案

**方案**: 重构滚动容器层次结构

**布局结构**:

```
.app-shell (overflow: hidden)
  └─ .page (overflow: hidden)
      ├─ .page-header (fixed, optional)
      ├─ .page-scrollable-content (overflow-y: auto) ⭐
      └─ .bottom-nav (fixed)
```

**CSS 修改**: `frontend/src/styles/app.css`

```css
/* 1. app-shell 不滚动 */
.app-shell {
  height: 100vh;
  width: 100vw;
  display: flex;
  flex-direction: column; /* ✅ 改为纵向布局 */
  padding-top: var(--statusbar-height);
  padding-right: env(safe-area-inset-right, 0);
  padding-bottom: 0;
  padding-left: env(safe-area-inset-left, 0);
  background: linear-gradient(to bottom, #d1fae5 0, var(--bg) 120px);
  box-sizing: border-box;
  overflow: hidden; /* ✅ 禁止滚动 */
}

/* 2. page 作为容器 */
.page {
  width: 100%;
  max-width: 100%;
  background: var(--card);
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 0;
  overflow: hidden; /* ✅ 本身不滚动 */
}

/* 3. 新增:可滚动内容区域 */
.page-scrollable-content {
  flex: 1;
  overflow-y: auto; /* ✅ 只有这里可以滚动 */
  overflow-x: hidden;
  display: flex;
  flex-direction: column;
  gap: 20px;
  -webkit-overflow-scrolling: touch;
  scroll-behavior: smooth;
  transform: translateZ(0); /* GPU 加速 */
}
```

**Rust 组件修改**: 所有页面组件

**修改模式**:

```rust
// 修改前
view! {
    <section class="page page-xxx">
        // 内容直接在这里
        <div class="content">...</div>
    </section>
}

// 修改后
view! {
    <section class="page page-xxx">
        {/* 固定头部(可选) */}
        <div class="page-header">...</div>

        {/* ✅ 可滚动内容 */}
        <div class="page-scrollable-content">
            <div class="content">...</div>
        </div>
    </section>
}
```

**需要修改的文件**:

- `frontend/src/pages/capture.rs`
- `frontend/src/pages/profile.rs`
- `frontend/src/pages/login.rs`
- `frontend/src/pages/history.rs`
- `frontend/src/pages/result.rs`
- `frontend/src/pages/detail.rs`
- `frontend/src/pages/summary.rs`
- `frontend/src/pages/analyzing.rs`
- `frontend/src/pages/ocr.rs`
- `frontend/src/pages/confirm.rs`

**实施步骤**:

1. 修改 CSS (10 分钟)
2. 逐个修改页面组件 (40 分钟)
3. 测试所有页面滚动行为 (10 分钟)

---

### 问题 3: 首页元素尺寸精细化 (FR-017-011)

#### 当前实现

**文件**: `frontend/src/styles/app.css`

```css
.primary-cta {
  height: 58px; /* ❌ 过高 */
  font-size: 16px;
}

.steps-list .step-icon {
  width: 58px; /* ❌ 过大 */
  height: 58px;
}
```

#### 技术方案

**方案**: 调整按钮和步骤元素尺寸

**文件修改**: `frontend/src/styles/app.css`

```css
/* 1. 主按钮 */
.primary-cta {
  height: 50px; /* ✅ 从 58px 减小 */
  border-radius: 18px; /* ✅ 从 22px 减小 */
  font-size: 15px; /* ✅ 从 16px 减小 */
  box-shadow: 0 10px 20px rgba(16, 185, 129, 0.3);
}

/* 2. 步骤卡片 */
.steps-card {
  padding: 16px; /* ✅ 从 20px 减小 */
}

.card-title {
  font-size: 15px; /* ✅ 从 16px 减小 */
  margin: 0 0 14px 0;
}

.steps-list {
  gap: 14px; /* ✅ 从 16px 减小 */
}

/* 3. 步骤图标 */
.steps-list .step-icon {
  width: 50px; /* ✅ 从 58px 减小 */
  height: 50px;
  border-radius: 16px;
  box-shadow: 0 8px 16px rgba(16, 185, 129, 0.12);
}

.steps-list .step-number {
  width: 22px; /* ✅ 从 24px 减小 */
  height: 22px;
  font-size: 11px;
}

.steps-list .step-icon .icon {
  width: 20px; /* ✅ 从 22px 减小 */
  height: 20px;
}

/* 4. 步骤文本 */
.steps-list .step-content h3 {
  font-size: 14px; /* ✅ 从 15px 减小 */
  margin: 0 0 3px 0;
}

.steps-list .step-content p {
  font-size: 12px; /* ✅ 从 13px 减小 */
  line-height: 1.4;
}

/* 5. 小屏优化 */
@media (max-width: 375px) {
  .primary-cta {
    height: 46px;
    font-size: 14px;
  }

  .steps-list .step-icon {
    width: 46px;
    height: 46px;
  }
}
```

**实施步骤**:

1. 定位并修改相关样式 (20 分钟)
2. 在模拟器上测试首页 (10 分钟)

---

### 问题 4: 按钮尺寸统一规范 (FR-017-012)

#### 当前实现

**文件**: `frontend/src/styles/app.css` + `bottom-nav.css`

```css
.primary-cta {
  height: 58px;
}
.secondary-cta {
  height: 56px;
} /* ❌ 不一致 */
.primary-button {
  /* ❌ 没有明确高度 */
}
```

#### 技术方案

**方案**: 建立统一按钮尺寸规范

**按钮尺寸标准**:
| 类型 | 高度 | 字号 | 用途 |
|------|------|------|------|
| `.primary-cta` | 48px | 15px | 主要操作 |
| `.secondary-cta` | 44px | 14px | 次要操作 |
| `.primary-button` | 46px | 15px | 通用主按钮 |
| `.link-button` | 36px | 14px | 链接按钮 |

**文件修改**: `frontend/src/styles/app.css`

```css
/* 1. 主要操作按钮 */
.primary-cta {
  width: 100%;
  height: 48px; /* ✅ 统一高度 */
  border-radius: 16px;
  font-size: 15px;
  font-weight: 600;
  background: linear-gradient(135deg, #10b981, #059669);
  box-shadow: 0 8px 16px rgba(16, 185, 129, 0.25);
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 2. 次要操作按钮 */
.secondary-cta {
  width: 100%;
  height: 44px; /* ✅ 从 56px 减小 */
  border-radius: 14px;
  font-size: 14px;
  font-weight: 600;
  border: 1px solid #d7e2eb;
  background: #ffffff;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 3. 通用主按钮 */
.primary-button {
  width: 100%;
  height: 46px; /* ✅ 明确高度 */
  border-radius: 14px;
  font-size: 15px;
  font-weight: 600;
  background: linear-gradient(135deg, #10b981, #059669);
  box-shadow: 0 6px 14px rgba(16, 185, 129, 0.25);
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 4. 链接按钮 */
.link-button {
  height: 36px;
  padding: 0 16px;
  font-size: 14px;
  font-weight: 500;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

/* 5. 响应式 */
@media (max-width: 375px) {
  .primary-cta {
    height: 44px;
    font-size: 14px;
  }
  .secondary-cta {
    height: 42px;
    font-size: 13px;
  }
  .primary-button {
    height: 44px;
    font-size: 14px;
  }
}
```

**实施步骤**:

1. 修改所有按钮样式 (30 分钟)
2. 测试所有页面按钮显示 (10 分钟)

---

### 问题 5: 顶部背景对比度优化 (FR-017-013)

#### 当前实现

**文件**: `frontend/src/styles/app.css` (L62)

```css
.app-shell {
  background: linear-gradient(to bottom, #d1fae5 0, var(--bg) 120px);
  /* ❌ 浅色背景,黑色状态栏文字对比度不足 */
}
```

#### 技术方案

**方案 A**: 使用更深的背景色 (推荐)

**文件修改**: `frontend/src/styles/app.css`

```css
.app-shell {
  background: linear-gradient(
    to bottom,
    #a7f3d0 0,
    /* ✅ 更深的绿色,对比度更好 */ var(--bg) 120px
  );
}
```

**方案 B**: 配置系统状态栏样式

**文件修改**: `src-tauri/tauri.conf.json`

```json
{
  "tauri": {
    "windows": [
      {
        "statusBarStyle": "dark-content" /* 黑色状态栏文字 */
      }
    ]
  }
}
```

**推荐**: 组合使用方案 A + 方案 B

**实施步骤**:

1. 修改背景色 (5 分钟)
2. 配置 Tauri (5 分钟)
3. 在不同系统主题下测试 (5 分钟)

---

### 问题 6: 底部导航栏尺寸优化 (FR-017-014)

#### 当前实现

**文件**: `frontend/src/styles/bottom-nav.css`

```css
.bottom-nav {
  height: 68px; /* ❌ 过高 */
}

.tab-icon {
  font-size: 22px; /* ❌ 偏大 */
}

.tab-label {
  font-size: 10px;
}
```

#### 技术方案

**方案**: 调整导航栏高度和图标文字尺寸

**文件修改**: `frontend/src/styles/bottom-nav.css`

```css
/* 1. 导航栏容器 */
.bottom-nav {
  height: 45px; /* ✅ 从 68px 减小 */
  padding: 2px env(safe-area-inset-right) env(safe-area-inset-bottom)
    env(safe-area-inset-left);
  box-shadow: 0 -4px 16px rgba(15, 23, 42, 0.08);
}

/* 2. Tab 项 */
.tab-item {
  padding: 4px 0; /* ✅ 从 6px 减小 */
  gap: 4px; /* ✅ 从 6px 减小 */
}

/* 3. 图标 */
.tab-icon {
  font-size: 20px; /* ✅ 从 22px 减小 */
}

/* 4. 文字 */
.tab-label {
  font-size: 11px; /* ✅ 从 10px 微调 */
  font-weight: 500; /* ✅ 从 600 减小 */
}

/* 5. 激活状态背景 */
.tab-item.active .tab-icon::before {
  inset: -6px; /* ✅ 从 -8px 减小 */
  border-radius: 10px; /* ✅ 从 12px 减小 */
}

/* 6. 相应调整内容区域 padding */
.main-content {
  padding-bottom: calc(45px + env(safe-area-inset-bottom, 0));
  /* ✅ 从 68px 改为 45px */
}

/* 7. 响应式 */
@media (max-width: 375px) {
  .bottom-nav {
    height: 42px;
  }

  .tab-icon {
    font-size: 18px;
  }

  .tab-label {
    font-size: 10px;
  }
}
```

**实施步骤**:

1. 修改 `bottom-nav.css` (10 分钟)
2. 测试所有页面导航栏显示 (5 分钟)

---

## 实施优先级与时间规划

### 优先级排序

1. **P0 (立即修复)** - 30 分钟
   - 问题 5: 顶部背景对比度 (15 分钟)
   - 问题 6: 底部导航栏 (15 分钟)

2. **P1 (高优先级)** - 1.5 小时
   - 问题 1: Modal 高度 (20 分钟)
   - 问题 4: 按钮统一 (40 分钟)
   - 问题 3: 首页元素 (30 分钟)

3. **P2 (中优先级)** - 1 小时
   - 问题 2: 滚动区域 (60 分钟)

**总计**: 约 3 小时

### 实施步骤

#### 第一天 (1.5 小时)

**上午**: P0 + P1 部分

1. 问题 5: 背景透明度 (15 分钟)
2. 问题 6: 底部导航栏 (15 分钟)
3. 问题 1: Modal 高度 (20 分钟)
4. 问题 4: 按钮统一 (40 分钟)

**验收**: 在模拟器上测试,确认视觉改善

#### 第二天 (1.5 小时)

**上午**: P1 剩余 + P2

1. 问题 3: 首页元素 (30 分钟)
2. 问题 2: 滚动区域 - CSS (10 分钟)
3. 问题 2: 滚动区域 - 组件 (40 分钟)
4. 全面测试 (10 分钟)

**验收**: 所有问题已修复,滚动行为正确

## 重点页面优化计划

### 1. 拍照页面 (`capture.rs`)

#### 当前问题 (待在模拟器上确认)

- 相机预览区域大小
- 按钮位置和大小
- 图片预览样式

#### 优化检查清单

```markdown
- [ ] 相机预览区域占据合理空间
- [ ] 拍照按钮足够大 (≥ 64px)
- [ ] 拍照按钮位置符合人体工程学
- [ ] 切换相机/相册按钮易于点击
- [ ] 图片预览清晰
- [ ] 重拍/确认按钮明显
```

### 2. 结果页面 (`result.rs`)

#### 当前问题 (待在模拟器上确认)

- 配料卡片间距
- 健康评分显示
- 滚动性能

#### 优化检查清单

```markdown
- [ ] 健康评分卡片醒目
- [ ] 配料列表滚动流畅
- [ ] 配料卡片信息清晰
- [ ] 风险标签颜色对比度足够
- [ ] 底部导航不遮挡内容
```

### 3. 历史页面 (`history.rs`)

#### 当前问题 (待在模拟器上确认)

- 卡片布局
- 缩略图显示
- 列表滚动

#### 优化检查清单

```markdown
- [ ] 卡片布局紧凑但不拥挤
- [ ] 缩略图加载流畅
- [ ] 列表滚动性能良好
- [ ] 空状态友好
- [ ] 下拉刷新体验好
```

## 测试流程

### 1. 视觉回归测试

```bash
# 在每个关键页面截图
adb exec-out screencap -p > screenshots/capture-page.png
adb exec-out screencap -p > screenshots/result-page.png
adb exec-out screencap -p > screenshots/history-page.png

# 对比前后截图,确认改进
```

### 2. 交互测试

```markdown
测试用例:

- [ ] 点击所有按钮,确认反馈明显
- [ ] 滚动所有列表,确认流畅
- [ ] 输入文本,确认键盘不遮挡
- [ ] 切换页面,确认动画流畅
- [ ] 旋转屏幕,确认布局适配(如适用)
```

### 3. 性能测试

```bash
# 使用 Chrome DevTools 检查性能
# 1. 打开 chrome://inspect/#devices
# 2. 点击 "inspect"
# 3. 切换到 "Performance" 标签
# 4. 录制交互过程
# 5. 分析 FPS、渲染时间
```

## 实施阶段

### 阶段 1: 环境设置与问题识别 (Day 1)

- [ ] 配置 Android 模拟器
- [ ] 测试应用构建和安装
- [ ] 建立快速迭代工作流
- [ ] 在模拟器上测试所有页面
- [ ] 记录发现的所有 UI 问题
- [ ] 对问题进行优先级排序

### 阶段 2: 布局与间距优化 (Day 2-3)

- [ ] 修复内容超出边界问题
- [ ] 调整页面边距和内边距
- [ ] 优化卡片和列表项间距
- [ ] 处理安全区域
- [ ] 测试不同屏幕尺寸

### 阶段 3: 触摸交互优化 (Day 3-4)

- [ ] 增大按钮触摸区域
- [ ] 添加点击反馈效果
- [ ] 优化滑动交互
- [ ] 测试所有交互元素

### 阶段 4: 字体与文本优化 (Day 4-5)

- [ ] 调整字体大小
- [ ] 优化行高和字重
- [ ] 修复文本换行问题
- [ ] 检查颜色对比度

### 阶段 5: 细节优化与测试 (Day 5-6)

- [ ] 优化图片和图标显示
- [ ] 改进加载和错误状态
- [ ] 优化输入控件
- [ ] 全面测试所有页面
- [ ] 修复遗留问题

### 阶段 6: 文档与总结 (Day 6)

- [ ] 更新设计系统文档
- [ ] 记录移动端 UI 最佳实践
- [ ] 创建移动端 UI 检查清单
- [ ] 总结经验教训

## 最佳实践指南

### CSS 编写规范

```css
/* 1. 使用相对单位 */
.element {
  font-size: 1rem; /* ✓ 相对单位 */
  padding: 16px; /* ✓ 固定间距可用 px */
  width: 100%; /* ✓ 百分比 */
}

/* 2. 使用 CSS 变量 */
.element {
  color: var(--text-primary); /* ✓ */
  background: #333333; /* ✗ 硬编码 */
}

/* 3. 移动优先 */
.element {
  font-size: 16px; /* 移动端默认 */
}

@media (min-width: 768px) {
  .element {
    font-size: 18px; /* 平板/桌面端 */
  }
}

/* 4. 使用 Flexbox/Grid */
.container {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}
```

### 组件开发规范

```rust
// 1. 使用语义化 HTML
view! {
    <button class="button-primary">  // ✓
        "确认"
    </button>

    <div class="button-primary">     // ✗
        "确认"
    </div>
}

// 2. 提供可访问性属性
view! {
    <button
        class="icon-button"
        aria-label="关闭"  // ✓
    >
        <IconClose />
    </button>
}

// 3. 使用响应式 Signal
let is_loading = RwSignal::new(false);

view! {
    <button
        class="button"
        disabled=move || is_loading.get()  // ✓
    >
        {move || if is_loading.get() {
            "加载中..."
        } else {
            "提交"
        }}
    </button>
}
```

## 移动端 UI 检查清单

### 布局

- [ ] 内容不超出屏幕边界
- [ ] 安全区域处理正确
- [ ] 滚动区域明确
- [ ] 固定定位元素位置正确
- [ ] 不同屏幕尺寸显示正常

### 交互

- [ ] 按钮最小 48x48 dp
- [ ] 点击反馈明显
- [ ] 相邻元素间距 ≥ 8dp
- [ ] 滚动流畅(60fps)
- [ ] 手势响应正常

### 文本

- [ ] 正文字体 ≥ 14sp
- [ ] 行高合理(1.5)
- [ ] 长文本自动换行
- [ ] 颜色对比度 ≥ 4.5:1
- [ ] 中英文混排正常

### 视觉

- [ ] 页面边距统一(16dp)
- [ ] 元素间距使用 8dp 系统
- [ ] 视觉层次清晰
- [ ] 图标大小统一
- [ ] 图片加载状态明确

### 状态

- [ ] 加载动画明显
- [ ] Toast 位置合理
- [ ] 错误提示清晰
- [ ] 空状态友好
- [ ] 成功反馈及时

## 常见问题与解决方案

### 问题 1: 页面在模拟器上显示不全

**原因**: 未处理安全区域,或高度计算错误

**解决**:

```css
.page-content {
  height: calc(
    100vh - env(safe-area-inset-top) - env(safe-area-inset-bottom) -
      var(--bottom-nav-height)
  );
}
```

### 问题 2: 按钮点击没反应

**原因**: 触摸区域太小,或被其他元素遮挡

**解决**:

```css
.button {
  min-height: 48px;
  min-width: 48px;
  z-index: 1;
}
```

### 问题 3: 滚动卡顿

**原因**: 列表项过于复杂,或动画过多

**解决**:

```css
/* 启用硬件加速 */
.list-item {
  transform: translateZ(0);
  will-change: transform;
}

/* 简化动画 */
.list-item {
  transition: transform 0.2s ease;
}
```

### 问题 4: 键盘遮挡输入框

**原因**: 未处理键盘弹出事件

**解决**:

```rust
// 输入框获得焦点时滚动到视图
let on_focus = move |_| {
    if let Some(input) = input_ref.get() {
        input.scroll_into_view_with_bool(true);
    }
};
```

## 风险与缓解

| 风险                    | 影响 | 可能性 | 缓解措施                   |
| ----------------------- | ---- | ------ | -------------------------- |
| 模拟器性能不代表真机    | 中   | 中     | 在真机上进行最终测试       |
| CSS 改动影响桌面端      | 中   | 低     | 使用媒体查询隔离移动端样式 |
| 构建时间过长影响迭代    | 低   | 中     | 使用热重载模式开发         |
| 不同 Android 版本兼容性 | 低   | 低     | 测试最低支持版本(API 26)   |

## 参考资料

- [Material Design - Layout](https://m3.material.io/foundations/layout/understanding-layout/overview)
- [Android Design Guidelines](https://developer.android.com/design)
- [Tauri Android Guide](https://tauri.app/v2/guides/building/android/)
- [Leptos Documentation](https://docs.rs/leptos/latest/leptos/)
- 项目设计系统: `docs/design/figma-design-system.md`

---

## 变更记录

| 版本 | 日期       | 作者          | 描述                                                                    |
| ---- | ---------- | ------------- | ----------------------------------------------------------------------- |
| 1.0  | 2026-01-31 | Claude + User | 初始版本                                                                |
| 2.0  | 2026-01-31 | Claude + User | 新增"具体问题技术方案"章节,详细说明 FR-017-009 至 FR-017-014 的实施方案 |
