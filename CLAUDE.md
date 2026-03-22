# LearnKit

个人学习工具 — 将任何主题转化为结构化的交互式学习体验。

## Skills

| Skill | 说明 |
|-------|------|
| `/learn-create` | 创建学习教程，定义 scope |
| `/learn-research` | 收集教学资源 |
| `/learn-prepare` | 备课，生成教案 HTML |
| `/learn-answer` | 回答划词提问（系统调用） |

## 快速开始

```
/learn-create 游戏开发
```

## 前提

需要安装并编译 `learnkit` CLI：

```bash
cd cli && cargo build --release
# 将 target/release/learnkit 加入 PATH
```

启动服务：

```bash
learnkit serve --port 13135
```

## 工作区

数据存储在 `~/.learnkit/` 下，每个 program 一个目录。
