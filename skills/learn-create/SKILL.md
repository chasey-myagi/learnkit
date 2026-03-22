---
name: learn-create
description: >
  创建一个新的学习教程（Program）。与用户讨论学习目标和范围，生成结构化大纲 scope.md，
  创建 workspace 目录。触发方式：用户输入 /learn-create 或 "我想学 XX"。
  Triggers on: "我想学", "开始学习", "创建教程", "learn-create", "新建课程"
---

# Learn Create — 创建学习教程

创建一个新的学习教程（Program），与用户讨论后生成结构化大纲。

## 流程

1. **接收主题**：用户指定想学的主题（如"游戏开发"、"强化学习"）
2. **讨论范围**：与用户对话，理解：
   - 学习目标（想达到什么水平）
   - 已有基础（零基础 / 有一些了解 / 有经验）
   - 期望深度（概览 / 实用 / 深入）
   - 时间投入（大概想花多长时间学）
3. **提出 Subjects 划分**：将主题拆分为 3-6 个科目（Subject），向用户确认
4. **规划 Lessons**：每个 Subject 下规划 2-8 个课程（Lesson），每个 Lesson 列出 Sections
5. **生成 scope.md**：写入 YAML frontmatter + Markdown 格式
6. **调用 CLI 创建 workspace**

## 术语

- **Program**: 顶层学习教程（如"游戏开发"）
- **Subject**: 科目/子领域（如"游戏设计"）
- **Lesson**: 课程/教学单元
- **Section**: 小节

## Step 1: 与用户讨论

用 AskUserQuestion 工具向用户确认：
- 学习目标和深度
- 已有基础
- Subject 划分方案

**一次只问一个问题，不要信息过载。**

## Step 2: 生成 scope.md

scope.md 格式：

```yaml
---
program: {slug}
title: {标题}
created: {YYYY-MM-DD}
difficulty: {beginner/intermediate/advanced} → {target}
subjects:
  - slug: {kebab-case}
    title: {科目名}
    lessons:
      - slug: {kebab-case}
        title: {课程名}
        sections:
          - {小节名 1}
          - {小节名 2}
  - slug: ...
---

# {标题}

{学习建议和补充说明}
```

## Step 3: 调用 CLI

```bash
# 1. 创建 workspace
learnkit init {program-slug}

# 2. 将 scope.md 写入
# 先将内容写入临时文件，再调用 scope-write
learnkit scope-write {program-slug} --file /tmp/scope-{slug}.md
```

## 注意事项

- slug 用 kebab-case（如 game-dev、reinforcement-learning）
- 每个 Subject 至少 2 个 Lessons
- 每个 Lesson 至少 2 个 Sections
- Sections 名称要具体，不要太笼统（"概述" 太模糊，"MDA 框架的三层模型" 更好）
- 创建完成后提示用户可以运行 `/learn-research` 收集教学资源
