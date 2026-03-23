# LearnKit

将任何主题转化为结构化的交互式学习体验。Claude Code Skills + Rust CLI + React WebUI。

## 安装

```bash
# 从 marketplace 安装
claude plugin marketplace add chasey-myagi/learnkit
claude plugin install learnkit

# 重载插件（在 Claude Code 会话中）
/reload-plugins
```

安装后运行 `/learn-setup` 配置环境（编译 CLI + 启动 Backend）。

### 更新

```bash
claude plugin update learnkit
```

## 快速开始

```bash
/learn-setup                  # 配置环境（首次）
/learn-create 游戏开发         # 创建学习教程
/learn-research               # 收集教学资源
/learn-prepare                # 备课生成教案
open http://localhost:13135   # 打开浏览器学习
```

## 架构

```
用户 → [Skill] → Claude Code → [CLI] → HTML 教案 → 浏览器
  ↑                                                    ↓
  └──────────── 划词提问 → Backend → Claude Code ←─────┘
```

## 技术栈

| 组件 | 技术 | 职责 |
|------|------|------|
| CLI + Backend | Rust (Axum + rusqlite) | 文件管理、SQLite、HTTP API、spawn claude -p |
| Frontend | React (Vite + TypeScript) | 应用壳（Program 列表、Lesson 进度） |
| 教案模板 | HTML + 纯 JS | 三套风格（技术文档/杂志/笔记本），划词提问 |
| Skills | Claude Code Skills | learn-create / learn-research / learn-prepare / learn-answer |

## Skills

| Skill | 类型 | 说明 |
|-------|------|------|
| `/learn-setup` | Tool | 配置环境，编译 CLI，启动 Backend |
| `/learn-create` | SOP | 创建学习教程，定义 scope |
| `/learn-research` | SOP | 收集教学资源 |
| `/learn-prepare` | SOP | 备课，生成教案 HTML |
| `/learn-answer` | Tool | 回答划词提问（系统自动调用） |

## Hooks

| 事件 | 触发条件 | 效果 |
|------|----------|------|
| PostToolUse:Write | 写入教案 HTML | 自动验证模板完整性 |
| PostToolUse:Bash | 检测到教案变更 | 检查备课库存，提示 /learn-prepare |
| PreToolUse:Bash | 启动 learnkit serve | 检测 Backend 是否已在运行 |
| Stop | 会话结束 | 提醒教案库存不足的 program |

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
# CLI (Rust)
cd cli && cargo build --release

# Frontend (React)
cd frontend && npm install && npm run dev

# 测试
npm test
```

## 版本历史

- **v0.2.1** — 首个公开发布版本，完整学习流程

## License

[BUSL-1.1](LICENSE)
