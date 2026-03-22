import { describe, test, expect, afterEach, beforeEach } from 'vitest';
import {
  renderAllComponents,
  renderCallouts,
  renderDefinitions,
  renderExpandables,
  renderComparisons,
  renderTabs,
  renderSteps,
  renderTimelines,
  renderFormulas,
  renderDiagrams,
  renderFigures,
  renderFlowcharts,
} from '../../cli/templates/lesson.js';

afterEach(() => {
  document.body.innerHTML = '';
});

// ============================================================
// lk-callout 提示框
// ============================================================
describe('提示框 (lk-callout)', () => {
  test('渲染 tip 类型提示框', () => {
    document.body.innerHTML = `
      <lk-callout type="tip">使用 Ctrl+S 保存。</lk-callout>
    `;
    renderCallouts(document.body);

    const callout = document.querySelector('.callout');
    expect(callout).not.toBeNull();
    expect(callout.classList.contains('callout-tip')).toBe(true);
    expect(callout.textContent).toContain('使用 Ctrl+S 保存。');
  });

  test('渲染 warning 类型提示框', () => {
    document.body.innerHTML = `
      <lk-callout type="warning">此操作不可逆！</lk-callout>
    `;
    renderCallouts(document.body);

    const callout = document.querySelector('.callout');
    expect(callout).not.toBeNull();
    expect(callout.classList.contains('callout-warning')).toBe(true);
  });

  test('渲染 note 类型提示框', () => {
    document.body.innerHTML = `
      <lk-callout type="note">这是一个注释。</lk-callout>
    `;
    renderCallouts(document.body);

    const callout = document.querySelector('.callout.callout-note');
    expect(callout).not.toBeNull();
  });

  test('渲染 important 类型提示框', () => {
    document.body.innerHTML = `
      <lk-callout type="important">重要信息。</lk-callout>
    `;
    renderCallouts(document.body);

    const callout = document.querySelector('.callout.callout-important');
    expect(callout).not.toBeNull();
  });

  test('每种类型有对应图标', () => {
    document.body.innerHTML = `
      <lk-callout type="tip">提示</lk-callout>
      <lk-callout type="warning">警告</lk-callout>
      <lk-callout type="note">注释</lk-callout>
      <lk-callout type="important">重要</lk-callout>
    `;
    renderCallouts(document.body);

    const callouts = document.querySelectorAll('.callout');
    expect(callouts.length).toBe(4);
    callouts.forEach(c => {
      const icon = c.querySelector('.callout-icon');
      expect(icon).not.toBeNull();
    });
  });

  test('无 type 属性时使用默认样式', () => {
    document.body.innerHTML = `
      <lk-callout>默认提示。</lk-callout>
    `;
    expect(() => renderCallouts(document.body)).not.toThrow();
    const callout = document.querySelector('.callout');
    expect(callout).not.toBeNull();
  });

  test('空内容不报错', () => {
    document.body.innerHTML = `<lk-callout type="tip"></lk-callout>`;
    expect(() => renderCallouts(document.body)).not.toThrow();
  });
});

// ============================================================
// lk-definition 术语卡片
// ============================================================
describe('术语卡片 (lk-definition)', () => {
  test('渲染术语卡片', () => {
    document.body.innerHTML = `
      <lk-definition term="ECS">Entity-Component-System，一种数据驱动架构。</lk-definition>
    `;
    renderDefinitions(document.body);

    const card = document.querySelector('.definition-card');
    expect(card).not.toBeNull();
  });

  test('显示术语名称', () => {
    document.body.innerHTML = `
      <lk-definition term="ECS">Entity-Component-System。</lk-definition>
    `;
    renderDefinitions(document.body);

    const term = document.querySelector('.definition-term');
    expect(term).not.toBeNull();
    expect(term.textContent).toContain('ECS');
  });

  test('显示定义内容', () => {
    document.body.innerHTML = `
      <lk-definition term="ECS">Entity-Component-System，一种数据驱动架构。</lk-definition>
    `;
    renderDefinitions(document.body);

    const body = document.querySelector('.definition-body, .definition-content');
    expect(body).not.toBeNull();
    expect(body.textContent).toContain('Entity-Component-System');
  });

  test('无 term 属性时不报错', () => {
    document.body.innerHTML = `
      <lk-definition>某个定义。</lk-definition>
    `;
    expect(() => renderDefinitions(document.body)).not.toThrow();
  });

  test('空内容不报错', () => {
    document.body.innerHTML = `<lk-definition term="空"></lk-definition>`;
    expect(() => renderDefinitions(document.body)).not.toThrow();
  });

  test('多个术语卡片同时渲染', () => {
    document.body.innerHTML = `
      <lk-definition term="ECS">架构模式</lk-definition>
      <lk-definition term="OOP">面向对象</lk-definition>
    `;
    renderDefinitions(document.body);

    const cards = document.querySelectorAll('.definition-card');
    expect(cards.length).toBe(2);
  });
});

