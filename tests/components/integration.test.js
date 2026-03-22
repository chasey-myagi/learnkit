import { describe, test, expect, afterEach } from 'vitest';
import { renderAllComponents } from '../../cli/templates/lesson.js';

afterEach(() => {
  document.body.innerHTML = '';
});

// ============================================================
// 多组件混合渲染
// ============================================================
describe('多组件混合渲染', () => {
  test('一个文档中包含多种组件，全部正确渲染', () => {
    document.body.innerHTML = `
      <h2>游戏设计模式</h2>

      <lk-callout type="tip">本章覆盖常用的游戏设计模式。</lk-callout>

      <lk-definition term="ECS">Entity-Component-System 架构。</lk-definition>

      <lk-code lang="gdscript" title="component.gd">
extends Node
class_name Component
      </lk-code>

      <lk-quiz type="single" question="ECS 中的 C 代表什么？">
        <lk-option>Class</lk-option>
        <lk-option correct>Component</lk-option>
        <lk-option>Controller</lk-option>
      </lk-quiz>

      <lk-expandable title="延伸阅读">
        参考 Bob Nystrom 的《Game Programming Patterns》。
      </lk-expandable>

      <lk-formula caption="伤害计算">damage = max(0, ATK - DEF)</lk-formula>
    `;
    renderAllComponents(document.body);

    // 验证每种组件都被渲染
    expect(document.querySelector('.callout')).not.toBeNull();
    expect(document.querySelector('.definition-card')).not.toBeNull();
    expect(document.querySelector('.code-block')).not.toBeNull();
    expect(document.querySelector('.quiz-block')).not.toBeNull();
    expect(document.querySelector('.expandable')).not.toBeNull();
    expect(document.querySelector('.formula-block')).not.toBeNull();
  });

  test('包含所有组件类型的完整文档', () => {
    document.body.innerHTML = `
      <lk-callout type="note">注释</lk-callout>
      <lk-definition term="A">定义A</lk-definition>
      <lk-expandable title="展开">内容</lk-expandable>
      <lk-code lang="python">pass</lk-code>
      <lk-code-tabs>
        <lk-code-tab label="Python">pass</lk-code-tab>
      </lk-code-tabs>
      <lk-diff title="变更">+new</lk-diff>
      <lk-filetree title="树">file.txt</lk-filetree>
      <lk-quiz type="single" question="Q?">
        <lk-option correct>A</lk-option>
      </lk-quiz>
      <lk-quiz type="truefalse" question="T?" answer="true"></lk-quiz>
      <lk-vs>
        <lk-vs-side title="Left">左</lk-vs-side>
        <lk-vs-side title="Right">右</lk-vs-side>
      </lk-vs>
      <lk-tabs>
        <lk-tab label="Tab1">内容1</lk-tab>
      </lk-tabs>
      <lk-steps>
        <lk-step title="步骤1">做什么</lk-step>
      </lk-steps>
      <lk-timeline>
        <lk-timeline-item date="2024" title="事件">描述</lk-timeline-item>
      </lk-timeline>
      <lk-formula caption="公式">x=1</lk-formula>
      <lk-formula-inline>y=2</lk-formula-inline>
      <lk-diagram title="图">ASCII</lk-diagram>
    `;
    renderAllComponents(document.body);

    expect(document.querySelector('.callout')).not.toBeNull();
    expect(document.querySelector('.definition-card')).not.toBeNull();
    expect(document.querySelector('.expandable')).not.toBeNull();
    expect(document.querySelector('.code-block')).not.toBeNull();
    expect(document.querySelector('.code-tabs')).not.toBeNull();
    expect(document.querySelector('.diff-block')).not.toBeNull();
    expect(document.querySelector('.filetree-block')).not.toBeNull();
    expect(document.querySelectorAll('.quiz-block').length).toBe(2);
    expect(document.querySelector('.vs-block, .comparison-block')).not.toBeNull();
    expect(document.querySelector('.tabs-block, .content-tabs')).not.toBeNull();
    expect(document.querySelector('.steps-block')).not.toBeNull();
    expect(document.querySelector('.timeline-block')).not.toBeNull();
    expect(document.querySelector('.formula-block')).not.toBeNull();
    expect(document.querySelector('.formula-inline')).not.toBeNull();
    expect(document.querySelector('.diagram-block')).not.toBeNull();
  });

  test('组件之间互不干扰', () => {
    document.body.innerHTML = `
      <lk-callout type="warning">注意事项</lk-callout>
      <lk-quiz type="single" question="问题">
        <lk-option correct>A</lk-option>
        <lk-option>B</lk-option>
      </lk-quiz>
      <lk-code lang="python">print("test")</lk-code>
    `;
    renderAllComponents(document.body);

    // callout 不应包含 quiz 内容
    const callout = document.querySelector('.callout');
    expect(callout.textContent).not.toContain('问题');

    // quiz 不应包含 code 内容
    const quiz = document.querySelector('.quiz-block');
    expect(quiz.textContent).not.toContain('print("test")');
  });
});

