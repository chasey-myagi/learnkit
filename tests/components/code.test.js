import { describe, test, expect, afterEach, beforeEach } from 'vitest';
import { renderAllComponents, renderCodeBlocks } from '../../cli/templates/lesson.js';

afterEach(() => {
  document.body.innerHTML = '';
});

// ============================================================
// lk-code 代码块
// ============================================================
describe('代码块 (lk-code)', () => {
  test('渲染 .code-block 容器', () => {
    document.body.innerHTML = `
      <lk-code lang="python">print("hello")</lk-code>
    `;
    renderCodeBlocks(document.body);

    const block = document.querySelector('.code-block');
    expect(block).not.toBeNull();
  });

  test('显示语言标签', () => {
    document.body.innerHTML = `
      <lk-code lang="python">print("hello")</lk-code>
    `;
    renderCodeBlocks(document.body);

    const langLabel = document.querySelector('.code-lang');
    expect(langLabel).not.toBeNull();
    expect(langLabel.textContent.trim().toLowerCase()).toContain('python');
  });

  test('代码内容正确渲染', () => {
    document.body.innerHTML = `
      <lk-code lang="javascript">const x = 42;</lk-code>
    `;
    renderCodeBlocks(document.body);

    const codeContent = document.querySelector('.code-block code, .code-block pre');
    expect(codeContent).not.toBeNull();
    expect(codeContent.textContent).toContain('const x = 42;');
  });

  test('显示可选标题', () => {
    document.body.innerHTML = `
      <lk-code lang="python" title="main.py">print("hello")</lk-code>
    `;
    renderCodeBlocks(document.body);

    const title = document.querySelector('.code-block .code-title');
    expect(title).not.toBeNull();
    expect(title.textContent).toContain('main.py');
  });

  test('无 title 时不渲染标题栏', () => {
    document.body.innerHTML = `
      <lk-code lang="python">print("hello")</lk-code>
    `;
    renderCodeBlocks(document.body);

    const title = document.querySelector('.code-block .code-title');
    // 无 title 时可以不渲染，或为空
    if (title) {
      expect(title.textContent.trim()).toBe('');
    }
  });

  test('无 lang 属性时仍正常渲染', () => {
    document.body.innerHTML = `
      <lk-code>some code</lk-code>
    `;
    expect(() => renderCodeBlocks(document.body)).not.toThrow();

    const block = document.querySelector('.code-block');
    expect(block).not.toBeNull();
  });

  test('多行代码正确保留换行', () => {
    document.body.innerHTML = `
      <lk-code lang="python">def foo():
    return 42

print(foo())</lk-code>
    `;
    renderCodeBlocks(document.body);

    const codeContent = document.querySelector('.code-block code, .code-block pre');
    expect(codeContent.textContent).toContain('def foo():');
    expect(codeContent.textContent).toContain('return 42');
    expect(codeContent.textContent).toContain('print(foo())');
  });

  test('空代码块不报错', () => {
    document.body.innerHTML = `
      <lk-code lang="python"></lk-code>
    `;
    expect(() => renderCodeBlocks(document.body)).not.toThrow();
  });
});

// ============================================================
// lk-code-tabs 多语言切换
// ============================================================
describe('代码 Tab 切换 (lk-code-tabs)', () => {
  beforeEach(() => {
    document.body.innerHTML = `
      <lk-code-tabs>
        <lk-code-tab label="GDScript">
          func _ready():
              pass
        </lk-code-tab>
        <lk-code-tab label="C#">
          public override void _Ready() { }
        </lk-code-tab>
        <lk-code-tab label="Rust">
          fn ready(&amp;mut self) {}
        </lk-code-tab>
      </lk-code-tabs>
    `;
    renderCodeBlocks(document.body);
  });

  test('渲染 Tab 头部', () => {
    const tabs = document.querySelectorAll('.code-tabs .code-tab-btn');
    expect(tabs.length).toBe(3);
  });

  test('Tab 标签文字正确', () => {
    const tabs = document.querySelectorAll('.code-tab-btn');
    const labels = Array.from(tabs).map(t => t.textContent.trim());
    expect(labels).toContain('GDScript');
    expect(labels).toContain('C#');
    expect(labels).toContain('Rust');
  });

  test('默认激活第一个 Tab', () => {
    const tabs = document.querySelectorAll('.code-tab-btn');
    expect(tabs[0].classList.contains('active')).toBe(true);

    const panels = document.querySelectorAll('.code-tab-panel');
    expect(panels[0].style.display !== 'none' && !panels[0].hidden).toBe(true);
  });

  test('点击 Tab 切换面板', () => {
    const tabs = document.querySelectorAll('.code-tab-btn');
    const panels = document.querySelectorAll('.code-tab-panel');

    // 点击第二个 Tab
    tabs[1].click();

    expect(tabs[1].classList.contains('active')).toBe(true);
    expect(tabs[0].classList.contains('active')).toBe(false);

    // 第二个面板可见，第一个隐藏
    expect(panels[1].style.display !== 'none' && !panels[1].hidden).toBe(true);
    expect(panels[0].style.display === 'none' || panels[0].hidden).toBe(true);
  });

  test('面板内容正确渲染', () => {
    const panels = document.querySelectorAll('.code-tab-panel');
    expect(panels[0].textContent).toContain('func _ready()');
    expect(panels[1].textContent).toContain('_Ready()');
    expect(panels[2].textContent).toContain('fn ready');
  });

  test('只有一个 Tab 时正常渲染', () => {
    document.body.innerHTML = `
      <lk-code-tabs>
        <lk-code-tab label="Only">code here</lk-code-tab>
      </lk-code-tabs>
    `;
    renderCodeBlocks(document.body);

    const tabs = document.querySelectorAll('.code-tab-btn');
    expect(tabs.length).toBe(1);
  });

  test('空 Tab 列表不报错', () => {
    document.body.innerHTML = `
      <lk-code-tabs></lk-code-tabs>
    `;
    expect(() => renderCodeBlocks(document.body)).not.toThrow();
  });
});