// ============================================================
// lk-expandable 可展开块
// ============================================================
describe('可展开块 (lk-expandable)', () => {
  beforeEach(() => {
    document.body.innerHTML = `
      <lk-expandable title="思考：为什么用 ECS？">
        因为 ECS 将数据和逻辑分离，便于并行处理和缓存友好。
      </lk-expandable>
    `;
    renderExpandables(document.body);
  });

  test('渲染可展开块', () => {
    const block = document.querySelector('.expandable');
    expect(block).not.toBeNull();
  });

  test('显示标题', () => {
    const header = document.querySelector('.expandable-header, .expandable-toggle');
    expect(header).not.toBeNull();
    expect(header.textContent).toContain('思考：为什么用 ECS？');
  });

  test('默认折叠（内容不可见）', () => {
    const content = document.querySelector('.expandable-content');
    expect(content).not.toBeNull();
    expect(
      content.style.display === 'none' ||
      content.hidden ||
      !content.classList.contains('open')
    ).toBe(true);
  });

  test('点击标题后展开', () => {
    const header = document.querySelector('.expandable-header, .expandable-toggle');
    header.click();

    const content = document.querySelector('.expandable-content');
    expect(
      content.style.display !== 'none' &&
      !content.hidden
    ).toBe(true);
    expect(content.textContent).toContain('ECS 将数据和逻辑分离');
  });

  test('再次点击后折叠', () => {
    const header = document.querySelector('.expandable-header, .expandable-toggle');
    header.click(); // 展开
    header.click(); // 折叠

    const content = document.querySelector('.expandable-content');
    expect(
      content.style.display === 'none' ||
      content.hidden ||
      !content.classList.contains('open')
    ).toBe(true);
  });

  test('展开/折叠切换图标状态', () => {
    const header = document.querySelector('.expandable-header, .expandable-toggle');
    const block = document.querySelector('.expandable');

    header.click();
    expect(block.classList.contains('open') || block.classList.contains('expanded')).toBe(true);

    header.click();
    expect(block.classList.contains('open') || block.classList.contains('expanded')).toBe(false);
  });

  test('无 title 时不报错', () => {
    document.body.innerHTML = `
      <lk-expandable>内容</lk-expandable>
    `;
    expect(() => renderExpandables(document.body)).not.toThrow();
  });
});

// ============================================================
// lk-vs 对比卡片
// ============================================================
describe('对比卡片 (lk-vs)', () => {
  beforeEach(() => {
    document.body.innerHTML = `
      <lk-vs>
        <lk-vs-side title="Godot">
          开源免费，GDScript 简单易学
        </lk-vs-side>
        <lk-vs-side title="Unity">
          生态成熟，C# 性能强
        </lk-vs-side>
      </lk-vs>
    `;
    renderComparisons(document.body);
  });

  test('渲染对比块', () => {
    const vs = document.querySelector('.vs-block, .comparison-block');
    expect(vs).not.toBeNull();
  });

  test('渲染两个对比面', () => {
    const sides = document.querySelectorAll('.vs-side, .comparison-side');
    expect(sides.length).toBe(2);
  });

  test('每个面显示标题', () => {
    const sides = document.querySelectorAll('.vs-side, .comparison-side');
    expect(sides[0].textContent).toContain('Godot');
    expect(sides[1].textContent).toContain('Unity');
  });

  test('每个面显示内容', () => {
    const sides = document.querySelectorAll('.vs-side, .comparison-side');
    expect(sides[0].textContent).toContain('开源免费');
    expect(sides[1].textContent).toContain('生态成熟');
  });

  test('只有一个 side 时正常渲染', () => {
    document.body.innerHTML = `
      <lk-vs>
        <lk-vs-side title="唯一">内容</lk-vs-side>
      </lk-vs>
    `;
    expect(() => renderComparisons(document.body)).not.toThrow();
    const sides = document.querySelectorAll('.vs-side, .comparison-side');
    expect(sides.length).toBe(1);
  });

  test('空 vs 不报错', () => {
    document.body.innerHTML = `<lk-vs></lk-vs>`;
    expect(() => renderComparisons(document.body)).not.toThrow();
  });
});