// ============================================================
// 组件嵌套场景
// ============================================================
describe('组件嵌套', () => {
  test('步骤条内包含代码块', () => {
    document.body.innerHTML = `
      <lk-steps>
        <lk-step title="编写代码">
          <lk-code lang="gdscript">func _ready(): pass</lk-code>
        </lk-step>
        <lk-step title="测试">运行项目。</lk-step>
      </lk-steps>
    `;
    renderAllComponents(document.body);

    const steps = document.querySelector('.steps-block');
    expect(steps).not.toBeNull();

    // 嵌套的代码块也应该被渲染
    const codeInStep = steps.querySelector('.code-block');
    expect(codeInStep).not.toBeNull();
  });

  test('可展开块内包含提示框', () => {
    document.body.innerHTML = `
      <lk-expandable title="详细说明">
        <lk-callout type="warning">注意内存管理！</lk-callout>
      </lk-expandable>
    `;
    renderAllComponents(document.body);

    const expandable = document.querySelector('.expandable');
    expect(expandable).not.toBeNull();

    const calloutInExpandable = expandable.querySelector('.callout');
    expect(calloutInExpandable).not.toBeNull();
  });

  test('Tab 内包含代码块和提示框', () => {
    document.body.innerHTML = `
      <lk-tabs>
        <lk-tab label="方案A">
          <lk-code lang="python">print("A")</lk-code>
          <lk-callout type="tip">推荐此方案。</lk-callout>
        </lk-tab>
        <lk-tab label="方案B">
          <lk-code lang="python">print("B")</lk-code>
        </lk-tab>
      </lk-tabs>
    `;
    renderAllComponents(document.body);

    const tabsBlock = document.querySelector('.tabs-block, .content-tabs');
    expect(tabsBlock).not.toBeNull();

    const codeBlocks = tabsBlock.querySelectorAll('.code-block');
    expect(codeBlocks.length).toBe(2);

    const callout = tabsBlock.querySelector('.callout');
    expect(callout).not.toBeNull();
  });

  test('对比卡片内包含代码块', () => {
    document.body.innerHTML = `
      <lk-vs>
        <lk-vs-side title="GDScript">
          <lk-code lang="gdscript">func _ready(): pass</lk-code>
        </lk-vs-side>
        <lk-vs-side title="C#">
          <lk-code lang="csharp">public override void _Ready() {}</lk-code>
        </lk-vs-side>
      </lk-vs>
    `;
    renderAllComponents(document.body);

    const vsBlock = document.querySelector('.vs-block, .comparison-block');
    expect(vsBlock).not.toBeNull();

    const codeBlocks = vsBlock.querySelectorAll('.code-block');
    expect(codeBlocks.length).toBe(2);
  });
});

// ============================================================
// 空内容处理
// ============================================================
describe('空内容处理', () => {
  test('空 body 调用 renderAllComponents 不报错', () => {
    document.body.innerHTML = '';
    expect(() => renderAllComponents(document.body)).not.toThrow();
  });

  test('只有普通 HTML 内容时不报错', () => {
    document.body.innerHTML = `
      <h1>标题</h1>
      <p>普通段落，没有任何 lk- 标签。</p>
      <ul><li>列表项</li></ul>
    `;
    expect(() => renderAllComponents(document.body)).not.toThrow();

    // 原始 HTML 内容应保持不变
    expect(document.querySelector('h1').textContent).toBe('标题');
    expect(document.querySelector('p').textContent).toContain('普通段落');
  });

  test('root 为 null 或 undefined 时优雅处理', () => {
    expect(() => renderAllComponents(null)).not.toThrow();
    expect(() => renderAllComponents(undefined)).not.toThrow();
  });
});