// ============================================================
// lk-diff 代码 diff
// ============================================================
describe('代码 Diff (lk-diff)', () => {
  beforeEach(() => {
    document.body.innerHTML = `
      <lk-diff title="修改移动逻辑">-var speed = 100
+var speed = 200
 var gravity = 980</lk-diff>
    `;
    renderCodeBlocks(document.body);
  });

  test('渲染 diff 块', () => {
    const diffBlock = document.querySelector('.diff-block');
    expect(diffBlock).not.toBeNull();
  });

  test('显示标题', () => {
    const title = document.querySelector('.diff-block .diff-title');
    expect(title).not.toBeNull();
    expect(title.textContent).toContain('修改移动逻辑');
  });

  test('删除行有红色样式', () => {
    const removedLines = document.querySelectorAll('.diff-line.removed, .diff-removed');
    expect(removedLines.length).toBeGreaterThan(0);
    const removedText = Array.from(removedLines).map(l => l.textContent).join('');
    expect(removedText).toContain('var speed = 100');
  });

  test('新增行有绿色样式', () => {
    const addedLines = document.querySelectorAll('.diff-line.added, .diff-added');
    expect(addedLines.length).toBeGreaterThan(0);
    const addedText = Array.from(addedLines).map(l => l.textContent).join('');
    expect(addedText).toContain('var speed = 200');
  });

  test('未改变行正常显示', () => {
    const unchangedLines = document.querySelectorAll('.diff-line.unchanged, .diff-unchanged, .diff-line.context');
    expect(unchangedLines.length).toBeGreaterThan(0);
    const contextText = Array.from(unchangedLines).map(l => l.textContent).join('');
    expect(contextText).toContain('var gravity = 980');
  });

  test('无 title 时不渲染标题', () => {
    document.body.innerHTML = `
      <lk-diff>+added line</lk-diff>
    `;
    renderCodeBlocks(document.body);

    const title = document.querySelector('.diff-block .diff-title');
    if (title) {
      expect(title.textContent.trim()).toBe('');
    }
  });

  test('空 diff 不报错', () => {
    document.body.innerHTML = `<lk-diff title="空"></lk-diff>`;
    expect(() => renderCodeBlocks(document.body)).not.toThrow();
  });
});

// ============================================================
// lk-filetree 文件树
// ============================================================
describe('文件树 (lk-filetree)', () => {
  beforeEach(() => {
    document.body.innerHTML = `
      <lk-filetree title="项目结构">
project/
├── src/
│   ├── main.gd
│   └── player.gd
├── scenes/
│   └── main.tscn
└── project.godot
      </lk-filetree>
    `;
    renderCodeBlocks(document.body);
  });

  test('渲染文件树块', () => {
    const tree = document.querySelector('.filetree-block');
    expect(tree).not.toBeNull();
  });

  test('显示标题', () => {
    const title = document.querySelector('.filetree-block .filetree-title');
    expect(title).not.toBeNull();
    expect(title.textContent).toContain('项目结构');
  });

  test('保留树形结构内容', () => {
    const content = document.querySelector('.filetree-block pre, .filetree-block .filetree-content');
    expect(content).not.toBeNull();
    expect(content.textContent).toContain('main.gd');
    expect(content.textContent).toContain('player.gd');
    expect(content.textContent).toContain('project.godot');
  });

  test('保留树形连接线字符', () => {
    const content = document.querySelector('.filetree-block pre, .filetree-block .filetree-content');
    expect(content.textContent).toContain('├──');
    expect(content.textContent).toContain('└──');
  });

  test('无 title 时不渲染标题', () => {
    document.body.innerHTML = `
      <lk-filetree>
file1.txt
file2.txt
      </lk-filetree>
    `;
    renderCodeBlocks(document.body);

    const title = document.querySelector('.filetree-block .filetree-title');
    if (title) {
      expect(title.textContent.trim()).toBe('');
    }
  });

  test('空文件树不报错', () => {
    document.body.innerHTML = `<lk-filetree title="空"></lk-filetree>`;
    expect(() => renderCodeBlocks(document.body)).not.toThrow();
  });
});