// ============================================================
// lk-tabs 内容 Tab 切换
// ============================================================
describe('内容 Tab 切换 (lk-tabs)', () => {
  beforeEach(() => {
    document.body.innerHTML = `
      <lk-tabs>
        <lk-tab label="Godot">Godot 使用场景树。</lk-tab>
        <lk-tab label="Unity">Unity 使用 GameObject。</lk-tab>
      </lk-tabs>
    `;
    renderTabs(document.body);
  });

  test('渲染 Tab 容器', () => {
    const tabs = document.querySelector('.tabs-block, .content-tabs');
    expect(tabs).not.toBeNull();
  });

  test('渲染 Tab 按钮', () => {
    const btns = document.querySelectorAll('.tab-btn');
    expect(btns.length).toBe(2);
    expect(btns[0].textContent.trim()).toBe('Godot');
    expect(btns[1].textContent.trim()).toBe('Unity');
  });

  test('默认激活第一个 Tab', () => {
    const btns = document.querySelectorAll('.tab-btn');
    expect(btns[0].classList.contains('active')).toBe(true);

    const panels = document.querySelectorAll('.tab-panel');
    expect(panels[0].style.display !== 'none' && !panels[0].hidden).toBe(true);
  });

  test('点击切换 Tab', () => {
    const btns = document.querySelectorAll('.tab-btn');
    const panels = document.querySelectorAll('.tab-panel');

    btns[1].click();

    expect(btns[1].classList.contains('active')).toBe(true);
    expect(btns[0].classList.contains('active')).toBe(false);
    expect(panels[1].style.display !== 'none' && !panels[1].hidden).toBe(true);
    expect(panels[0].style.display === 'none' || panels[0].hidden).toBe(true);
  });

  test('面板内容正确', () => {
    const panels = document.querySelectorAll('.tab-panel');
    expect(panels[0].textContent).toContain('场景树');
    expect(panels[1].textContent).toContain('GameObject');
  });

  test('空 Tab 列表不报错', () => {
    document.body.innerHTML = `<lk-tabs></lk-tabs>`;
    expect(() => renderTabs(document.body)).not.toThrow();
  });
});

// ============================================================
// lk-steps 步骤条
// ============================================================
describe('步骤条 (lk-steps)', () => {
  beforeEach(() => {
    document.body.innerHTML = `
      <lk-steps>
        <lk-step title="创建项目">在 Godot 中新建项目。</lk-step>
        <lk-step title="添加场景">创建主场景。</lk-step>
        <lk-step title="编写脚本">给节点添加 GDScript。</lk-step>
      </lk-steps>
    `;
    renderSteps(document.body);
  });

  test('渲染步骤条容器', () => {
    const steps = document.querySelector('.steps-block');
    expect(steps).not.toBeNull();
  });

  test('渲染正确数量的步骤', () => {
    const items = document.querySelectorAll('.step-item');
    expect(items.length).toBe(3);
  });

  test('每个步骤显示序号', () => {
    const items = document.querySelectorAll('.step-item');
    items.forEach((item, i) => {
      const number = item.querySelector('.step-number');
      expect(number).not.toBeNull();
      expect(number.textContent.trim()).toBe(String(i + 1));
    });
  });

  test('每个步骤显示标题', () => {
    const titles = document.querySelectorAll('.step-title');
    expect(titles[0].textContent).toContain('创建项目');
    expect(titles[1].textContent).toContain('添加场景');
    expect(titles[2].textContent).toContain('编写脚本');
  });

  test('每个步骤显示内容', () => {
    const items = document.querySelectorAll('.step-item');
    expect(items[0].textContent).toContain('在 Godot 中新建项目');
    expect(items[1].textContent).toContain('创建主场景');
    expect(items[2].textContent).toContain('给节点添加 GDScript');
  });

  test('空步骤条不报错', () => {
    document.body.innerHTML = `<lk-steps></lk-steps>`;
    expect(() => renderSteps(document.body)).not.toThrow();
  });

  test('无 title 的步骤不报错', () => {
    document.body.innerHTML = `
      <lk-steps>
        <lk-step>无标题步骤。</lk-step>
      </lk-steps>
    `;
    expect(() => renderSteps(document.body)).not.toThrow();
  });
});

