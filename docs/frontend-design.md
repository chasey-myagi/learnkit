# LearnKit 前端设计文档

## 概述

前端分两层，技术栈和来源完全独立：

| 层 | 技术 | 来源 | 用途 |
|----|------|------|------|
| **应用壳** | React + Vite + TS | LearnKit 自带 | 导航、列表、进度 |
| **教案页面** | 独立 HTML + 纯 JS | Claude Code 生成 + 模板注入 | 学习内容、提问交互 |

两层通过 URL 跳转衔接，无 iframe。

---

## 应用壳组件清单（React）

### 页面级

| 页面 | 路由 | 说明 |
|------|------|------|
| Program 列表 | `/` | 首页，展示所有学习教程 |
| Program 详情 | `/program/:slug` | 某个教程的 lesson 列表 + 进度 |

### 组件级

| # | 组件 | 所在页面 | 说明 | cases.html |
|---|------|---------|------|------------|
| 1 | **AppHeader** | 全局 | 应用顶栏，Logo + 应用名 + 主题切换 | `app-header-cases.html` |
| 2 | **ProgramCard** | Program 列表 | 教程卡片：标题、描述、进度、subjects 数量、创建时间 | `program-card-cases.html` |
| 3 | **SubjectGroup** | Program 详情 | 科目分组容器：科目标题 + 进度条 + 折叠/展开 lessons | `subject-group-cases.html` |
| 4 | **LessonItem** | Program 详情 | 课程列表项：标题、状态标签、section 数量、点击跳转 | `lesson-item-cases.html` |
| 5 | **ProgressBar** | 多处 | 进度条/指示器（program 级、subject 级） | `progress-bar-cases.html` |
| 6 | **StatusBadge** | LessonItem | 状态标签：pending / prepared / in_progress / completed / failed | `status-badge-cases.html` |
| 7 | **EmptyState** | Program 列表 | 空状态提示（无 program 时显示） | `empty-state-cases.html` |

---

## 教案模板组件清单（独立 HTML + 纯 JS）

这些组件嵌入教案 HTML 模板中，不依赖 React。

| # | 组件 | 说明 | cases.html |
|---|------|------|------------|
| 8 | **LessonNav** | 教案导航栏：返回列表、标题面包屑、上/下课、主题切换 | `lesson-nav-cases.html` |
| 9 | **LessonTOC** | 目录侧边栏：sections 锚点、当前阅读位置高亮 | `lesson-toc-cases.html` |
| 10 | **AskToolbar** | 划词提问工具栏：选中文字后浮现的提问按钮 | `ask-toolbar-cases.html` |
| 11 | **AskInput** | 提问输入框：预填选中文本作引用、输入问题、提交 | `ask-input-cases.html` |
| 12 | **AnswerCard** | 回答悬浮卡片：显示 AI 回答、可关闭/固定、loading 状态 | `answer-card-cases.html` |
| 13 | **QAHistoryPanel** | 问答历史面板：侧边栏/抽屉，列出当前教案所有问答 | `qa-history-cases.html` |
| 14 | **SectionProgress** | section 阅读进度指示器：已读/未读标记 | `section-progress-cases.html` |

---

## 设计流程

```
Step 1: 建立设计上下文
  └── /teach-impeccable → 更新 DESIGN.md

Step 2: 逐个组件生成 cases.html
  └── 每个组件 ≥5 种风格方案
  └── 使用 impeccable:frontend-design skill

Step 3: 用户逐个挑选
  └── 每个 cases.html 中选出最终方案

Step 4: 合成确认
  └── UI.html — 应用壳全貌（所有选中组件组装）
  └── UX.html — 交互原型（hover、loading、过渡动效）
  └── lesson-template-final.html — 教案完整体验

Step 5: 更新 DESIGN.md
  └── 记录确认后的设计 Token、组件规范

Step 6: 开始正式前端代码开发
  └── 严格按 DESIGN.md 实现
  └── 全程使用 impeccable skills
```

---

## cases.html 规范

每份 cases.html 应包含：

1. **标题区** — 组件名称 + 用途说明
2. **方案 A ~ E（至少 5 个）** — 每个方案独立展示
   - 方案名 + 一句话风格描述
   - 完整可渲染的组件实例
   - 包含所有状态变体（hover、active、disabled、loading 等）
   - 日间/夜间两种模式预览
3. **对比区**（可选）— 并排展示方便用户比较
4. **自包含** — 单文件 HTML，不依赖外部资源

---

## 文件结构

```
frontend/
├── design/                          # 设计阶段产物
│   ├── app-header-cases.html        # 组件 1
│   ├── program-card-cases.html      # 组件 2
│   ├── subject-group-cases.html     # 组件 3
│   ├── lesson-item-cases.html       # 组件 4
│   ├── progress-bar-cases.html      # 组件 5
│   ├── status-badge-cases.html      # 组件 6
│   ├── empty-state-cases.html       # 组件 7
│   ├── lesson-nav-cases.html        # 组件 8
│   ├── lesson-toc-cases.html        # 组件 9
│   ├── ask-toolbar-cases.html       # 组件 10
│   ├── ask-input-cases.html         # 组件 11
│   ├── answer-card-cases.html       # 组件 12
│   ├── qa-history-cases.html        # 组件 13
│   ├── section-progress-cases.html  # 组件 14
│   ├── UI.html                      # 应用壳最终全貌
│   ├── UX.html                      # 交互原型
│   └── lesson-template-final.html   # 教案模板最终设计
└── src/                             # 正式代码（Phase 5 才开始）
```
