# LearnKit Skills 设计

## 架构总览

```
┌──────────┐        HTTP        ┌──────────────────┐    claude CLI     ┌─────────────────┐
│  WebUI   │ ←───────────────→  │    Backend        │ ───────────────→ │   Claude Code   │
│ (浏览器)  │                    │  (HTTP + CLI)     │ ← 文件写入 ──── │  (Skills 驱动)   │
│ 教案展示   │                    │                   │                  │                 │
│ 划词提问   │                    │  learnkit CLI ←───────────────────── │  调用 CLI 写文件  │
└──────────┘                    └──────────────────┘                   └─────────────────┘
```

**关键原则**：
- Skills 规范 Claude Code 的行为（做什么、怎么输出）
- CLI 负责文件写入和状态管理（教案注册、进度更新、回答保存）
- Claude Code 是 Agent，不直接抓它的输出，而是让它通过 Skill + CLI 协作

## Skills 分类

### 用户 Skills（用户在 Claude Code 中手动触发）

| Skill | 触发方式 | 说明 |
|-------|---------|------|
| `/learn-create` | 用户主动 | 创建学习教程，定义 scope |
| `/learn-research` | 用户主动 | 收集教学资源 |
| `/learn-prepare` | 用户主动 | 手动触发备课 |

### 系统 Skills（Backend 通过 `claude -p` 自动调用）

| Skill | 触发方式 | 说明 |
|-------|---------|------|
| `/learn-prepare` | Backend 检测到剩余 ≤1 lesson 时 | 自动备课 |
| `/learn-answer` | Backend 收到 WebUI 提问时 | 生成回答并写入文件 |

> `/learn-prepare` 同时是用户 Skill 和系统 Skill，两种触发方式，同一个行为。

## 学习流程

```
/learn-create → /learn-research → /learn-prepare → 用户在 WebUI 自学 ─┐
                                       ↑                    ↓              │
                                       │              划词提问 → Backend    │
                                       │                    ↓              │
                                       │              claude -p            │
                                       │              "/learn-answer ..."  │
                                       │                    ↓              │
                                       │              answer-write → DB   │
                                       │                    ↓              │
                                       │              Backend → WebUI 卡片  │
                                       │                                   │
                                       └── Backend 检测 ≤1 lesson ←────────┘
                                            → 并发 claude -p "/learn-prepare ..."
                                            → lesson-write → HTML + DB
```

---

## 用户 Skills 详细设计

### `/learn-create` — 创建学习教程

**触发**：用户主动（如 `/learn-create 游戏开发`）

**流程**：
1. 接收用户输入的学习主题
2. 与用户对话，理解学习目标、已有基础、期望深度
3. 提出 subjects 划分方案，用户确认
4. 每个 subject 下规划 lessons 和 sections
5. 调用 `learnkit init <program-slug>` 创建 workspace
6. 调用 `learnkit scope-write <program>` 写入 scope.md

**输出**：
- workspace 目录结构
- `scope.md`（完整学习大纲）

**scope.md 结构示例**：
```markdown
# 游戏开发

## 元信息
- 创建时间: 2026-03-22
- 预估总课时: 24 lessons
- 难度: 入门 → 进阶

## 科目与课程

### 1. 游戏设计 (game-design)

#### 1.1 游戏设计概论 (game-design-intro)
- 什么是游戏设计
- MDA 框架详解
- 核心游戏循环

#### 1.2 玩家心理学 (player-psychology)
- Bartle 玩家分类
- 心流理论
- 动机设计

### 2. 游戏编程 (game-programming)
...
```

---

### `/learn-research` — 收集教学资源

**触发**：用户主动，在 create 之后、第一次 prepare 之前

**流程**：
1. 读取 scope.md 了解学习范围
2. 搜索相关资源：
   - 在线文档/教程
   - PDF 课本/论文
   - GitHub 学习仓库
   - 视频教程链接
3. 与用户确认需要下载/保存的资源
4. 调用 `learnkit resource-add <program> <url>` 下载资源到 workspace

**输出**：
- `resources/` 目录填充
- `resources/index.md` 资源索引

---

### `/learn-prepare` — 备课（生成教案）

**触发**：
- 用户主动：`/learn-prepare`（create 后首次）
- 系统自动：Backend 检测到剩余 ≤1 未学完 lesson 时，通过 `claude -p "/learn-prepare ..."` 触发

**核心原则**：
- **每次调用只生成一份教案** — 保证 Claude Code 聚焦，确保质量
- **并发由 Backend 控制** — Backend 并发多个 `claude -p "/learn-prepare ..."` session 生成多份教案
- 一次备课批次至少 **6 份教案** 或覆盖 **2 个 lesson**
- 内容较多的 lesson，每个 section 独立一份教案

**流程**（单次调用）：
1. 接收参数：program、subject、lesson（指定生成哪一份）
2. 读取 scope.md 获取该 lesson 的 sections 定义
3. 读取 resources/ 中的相关教学资源作为参考
4. 生成**一份**教案内容（遵循设计规范 / .impeccable.md）
5. 调用 `learnkit lesson-write <program> <subject> <lesson> --content-file <path>` 注册教案
6. 调用 `learnkit progress-update <program> <subject> <lesson> --status prepared` 更新进度

**输出**：
- `lessons/{subject}/{lesson}.html` — 教案文件
- SQLite lessons 表更新

---

## 系统 Skills 详细设计

### `/learn-answer` — 生成回答（系统 Skill）

**触发**：Backend 收到 WebUI 提问时，通过 `claude -p` 调用

**调用方式**：
```bash
claude -p "/learn-answer --program game-dev --lesson game-design/mda-framework --selection '核心游戏循环是嵌套的' --question '什么叫嵌套的循环？'"
```

**Skill 内部流程**：
1. 读取对应教案文件，获取完整上下文
2. 读取 scope.md 了解当前学习阶段
3. 生成针对性回答（简洁、基于教案上下文、匹配学习者水平）
4. 调用 `learnkit answer-write <program> --lesson <path> --question <q> --answer <a> --selection <s>`
   将回答写入标准格式文件并追加到 qa-history

**回答文件格式** (`answer-{timestamp}.json`)：
```json
{
  "timestamp": "2026-03-22T14:30:00Z",
  "program": "game-dev",
  "lesson": "game-design/mda-framework",
  "selection": "核心游戏循环是嵌套的",
  "question": "什么叫嵌套的循环？",
  "answer": "嵌套循环指的是游戏循环分为多个层级..."
}
```

**Backend 获取回答流程**：
1. Backend 调用 `claude -p "/learn-answer ..."`
2. Claude Code 执行 skill → 调用 `learnkit answer-write` → 写入文件
3. `claude` 进程退出
4. Backend 读取最新的 answer 文件
5. 返回给 WebUI 前端显示
