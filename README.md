# LearnKit

个人学习工具 — 将任何主题转化为结构化的交互式学习体验。

## 安装

```bash
# 在 Claude Code 中安装 plugin
/plugin chasey-myagi/learnkit

# 重新加载
/reload-plugins
```

安装后运行 `/learn-setup` 配置环境（编译 CLI + 启动 Backend）。

## 快速开始

```bash
# 1. 配置环境（首次）
/learn-setup

# 2. 创建学习教程
/learn-create 游戏开发

# 3. 收集教学资源
/learn-research

# 4. 备课生成教案
/learn-prepare

# 5. 打开浏览器学习
open http://localhost:13135
```

## 架构

```
用户 → [Skill] → Claude Code → [CLI] → HTML 教案 → 用户浏览器
  ↑                                                      ↓
  └──────────── 划词提问 → Backend → Claude Code ←───────┘
```

| 组件 | 技术 | 职责 |
|------|------|------|
| **CLI + Backend** | Rust (Axum + rusqlite) | 文件管理、SQLite、HTTP API、spawn claude -p |
| **Frontend** | React (Vite + TypeScript) | 应用壳（Program 列表、Lesson 进度） |
| **教案模板** | HTML + 纯 JS | 三套风格（技术文档/杂志/笔记本），划词提问 |
| **Skills** | Claude Code Skills | learn-create / learn-research / learn-prepare / learn-answer |

## 核心理念

- **Claude Code 是唯一的 Agent** — 不额外调 API，复用 Claude Code 订阅额度
- **Skills 规范行为，CLI 负责写入** — 不抓 claude -p 输出，通过文件系统交换数据
- **进程退出 = 完成信号** — Backend spawn 子进程，exit code 0 = 成功

## 项目结构

```
learnkit/
├── cli/                    # Rust CLI + Backend（单二进制）
│   ├── src/
│   │   ├── main.rs         # 入口，18 个子命令
│   │   ├── config.rs       # 路径配置
│   │   ├── scope.rs        # scope.md YAML 解析
│   │   ├── commands/       # 子命令实现
│   │   └── db/             # SQLite 操作层
│   └── Cargo.toml
├── frontend/
│   └── design/             # UI/UX 设计产物（cases + 确认稿）
├── skills/                 # Claude Code Skills（待实现）
└── docs/
    ├── architecture.md     # 系统架构设计
    ├── DESIGN.md           # UI/UX 设计冻结规范
    ├── plan.md             # 实施计划
    ├── terminology.md      # 术语表
    ├── frontend-design.md  # 前端组件清单
    └── dev-guidelines.md   # 开发规范
```

## 学习流程

```
/learn-create "游戏开发"     → 定义 scope，创建 workspace
/learn-research              → 收集教学资源
/learn-prepare               → 备课，生成教案 HTML
浏览器自学                    → 阅读教案，划词提问
自动备课                      → 剩余 ≤1 lesson 时 Backend 自动触发
```

## CLI 命令

```bash
learnkit init <slug>                    # 创建 program workspace
learnkit scope-write <prog> --file <f>  # 写入 scope.md
learnkit scope-read <prog>              # 读取 scope (JSON)
learnkit lesson-write <p> <s> <l> ...   # 写入教案 HTML
learnkit lesson-list <prog>             # 列出教案
learnkit progress <prog>                # 查看进度
learnkit check-prepare <prog>           # 检查是否需要备课
learnkit answer-write <prog> ...        # 保存划词提问回答
learnkit serve                          # 启动 Backend + WebUI
```

## 开发

```bash
cd cli
cargo build
cargo run -- --help
```

## 状态

🚧 开发中

- [x] Phase 1: Rust CLI（18 个命令，编译通过）
- [x] Phase 4: UI/UX 设计（应用壳 + 教案模板，已冻结）
- [ ] Phase 2: Rust Backend（Axum HTTP server）
- [ ] Phase 3: Claude Code Skills
- [ ] Phase 5: React 前端实现
- [ ] Phase 6: 教案 HTML 模板
- [ ] Phase 7: 集成测试

## License

Private — 个人使用