// ============================================================
// lk-timeline 时间线
// ============================================================
describe('时间线 (lk-timeline)', () => {
  beforeEach(() => {
    document.body.innerHTML = `
      <lk-timeline>
        <lk-timeline-item date="2014" title="Godot 1.0">首次公开发布。</lk-timeline-item>
        <lk-timeline-item date="2018" title="Godot 3.0">引入 PBR 渲染。</lk-timeline-item>
        <lk-timeline-item date="2023" title="Godot 4.0">Vulkan 渲染器。</lk-timeline-item>
      </lk-timeline>
    `;
    renderTimelines(document.body);
  });

  test('渲染时间线容器', () => {
    const timeline = document.querySelector('.timeline-block');
    expect(timeline).not.toBeNull();
  });

  test('渲染正确数量的时间节点', () => {
    const items = document.querySelectorAll('.timeline-item');
    expect(items.length).toBe(3);
  });

  test('显示日期', () => {
    const dates = document.querySelectorAll('.timeline-date');
    expect(dates[0].textContent).toContain('2014');
    expect(dates[1].textContent).toContain('2018');
    expect(dates[2].textContent).toContain('2023');
  });

  test('显示标题', () => {
    const titles = document.querySelectorAll('.timeline-title');
    expect(titles[0].textContent).toContain('Godot 1.0');
    expect(titles[1].textContent).toContain('Godot 3.0');
    expect(titles[2].textContent).toContain('Godot 4.0');
  });

  test('显示内容', () => {
    const items = document.querySelectorAll('.timeline-item');
    expect(items[0].textContent).toContain('首次公开发布');
    expect(items[2].textContent).toContain('Vulkan 渲染器');
  });

  test('空时间线不报错', () => {
    document.body.innerHTML = `<lk-timeline></lk-timeline>`;
    expect(() => renderTimelines(document.body)).not.toThrow();
  });
});

// ============================================================
// lk-formula / lk-formula-inline 公式
// ============================================================
describe('公式 (lk-formula / lk-formula-inline)', () => {
  test('渲染块级公式', () => {
    document.body.innerHTML = `
      <lk-formula caption="伤害公式">damage = ATK - DEF</lk-formula>
    `;
    renderFormulas(document.body);

    const block = document.querySelector('.formula-block');
    expect(block).not.toBeNull();
  });

  test('块级公式显示 caption', () => {
    document.body.innerHTML = `
      <lk-formula caption="伤害公式">damage = ATK - DEF</lk-formula>
    `;
    renderFormulas(document.body);

    const caption = document.querySelector('.formula-caption');
    expect(caption).not.toBeNull();
    expect(caption.textContent).toContain('伤害公式');
  });

  test('块级公式显示公式内容', () => {
    document.body.innerHTML = `
      <lk-formula caption="伤害公式">damage = ATK - DEF</lk-formula>
    `;
    renderFormulas(document.body);

    const content = document.querySelector('.formula-content, .formula-body');
    expect(content).not.toBeNull();
    expect(content.textContent).toContain('damage = ATK - DEF');
  });

  test('渲染行内公式', () => {
    document.body.innerHTML = `
      <p>经验公式为 <lk-formula-inline>XP = base * n^2</lk-formula-inline>。</p>
    `;
    renderFormulas(document.body);

    const inline = document.querySelector('.formula-inline');
    expect(inline).not.toBeNull();
    expect(inline.textContent).toContain('XP = base');
  });

  test('行内公式保持行内显示', () => {
    document.body.innerHTML = `
      <p>公式 <lk-formula-inline>E = mc^2</lk-formula-inline> 很著名。</p>
    `;
    renderFormulas(document.body);

    const inline = document.querySelector('.formula-inline');
    expect(inline).not.toBeNull();
    // 行内公式应该是 inline 或 inline-block
    const tagName = inline.tagName.toLowerCase();
    expect(['span', 'code'].includes(tagName) || inline.style.display === 'inline').toBe(true);
  });

  test('无 caption 的块级公式不报错', () => {
    document.body.innerHTML = `
      <lk-formula>x + y = z</lk-formula>
    `;
    expect(() => renderFormulas(document.body)).not.toThrow();
  });

  test('空公式不报错', () => {
    document.body.innerHTML = `<lk-formula caption="空"></lk-formula>`;
    expect(() => renderFormulas(document.body)).not.toThrow();
  });
});

