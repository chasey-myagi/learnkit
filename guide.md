# LearnKit 使用指南

## 安装后第一步

安装 LearnKit plugin 后，运行 `/learn-setup` 检测并配置环境。

或者直接运行 `/learn-create <主题>`，如果环境未就绪，会自动引导你配置。

## 环境要求

| 依赖 | 必须？ | 用途 |
|------|--------|------|
| Rust (cargo) | 是 | 编译 LearnKit CLI |
| learnkit CLI | 是 | 教案管理、数据库、文件操作 |
| learnkit serve | 是 | HTTP API + 教案页面服务 |
| Node.js | 否 | 仅开发模式需要 |

## 使用流程

```
1. /learn-create "游戏开发"
   → 与你讨论学习范围 → 生成 scope.md → 创建 workspace

2. /learn-research
   → 搜索教学资源 → 收集到 workspace

3. /learn-prepare
   → 生成教案 HTML → 浏览器中自学

4. 浏览器中学习
   → 打开 http://localhost:13135
   → 阅读教案，划词提问

5. 自动备课
   → 学完快了 Backend 自动生成下一批教案
```

## 常用命令

```bash
# CLI
learnkit init <slug>              # 创建 program
learnkit scope-write <p> --file   # 写入大纲
learnkit lesson-write <p> <s> <l> # 写入教案
learnkit progress <p>             # 查看进度
learnkit serve --port 13135       # 启动服务

# Skills
/learn-create <主题>              # 创建教程
/learn-research                   # 收集资源
/learn-prepare                    # 备课
/learn-setup                      # 环境检测
```

## 数据存储

```
~/.learnkit/
├── game-dev/                     # 每个 program 一个目录
│   ├── scope.md                  # 大纲
│   ├── learnkit.db               # SQLite 数据库
│   ├── lessons/                  # 教案 HTML
│   ├── resources/                # 教学资源
│   └── answers/                  # 问答临时文件
```

## 更新

```bash
# 在 Claude Code 中
/plugin  → 选择 learnkit → 更新

# 或手动
cd ~/.claude/plugins/cache/learnkit/
git pull
cd cli && cargo build --release
cp target/release/learnkit ~/.cargo/bin/
```
