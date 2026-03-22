# LearnKit UX 交互规范

## 划词提问（Ask）

核心原则：**没有弹窗、没有遮罩、没有多余 UI。一个输入框，一个 card。**

### 交互流程

```
1. 选中文字
   ↓ mouseup 检测到 selection.length ≥ 2
2. 内联输入框出现（选区下方 8px）
   - 动画：fadeIn + translateY(-4px → 0), 200ms
   - 只有一个 input，placeholder "输入你的问题..."
   - 自动聚焦
   ↓ 用户输入问题
3. 回车提交
   ↓
4. 输入框消失（fadeOut + translateY → -4px, 150ms）
   选区样式恢复正常
   ↓ 600ms 延迟（模拟请求）
5. Answer card 从选区下方滑出
   - 位置：选区 bottom + 12px，水平对齐选区左侧
   - 动画：fadeIn + translateY(8px → 0), 250ms
   - 内容流式渲染（30ms/字）
   ↓ 流式渲染完成
6. Card 保持显示
   - 右上角 × 关闭
   - 关闭动画各模板不同（见下方）
```

### 状态图

```
idle → [选中文字] → input_visible → [回车] → waiting → [首字到达] → streaming → [完成] → card_visible → [关闭] → idle
                                      ↑                                                                        │
                                      └── [Escape] ────────────────────────────────────────────────────────────┘
```

### 输入框规范

| 属性 | 值 |
|------|-----|
| 宽度 | 320px |
| 高度 | 36px |
| 圆角 | 8px（A 技术文档）/ 12px（B 杂志）/ 虚线边框（C 笔记本） |
| 字号 | 13px |
| 定位 | position: absolute, 选区 bottom + scrollY + 8px |
| 入场 | 200ms, cubic-bezier(0.22, 1, 0.36, 1) |
| 退场 | 150ms, forwards |
| 关闭方式 | 回车提交 / Escape / 点击外部 blur |

### Answer Card 规范

| 属性 | 值 |
|------|-----|
| 宽度 | 400px |
| 最大高度 | 320px（可滚动） |
| 定位 | position: absolute, 选区 bottom + scrollY + 12px |
| 阴影 | 0 4px 20px rgba(0,0,0,0.15) |
| 结构 | header（"回答" + ×）+ body（流式内容） |
| 入场 | 250ms, cubic-bezier(0.22, 1, 0.36, 1) |
| 流式速度 | 30ms/字，\n\n 分段 |

### 各模板关闭动画

| 模板 | 关闭动效 | duration |
|------|---------|----------|
| A 技术文档 | 向右滑出 translateX(24px) | 180ms |
| B 杂志 | 缩小淡出 scale(0.95) | 200ms |
| C 笔记本 | 下滑+微旋转 translateY(12px) rotate(1deg) | 200ms |

### 边界情况

| 场景 | 处理 |
|------|------|
| 输入框已显示时再次选中文字 | 移除旧输入框，创建新的 |
| Card 未关闭时再次提问 | 移除旧 card，显示新 card |
| 选区靠近屏幕右侧 | card left = min(rect.left, viewport - 420) |
| 选区在 ask-inline 或 answer-card 内 | 忽略，不弹输入框 |
| 选中文字 < 2 字符 | 忽略 |

---

## 问答历史（QA History）

### 交互流程

```
右下角 💬 按钮 → 点击展开侧边面板
面板显示当前教案的历史问答（倒序）
点击某条记录 → 高亮对应选区位置（可选）
再次点击按钮或点击外部 → 面板收起
```

### 面板规范

| 属性 | 值 |
|------|-----|
| 宽度 | 360px |
| 位置 | 右侧侧滑 |
| 入场 | translateX(100% → 0), 300ms |
| 退场 | translateX(0 → 100%), 200ms |

---

## 主题切换

### 交互

- 顶部导航栏右侧胶囊形切换器
- 点击切换日间/夜间
- 所有颜色属性有 200ms ease 过渡
- 选择持久化到 localStorage

---

## 手风琴折叠（Subject Group，仅应用壳详情页）

### 交互

- 点击 subject 标题栏 → 展开/折叠
- chevron 箭头旋转 180°, 250ms
- 展开态 subject 左侧显示 3px accent 色边框
- aria-expanded 状态同步

---

## 教案内置交互组件

### 测验题（Quiz）

```
显示问题 + 选项
↓ 点击选项
即时反馈：正确=绿色 / 错误=红色 + 解释文字
锁定防重复点击
```

### 可展开思考题

```
问题显示 + [展开参考思路 ▼]
↓ 点击
内容区 max-height 0 → auto, 300ms
▼ 旋转为 ▲
```

### 荧光笔高亮（仅 C 笔记本）

```
点击 .highlightable 文字
↓
荧光效果从左到右渐变出现
再次点击取消
支持键盘 Enter/Space
```

---

## 通用动效规范

| 类别 | duration | easing |
|------|----------|--------|
| 颜色变化 | 200ms | ease |
| 位移/变换 | 250ms | cubic-bezier(0.22, 1, 0.36, 1) |
| 布局展开 | 300ms | cubic-bezier(0.22, 1, 0.36, 1) |
| 退场 | 入场的 75% | cubic-bezier(0.4, 0, 1, 1) |

**禁止**：bounce、elastic easing、glow box-shadow、shimmer 动画、渐变文字

**必须**：`prefers-reduced-motion` 全局禁用所有动画
