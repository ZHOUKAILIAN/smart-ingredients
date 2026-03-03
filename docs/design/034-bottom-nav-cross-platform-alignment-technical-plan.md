# 034-底部导航跨端对齐修复技术方案

## 元数据

| 字段 | 值 |
| --- | --- |
| 文档编号 | 034-bottom-nav-cross-platform-alignment-technical-plan |
| 标题 | 底部导航跨端对齐修复技术方案 |
| 版本 | 1.0 |
| 状态 | 草稿 |
| 创建日期 | 2026-02-26 |
| 更新日期 | 2026-02-26 |
| 关联需求 | 034-bottom-nav-cross-platform-alignment |

## 问题定位

当前实现将以下因素叠加：
- `nav` 使用固定高度 + `padding-bottom: env(safe-area-inset-bottom)`。
- Tab 子项使用 `h-full` 和额外内边距。
- 主内容区 `MainLayout` 通过固定值 + safe-area 预留底部空间。

在 Android WebView 中上述叠加会产生与浏览器不同的布局计算，导致底部导航元素视觉中心偏移。

## 设计原则

- 把 safe-area 与可视导航高度拆分处理，避免高度和内边距重复计入。
- 导航容器内部增加一个“内容层”，由内容层负责图标文字居中。
- 主内容区底部预留值与导航真实占位一致。

## 实施方案

1. `BottomNav` 结构调整
- 外层 `nav` 负责固定定位、边框、背景、safe-area 补偿。
- 内层容器（`max-w-[480px] mx-auto`）负责固定可视高度（例如 56px）和 `items-center`。
- Tab 项移除 `h-full + py-1` 组合，改为明确的 `min-h` 与 `justify-center`，保证跨端一致。

2. `MainLayout` 底部预留调整
- 与 BottomNav 可视高度统一（如 56px + safe-area），避免内容区与导航区双重偏移。

3. 回归验证
- 浏览器端检查首页/历史/社区/我的四个 tab。
- Android 模拟器截图比对，确认视觉中心一致。

## 影响范围

- `frontend/src/components/bottom_nav.rs`
- `frontend/src/components/main_layout.rs`

## 风险与回滚

- 风险：极端设备 safe-area 读取差异。
- 缓解：保留 `env(safe-area-inset-bottom,0)` 兜底，且结构化拆分计算。
- 回滚：仅涉及两个组件文件，可快速回退到上一版 class 配置。
