# LearnKit 教案组件规范

> 本文档定义教案内可使用的所有组件。备课时（learn-prepare）必须严格遵循此处的 HTML 结构。
> 模板会提供所有组件的 CSS 和 JS，备课只需输出正确的 HTML 标记。

---

## 目录

**基础排版**
1. [基础排版](#基础排版)
2. [代码相关](#代码相关)

**内容组件**
3. [概念提示 Callout](#概念提示-callout)
4. [重点要点 Key Point](#重点要点-key-point)
5. [测验题 Quiz](#测验题-quiz)
6. [对比卡片 Compare](#对比卡片-compare)
7. [步骤流程 Steps](#步骤流程-steps)
8. [案例分析 Case Study](#案例分析-case-study)
9. [术语定义 Definition](#术语定义-definition)
10. [可展开区域 Expandable](#可展开区域-expandable)
11. [引用名言 Quote Block](#引用名言-quote-block)
12. [分隔装饰 Section Divider](#分隔装饰-section-divider)

**已有组件**
13. [可展开思考题（旧）](#可展开思考题旧)
14. [对比卡片 VS（旧）](#对比卡片-vs旧)
15. [多 Tab 切换](#多-tab-切换)
16. [快捷键标记](#快捷键标记)
17. [文件树](#文件树)
18. [公式](#公式)
19. [流程图](#流程图)
20. [时间线](#时间线)
21. [图片与说明](#图片与说明)

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

## 概念提示 Callout

五种类型，各有不同颜色标记：

| 类型 | class | 颜色 | 场景 |
|------|-------|------|------|
| 信息 | `callout-info` | 蓝 #3b82f6 | 核心概念、知识点 |
| 提示 | `callout-tip` | 绿 #10b981 | 最佳实践、小技巧 |
| 警告 | `callout-warning` | 橙 #f59e0b | 常见陷阱、注意事项 |
| 重要 | `callout-important` | 紫 #8b5cf6 | 必须掌握的前置知识 |
| 备注 | `callout-note` | 灰 | 补充说明、旁注 |

```html
<div class="callout callout-info">
  <div class="callout-title">核心概念</div>
  <p>MDA 框架将游戏拆解为三个层次...</p>
</div>

<div class="callout callout-tip">
  <div class="callout-title">提示</div>
  <p>使用 <code>delta</code> 使移动帧率无关。</p>
</div>

<div class="callout callout-warning">
  <div class="callout-title">注意</div>
  <p>不要在 <code>_process</code> 中直接修改物理状态。</p>
</div>

<div class="callout callout-important">
  <div class="callout-title">重要</div>
  <p>ECS 是 Bevy 的核心架构，必须理解。</p>
</div>

<div class="callout callout-note">
  <div class="callout-title">备注</div>
  <p>此模式在 Godot 4 中略有不同。</p>
</div>
```

**注意：**
- `callout-title` 内容自定义，不限于"提示""注意"等固定文字
- 每个 callout 内可包含多个 `<p>`、`<ul>`、`<code>` 等内容
- 旧模板中 `.callout-indicator` + `.callout-content` 结构仍可用，但推荐使用上述扁平结构

---

## 重点要点 Key Point

高亮一句话或一段关键知识。左侧有 accent 色竖线标记，背景有轻微渐变。

```html
<div class="key-point">
  设计者从 Mechanics 出发，而玩家从 Aesthetics 出发感受。
</div>
```

**使用场景：**
- 每个章节的核心结论
- 总结性的一句话
- 需要读者一眼看到的重点

**注意：**
- 内容应该简短，1-3 句话为佳
- 一个章节内不要超过 2 个 key-point
- 不要和 callout 重复使用——callout 是"补充说明"，key-point 是"核心结论"

---

## 测验题 Quiz

### 选择题（推荐简化结构）

```html
<div class="quiz">
  <div class="quiz-question">在 MDA 框架中，"紧张刺激"属于哪个层次？</div>
  <div class="quiz-options">
    <button class="quiz-option" data-correct="false">Mechanics</button>
    <button class="quiz-option" data-correct="false">Dynamics</button>
    <button class="quiz-option" data-correct="true"
            data-explanation="Aesthetics 描述玩家的主观情感体验。">
      Aesthetics
    </button>
  </div>
  <div class="quiz-feedback"></div>
</div>
```

### 选择题（完整结构，带字母标记）

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
  </div>
  <div class="quiz-feedback"></div>
</div>
```

### 填空题

```html
<div class="quiz-block" data-type="fill">
  <div class="quiz-header">填空题</div>
  <p class="quiz-question">A* 算法的评估函数 f(n) = g(n) + <input class="quiz-fill-input" data-answer="h(n)" placeholder="?">。</p>
  <button class="quiz-submit">检查答案</button>
  <div class="quiz-feedback"></div>
</div>
```

**属性：**
- `data-correct`：`"true"` 或 `"false"`，标记正确答案
- `data-explanation`：可选，答题后显示的解释文字

**注意：**
- `.quiz` 是简化结构，`.quiz-block` 是完整结构，两者 CSS 都在模板中
- 简化结构更适合快速出题，完整结构支持多种题型（单选、多选、填空、排序、匹配）
- 每篇教案至少包含 2 道测验题

---

## 对比卡片 Compare

两列或三列对比，自动响应布局。每列顶部有不同颜色标记（蓝、紫、绿）。

### 两列对比

```html
<div class="compare">
  <div class="compare-item">
    <div class="compare-title">设计者视角</div>
    <p>M → D → A</p>
  </div>
  <div class="compare-item">
    <div class="compare-title">玩家视角</div>
    <p>A → D → M</p>
  </div>
</div>
```

### 三列对比

```html
<div class="compare">
  <div class="compare-item">
    <div class="compare-title">Mechanics</div>
    <p>规则、系统、数据</p>
  </div>
  <div class="compare-item">
    <div class="compare-title">Dynamics</div>
    <p>行为、策略、涌现</p>
  </div>
  <div class="compare-item">
    <div class="compare-title">Aesthetics</div>
    <p>感受、情感、体验</p>
  </div>
</div>
```

**注意：**
- compare-item 内可包含 `<p>`、`<ul>`、`<ol>` 等内容
- 移动端自动堆叠为单列
- 颜色标记按子元素顺序自动分配（第1个蓝，第2个紫，第3个绿）

---

## 步骤流程 Steps

带编号圆和连接线的步骤列表，适合教程操作指引、设计流程。

```html
<div class="steps">
  <div class="step">
    <div class="step-number">1</div>
    <div class="step-content">
      <div class="step-title">确定核心体验</div>
      <p>从 Aesthetics 出发，想清楚你想让玩家感受到什么。</p>
    </div>
  </div>
  <div class="step">
    <div class="step-number">2</div>
    <div class="step-content">
      <div class="step-title">设计动态行为</div>
      <p>思考什么样的 Dynamics 能产生目标体验。</p>
    </div>
  </div>
  <div class="step">
    <div class="step-number">3</div>
    <div class="step-content">
      <div class="step-title">构建底层机制</div>
      <p>最后设计支撑动态行为的 Mechanics 规则和系统。</p>
    </div>
  </div>
</div>
```

**注意：**
- step-number 手动填写数字
- step-content 内可嵌套代码块、列表等内容
- 连接线自动在步骤间绘制（CSS 伪元素实现）

---

## 案例分析 Case Study

带标签和标题的案例卡片，适合分析具体游戏、产品、项目。

```html
<div class="case-study">
  <div class="case-study-header">
    <span class="case-study-label">案例</span>
    <span class="case-study-title">俄罗斯方块</span>
  </div>
  <div class="case-study-body">
    <p><strong>Mechanics：</strong>方块随机生成、旋转、下落、消行。</p>
    <p><strong>Dynamics：</strong>速度加快，策略从保守转向冒险。</p>
    <p><strong>Aesthetics：</strong>"挑战"和"紧张"。</p>
  </div>
</div>
```

**注意：**
- case-study-label 文字可自定义（"案例""实战""分析"等）
- case-study-body 内可包含任意内容，包括列表、代码块
- 每个案例应有明确的分析维度

---

## 术语定义 Definition

使用语义化的 `<dl>` 标签，适合概念密集的章节开头或术语表。

```html
<dl class="definition">
  <dt>Mechanics（机制）</dt>
  <dd>游戏的规则和系统——数据表示、算法、玩家可执行的操作。</dd>
  <dt>Dynamics（动态）</dt>
  <dd>机制在运行时产生的行为模式——玩家策略、系统涌现。</dd>
  <dt>Aesthetics（体验）</dt>
  <dd>玩家的主观情感体验——乐趣、紧张、好奇。</dd>
</dl>
```

**注意：**
- 每个 `<dt>` 前有紫色圆点装饰
- `<dd>` 之间有分隔线
- 也可以使用旧的 `.definition` div 结构（`.definition-term` + `.definition-body`），两种都有 CSS 支持

---

## 可展开区域 Expandable

基于 `<details>` 原生语义，默认折叠，点击展开。

```html
<details class="expandable">
  <summary>展开查看详细分析</summary>
  <div class="expandable-content">
    <p>详细内容...</p>
  </div>
</details>
```

**使用场景：**
- 思考题答案
- 延伸阅读
- 非核心但有用的补充信息
- 代码完整实现（摘要只展示关键部分）

**注意：**
- 模板同时支持 `<details>` 和旧的 `.expandable` + `.expandable-trigger` JS 方案
- 推荐使用 `<details>` 方案，更轻量且语义正确
- expandable-content 内可嵌套任意组件

---

## 引用名言 Quote Block

带来源归属的引用，使用语义化的 `<figure>` + `<blockquote>` 结构。

```html
<figure class="quote-block">
  <blockquote>游戏设计不是关于规则，而是关于体验。</blockquote>
  <figcaption>— Jesse Schell, The Art of Game Design</figcaption>
</figure>
```

**使用场景：**
- 行业专家名言
- 论文/书籍摘要
- 经典表述
- 原始资料引用

**注意：**
- 与普通 `<blockquote>` 不同，这是带来源归属的完整引用
- 引用文本会显示为斜体大号字
- figcaption 以"—"开头标注来源

---

## 分隔装饰 Section Divider

章节之间的视觉分隔符，中部有 accent 色渐变。

```html
<hr class="section-divider">
```

**使用场景：**
- 大章节之间的过渡
- 内容主题切换时的视觉断点

**注意：**
- 不要过度使用，一篇教案中 2-3 个为宜
- 普通 `<hr>` 仍可用于较轻的分隔

---

## 可展开思考题（旧）

旧的 JS 驱动方案，仍然可用。

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

## 对比卡片 VS（旧）

旧的 A vs B 双栏对比，仍然可用。

```html
<div class="vs-card">
  <div class="vs-card-side">
    <div class="vs-card-header">Godot 场景树</div>
    <ul>
      <li>节点继承体系</li>
      <li>信号通信</li>
    </ul>
  </div>
  <div class="vs-card-divider">VS</div>
  <div class="vs-card-side">
    <div class="vs-card-header">Bevy ECS</div>
    <ul>
      <li>数据驱动组合</li>
      <li>System 调度</li>
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
  </div>
  <div class="content-tab-panel active" data-tab="godot">
    <p>在 Godot 中，节点通过 <code>add_child()</code> 添加到场景树...</p>
  </div>
  <div class="content-tab-panel" data-tab="bevy">
    <p>在 Bevy 中，使用 <code>commands.spawn()</code> 创建实体...</p>
  </div>
</div>
```

---

## 快捷键标记

展示键盘按键组合。

```html
<p>按 <kbd>Ctrl</kbd> + <kbd>S</kbd> 保存场景。</p>
```

---

## 文件树

展示项目目录结构。

```html
<div class="file-tree">
  <div class="file-tree-title">项目结构</div>
  <pre class="file-tree-content">project/
├── src/
│   ├── main.rs
│   └── components/
└── Cargo.toml</pre>
</div>
```

---

## 公式

数学公式使用 `formula` 组件。

```html
<div class="formula">
  <div class="formula-content">damage = ATK × multiplier - DEF × armor</div>
  <div class="formula-caption">伤害计算公式</div>
</div>
```

行内公式：
```html
<p>经验曲线采用 <span class="formula-inline">XP(n) = base × n<sup>exp</sup></span> 的指数模型。</p>
```

---

## 流程图

### ASCII 流程图

```html
<div class="diagram">
  <div class="diagram-title">游戏主循环</div>
  <pre class="diagram-content">┌─────────┐     ┌──────────┐     ┌────────┐
│  Input  │ ──▶ │  Update  │ ──▶ │ Render │
└─────────┘     └──────────┘     └────────┘</pre>
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
  </div>
</div>
```

---

## 时间线

```html
<div class="timeline">
  <div class="timeline-item">
    <div class="timeline-marker"></div>
    <div class="timeline-content">
      <div class="timeline-date">2014</div>
      <div class="timeline-title">Godot 1.0 发布</div>
      <p>首个公开版本。</p>
    </div>
  </div>
</div>
```

---

## 图片与说明

```html
<figure class="figure">
  <img src="path/to/image.png" alt="描述文字">
  <figcaption>图 1：MDA 框架示意图</figcaption>
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
7. **key-point vs callout**：key-point 用于核心结论（一个章节最多 2 个），callout 用于补充说明
8. **quote-block vs blockquote**：quote-block 是带来源的正式引用，blockquote 是普通引用/强调
9. **section-divider 适度**：一篇教案中 2-3 个为宜，用于大主题切换
