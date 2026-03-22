# LearnKit 教案组件规范

> 本文档定义教案内可使用的所有组件。备课时（learn-prepare）必须严格遵循此处的 HTML 结构。
> 模板会提供所有组件的 CSS 和 JS，备课只需输出正确的 HTML 标记。

---

## 目录

1. [基础排版](#基础排版)
2. [代码相关](#代码相关)
3. [提示框 Callout](#提示框-callout)
4. [术语定义](#术语定义)
5. [测验题](#测验题)
6. [可展开思考题](#可展开思考题)
7. [对比卡片](#对比卡片)
8. [多 Tab 切换](#多-tab-切换)
9. [步骤条](#步骤条)
10. [快捷键标记](#快捷键标记)
11. [文件树](#文件树)
12. [公式](#公式)
13. [流程图](#流程图)
14. [时间线](#时间线)
15. [图片与说明](#图片与说明)

---

## 基础排版

标准 HTML 元素，无需特殊 class。

```html
<h2 id="section-slug">章节标题</h2>
<h3>子标题</h3>
<h4>小标题</h4>
<p>正文段落，<strong>加粗</strong>和<em>斜体</em>和<code>行内代码</code>。</p>
<ul>
  <li>无序列表项</li>
</ul>
<ol>
  <li>有序列表项</li>
</ol>
<blockquote><p>引用文字</p></blockquote>
<table>
  <thead><tr><th>表头</th><th>表头</th></tr></thead>
  <tbody><tr><td>单元格</td><td>单元格</td></tr></tbody>
</table>
<hr>
```

---

## 代码相关

### 代码块

带语言标签的代码块。`data-title` 可选，用于显示文件名或说明。

```html
<div class="code-block" data-lang="python" data-title="damage_calculator.py">
<pre><code class="language-python">def calculate_damage(base, multiplier):
    return base * multiplier</code></pre>
</div>
```

**属性：**
- `data-lang`：语言标识（显示在右上角标签）
- `data-title`：可选，文件名或标题（显示在顶部栏）

### 多语言代码切换

同一逻辑的多语言实现，用 Tab 切换。

```html
<div class="code-tabs">
  <div class="code-tabs-nav">
    <button class="code-tab active" data-tab="gdscript">GDScript</button>
    <button class="code-tab" data-tab="rust">Rust</button>
    <button class="code-tab" data-tab="csharp">C#</button>
  </div>
  <div class="code-tab-panel active" data-tab="gdscript">
    <pre><code class="language-gdscript">func _ready():
    print("Hello")</code></pre>
  </div>
  <div class="code-tab-panel" data-tab="rust">
    <pre><code class="language-rust">fn main() {
    println!("Hello");
}</code></pre>
  </div>
  <div class="code-tab-panel" data-tab="csharp">
    <pre><code class="language-csharp">public override void _Ready() {
    GD.Print("Hello");
}</code></pre>
  </div>
</div>
```

### 文件树

展示项目目录结构。

```html
<div class="file-tree">
  <div class="file-tree-title">项目结构</div>
  <pre class="file-tree-content">project/
├── src/
│   ├── main.rs
│   ├── components/
│   │   ├── player.rs
│   │   └── enemy.rs
│   └── systems/
│       └── movement.rs
├── assets/
│   ├── sprites/
│   └── audio/
└── Cargo.toml</pre>
</div>
```

### 代码 Diff

展示代码修改前后对比。

```html
<div class="code-diff" data-title="修改 player.gd">
  <pre><code class="language-diff">  func _physics_process(delta):
-     velocity.x = speed
+     velocity.x = speed * delta
      move_and_slide()</code></pre>
</div>
```

---

## 提示框 Callout

四种类型：`tip`、`warning`、`note`、`important`。

```html
<div class="callout callout-tip">
  <div class="callout-indicator"></div>
  <div class="callout-content">
    <div class="callout-title">提示</div>
    <p>使用 <code>delta</code> 使移动帧率无关。</p>
  </div>
</div>

<div class="callout callout-warning">
  <div class="callout-indicator"></div>
  <div class="callout-content">
    <div class="callout-title">注意</div>
    <p>不要在 <code>_process</code> 中直接修改物理状态。</p>
  </div>
</div>

<div class="callout callout-note">
  <div class="callout-indicator"></div>
  <div class="callout-content">
    <div class="callout-title">备注</div>
    <p>此模式在 Godot 4 中略有不同。</p>
  </div>
</div>

<div class="callout callout-important">
  <div class="callout-indicator"></div>
  <div class="callout-content">
    <div class="callout-title">重要</div>
    <p>ECS 是 Bevy 的核心架构，必须理解。</p>
  </div>
</div>
```

---

## 术语定义

关键概念的定义卡片。

```html
<div class="definition">
  <div class="definition-term">Entity-Component-System (ECS)</div>
  <div class="definition-body">
    <p>一种数据驱动的架构模式，将游戏对象拆分为实体（ID）、组件（数据）和系统（逻辑）三个正交维度，以组合取代继承。</p>
  </div>
</div>
```

---

## 测验题

### 单选题

```html
<div class="quiz-block" data-type="single">
  <div class="quiz-header">选择题</div>
  <p class="quiz-question">MDA 框架中的 D 代表什么？</p>
  <div class="quiz-options">
    <button class="quiz-option" data-correct="false" data-explanation="">
      <span class="quiz-option-letter">A</span> Design
    </button>
    <button class="quiz-option" data-correct="true" data-explanation="Dynamics 描述机制在运行时产生的行为模式。">
      <span class="quiz-option-letter">B</span> Dynamics
    </button>
    <button class="quiz-option" data-correct="false" data-explanation="">
      <span class="quiz-option-letter">C</span> Development
    </button>
    <button class="quiz-option" data-correct="false" data-explanation="">
      <span class="quiz-option-letter">D</span> Distribution
    </button>
  </div>
  <div class="quiz-feedback"></div>
</div>
```

### 多选题

```html
<div class="quiz-block" data-type="multi">
  <div class="quiz-header">多选题</div>
  <p class="quiz-question">以下哪些属于游戏设计模式？（选择所有正确答案）</p>
  <div class="quiz-options">
    <button class="quiz-option" data-correct="true">
      <span class="quiz-option-check"></span> 对象池模式
    </button>
    <button class="quiz-option" data-correct="true">
      <span class="quiz-option-check"></span> 观察者模式
    </button>
    <button class="quiz-option" data-correct="false">
      <span class="quiz-option-check"></span> 瀑布模型
    </button>
    <button class="quiz-option" data-correct="true">
      <span class="quiz-option-check"></span> 状态机模式
    </button>
  </div>
  <button class="quiz-submit">提交答案</button>
  <div class="quiz-feedback"></div>
</div>
```

### 判断题

```html
<div class="quiz-block" data-type="truefalse">
  <div class="quiz-header">判断题</div>
  <p class="quiz-question">在 ECS 架构中，组件（Component）包含行为逻辑。</p>
  <div class="quiz-options">
    <button class="quiz-option" data-correct="false" data-explanation="组件只持有数据，行为逻辑由系统（System）处理。">
      <span class="quiz-option-letter">✓</span> 正确
    </button>
    <button class="quiz-option" data-correct="true" data-explanation="组件只持有数据，行为逻辑由系统（System）处理。">
      <span class="quiz-option-letter">✗</span> 错误
    </button>
  </div>
  <div class="quiz-feedback"></div>
</div>
```

### 填空题

```html
<div class="quiz-block" data-type="fill">
  <div class="quiz-header">填空题</div>
  <p class="quiz-question">A* 算法的评估函数 f(n) = g(n) + <input class="quiz-fill-input" data-answer="h(n)" placeholder="?">，其中 g(n) 是实际代价，另一项是<input class="quiz-fill-input" data-answer="启发式估计" placeholder="?">。</p>
  <button class="quiz-submit">检查答案</button>
  <div class="quiz-feedback"></div>
</div>
```

### 排序题

```html
<div class="quiz-block" data-type="order">
  <div class="quiz-header">排序题</div>
  <p class="quiz-question">将渲染管线的阶段按正确顺序排列：</p>
  <div class="quiz-order-list">
    <div class="quiz-order-item" data-order="3" draggable="true">
      <span class="quiz-order-handle">⠿</span> 光栅化
    </div>
    <div class="quiz-order-item" data-order="1" draggable="true">
      <span class="quiz-order-handle">⠿</span> 顶点处理
    </div>
    <div class="quiz-order-item" data-order="4" draggable="true">
      <span class="quiz-order-handle">⠿</span> 片元着色
    </div>
    <div class="quiz-order-item" data-order="2" draggable="true">
      <span class="quiz-order-handle">⠿</span> 图元装配
    </div>
  </div>
  <button class="quiz-submit">检查顺序</button>
  <div class="quiz-feedback"></div>
</div>
```

### 连线匹配题

```html
<div class="quiz-block" data-type="match">
  <div class="quiz-header">匹配题</div>
  <p class="quiz-question">将设计模式与其主要用途连线：</p>
  <div class="quiz-match">
    <div class="quiz-match-left">
      <div class="quiz-match-item" data-match="a">观察者模式</div>
      <div class="quiz-match-item" data-match="b">对象池</div>
      <div class="quiz-match-item" data-match="c">命令模式</div>
    </div>
    <div class="quiz-match-right">
      <div class="quiz-match-item" data-match="c">撤销/重做</div>
      <div class="quiz-match-item" data-match="a">事件解耦</div>
      <div class="quiz-match-item" data-match="b">减少 GC 压力</div>
    </div>
  </div>
  <button class="quiz-submit">检查匹配</button>
  <div class="quiz-feedback"></div>
</div>
```

---

## 可展开思考题

```html
<div class="expandable">
  <button class="expandable-trigger">
    <span>思考：为什么格斗游戏通常不使用物理引擎？</span>
    <span class="expandable-chevron">▼</span>
  </button>
  <div class="expandable-body">
    <div class="expandable-body-inner">
      <p>格斗游戏需要精确到帧的判定...</p>
    </div>
  </div>
</div>
```

---

## 对比卡片

A vs B 双栏对比，适合引擎/技术/方案对比。

```html
<div class="vs-card">
  <div class="vs-card-side">
    <div class="vs-card-header">Godot 场景树</div>
    <ul>
      <li>节点继承体系</li>
      <li>信号通信</li>
      <li>GDScript 脚本绑定</li>
      <li>可视化编辑器</li>
    </ul>
  </div>
  <div class="vs-card-divider">VS</div>
  <div class="vs-card-side">
    <div class="vs-card-header">Bevy ECS</div>
    <ul>
      <li>数据驱动组合</li>
      <li>System 调度</li>
      <li>Rust 类型安全</li>
      <li>纯代码定义</li>
    </ul>
  </div>
</div>
```

---

## 多 Tab 切换

适用于同一概念在不同引擎/语境下的展示（非代码场景）。

```html
<div class="content-tabs">
  <div class="content-tabs-nav">
    <button class="content-tab active" data-tab="godot">Godot 中</button>
    <button class="content-tab" data-tab="bevy">Bevy 中</button>
    <button class="content-tab" data-tab="unity">Unity 中</button>
  </div>
  <div class="content-tab-panel active" data-tab="godot">
    <p>在 Godot 中，节点通过 <code>add_child()</code> 添加到场景树...</p>
  </div>
  <div class="content-tab-panel" data-tab="bevy">
    <p>在 Bevy 中，使用 <code>commands.spawn()</code> 创建实体...</p>
  </div>
  <div class="content-tab-panel" data-tab="unity">
    <p>在 Unity 中，通过 <code>Instantiate()</code> 实例化预制体...</p>
  </div>
</div>
```

---

## 步骤条

编号步骤的操作指引，适合教程类内容。

```html
<div class="steps">
  <div class="step">
    <div class="step-number">1</div>
    <div class="step-content">
      <div class="step-title">创建项目</div>
      <p>打开 Godot，点击「新建项目」，选择一个空目录。</p>
    </div>
  </div>
  <div class="step">
    <div class="step-number">2</div>
    <div class="step-content">
      <div class="step-title">添加根节点</div>
      <p>在场景面板中选择 <code>Node2D</code> 作为根节点。</p>
    </div>
  </div>
  <div class="step">
    <div class="step-number">3</div>
    <div class="step-content">
      <div class="step-title">编写脚本</div>
      <p>右键根节点，选择「附加脚本」，编写你的第一个 GDScript。</p>
    </div>
  </div>
</div>
```

---

## 快捷键标记

展示键盘按键组合。

```html
<p>按 <kbd>Ctrl</kbd> + <kbd>S</kbd> 保存场景。</p>
<p>使用 <kbd>F5</kbd> 运行项目，<kbd>F6</kbd> 运行当前场景。</p>
```

复杂快捷键表格：

```html
<div class="shortcut-table">
  <table>
    <thead><tr><th>操作</th><th>快捷键</th></tr></thead>
    <tbody>
      <tr><td>运行项目</td><td><kbd>F5</kbd></td></tr>
      <tr><td>运行当前场景</td><td><kbd>F6</kbd></td></tr>
      <tr><td>暂停</td><td><kbd>F7</kbd></td></tr>
      <tr><td>停止</td><td><kbd>F8</kbd></td></tr>
    </tbody>
  </table>
</div>
```

---

## 文件树

见 [代码相关](#文件树) 部分。

---

## 公式

数学公式使用 `formula` 组件。行内用 `formula-inline`。

块级公式：
```html
<div class="formula">
  <div class="formula-content">damage = (ATK × skill_multiplier - DEF × armor_rate) × element_bonus</div>
  <div class="formula-caption">伤害计算公式（减法型）</div>
</div>
```

行内公式：
```html
<p>经验曲线通常采用 <span class="formula-inline">XP(n) = base × n<sup>exponent</sup></span> 的指数模型。</p>
```

含变量解释的公式：
```html
<div class="formula">
  <div class="formula-content">f(n) = g(n) + h(n)</div>
  <div class="formula-vars">
    <div class="formula-var"><code>f(n)</code> — 节点 n 的总评估代价</div>
    <div class="formula-var"><code>g(n)</code> — 从起点到 n 的实际代价</div>
    <div class="formula-var"><code>h(n)</code> — 从 n 到终点的启发式估计</div>
  </div>
</div>
```

---

## 流程图

使用 ASCII 或 HTML 结构的流程图。

### ASCII 流程图

```html
<div class="diagram">
  <div class="diagram-title">游戏主循环</div>
  <pre class="diagram-content">┌─────────┐     ┌──────────┐     ┌────────┐
│  Input  │ ──▶ │  Update  │ ──▶ │ Render │
└─────────┘     └──────────┘     └────────┘
     ▲                                │
     └────────────────────────────────┘</pre>
</div>
```

### HTML 流程图

```html
<div class="flowchart">
  <div class="flowchart-title">状态机：敌人 AI</div>
  <div class="flowchart-nodes">
    <div class="flowchart-node">巡逻</div>
    <div class="flowchart-arrow">发现玩家 →</div>
    <div class="flowchart-node flowchart-node-active">追击</div>
    <div class="flowchart-arrow">进入攻击范围 →</div>
    <div class="flowchart-node">攻击</div>
  </div>
  <div class="flowchart-nodes">
    <div class="flowchart-arrow-down">↑ 丢失目标</div>
    <div class="flowchart-spacer"></div>
    <div class="flowchart-arrow-down">↓ HP &lt; 30%</div>
  </div>
  <div class="flowchart-nodes">
    <div class="flowchart-spacer"></div>
    <div class="flowchart-spacer"></div>
    <div class="flowchart-node flowchart-node-danger">逃跑</div>
  </div>
</div>
```

---

## 时间线

历史沿革、版本演进、开发阶段等。

```html
<div class="timeline">
  <div class="timeline-item">
    <div class="timeline-marker"></div>
    <div class="timeline-content">
      <div class="timeline-date">2014</div>
      <div class="timeline-title">Godot 1.0 发布</div>
      <p>首个公开版本，基于自研渲染器。</p>
    </div>
  </div>
  <div class="timeline-item">
    <div class="timeline-marker"></div>
    <div class="timeline-content">
      <div class="timeline-date">2018</div>
      <div class="timeline-title">Godot 3.0</div>
      <p>引入 PBR 渲染、GDNative、C# 支持。</p>
    </div>
  </div>
  <div class="timeline-item">
    <div class="timeline-marker"></div>
    <div class="timeline-content">
      <div class="timeline-date">2023</div>
      <div class="timeline-title">Godot 4.0</div>
      <p>全新 Vulkan 渲染器、GDScript 2.0、大幅 API 重构。</p>
    </div>
  </div>
</div>
```

---

## 图片与说明

带标题的图片。

```html
<figure class="figure">
  <img src="path/to/image.png" alt="描述文字">
  <figcaption>图 1：MDA 框架的三层模型示意图</figcaption>
</figure>
```

---

## 使用原则

1. **不要滥用**：每种组件在合适的场景使用，不为炫技而用
2. **至少包含 2-3 个交互组件**：每篇教案应包含测验题、思考题、或 Tab 切换等交互元素
3. **代码块必须标注语言**：`data-lang` 属性不能省略
4. **测验题覆盖核心概念**：测验题应检验关键知识点，而非细枝末节
5. **组件可嵌套**：例如步骤条内可以包含代码块，思考题内可以包含列表
6. **保持克制**：不要在一个 section 内堆砌过多组件，保持阅读节奏
