---
name: learn-research
description: >
  为学习教程收集教学资源。搜索相关文档、PDF、GitHub 仓库等，
  与用户确认后下载到 workspace。在 create 之后、prepare 之前使用。
  Triggers on: "收集资源", "找教学资料", "research", "learn-research", "找资源"
---

# Learn Research — 收集教学资源

为指定的 Program 收集教学资源（文档、PDF、GitHub 仓库等）。

## 前置检测

```bash
which learnkit > /dev/null 2>&1 && curl -s http://localhost:13135/api/health > /dev/null 2>&1
```

如果失败 → 提示 `/learn-setup`。详见 `.claude/guide.md`。

## 前提

必须已经运行过 `/learn-create` 创建了 program 和 scope.md。

## 流程

### 1. 读取 Scope

```bash
learnkit scope-read {program}
```

了解学习范围和主题，确定需要搜索的方向。

### 2. 搜索资源

根据 scope 中的 subjects 和 lessons，搜索以下类型的资源：

| 类型 | 搜索方向 | 示例 |
|------|---------|------|
| **文档/教程** | 官方文档、高质量教程 | MDN、Godot 文档 |
| **GitHub 仓库** | 学习型仓库、示例代码 | awesome-xxx、tutorial repos |
| **PDF/论文** | 课本、学术论文 | 游戏设计原理 PDF |
| **视频教程** | YouTube、B站 | GDC 演讲 |

使用 WebSearch 工具搜索。每个 subject 搜索 2-3 个关键词。

### 3. 与用户确认

将搜索到的资源列表呈现给用户：
- 资源名称 + URL
- 类型标注（doc/repo/pdf）
- 一句话说明为什么推荐

让用户选择要保存的资源。

### 4. 保存资源

对用户确认的每个资源：

```bash
learnkit resource-add {program} "{url}" --type {doc|repo|pdf}
```

当前只记录 URL 到数据库，不实际下载。用户需要时手动下载或后续扩展自动下载。

### 5. 更新资源索引

完成后提示用户：
- 已保存 X 个资源
- 可以运行 `/learn-prepare` 开始备课
- 备课时会参考这些资源

## 注意

- 搜索时使用中英文关键词（扩大覆盖面）
- 优先推荐免费资源
- 不要推荐过时的资源（检查发布日期）
- 每次搜索不要超过 10 个结果呈现给用户（避免信息过载）
