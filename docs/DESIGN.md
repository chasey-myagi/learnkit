# LearnKit UI/UX 设计规范

> 更新日期：2026-03-22

---

## 1. 设计原则

- 禁止 AI slop：无 glow box-shadow、shimmer/pulse/breathe 装饰动画、渐变文字（background-clip: text）、bounce/elastic easing、紫色霓虹配色
- 克制优雅，排版和信息层级说话
- 深色优先，中性色调（zinc/gray），避免过度饱和
- 日间模式用纯净浅灰，不带色彩偏向
- 每个动效有目的，无装饰性动画
- 进度条、圆环等使用实色，不使用渐变

---

## 2. 配色 Token

### 深色模式

```css
--bg: #111113;
--card: #1a1a1d;
--border: #2a2a2e;
--text: #e4e4e7;
--text-secondary: #85858d;
--accent: #3b82f6;
--accent-light: #60a5fa;
--accent-hover: rgba(59, 130, 246, 0.08);
--surface: #161618;
```

### 日间模式

```css
--bg: #fafafa;
--card: #ffffff;
--border: #e4e4e7;
--text: #18181b;
--text-secondary: #71717a;
--accent-hover: rgba(59, 130, 246, 0.06);
```

### 状态色（通用）

```css
--completed: #22c55e;
--in-progress: #eab308;
--prepared: #06b6d4;
--pending: #71717a;
--failed: #ef4444;
```

---

## 3. 排版 Token

| Token | 值 |
|-------|-----|
| 字体 | `system-ui, -apple-system, sans-serif` |
| 圆角 | `--radius: 10px` / `--radius-sm: 6px` |
| 大标题字间距 | `-0.5px` |
| 中标题字间距 | `-0.3px` |
| 标题行高 | `1.2 ~ 1.3` |
| 正文行高 | `1.6` |
| 数字排版 | `font-variant-numeric: tabular-nums` |

---

## 4. 应用壳组件

| 组件 | 方案 | 规格 |
|------|------|------|
| **AppHeader** | 简洁分隔线 | 高度 56px；纯文字 logo（font-semibold，不着色）；底部 1px border 分隔；右侧胶囊形日/夜切换器 |
| **ProgramCard** | B+E 混合 | 两列网格；SVG 图标 + 环形进度指示；纵向排列（图标 → 标题 → 描述 → 进度） |
| **SubjectGroup** | F 手风琴+时间线 | 手风琴折叠 subject；展开态左侧 3px accent 边框；lesson 用时间线串联；chevron 旋转 180°/250ms |
| **ProgressBar** | 实色填充 | 实色 accent 填充（完成态用 completed 色）；无渐变、无 shimmer；圆角与容器一致 |
| **StatusBadge** | Ghost 文字 | 半透明彩色背景（各状态色 12% opacity）；无 glow box-shadow；无呼吸/脉冲动画；字号 12px |

---

## 5. 教案模板

三套风格，每份教案整体使用同一套，不混搭。

| 风格 | 关键特征 | 适用场景 |
|------|---------|---------|
| **A 技术文档** | 左侧固定目录；720px 正文居中；代码块友好；输入框圆角 8px | 编程/技术类 |
| **B 杂志排版** | 大标题留白充裕；serif 正文；沉浸阅读；输入框圆角 12px | 设计/理论类 |
| **C 笔记本** | 纸质纹理背景；荧光笔高亮交互；侧边批注；输入框虚线边框 | 通识/探索类 |

---

## 6. 划词提问交互

### 状态机

```
idle → [选中 ≥2 字符] → input_visible → [回车] → waiting → [首字到达] → streaming → [完成] → card_visible → [关闭/Escape] → idle
```

### 输入框

| 属性 | 值 |
|------|-----|
| 尺寸 | 320px x 36px |
| 字号 | 13px |
| 定位 | `position: absolute`，选区 bottom + scrollY + 8px |
| 圆角 | 8px（A）/ 12px（B）/ 虚线边框（C） |
| placeholder | "输入你的问题..." |
| 入场 | fadeIn + translateY(-4px → 0)，200ms，`cubic-bezier(0.22, 1, 0.36, 1)` |
| 退场 | fadeOut + translateY(0 → -4px)，150ms |
| 关闭 | 回车提交 / Escape / blur |

