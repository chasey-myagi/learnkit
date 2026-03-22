---
name: learn-prepare
description: >
  为指定的 lesson 生成一份教案 HTML。每次调用只生成一份教案，确保质量。
  读取 scope.md 和 resources 作为参考，生成内容后通过 learnkit CLI 注册。
  可由用户手动触发，也可由 Backend 通过 claude -p 自动触发。
  Triggers on: "备课", "生成教案", "prepare", "learn-prepare"
---

# Learn Prepare — 备课（生成教案）

为指定 lesson 生成一份教案 HTML。

## 前置检测

```bash
which learnkit > /dev/null 2>&1 && curl -s http://localhost:13135/api/health > /dev/null 2>&1
```

如果失败 → 提示 `/learn-setup`。详见 `.claude/guide.md`。

## 核心原则

- **每次只生成一份教案** — 聚焦单一内容，保证质量
- **并发由 Backend 控制** — Backend 并发多个 claude -p session
- **内容是文章不是课程** — 没有 "Section 1"、"2/5 已读" 这种系统术语

## 参数

接收方式：用户手动（`/learn-prepare game-dev game-design mda-framework`）或 Backend 通过 `claude -p` 自动调用。

| 参数 | 说明 |
|------|------|
| program | Program slug（如 game-dev） |
| subject | Subject slug（如 game-design） |
| lesson | Lesson slug（如 mda-framework） |

如果用户没指定具体 lesson，用 `learnkit next {program}` 获取下一个待备课的 lesson。

## 流程

### Step 1: 解析参数

从用户输入或 `claude -p` prompt 中解析出 program、subject、lesson。

```
示例输入:
  /learn-prepare game-dev game-design mda-framework
  → program=game-dev, subject=game-design, lesson=mda-framework

  /learn-prepare game-dev
  → program=game-dev, 调用 learnkit next game-dev 获取 subject 和 lesson
```

### Step 2: 读取上下文

```bash
# 获取 scope 信息（JSON 输出）
learnkit scope-read {program}

# 获取所有 lesson 状态
learnkit lesson-list {program}
```

从 scope JSON 中找到目标 lesson 的 sections 定义（标题列表）。
读取 `~/.learnkit/{program}/resources/` 中的相关教学资源作为参考素材。

### Step 3: 生成教案内容

生成一份高质量的 **HTML body 内容**（不是完整 HTML 页面，只是 body 内容片段）。

#### 内容结构

- 正文用 `<h2>` 分主题，`<h3>` 分子话题
- 标题就是标题，不要编号（"什么是 MDA 框架"，不是 "Section 1: 什么是 MDA"）
- **每个 section 对应 scope.md 中定义的一个 section 标题，用 `<h2>` 起头**
- 在 `<h2>` 上加 `id` 属性作为锚点，值为 section 标题的 kebab-case（如 `<h2 id="mda-框架详解">MDA 框架详解</h2>`）

#### 内容要求

- 深入浅出，理论 + 实例 + 案例分析
- 丰富使用交互组件，提升学习体验
- 重点概念用 `<strong>` 高亮

#### 组件使用

**必须阅读 `docs/COMPONENTS.md` 获取所有可用组件的完整 HTML 结构。**

可用组件清单：

| 类别 | 组件 |
|------|------|
| 基础 | h2/h3/h4、p、ul/ol、table、blockquote、hr |
| 代码 | 代码块 `.code-block`、多语言切换 `.code-tabs`、文件树 `.file-tree`、代码 diff `.code-diff` |
| 提示 | 提示框 `.callout`（tip/warning/note/important 四种） |
| 定义 | 术语定义 `.definition` |
| 测验 | 单选题、多选题、判断题、填空题、排序题、连线匹配题（均用 `.quiz-block`） |
| 思考 | 可展开思考题 `.expandable` |
| 展示 | 对比卡片 `.vs-card`、多 Tab 切换 `.content-tabs`、步骤条 `.steps` |
| 标记 | 快捷键 `<kbd>`、公式 `.formula` / `.formula-inline` |
| 图表 | 流程图 `.diagram` / `.flowchart`、时间线 `.timeline`、图片说明 `<figure>` |

**使用原则：**

- 每篇教案至少包含 **2-3 个交互组件**（测验题、思考题、Tab 切换等）
- 根据内容类型选择合适组件：
  - 编程类 → 代码块 + 多语言切换 + 文件树
  - 设计理论 → 对比卡片 + 公式 + 流程图
  - 引擎教程 → 步骤条 + 快捷键 + 代码块
  - 所有类型 → 提示框 + 测验题 + 思考题
- 代码块必须使用 `.code-block` 容器并标注 `data-lang`
- 不要在单个 section 内堆砌过多组件，保持阅读节奏
- 组件可嵌套：步骤条内可包含代码块，思考题内可包含列表

#### 内容长度

- 每个 section 约 300-600 字
- 一份教案总计 1500-4000 字
- 质量 > 数量

### Step 4: 写入文件

将生成的 HTML body 内容写入临时文件，然后调用 CLI 注册：

```bash
# 1. 将内容写入临时文件（使用 Write 工具写入）
#    路径: /tmp/lesson-{lesson}.html

# 2. 调用 lesson-write（会注入模板 + 验证 + 注册到 DB）
learnkit lesson-write {program} {subject} {lesson} --content-file /tmp/lesson-{lesson}.html

# 3. 更新进度
learnkit progress-update {program} {subject} {lesson} --status prepared
```

### Step 5: 处理验证失败

如果 `lesson-write` 返回非零退出码（验证失败）：
1. 读取错误输出，找到具体失败项
2. 根据失败项修复内容（如 section 锚点不匹配、缺少必要标签等）
3. 重新写入临时文件
4. 重新调用 `lesson-write`
5. 最多重试 2 次

## 禁止事项

- **不要** 生成完整 HTML 页面 — `lesson-write` 会注入模板
- **不要** 在内容中包含 `<html>`, `<head>`, `<body>`, `<style>`, `<script>` 标签
- **不要** 包含划词提问的 JS — 模板自带
- **不要** 在标题中加编号（"Section 1:"、"1.1"）
- **不要** 使用课程系统术语（"本节课"、"下一课"、"学完本课"）
- **不要** 加 AI slop（glow、shimmer、渐变文字、emoji 标题）
- **专注于** 教学内容本身的质量

## 完成确认

成功生成后，输出：

```
教案已生成: {subject}/{lesson}
文件: ~/.learnkit/{program}/lessons/{subject}/{lesson}.html
状态: prepared
```
