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

## 8. 教案内置组件

> 完整的 HTML 结构定义见 `docs/COMPONENTS.md`。本节定义每个组件的视觉规格与交互行为。

### 8.1 代码块

| 属性 | 值 |
|------|-----|
| 容器 | `.code-block`，`var(--surface)` 背景，`1px var(--border)` 边框 |
| 语言标签 | 右上角，`12px` 大写字母，`var(--text-secondary)` |
| 文件名栏 | `data-title` 存在时顶部显示，`var(--card)` 背景，底部 `1px` 分隔线 |
| 代码字体 | `var(--mono)`，`13px`，行高 `1.6`，`tab-size: 2` |
| 圆角 | `var(--radius)` |
| 溢出 | `overflow-x: auto`，自定义滚动条 |

### 8.2 多语言代码切换

| 属性 | 值 |
|------|-----|
| 容器 | `.code-tabs`，与代码块相同的背景和边框 |
| Tab 导航 | `.code-tabs-nav`，`var(--card)` 背景，底部 `1px` 分隔线 |
| Tab 按钮 | `13px`，`padding: 8px 16px`，默认 `var(--text-secondary)` |
| 激活态 | `var(--text)` 色 + 底部 `2px var(--accent)` 下划线 |
| 切换 | 无动画，直接显示/隐藏对应面板 |
| 面板 | `.code-tab-panel`，非 `active` 时 `display: none` |

### 8.3 文件树

| 属性 | 值 |
|------|-----|
| 容器 | `.file-tree`，`var(--surface)` 背景，`1px var(--border)` 边框 |
| 标题 | `.file-tree-title`，`13px`，`font-weight: 600`，`var(--text-secondary)` |
| 内容 | `pre` 元素，`var(--mono)`，`13px`，行高 `1.5` |
| 圆角 | `var(--radius)` |
| 特殊字符 | `├── └── │` 使用 `var(--text-secondary)` 色 |

### 8.4 代码 Diff

| 属性 | 值 |
|------|-----|
| 容器 | `.code-diff`，与代码块相同基础样式 |
| 删除行（`-` 开头） | 背景 `rgba(255, 107, 107, 0.1)`，左边框 `3px var(--failed)` |
| 新增行（`+` 开头） | 背景 `rgba(16, 172, 132, 0.1)`，左边框 `3px var(--completed)` |
| 未变行 | 无特殊样式 |

### 8.5 提示框 Callout

四种变体共享基础布局，仅颜色不同。

| 属性 | 值 |
|------|-----|
| 布局 | flex 横排，左侧 `3px` 指示条 + 右侧内容 |
| 圆角 | 右侧 `var(--radius-sm)` |
| 内边距 | `14px 18px` |
| 标题 | `13px`，`font-weight: 600`，与指示条同色 |
| 正文 | `14px`，行高 `1.7` |
| 外间距 | `16px 0 20px` |

变体颜色：

| 类型 | class | 指示条 / 标题色 | 背景 |
|------|-------|----------------|------|
| 提示 | `.callout-tip` | `var(--completed)` 绿 | `rgba(16, 172, 132, 0.06)` |
| 注意 | `.callout-warning` | `var(--in-progress)` 黄 | `rgba(254, 202, 87, 0.06)` |
| 备注 | `.callout-note` | `var(--prepared)` 青 | `rgba(72, 219, 251, 0.06)` |
| 重要 | `.callout-important` | `var(--accent)` 紫蓝 | `rgba(108, 92, 231, 0.06)` |

### 8.6 术语定义

| 属性 | 值 |
|------|-----|
| 容器 | `.definition`，`var(--surface)` 背景，`1px var(--border)` 边框 |
| 术语 | `.definition-term`，`15px`，`font-weight: 600`，`var(--accent-light)` 色 |
| 定义体 | `.definition-body`，`14px`，`var(--text-secondary)` 色，行高 `1.7` |
| 左边框 | `3px var(--accent)` |
| 圆角 | 右侧 `var(--radius-sm)` |
| 内边距 | `16px 18px` |

### 8.7 测验题

#### 通用规格

| 属性 | 值 |
|------|-----|
| 容器 | `.quiz-block`，`var(--surface)` 背景，`1px var(--border)` 边框 |
| 类型标签 | `.quiz-header`，`13px`，`font-weight: 600`，`var(--accent)` 色 |
| 问题 | `.quiz-question`，`15px`，`font-weight: 500`，行高 `1.7` |
| 圆角 | `var(--radius)` |
| 内边距 | `24px` |
| 外间距 | `28px 0` |

#### 单选题 `data-type="single"`

