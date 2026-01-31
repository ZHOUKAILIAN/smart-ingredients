# 017-移动端 UI 精细化优化技术方案

## 元数据

| 字段     | 值                                   |
| -------- | ------------------------------------ |
| 文档编号 | 017-mobile-ui-refinement             |
| 标题     | 移动端 UI 精细化优化技术方案         |
| 版本     | 1.0                                  |
| 状态     | 草稿                                 |
| 创建日期 | 2026-01-31                           |
| 更新日期 | 2026-01-31                           |
| 作者     | Claude + User                        |
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

| 设备名称 | 屏幕尺寸 | 分辨率 | 密度 | 用途 |
|---------|---------|--------|------|------|
| Pixel 6 | 6.4" | 1080x2400 | 420dpi | 主要测试设备 |
| Pixel 6 Pro | 6.7" | 1440x3120 | 560dpi | 大屏高密度测试 |
| Pixel 4a | 5.8" | 1080x2340 | 440dpi | 中等屏幕测试 |

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
html, body {
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
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  width: 0;
  height: 0;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.3);
  transform: translate(-50%, -50%);
  transition: width 0.3s, height 0.3s;
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

h1 { font-size: var(--font-size-h1); }
h2 { font-size: var(--font-size-h2); }
h3 { font-size: var(--font-size-h3); }
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
  --spacing-xs: 4px;   /* 0.25rem */
  --spacing-sm: 8px;   /* 0.5rem */
  --spacing-md: 16px;  /* 1rem */
  --spacing-lg: 24px;  /* 1.5rem */
  --spacing-xl: 32px;  /* 2rem */
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
  background: linear-gradient(
    90deg,
    #f0f0f0 25%,
    #e0e0e0 50%,
    #f0f0f0 75%
  );
  background-size: 200% 100%;
  animation: skeleton-loading 1.5s infinite;
}

@keyframes skeleton-loading {
  0% { background-position: 200% 0; }
  100% { background-position: -200% 0; }
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
  to { transform: rotate(360deg); }
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
  padding: 16px;   /* ✓ 固定间距可用 px */
  width: 100%;     /* ✓ 百分比 */
}

/* 2. 使用 CSS 变量 */
.element {
  color: var(--text-primary); /* ✓ */
  background: #333333;         /* ✗ 硬编码 */
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
  height: calc(100vh - env(safe-area-inset-top) - env(safe-area-inset-bottom) - var(--bottom-nav-height));
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

| 风险 | 影响 | 可能性 | 缓解措施 |
|------|------|--------|----------|
| 模拟器性能不代表真机 | 中 | 中 | 在真机上进行最终测试 |
| CSS 改动影响桌面端 | 中 | 低 | 使用媒体查询隔离移动端样式 |
| 构建时间过长影响迭代 | 低 | 中 | 使用热重载模式开发 |
| 不同 Android 版本兼容性 | 低 | 低 | 测试最低支持版本(API 26) |

## 参考资料

- [Material Design - Layout](https://m3.material.io/foundations/layout/understanding-layout/overview)
- [Android Design Guidelines](https://developer.android.com/design)
- [Tauri Android Guide](https://tauri.app/v2/guides/building/android/)
- [Leptos Documentation](https://docs.rs/leptos/latest/leptos/)
- 项目设计系统: `docs/design/figma-design-system.md`

---

## 变更记录

| 版本 | 日期 | 作者 | 描述 |
|------|------|------|------|
| 1.0 | 2026-01-31 | Claude + User | 初始版本 |