### Answer Card

| 属性 | 值 |
|------|-----|
| 宽度 | 400px |
| 最大高度 | 320px（overflow-y: auto） |
| 定位 | 选区 bottom + scrollY + 12px |
| 阴影 | `0 4px 20px rgba(0,0,0,0.15)` |
| 结构 | header（标题 "回答" + x 关闭）+ body（流式内容） |
| 入场 | fadeIn + translateY(8px → 0)，250ms，`cubic-bezier(0.22, 1, 0.36, 1)` |
| 流式渲染 | 30ms/字，`\n\n` 分段 |

### 关闭动画（按模板区分）

| 模板 | 动效 | duration |
|------|------|----------|
| A 技术文档 | `translateX(24px)` 右滑出 + fadeOut | 180ms |
| B 杂志 | `scale(0.95)` 缩小 + fadeOut | 200ms |
| C 笔记本 | `translateY(12px) rotate(1deg)` 下滑微旋 + fadeOut | 200ms |

### 边界处理

| 场景 | 处理 |
|------|------|
| 输入框已显示时再次选中 | 移除旧输入框，创建新的 |
| Card 未关闭时再次提问 | 移除旧 card，显示新 card |
| 选区靠近屏幕右侧 | `card.left = min(rect.left, viewport - 420)` |
| 选区在输入框或 card 内部 | 忽略，不弹输入框 |
| 选中文字 < 2 字符 | 忽略 |

---

## 7. 动效规范

| 类别 | duration | easing |
|------|----------|--------|
| 颜色变化 | 200ms | `ease` |
| 位移/变换 | 250ms | `cubic-bezier(0.22, 1, 0.36, 1)` |
| 布局展开 | 300ms | `cubic-bezier(0.22, 1, 0.36, 1)` |
| 退场 | 入场的 75% | `cubic-bezier(0.4, 0, 1, 1)` |

**禁止**：bounce easing、elastic easing、glow box-shadow、shimmer 动画、渐变文字

**强制**：所有动画必须响应 `prefers-reduced-motion: reduce`，命中时全局禁用动画。

---

## 8. 教案内置交互组件

### 测验题（Quiz）

- 选择题：显示问题 + 选项，点击后即时反馈
- 填空题：输入框 + 提交按钮
- 正确 → `--completed` 绿色背景；错误 → `--failed` 红色背景 + 解释文字
- 提交后锁定，防重复点击

### 可展开思考题

- 默认显示问题 + `[展开参考思路]` 按钮
- 点击展开：`max-height: 0 → auto`，300ms，`cubic-bezier(0.22, 1, 0.36, 1)`
- chevron 箭头旋转指示展开/折叠状态

### 可交互流程图

- 嵌入教案正文的 SVG/HTML 流程图
- 支持点击节点展开详细说明

### 荧光笔高亮（仅 C 笔记本）

- 点击 `.highlightable` 文字，荧光效果从左到右渐变出现
- 再次点击取消高亮
- 支持键盘 `Enter` / `Space` 触发
- 无障碍：`role="button"` + `tabindex="0"`

---

## 9. QA 历史面板

| 属性 | 值 |
|------|-----|
| 触发 | 右下角按钮 |
| 宽度 | 360px |
| 位置 | 右侧侧滑面板 |
| 入场 | `translateX(100% → 0)`，300ms |
| 退场 | `translateX(0 → 100%)`，200ms |
| 排序 | 倒序（最新在上） |
| 关闭 | 再次点击按钮 / 点击面板外部 |

---

## 10. 主题切换

| 属性 | 值 |
|------|-----|
| 位置 | AppHeader 右侧胶囊形切换器 |
| 过渡 | 所有颜色属性 `transition: 200ms ease` |
| 持久化 | `localStorage` |
| 默认值 | 跟随系统 `prefers-color-scheme` |
| CSS 实现 | `[data-theme="dark"]` / `[data-theme="light"]` 根属性切换 |