| 属性 | 值 |
|------|-----|
| 选项 | `.quiz-option`，`padding: 12px 16px`，`var(--card)` 背景 |
| 字母圆圈 | `.quiz-option-letter`，`26px` 圆，`var(--surface)` 背景 |
| hover | 边框 `var(--accent)`，背景 `rgba(108, 92, 231, 0.06)` |
| 正确反馈 | 边框+字母 `var(--completed)` 绿，背景 `rgba(16, 172, 132, 0.1)` |
| 错误反馈 | 选中项红色 + 正确项同时高亮绿色 |
| 锁定 | 点击后所有选项 `.answered`，`cursor: default`，非选中项 `opacity: 0.65` |

#### 多选题 `data-type="multi"`

| 属性 | 值 |
|------|-----|
| 选项 | 与单选相同布局，字母圆圈替换为 `.quiz-option-check` 方形复选框 |
| 复选框 | `18px` 方形，`var(--surface)` 背景，`2px var(--border)` 边框，圆角 `4px` |
| 选中态 | 背景 `var(--accent)`，白色 `✓` 图标 |
| 提交按钮 | `.quiz-submit`，`var(--accent)` 背景，白色文字，`padding: 8px 20px` |
| 反馈 | 提交后：正确选项绿色勾，错误选项红色叉，漏选项黄色提示 |

#### 判断题 `data-type="truefalse"`

| 属性 | 值 |
|------|-----|
| 布局 | 两个选项横排，`gap: 12px`，各占 `50%` 宽度 |
| 正确按钮 | 字母圆圈显示 `✓` |
| 错误按钮 | 字母圆圈显示 `✗` |
| 交互 | 与单选题相同 |

#### 填空题 `data-type="fill"`

| 属性 | 值 |
|------|-----|
| 输入框 | `.quiz-fill-input`，行内 `inline-block`，宽度 `120px`，底部 `2px var(--border)` 下划线 |
| 背景 | 透明 |
| 聚焦 | 下划线色变 `var(--accent)` |
| 提交 | `.quiz-submit` 按钮 |
| 正确 | 输入框下划线绿色，文字绿色 |
| 错误 | 输入框下划线红色，正确答案显示在下方 |
| 匹配 | 大小写不敏感，trim 后比较 |

#### 排序题 `data-type="order"`

| 属性 | 值 |
|------|-----|
| 列表项 | `.quiz-order-item`，`padding: 10px 14px`，可拖拽 |
| 拖拽手柄 | `.quiz-order-handle`，`⠿` 符号，`var(--text-secondary)` |
| 拖拽态 | `opacity: 0.5`，虚线边框 |
| 放置指示 | 目标位置上方显示 `2px var(--accent)` 横线 |
| 提交 | `.quiz-submit` 按钮 |
| 正确项 | 左边框 `3px var(--completed)` |
| 错误项 | 左边框 `3px var(--failed)`，显示正确位置编号 |

#### 连线匹配题 `data-type="match"`

| 属性 | 值 |
|------|-----|
| 布局 | 左右两列，`gap: 40px` |
| 匹配项 | `.quiz-match-item`，`padding: 10px 14px`，`var(--card)` 背景 |
| 交互 | 点击左侧项 → 高亮 → 点击右侧项 → 连线 |
| 已连接 | 两侧同色边框（每对一个颜色，从预设色板取） |
| 提交 | `.quiz-submit` 按钮 |
| 正确配对 | 绿色连线 |
| 错误配对 | 红色连线 + 正确配对虚线提示 |

### 8.8 可展开思考题

| 属性 | 值 |
|------|-----|
| 容器 | `.expandable`，`var(--surface)` 背景，`1px var(--border)` 边框 |
| 触发器 | `.expandable-trigger`，`14px`，`font-weight: 500`，flex 布局 |
| hover | 文字色变 `var(--accent)` |
| chevron | `▼`，`12px`，`var(--text-secondary)` |
| 展开动画 | `max-height: 0 → 800px`，`300ms`，`cubic-bezier(0.22, 1, 0.36, 1)` |
| chevron 旋转 | `rotate(180°)`，`250ms`，同上 easing |
| 内容区 | `.expandable-body-inner`，`14px`，`var(--text-secondary)`，`padding: 0 18px 14px` |

### 8.9 对比卡片

