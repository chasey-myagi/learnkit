# LearnKit 术语表

## 核心概念

| 中文 | 英文 | 变量名 | 说明 | 示例 |
|------|------|--------|------|------|
| 学习教程 | Program | `program` | 顶层实体，一个完整学习主题 | "游戏开发"、"强化学习" |
| 科目 | Subject | `subject` | Program 下的子领域 | "游戏设计"、"游戏编程"、"游戏美术" |
| 课程 | Lesson | `lesson` | 一个完整教学单元，对应一个 HTML 教案 | "MDA 框架"、"核心游戏循环" |
| 小节 | Section | `section` | Lesson 内的章节 | "什么是 MDA"、"案例分析" |
| 大纲 | Scope | `scope` | 定义整个 Program 结构的文档 (scope.md) | — |
| 工作区 | Workspace | `workspace` | Program 的根目录 | `~/.learnkit/game-dev/` |
| 教案 | Lesson Plan | `lesson_plan` | 生成的 HTML 教学文件 | `mda-framework.html` |
| 备课 | Prepare | `prepare` | 生成教案的过程 | — |
| 教学资源 | Resource | `resource` | 收集的参考资料（PDF/文档/仓库） | `resources/docs/game-design-book.pdf` |
| 问答记录 | QA History | `qa_history` | 学习中的提问与回答历史 | `qa-history.json` |
| 进度 | Progress | `progress` | 学习进度追踪数据 | `progress.json` |

## 层级关系

```
Program（学习教程）
  ├── Subject（科目）
  │     ├── Lesson（课程）
  │     │     ├── Section（小节）
  │     │     └── Section
  │     └── Lesson
  └── Subject
```

## 文件结构

```
~/.learnkit/                              # 全局根目录
├── game-dev/                             # Program workspace (slug)
│   ├── scope.md                          # 大纲（YAML frontmatter + Markdown）
│   ├── learnkit.db                       # SQLite 数据库（进度、问答、索引）
│   ├── resources/                        # 教学资源
│   │   ├── docs/                         # 文档、PDF
│   │   ├── repos/                        # GitHub 仓库（shallow clone）
│   │   └── index.md                      # 资源索引
│   ├── lessons/                          # 教案 HTML
│   │   ├── game-design/                  # Subject 目录 (slug)
│   │   │   ├── mda-framework.html        # Lesson 教案
│   │   │   └── core-game-loop.html
│   │   └── game-programming/
│   │       └── engine-selection.html
│   └── answers/                          # 临时文件交换区
│       └── {request-id}.json
└── reinforcement-learning/               # 另一个 Program
    ├── scope.md
    ├── learnkit.db
    ├── resources/
    ├── lessons/
    └── answers/
```

> 本地开发环境使用 `~/cc/.learnkit/` 替代 `~/.learnkit/`。

## 命名规范

- **目录名 / 文件名**：使用 kebab-case slug（如 `game-dev`、`mda-framework`）
- **变量名**：使用 snake_case（如 `program`、`lesson_plan`）
- **显示名**：使用原始中/英文标题（如 "游戏开发"、"MDA 框架"）

## 架构角色

| 组件 | 面向谁 | 职责 |
|------|--------|------|
| **Skills** | 用户 | 用户在 Claude Code 中发起学习意图的入口 |
| **Claude Code** | 桥梁 | 理解意图 → 编排教学逻辑 → 调用 CLI 执行 |
| **CLI** | Claude Code | 教案生成/管理/进度追踪/前端页面输出 |
| **HTML 教案** | 用户 | 可视化学习内容，用户在浏览器中自学 |

```
用户 → [Skill] → Claude Code → [CLI] → HTML 教案 → 用户浏览器
  ↑                                                      ↓
  └──────────── 遇到问题，回到 Skill 提问 ←──────────────┘
```

## 学习流程阶段

| 阶段 | 触发 | 说明 |
|------|------|------|
| 1. 创建 | `/learn-create`（用户） | 定义 scope，创建 workspace |
| 2. 调研 | `/learn-research`（用户） | 收集教学资源到 workspace |
| 3. 备课 | `/learn-prepare`（用户/系统） | 生成教案 HTML |
| 4. 自学 | WebUI（浏览器） | 用户阅读教案 |
| 5. 提问 | WebUI → Backend → `/learn-answer`（系统） | 划词提问 → 悬浮卡片回答 |
| 6. 自动备课 | Backend 检测（系统） | 剩余 ≤1 lesson 时自动触发 |
| 7. 循环 3-6 | — | 持续学习 |
