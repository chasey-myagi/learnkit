---
name: learn-setup
description: >
  检测并配置 LearnKit 运行环境。检查 Rust CLI 是否编译、Backend 是否运行。
  缺什么引导安装什么。其他 learn-* skill 在发现环境未就绪时会自动调用此 skill。
  Triggers on: "learn-setup", "配置 learnkit", "安装 learnkit"
---

# Learn Setup — 环境配置向导

检测 LearnKit 运行环境，缺什么装什么。

## 检测流程

按顺序检测，遇到缺失就修复：

### 1. 检测 Rust 工具链

```bash
rustc --version && cargo --version
```

如果缺失，告诉用户：
> 需要安装 Rust：`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### 2. 检测 learnkit CLI

```bash
which learnkit
```

如果不在 PATH 中，**自动编译安装**：

```bash
# 找到 plugin 缓存中的源码
PLUGIN_DIR=$(find ~/.claude/plugins/cache/learnkit -name "Cargo.toml" -path "*/cli/*" | head -1 | xargs dirname)

if [ -z "$PLUGIN_DIR" ]; then
  # 缓存不存在，从 GitHub 编译
  cd /tmp
  git clone --depth 1 https://github.com/chasey-myagi/learnkit.git learnkit-build
  cd learnkit-build/cli
  cargo build --release
  cp target/release/learnkit ~/.cargo/bin/
  rm -rf /tmp/learnkit-build
else
  # 从 plugin 缓存编译
  cd "$PLUGIN_DIR"
  cargo build --release
  cp target/release/learnkit ~/.cargo/bin/
fi

# 验证
learnkit --help
```

### 3. 检测 Backend

```bash
curl -s http://localhost:13135/api/health
```

如果未运行，**自动启动**：

```bash
learnkit serve --port 13135 &
sleep 2
curl -s http://localhost:13135/api/health
```

### 4. 检测数据目录

```bash
ls ~/.learnkit/ 2>/dev/null
```

不存在没关系，`learnkit init` 会自动创建。

## 输出状态报告

```
LearnKit 环境检测
==================
✅ Rust: rustc 1.xx.x
✅ CLI: learnkit v0.1.0 (~/.cargo/bin/learnkit)
✅ Backend: http://localhost:13135 运行中
✅ 数据目录: ~/.learnkit/

环境就绪！开始使用：
  /learn-create <主题>    — 创建学习教程
  /learn-research         — 收集教学资源
  /learn-prepare          — 备课生成教案
```

## 被其他 skill 调用

其他 learn-* skill 执行前做快速检测：

```bash
which learnkit > /dev/null 2>&1 && curl -s http://localhost:13135/api/health > /dev/null 2>&1
```

失败 → 提示用户运行 `/learn-setup`。