| 属性 | 值 |
|------|-----|
| 容器 | `.vs-card`，flex 横排，`var(--surface)` 背景，`1px var(--border)` 边框 |
| 两侧 | `.vs-card-side`，各占 `1fr`，`padding: 20px` |
| 标题 | `.vs-card-header`，`15px`，`font-weight: 600` |
| 分隔 | `.vs-card-divider`，`40px` 宽居中，`var(--text-secondary)` 色，`font-weight: 700` |
| 列表 | `14px`，行高 `1.7` |
| 圆角 | `var(--radius)` |
| 响应式 | `≤ 640px` 时切换为纵排 |

### 8.10 多 Tab 切换

| 属性 | 值 |
|------|-----|
| 容器 | `.content-tabs`，`var(--surface)` 背景，`1px var(--border)` 边框 |
| Tab 导航 | `.content-tabs-nav`，flex 横排，底部 `1px var(--border)` 分隔 |
| Tab 按钮 | `.content-tab`，`13px`，`padding: 10px 18px` |
| 激活态 | `var(--text)` 色 + 底部 `2px var(--accent)` |
| 面板 | `.content-tab-panel`，`padding: 18px`，非 active 时 `display: none` |
| 切换 | 即时切换，无过渡动画 |

### 8.11 步骤条

| 属性 | 值 |
|------|-----|
| 容器 | `.steps`，左侧 `2px var(--border)` 竖线贯穿 |
| 步骤项 | `.step`，flex 横排，`margin-bottom: 24px` |
| 编号 | `.step-number`，`28px` 圆，`var(--accent)` 背景，白色文字，`13px font-weight: 600` |
| 竖线连接 | 编号圆心与上下步骤通过 `2px` 竖线连接 |
| 标题 | `.step-title`，`15px`，`font-weight: 600` |
| 内容 | `14px`，行高 `1.7` |

### 8.12 快捷键标记

| 属性 | 值 |
|------|-----|
| 元素 | `<kbd>` |
| 样式 | `var(--surface)` 背景，`1px var(--border)` 边框+底部 `2px` 加粗边框（3D 按键感） |
| 字体 | `var(--mono)`，`12px` |
| 内边距 | `2px 8px` |
| 圆角 | `4px` |
| `+` 分隔 | 正常文本，两侧 `4px` 间距 |

### 8.13 公式

| 属性 | 值 |
|------|-----|
| 块级容器 | `.formula`，`var(--surface)` 背景，居中对齐 |
| 公式文字 | `.formula-content`，`var(--mono)`，`15px`，`font-weight: 500` |
| 标题 | `.formula-caption`，`12px`，`var(--text-secondary)`，居中 |
| 变量解释 | `.formula-vars`，`13px`，顶部 `1px var(--border)` 分隔线 |
| 行内公式 | `.formula-inline`，`var(--mono)`，`var(--accent-light)` 色 |
| 上下标 | 使用 HTML `<sup>` `<sub>` |

### 8.14 流程图

#### ASCII 流程图

| 属性 | 值 |
|------|-----|
| 容器 | `.diagram`，`var(--surface)` 背景，`1px var(--border)` 边框 |
| 标题 | `.diagram-title`，`13px`，`font-weight: 600`，`var(--text-secondary)` |
| 内容 | `pre.diagram-content`，`var(--mono)`，`13px`，居中对齐 |
| 箭头字符 | `→ ← ↑ ↓ ▶` 用 `var(--accent)` 色 |

#### HTML 流程图

| 属性 | 值 |
|------|-----|
| 节点 | `.flowchart-node`，`var(--card)` 背景，`1px var(--border)` 边框，`padding: 8px 16px` |
| 活跃节点 | `.flowchart-node-active`，边框 `var(--accent)` |
| 危险节点 | `.flowchart-node-danger`，边框 `var(--failed)` |
| 箭头 | `.flowchart-arrow`，`var(--text-secondary)`，`13px` |

### 8.15 时间线

| 属性 | 值 |
|------|-----|
| 容器 | `.timeline`，左侧 `2px var(--border)` 竖线 |
| 标记点 | `.timeline-marker`，`10px` 实心圆，`var(--accent)` 色，覆盖在竖线上 |
| 日期 | `.timeline-date`，`12px`，`font-weight: 600`，`var(--accent-light)` 色 |
| 标题 | `.timeline-title`，`15px`，`font-weight: 600` |
| 内容 | `14px`，行高 `1.7`，`var(--text-secondary)` |
| 间距 | 每项 `margin-bottom: 28px` |

### 8.16 图片与说明

| 属性 | 值 |
|------|-----|
| 容器 | `<figure class="figure">` |
| 图片 | `max-width: 100%`，圆角 `var(--radius)` |
| 说明 | `<figcaption>`，`13px`，`var(--text-secondary)`，居中，`margin-top: 8px` |

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
