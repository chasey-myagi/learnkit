# LearnKit CLI + Backend 设计

## 架构定位

LearnKit 不只是 CLI，它是 **WebUI + Backend + CLI 三位一体**：

| 组件 | 面向 | 职责 |
|------|------|------|
| **WebUI** | 用户 | 教案展示、划词提问、进度浏览 |
| **Backend (HTTP Server)** | WebUI + Claude Code | serve 前端、API 接口、自动备课检测 |
| **CLI** | Claude Code + Backend | 文件写入、状态管理、教案注册 |

```
┌──────────┐        HTTP        ┌──────────────────┐    claude -p     ┌─────────────────┐
│  WebUI   │ ←───────────────→  │    Backend        │ ──────────────→ │   Claude Code   │
│ (浏览器)  │                    │  (HTTP Server)    │                  │  (Skills 驱动)   │
│ 教案展示   │                    │                   │                  │                 │
│ 划词提问   │                    │  ← ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─  │  调用 learnkit   │
└──────────┘                    │  CLI 命令接口      │                  │  CLI 写文件      │
                                └──────────────────┘                   └─────────────────┘
```

## CLI 命令列表

### Workspace 管理

```bash
learnkit init <program-slug>              # 创建 workspace 目录结构
learnkit list                             # 列出所有 programs
learnkit info <program>                   # 输出 program 详情
```

### Scope 管理

```bash
learnkit scope-write <program> --file <path>   # 写入/更新 scope.md
learnkit scope-read <program>                   # 读取 scope（JSON 输出）
```

### 教案管理

```bash
learnkit lesson-write <program> <subject> <lesson> --content-file <path>
                                          # 写入教案 HTML 并注册
learnkit lesson-list <program> [--status prepared|pending|completed]
                                          # 列出 lessons 及状态
learnkit lesson-open <program> <subject> <lesson>
                                          # 在浏览器中打开教案
learnkit next <program>                   # 获取下一个未完成的 lesson
```

### 资源管理

```bash
learnkit resource-add <program> <url> [--type doc|repo|pdf]
                                          # 下载资源到 workspace
learnkit resource-list <program>          # 列出已收集的资源
```

### 进度追踪

```bash
learnkit progress <program>               # 输出学习进度（JSON）
learnkit progress-update <program> <subject> <lesson> --status <status>
                                          # 更新 lesson 状态
learnkit check-prepare <program>          # 检测是否需要备课
                                          # 退出码: 0=OK, 1=NEED_PREPARE
```

### 问答管理

```bash
learnkit answer-write <program> --lesson <path> --question <q> --answer <a> --selection <s>
                                          # 保存回答到文件 + 追加 qa-history
learnkit qa-history <program> [--lesson <path>]
                                          # 输出问答历史
```

### 服务

```bash
learnkit serve <program> [--port 3377]    # 启动本地 HTTP 服务
```

---

## Backend HTTP API

`learnkit serve` 启动的 HTTP 服务：

### 教案服务

```
GET  /                                        → 教案列表首页（WebUI）
GET  /lessons/{subject}/{lesson}.html         → 教案页面
GET  /api/lessons                             → 教案列表（JSON）
GET  /api/scope                               → 大纲数据（JSON）
```

### 问答 API

```
POST /api/ask                                 → 提交提问
     Body: { selection, question, lessonPath }
     流程: Backend → claude -p "/learn-answer ..." → 等待回答文件 → 返回
     Response: { answer, timestamp }

GET  /api/qa-history?lesson={path}            → 获取问答历史
```

### 进度 API

```
GET  /api/progress                            → 学习进度数据
POST /api/progress                            → 上报进度（前端滚动阅读时）
     Body: { lessonPath, section, status }
```

### 备课状态

```
GET  /api/prepare-status                      → 备课状态（是否需要备课）
POST /api/prepare                             → 手动触发备课
     流程: Backend → claude -p "/learn-prepare ..." → 等待教案文件
```

---

## WebUI 前端设计

### 教案列表首页

- 显示所有 subjects 和 lessons
- 每个 lesson 显示状态（待备课 / 已备课 / 学习中 / 已完成）
- 整体进度条
- 点击 lesson 进入教案页面

### 教案页面

基础功能：
- 导航栏（program 名、subject、lesson 标题、上/下页）
- 目录（sections 锚点）
- 正文区域（教案内容）
- 主题切换（日间/夜间模式）

**划词提问交互**：
1. 用户选中一段文字
2. 选区上方出现工具栏 `[📝 提问]`
3. 点击后弹出输入框（预填选中文本作为上下文引用）
4. 用户输入问题，回车提交
5. 显示 loading 状态
6. 回答以悬浮卡片形式显示在选区旁边
7. 卡片可关闭、可固定
8. 问答自动保存到历史

**问答历史面板**：
- 侧边栏或底部抽屉
- 按时间倒序显示当前教案的所有问答
- 可点击跳转到对应选区位置

**进度上报**：
- 滚动阅读时，自动标记 section 为"已读"
- 通过 `POST /api/progress` 上报

---

## 自动备课检测

Backend 定时（如每 60 秒）执行：

```bash
learnkit check-prepare <program>
```

如果返回 `NEED_PREPARE`（退出码 1），Backend 获取需要准备的 lessons 列表，**并发**启动多个 claude session：

```bash
# 并发执行，每个 session 只生成一份教案
claude -p "/learn-prepare game-dev game-design mda-framework" &
claude -p "/learn-prepare game-dev game-design core-game-loop" &
claude -p "/learn-prepare game-dev game-design player-psychology" &
# ... 一次至少 6 份或 2 个 lesson
wait
```

**每个 `/learn-prepare` 调用只生成一份教案**，这样：
- Claude Code 聚焦在单一内容上，教案质量有保障
- 不会因为一次生成太多而超时或质量下降
- 并发由 Backend 控制，充分利用多 session 能力

---

## Workspace 目录结构（最终版）

```
~/.learnkit/{program}/
├── scope.md                         # 大纲
├── progress.json                    # 学习进度
├── qa-history.json                  # 问答历史
├── resources/                       # 教学资源
│   ├── docs/                        # 文档、PDF
│   ├── repos/                       # GitHub 仓库
│   └── index.md                     # 资源索引
├── lessons/                         # 教案 HTML
│   ├── {subject}/
│   │   ├── {lesson}.html
│   │   └── {lesson}.html
│   └── {subject}/
│       └── ...
└── answers/                         # 回答文件（临时交换）
    └── answer-{timestamp}.json
```

## 技术栈（待定）

| 组件 | 候选方案 |
|------|---------|
| Backend | Node.js (Express/Fastify) 或 Python (FastAPI) |
| WebUI | 内嵌在 Backend 中的静态页面，或 React SPA |
| CLI | 与 Backend 同一代码库，通过入口区分 |
| 教案模板 | 统一 HTML 模板，遵循 .impeccable.md 设计规范 |