// ============================================================
// lk-diagram 图表
// ============================================================
describe('图表 (lk-diagram)', () => {
  test('渲染图表块', () => {
    document.body.innerHTML = `
      <lk-diagram title="游戏循环">
┌─────────┐
│  Input   │
└────┬─────┘
     ↓
┌─────────┐
│  Update  │
└────┬─────┘
     ↓
┌─────────┐
│  Render  │
└─────────┘
      </lk-diagram>
    `;
    renderDiagrams(document.body);

    const block = document.querySelector('.diagram-block');
    expect(block).not.toBeNull();
  });

  test('显示标题', () => {
    document.body.innerHTML = `
      <lk-diagram title="游戏循环">ASCII content</lk-diagram>
    `;
    renderDiagrams(document.body);

    const title = document.querySelector('.diagram-title');
    expect(title).not.toBeNull();
    expect(title.textContent).toContain('游戏循环');
  });

  test('保留 ASCII 格式', () => {
    document.body.innerHTML = `
      <lk-diagram title="流程">
┌───┐
│ A │ → │ B │
└───┘
      </lk-diagram>
    `;
    renderDiagrams(document.body);

    const content = document.querySelector('.diagram-content, .diagram-block pre');
    expect(content).not.toBeNull();
    expect(content.textContent).toContain('┌───┐');
  });

  test('使用等宽字体渲染', () => {
    document.body.innerHTML = `
      <lk-diagram title="图">content</lk-diagram>
    `;
    renderDiagrams(document.body);

    const content = document.querySelector('.diagram-content, .diagram-block pre');
    expect(content).not.toBeNull();
    // 应使用 pre 标签或等宽字体样式
    expect(
      content.tagName.toLowerCase() === 'pre' ||
      content.closest('pre') !== null
    ).toBe(true);
  });

  test('无 title 时不报错', () => {
    document.body.innerHTML = `<lk-diagram>图</lk-diagram>`;
    expect(() => renderDiagrams(document.body)).not.toThrow();
  });

  test('空图表不报错', () => {
    document.body.innerHTML = `<lk-diagram title="空"></lk-diagram>`;
    expect(() => renderDiagrams(document.body)).not.toThrow();
  });
});

// ============================================================
// lk-figure 图片组件
// ============================================================
describe('图片组件 (lk-figure)', () => {
  test('渲染 figure + img + figcaption 结构', () => {
    document.body.innerHTML = `
      <lk-figure src="path/to/image.png" alt="描述" caption="图 1：示意图"></lk-figure>
    `;
    renderFigures(document.body);

    const figure = document.querySelector('figure');
    expect(figure).not.toBeNull();
    const img = figure.querySelector('img');
    expect(img).not.toBeNull();
    const figcaption = figure.querySelector('figcaption');
    expect(figcaption).not.toBeNull();
  });

  test('src 和 alt 属性正确传递到 img 元素', () => {
    document.body.innerHTML = `
      <lk-figure src="path/to/image.png" alt="描述" caption="图 1：示意图"></lk-figure>
    `;
    renderFigures(document.body);

    const img = document.querySelector('figure img');
    expect(img.getAttribute('src')).toBe('path/to/image.png');
    expect(img.getAttribute('alt')).toBe('描述');
  });

  test('caption 渲染为 figcaption', () => {
    document.body.innerHTML = `
      <lk-figure src="path/to/image.png" alt="描述" caption="图 1：示意图"></lk-figure>
    `;
    renderFigures(document.body);

    const figcaption = document.querySelector('figure figcaption');
    expect(figcaption).not.toBeNull();
    expect(figcaption.textContent).toContain('图 1：示意图');
  });

  test('无 caption 时不渲染 figcaption', () => {
    document.body.innerHTML = `
      <lk-figure src="path/to/image.png" alt="描述"></lk-figure>
    `;
    renderFigures(document.body);

    const figure = document.querySelector('figure');
    expect(figure).not.toBeNull();
    const figcaption = figure.querySelector('figcaption');
    expect(figcaption).toBeNull();
  });

  test('无 src 时不报错', () => {
    document.body.innerHTML = `
      <lk-figure alt="无图"></lk-figure>
    `;
    expect(() => renderFigures(document.body)).not.toThrow();
  });

  test('空标签不报错', () => {
    document.body.innerHTML = `<lk-figure></lk-figure>`;
    expect(() => renderFigures(document.body)).not.toThrow();
  });
});