// ============================================================
// 未知标签处理
// ============================================================
describe('未知标签处理', () => {
  test('未知 lk- 标签不报错', () => {
    document.body.innerHTML = `
      <lk-unknown>未知组件内容</lk-unknown>
      <lk-foo bar="baz">另一个未知组件</lk-foo>
    `;
    expect(() => renderAllComponents(document.body)).not.toThrow();
  });

  test('未知标签不影响已知标签的渲染', () => {
    document.body.innerHTML = `
      <lk-unknown>未知</lk-unknown>
      <lk-callout type="tip">提示内容</lk-callout>
      <lk-nonexistent>不存在</lk-nonexistent>
    `;
    renderAllComponents(document.body);

    const callout = document.querySelector('.callout');
    expect(callout).not.toBeNull();
    expect(callout.textContent).toContain('提示内容');
  });
});

// ============================================================
// 幂等性
// ============================================================
describe('幂等性（不重复渲染）', () => {
  test('多次调用 renderAllComponents 不产生重复 DOM', () => {
    document.body.innerHTML = `
      <lk-callout type="tip">提示内容</lk-callout>
      <lk-quiz type="single" question="问题">
        <lk-option correct>A</lk-option>
        <lk-option>B</lk-option>
      </lk-quiz>
    `;
    renderAllComponents(document.body);
    const firstCalloutCount = document.querySelectorAll('.callout').length;
    const firstQuizCount = document.querySelectorAll('.quiz-block').length;

    // 第二次调用
    renderAllComponents(document.body);
    const secondCalloutCount = document.querySelectorAll('.callout').length;
    const secondQuizCount = document.querySelectorAll('.quiz-block').length;

    expect(secondCalloutCount).toBe(firstCalloutCount);
    expect(secondQuizCount).toBe(firstQuizCount);
  });

  test('多次调用不丢失交互状态', () => {
    document.body.innerHTML = `
      <lk-quiz type="single" question="Q">
        <lk-option correct>A</lk-option>
        <lk-option>B</lk-option>
      </lk-quiz>
    `;
    renderAllComponents(document.body);

    // 先点击一个选项
    const option = document.querySelector('.quiz-option');
    option.click();
    expect(option.classList.contains('answered')).toBe(true);

    // 再次调用渲染
    renderAllComponents(document.body);

    // 已回答状态应保持
    const optionAfter = document.querySelector('.quiz-option');
    expect(optionAfter.classList.contains('answered')).toBe(true);
  });

  test('多次调用不改变代码块数量', () => {
    document.body.innerHTML = `
      <lk-code lang="python">pass</lk-code>
      <lk-code lang="javascript">void 0</lk-code>
    `;
    renderAllComponents(document.body);
    renderAllComponents(document.body);
    renderAllComponents(document.body);

    const codeBlocks = document.querySelectorAll('.code-block');
    expect(codeBlocks.length).toBe(2);
  });

  test('多次调用不改变展开块数量', () => {
    document.body.innerHTML = `
      <lk-expandable title="A">内容A</lk-expandable>
      <lk-expandable title="B">内容B</lk-expandable>
    `;
    renderAllComponents(document.body);
    const firstCount = document.querySelectorAll('.expandable').length;

    renderAllComponents(document.body);
    const secondCount = document.querySelectorAll('.expandable').length;

    expect(secondCount).toBe(firstCount);
    expect(secondCount).toBe(2);
  });
});

// ============================================================
// renderAllComponents 参数
// ============================================================
describe('renderAllComponents 接口', () => {
  test('接受自定义根元素', () => {
    document.body.innerHTML = `
      <div id="main">
        <lk-callout type="tip">在 main 内</lk-callout>
      </div>
      <div id="sidebar">
        <lk-callout type="note">在 sidebar 内</lk-callout>
      </div>
    `;
    const main = document.getElementById('main');
    renderAllComponents(main);

    // 只有 main 内的 callout 被渲染
    const mainCallout = main.querySelector('.callout');
    expect(mainCallout).not.toBeNull();

    // sidebar 内的 callout 不应被渲染
    const sidebar = document.getElementById('sidebar');
    const sidebarLk = sidebar.querySelector('lk-callout');
    const sidebarCallout = sidebar.querySelector('.callout');
    // 如果 lk-callout 还在说明未渲染，或者没有 .callout
    expect(sidebarLk !== null || sidebarCallout === null).toBe(true);
  });

  test('使用 document.body 作为根元素时渲染所有组件', () => {
    document.body.innerHTML = `
      <lk-callout type="tip">提示1</lk-callout>
      <div>
        <lk-callout type="warning">提示2</lk-callout>
      </div>
    `;
    renderAllComponents(document.body);

    const callouts = document.querySelectorAll('.callout');
    expect(callouts.length).toBe(2);
  });
});
