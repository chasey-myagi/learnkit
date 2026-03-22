---
name: learn-answer
description: >
  回答用户在教案中的划词提问。系统 Skill，由 Backend 通过 claude -p 自动调用。
  读取教案上下文，生成精准回答，通过 learnkit CLI 保存回答文件。
  Triggers on: "learn-answer"（仅系统调用）
---

# Learn Answer — 回答划词提问

系统 Skill，由 Backend 通过 `claude -p` 自动调用，回答用户在教案中的划词提问。

## 调用方式

```bash
claude -p "/learn-answer --request-id q-abc123 --program game-dev --lesson game-design/mda-framework --selection '核心游戏循环是嵌套的' --question '什么叫嵌套的循环？'"
```

## 参数

| 参数 | 说明 |
|------|------|
| request-id | 请求 ID（Backend 生成，用于文件命名） |
| program | Program slug |
| lesson | Lesson 路径（subject/lesson 格式） |
| selection | 用户选中的文字 |
| question | 用户提出的问题 |

## 流程

### 1. 读取教案上下文

```bash
# 获取 scope 了解学习阶段
learnkit scope-read {program}
```

同时读取对应的教案 HTML 文件：`~/.learnkit/{program}/lessons/{lesson}.html`

从教案内容中提取与选中文字相关的上下文段落。

### 2. 生成回答

**回答原则：**
- 基于教案内容的上下文回答，不脱离教案主题
- 简洁精准，不超过 300 字
- 如果问题涉及教案未覆盖的内容，简要说明并建议后续学习
- 用词匹配学习者水平（从 scope 的 difficulty 判断）
- 可以用列表、粗体来组织回答，但不要用代码块（除非问的是代码问题）

**回答格式：**
纯文本，段落之间用 `\n\n` 分隔。不用 HTML 标签。

### 3. 保存回答

```bash
learnkit answer-write {program} \
  --request-id {request-id} \
  --lesson {lesson} \
  --selection "{selection}" \
  --question "{question}" \
  --answer "{answer}"
```

这会：
1. 写入 `answers/{request-id}.json`
2. 插入 SQLite qa_history 表

### 4. 完成

CLI 命令执行成功后，直接结束。Backend 会检测进程退出并读取回答文件。

**不需要输出任何内容到 stdout** — Backend 不读 stdout，只读文件。

## 注意

- 这是系统 Skill，不与用户交互
- 不要用 AskUserQuestion — 没有用户在听
- 整个流程应该在 30 秒内完成
- 回答要简洁，不要长篇大论