// ============================================================
// lk-flowchart HTML 流程图
// ============================================================
describe('HTML 流程图 (lk-flowchart)', () => {
  test('渲染 .flowchart 容器', () => {
    document.body.innerHTML = `
      <lk-flowchart title="状态机">
        <lk-flowchart-node>巡逻</lk-flowchart-node>
        <lk-flowchart-arrow>发现玩家 →</lk-flowchart-arrow>
        <lk-flowchart-node active>追击</lk-flowchart-node>
      </lk-flowchart>
    `;
    renderFlowcharts(document.body);

    const flowchart = document.querySelector('.flowchart');
    expect(flowchart).not.toBeNull();
  });

  test('节点渲染为 .flowchart-node', () => {
    document.body.innerHTML = `
      <lk-flowchart title="状态机">
        <lk-flowchart-node>巡逻</lk-flowchart-node>
        <lk-flowchart-arrow>发现玩家 →</lk-flowchart-arrow>
        <lk-flowchart-node active>追击</lk-flowchart-node>
      </lk-flowchart>
    `;
    renderFlowcharts(document.body);

    const nodes = document.querySelectorAll('.flowchart-node');
    expect(nodes.length).toBe(2);
    expect(nodes[0].textContent).toContain('巡逻');
    expect(nodes[1].textContent).toContain('追击');
  });

  test('箭头渲染为 .flowchart-arrow', () => {
    document.body.innerHTML = `
      <lk-flowchart title="状态机">
        <lk-flowchart-node>巡逻</lk-flowchart-node>
        <lk-flowchart-arrow>发现玩家 →</lk-flowchart-arrow>
        <lk-flowchart-node active>追击</lk-flowchart-node>
      </lk-flowchart>
    `;
    renderFlowcharts(document.body);

    const arrows = document.querySelectorAll('.flowchart-arrow');
    expect(arrows.length).toBe(1);
    expect(arrows[0].textContent).toContain('发现玩家');
  });

  test('active 节点有 .flowchart-node-active 类', () => {
    document.body.innerHTML = `
      <lk-flowchart title="状态机">
        <lk-flowchart-node>巡逻</lk-flowchart-node>
        <lk-flowchart-arrow>发现玩家 →</lk-flowchart-arrow>
        <lk-flowchart-node active>追击</lk-flowchart-node>
      </lk-flowchart>
    `;
    renderFlowcharts(document.body);

    const activeNodes = document.querySelectorAll('.flowchart-node-active');
    expect(activeNodes.length).toBe(1);
    expect(activeNodes[0].textContent).toContain('追击');
  });

  test('空流程图不报错', () => {
    document.body.innerHTML = `<lk-flowchart title="空"></lk-flowchart>`;
    expect(() => renderFlowcharts(document.body)).not.toThrow();
  });
});

// ============================================================
// Unicode / Emoji 边界测试（内容组件）
// ============================================================
describe('内容组件 Unicode/Emoji 边界', () => {
  test('提示框内容含中文标点（「」、——等）正确渲染', () => {
    document.body.innerHTML = `
      <lk-callout type="tip">「提示」——这是一个包含中文标点的提示框。</lk-callout>
    `;
    renderCallouts(document.body);

    const callout = document.querySelector('.callout');
    expect(callout).not.toBeNull();
    expect(callout.textContent).toContain('「提示」——这是一个包含中文标点的提示框。');
  });

  test('超长内容文本（500 字）正确渲染', () => {
    const longText = '这是一段超长的文本内容。'.repeat(50); // ~300 字符 × 重复
    document.body.innerHTML = `
      <lk-callout type="note">${longText}</lk-callout>
    `;
    renderCallouts(document.body);

    const callout = document.querySelector('.callout');
    expect(callout).not.toBeNull();
    expect(callout.textContent).toContain('这是一段超长的文本内容。');
  });
});
