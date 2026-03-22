---
name: learn-setup
description: >
  检测并配置 LearnKit 运行环境。检查 Rust CLI 是否编译、Backend 是否运行、Frontend 是否构建。
  缺什么引导安装什么。其他 learn-* skill 在发现环境未就绪时会自动调用此 skill。
  Triggers on: "learn-setup", "配置 learnkit", "安装 learnkit"
---

# Learn Setup — 环境配置向导

检测 LearnKit 运行环境，缺什么装什么。

## 检测清单

按顺序检测，遇到缺失就引导用户修复：

### 1. Rust 工具链

```bash
rustc --version
cargo --version
```

如果缺失：
> 需要安装 Rust。运行：`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### 2. LearnKit CLI 二进制

```bash
which learnkit || ls ~/.cargo/bin/learnkit
```

如果缺失，需要编译：

```bash
# 找到 plugin 缓存中的 learnkit 源码
# 或者从 GitHub 克隆
cd /tmp
git clone https://github.com/chasey-myagi/learnkit.git learnkit-build
cd learnkit-build/cli
cargo build --release
cp target/release/learnkit ~/.cargo/bin/
```

验证：
```bash
learnkit --help
```

### 3. LearnKit Backend 运行状态

```bash
curl -s http://localhost:13135/api/health
```

如果未运行：
```bash
learnkit serve --port 13135 &
```

等待 2 秒后重新检测。

### 4. Node.js（可选，开发模式需要）

```bash
node --version
```

如果用户需要开发模式（前端热重载），需要 Node.js。
生产使用时，Backend 直接 serve 前端静态文件，不需要 Node。

### 5. 数据目录

```bash
ls ~/.learnkit/ 2>/dev/null || echo "not found"
```

如果不存在，`learnkit init` 会自动创建，无需手动处理。

## 输出

检测完成后输出状态报告：

```
LearnKit 环境检测
==================
✅ Rust 工具链: rustc 1.xx.x
✅ LearnKit CLI: v0.1.0
✅ Backend: http://localhost:13135 运行中
⬚  Frontend: 未构建（可选，使用 Backend serve）
✅ 数据目录: ~/.learnkit/

环境就绪！你可以开始使用：
  /learn-create <主题>    — 创建学习教程
  /learn-research         — 收集教学资源
  /learn-prepare          — 备课生成教案
```

## 自动调用

其他 skill（learn-create / learn-research / learn-prepare）在执行前应先检测关键依赖：

```bash
# 快速检测（不走完整 setup）
which learnkit > /dev/null 2>&1 && curl -s http://localhost:13135/api/health > /dev/null 2>&1
```

如果检测失败，提示用户运行 `/learn-setup`。
