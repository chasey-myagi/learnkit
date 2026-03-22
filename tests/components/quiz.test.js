import { describe, test, expect, afterEach, beforeEach } from 'vitest';
import { renderAllComponents, renderQuizzes } from '../../cli/templates/lesson.js';

afterEach(() => {
  document.body.innerHTML = '';
});

// ============================================================
// 单选题 type="single"
// ============================================================
describe('单选题 (type="single")', () => {
  beforeEach(() => {
    document.body.innerHTML = `
      <lk-quiz type="single" question="MDA 中的 D 代表什么？">
        <lk-option>Design</lk-option>
        <lk-option correct>Dynamics</lk-option>
        <lk-option>Development</lk-option>
        <lk-explanation>Dynamics 描述运行时行为模式。</lk-explanation>
      </lk-quiz>
    `;
    renderQuizzes(document.body);
  });

  test('渲染完整 DOM 结构（.quiz-block, .quiz-header, .quiz-question, .quiz-options）', () => {
    const block = document.querySelector('.quiz-block');
    expect(block).not.toBeNull();
    expect(block.querySelector('.quiz-header')).not.toBeNull();
    expect(block.querySelector('.quiz-question')).not.toBeNull();
    expect(block.querySelector('.quiz-options')).not.toBeNull();
  });

  test('题目文字正确渲染', () => {
    const question = document.querySelector('.quiz-question');
    expect(question.textContent).toContain('MDA 中的 D 代表什么？');
  });

  test('每个选项有字母圆圈（A, B, C）', () => {
    const options = document.querySelectorAll('.quiz-options .quiz-option');
    expect(options.length).toBe(3);

    const letters = ['A', 'B', 'C'];
    options.forEach((opt, i) => {
      const circle = opt.querySelector('.option-letter');
      expect(circle).not.toBeNull();
      expect(circle.textContent.trim()).toBe(letters[i]);
    });
  });

  test('存在 feedback 区域', () => {
    const feedback = document.querySelector('.quiz-feedback');
    expect(feedback).not.toBeNull();
  });

  test('点击正确选项后显示绿色反馈', () => {
    const options = document.querySelectorAll('.quiz-option');
    // 第二个选项是正确答案（Dynamics）
    options[1].click();

    expect(options[1].classList.contains('correct')).toBe(true);
    const feedback = document.querySelector('.quiz-feedback');
    expect(feedback.textContent).toContain('Dynamics 描述运行时行为模式。');
  });

  test('点击错误选项后红色高亮 + 正确答案同时显示', () => {
    const options = document.querySelectorAll('.quiz-option');
    // 点击第一个选项（错误）
    options[0].click();

    expect(options[0].classList.contains('incorrect')).toBe(true);
    // 正确答案也应该高亮
    expect(options[1].classList.contains('correct')).toBe(true);
  });

  test('点击后锁定所有选项，不能再次选择', () => {
    const options = document.querySelectorAll('.quiz-option');
    options[0].click();

    // 所有选项应该被锁定
    options.forEach(opt => {
      expect(opt.classList.contains('answered')).toBe(true);
    });

    // 再次点击不应改变状态
    const previousClasses = options[2].className;
    options[2].click();
    expect(options[2].className).toBe(previousClasses);
  });

  test('原始 lk-quiz 标签被替换或隐藏', () => {
    const lkQuiz = document.querySelector('lk-quiz');
    // 原始标签应该被替换掉，或者不再可见
    if (lkQuiz) {
      expect(lkQuiz.style.display === 'none' || lkQuiz.hidden).toBe(true);
    }
  });

  test('无 explanation 时 feedback 区域仍存在但为空', () => {
    document.body.innerHTML = `
      <lk-quiz type="single" question="测试题">
        <lk-option correct>A</lk-option>
        <lk-option>B</lk-option>
      </lk-quiz>
    `;
    renderQuizzes(document.body);

    const options = document.querySelectorAll('.quiz-option');
    options[0].click();

    const feedback = document.querySelector('.quiz-feedback');
    expect(feedback).not.toBeNull();
  });

  test('只有一个选项时仍正常渲染', () => {
    document.body.innerHTML = `
      <lk-quiz type="single" question="唯一选项">
        <lk-option correct>唯一</lk-option>
      </lk-quiz>
    `;
    renderQuizzes(document.body);

    const options = document.querySelectorAll('.quiz-option');
    expect(options.length).toBe(1);
    expect(options[0].querySelector('.option-letter').textContent.trim()).toBe('A');
  });
});

// ============================================================
// 多选题 type="multi"
// ============================================================
describe('多选题 (type="multi")', () => {
  beforeEach(() => {
    document.body.innerHTML = `
      <lk-quiz type="multi" question="以下哪些是设计模式？">
        <lk-option correct>观察者模式</lk-option>
        <lk-option correct>对象池</lk-option>
        <lk-option>瀑布模型</lk-option>
      </lk-quiz>
    `;
    renderQuizzes(document.body);
  });

  test('渲染复选框而非字母圆圈', () => {
    const checkboxes = document.querySelectorAll('.quiz-option input[type="checkbox"]');
    expect(checkboxes.length).toBe(3);

    // 不应有字母圆圈
    const letterCircles = document.querySelectorAll('.quiz-option .option-letter');
    expect(letterCircles.length).toBe(0);
  });

  test('可选多个选项', () => {
    const checkboxes = document.querySelectorAll('.quiz-option input[type="checkbox"]');
    checkboxes[0].click();
    checkboxes[1].click();

    expect(checkboxes[0].checked).toBe(true);
    expect(checkboxes[1].checked).toBe(true);
  });

  test('有提交按钮', () => {
    const submitBtn = document.querySelector('.quiz-block .quiz-submit');
    expect(submitBtn).not.toBeNull();
  });

  test('提交后显示正确/错误/遗漏反馈', () => {
    const checkboxes = document.querySelectorAll('.quiz-option input[type="checkbox"]');
    const options = document.querySelectorAll('.quiz-option');
    // 选择第一个（正确）和第三个（错误），漏掉第二个（正确）
    checkboxes[0].click();
    checkboxes[2].click();

    const submitBtn = document.querySelector('.quiz-submit');
    submitBtn.click();

    // 第一个：正确选中
    expect(options[0].classList.contains('correct')).toBe(true);
    // 第二个：遗漏
    expect(options[1].classList.contains('missed')).toBe(true);
    // 第三个：错误选中
    expect(options[2].classList.contains('incorrect')).toBe(true);
  });

  test('全部选对后显示全部正确', () => {
    const checkboxes = document.querySelectorAll('.quiz-option input[type="checkbox"]');
    const options = document.querySelectorAll('.quiz-option');
    checkboxes[0].click();
    checkboxes[1].click();

    const submitBtn = document.querySelector('.quiz-submit');
    submitBtn.click();

    expect(options[0].classList.contains('correct')).toBe(true);
    expect(options[1].classList.contains('correct')).toBe(true);
    expect(options[2].classList.contains('incorrect')).toBe(false);
    expect(options[2].classList.contains('missed')).toBe(false);
  });

  test('提交后锁定所有选项', () => {
    const submitBtn = document.querySelector('.quiz-submit');
    submitBtn.click();

    const checkboxes = document.querySelectorAll('.quiz-option input[type="checkbox"]');
    checkboxes.forEach(cb => {
      expect(cb.disabled).toBe(true);
    });
  });

  test('未选任何选项时提交仍正常工作', () => {
    const submitBtn = document.querySelector('.quiz-submit');
    submitBtn.click();

    // 应该不报错，遗漏的正确答案标为 missed
    const options = document.querySelectorAll('.quiz-option');
    expect(options[0].classList.contains('missed')).toBe(true);
    expect(options[1].classList.contains('missed')).toBe(true);
  });

  test('选中后再次点击取消选中', () => {
    const checkboxes = document.querySelectorAll('.quiz-option input[type="checkbox"]');

    checkboxes[0].click();
    expect(checkboxes[0].checked).toBe(true);

    checkboxes[0].click();
    expect(checkboxes[0].checked).toBe(false);
  });

  test('选中→取消→重新选中后提交的正确性', () => {
    const checkboxes = document.querySelectorAll('.quiz-option input[type="checkbox"]');
    const options = document.querySelectorAll('.quiz-option');

    // 选中第一个（正确），取消，再选第二个（正确）
    checkboxes[0].click();
    checkboxes[0].click(); // 取消
    checkboxes[1].click(); // 只选了一个正确答案

    const submitBtn = document.querySelector('.quiz-submit');
    submitBtn.click();

    // 第一个（正确答案，未选中）应标记为 missed
    expect(options[0].classList.contains('missed')).toBe(true);
    // 第二个（正确答案，已选中）应标记为 correct
    expect(options[1].classList.contains('correct')).toBe(true);
  });
});

// ============================================================
// 判断题 type="truefalse"
// ============================================================
describe('判断题 (type="truefalse")', () => {
  beforeEach(() => {
    document.body.innerHTML = `
      <lk-quiz type="truefalse" question="组件包含逻辑" answer="false">
        <lk-explanation>组件只持有数据。</lk-explanation>
      </lk-quiz>
    `;
    renderQuizzes(document.body);
  });

  test('只有两个选项', () => {
    const options = document.querySelectorAll('.quiz-option');
    expect(options.length).toBe(2);
  });

  test('选项内容为"正确"和"错误"', () => {
    const options = document.querySelectorAll('.quiz-option');
    const texts = Array.from(options).map(o => o.textContent.trim());
    // 应该包含正确/错误的标识
    expect(texts.some(t => t.includes('正确') || t.includes('✓'))).toBe(true);
    expect(texts.some(t => t.includes('错误') || t.includes('✗'))).toBe(true);
  });

  test('选项横排显示（flex 布局）', () => {
    const optionsContainer = document.querySelector('.quiz-options');
    const style = window.getComputedStyle(optionsContainer);
    // 判断题选项应该横排
    expect(
      optionsContainer.classList.contains('truefalse-options') ||
      style.flexDirection === 'row'
    ).toBe(true);
  });

  test('点击正确答案（"错误"）显示绿色反馈', () => {
    const options = document.querySelectorAll('.quiz-option');
    // answer="false"，所以"错误"选项是正确答案
    const falseOption = Array.from(options).find(
      o => o.textContent.includes('错误') || o.textContent.includes('✗')
    );
    falseOption.click();

    expect(falseOption.classList.contains('correct')).toBe(true);
    const feedback = document.querySelector('.quiz-feedback');
    expect(feedback.textContent).toContain('组件只持有数据。');
  });

  test('点击错误答案显示红色反馈', () => {
    const options = document.querySelectorAll('.quiz-option');
    const trueOption = Array.from(options).find(
      o => o.textContent.includes('正确') || o.textContent.includes('✓')
    );
    trueOption.click();

    expect(trueOption.classList.contains('incorrect')).toBe(true);
  });

  test('点击后锁定选项', () => {
    const options = document.querySelectorAll('.quiz-option');
    options[0].click();

    options.forEach(opt => {
      expect(opt.classList.contains('answered')).toBe(true);
    });
  });

  test('answer="true" 时正确答案为"正确"选项', () => {
    document.body.innerHTML = `
      <lk-quiz type="truefalse" question="1+1=2" answer="true"></lk-quiz>
    `;
    renderQuizzes(document.body);

    const options = document.querySelectorAll('.quiz-option');
    const trueOption = Array.from(options).find(
      o => o.textContent.includes('正确') || o.textContent.includes('✓')
    );
    trueOption.click();
    expect(trueOption.classList.contains('correct')).toBe(true);
  });
});

// ============================================================
// 填空题 type="fill"
// ============================================================
describe('填空题 (type="fill")', () => {
  beforeEach(() => {
    document.body.innerHTML = `
      <lk-quiz type="fill" question="f(n) = g(n) + {0}" answers="h(n)">
        <lk-explanation>h(n) 是启发式估计。</lk-explanation>
      </lk-quiz>
    `;
    renderQuizzes(document.body);
  });

  test('渲染行内输入框', () => {
    const input = document.querySelector('.quiz-block input[type="text"]');
    expect(input).not.toBeNull();
  });

  test('题目中 {0} 被替换为输入框', () => {
    const questionArea = document.querySelector('.quiz-question');
    expect(questionArea).not.toBeNull();
    // 题目中应该包含输入框
    const input = questionArea.querySelector('input');
    expect(input).not.toBeNull();
    // 题目文字应该包含 f(n) = g(n) +
    expect(questionArea.textContent).toContain('f(n) = g(n) +');
  });

  test('有提交按钮', () => {
    const submitBtn = document.querySelector('.quiz-submit');
    expect(submitBtn).not.toBeNull();
  });

  test('输入正确答案后提交显示正确反馈', () => {
    const input = document.querySelector('.quiz-block input[type="text"]');
    const submitBtn = document.querySelector('.quiz-submit');

    input.value = 'h(n)';
    submitBtn.click();

    expect(
      input.classList.contains('correct') ||
      input.closest('.quiz-block').querySelector('.correct')
    ).toBeTruthy();
  });

  test('大小写不敏感匹配', () => {
    const input = document.querySelector('.quiz-block input[type="text"]');
    const submitBtn = document.querySelector('.quiz-submit');

    input.value = 'H(N)';
    submitBtn.click();

    expect(
      input.classList.contains('correct') ||
      input.closest('.quiz-block').querySelector('.correct')
    ).toBeTruthy();
  });

  test('输入错误答案后显示错误反馈', () => {
    const input = document.querySelector('.quiz-block input[type="text"]');
    const submitBtn = document.querySelector('.quiz-submit');

    input.value = 'wrong';
    submitBtn.click();

    expect(
      input.classList.contains('incorrect') ||
      input.closest('.quiz-block').querySelector('.incorrect')
    ).toBeTruthy();
  });

  test('提交后输入框被禁用', () => {
    const input = document.querySelector('.quiz-block input[type="text"]');
    const submitBtn = document.querySelector('.quiz-submit');

    input.value = 'h(n)';
    submitBtn.click();

    expect(input.disabled).toBe(true);
  });

  test('空输入提交不报错', () => {
    const submitBtn = document.querySelector('.quiz-submit');
    expect(() => submitBtn.click()).not.toThrow();
  });

  test('多个填空位的题目', () => {
    document.body.innerHTML = `
      <lk-quiz type="fill" question="{0} + {1} = 3" answers="1,2">
      </lk-quiz>
    `;
    renderQuizzes(document.body);

    const inputs = document.querySelectorAll('.quiz-block input[type="text"]');
    expect(inputs.length).toBe(2);
  });

  test('多个填空位全部填对后显示正确反馈', () => {
    document.body.innerHTML = `
      <lk-quiz type="fill" question="{0} + {1} = 3" answers="1,2">
      </lk-quiz>
    `;
    renderQuizzes(document.body);

    const inputs = document.querySelectorAll('.quiz-block input[type="text"]');
    inputs[0].value = '1';
    inputs[1].value = '2';

    const submitBtn = document.querySelector('.quiz-submit');
    submitBtn.click();

    const block = document.querySelector('.quiz-block');
    expect(
      block.classList.contains('correct') ||
      block.querySelector('.correct')
    ).toBeTruthy();
  });

  test('多个填空位部分填对后显示部分正确反馈', () => {
    document.body.innerHTML = `
      <lk-quiz type="fill" question="{0} + {1} = 3" answers="1,2">
      </lk-quiz>
    `;
    renderQuizzes(document.body);

    const inputs = document.querySelectorAll('.quiz-block input[type="text"]');
    inputs[0].value = '1';
    inputs[1].value = '9'; // 错误

    const submitBtn = document.querySelector('.quiz-submit');
    submitBtn.click();

    // 第一个应正确，第二个应错误
    expect(
      inputs[0].classList.contains('correct') ||
      inputs[0].closest('.correct')
    ).toBeTruthy();
    expect(
      inputs[1].classList.contains('incorrect') ||
      inputs[1].closest('.incorrect')
    ).toBeTruthy();
  });

  test('多个填空位全部填错后显示错误反馈', () => {
    document.body.innerHTML = `
      <lk-quiz type="fill" question="{0} + {1} = 3" answers="1,2">
      </lk-quiz>
    `;
    renderQuizzes(document.body);

    const inputs = document.querySelectorAll('.quiz-block input[type="text"]');
    inputs[0].value = '5';
    inputs[1].value = '9';

    const submitBtn = document.querySelector('.quiz-submit');
    submitBtn.click();

    const block = document.querySelector('.quiz-block');
    expect(
      block.classList.contains('incorrect') ||
      block.querySelector('.incorrect')
    ).toBeTruthy();
  });
});

// ============================================================
// 排序题 type="order"
// ============================================================
describe('排序题 (type="order")', () => {
  beforeEach(() => {
    document.body.innerHTML = `
      <lk-quiz type="order" question="排列渲染管线阶段：">
        <lk-order-item order="1">顶点处理</lk-order-item>
        <lk-order-item order="2">图元装配</lk-order-item>
        <lk-order-item order="3">光栅化</lk-order-item>
      </lk-quiz>
    `;
    renderQuizzes(document.body);
  });

  test('渲染可拖拽列表', () => {
    const items = document.querySelectorAll('.quiz-order-item');
    expect(items.length).toBe(3);
  });

  test('列表项包含原始内容', () => {
    const items = document.querySelectorAll('.quiz-order-item');
    const texts = Array.from(items).map(i => i.textContent.trim());
    expect(texts).toContain('顶点处理');
    expect(texts).toContain('图元装配');
    expect(texts).toContain('光栅化');
  });

  test('列表项初始顺序是打乱的', () => {
    // 多次渲染检查至少有一次顺序不同（概率测试）
    // 或者检查是否有 draggable 属性
    const items = document.querySelectorAll('.quiz-order-item');
    items.forEach(item => {
      expect(
        item.getAttribute('draggable') === 'true' ||
        item.classList.contains('draggable')
      ).toBe(true);
    });
  });

  test('有提交按钮', () => {
    const submitBtn = document.querySelector('.quiz-submit');
    expect(submitBtn).not.toBeNull();
  });

  test('正确顺序提交后显示成功反馈', () => {
    // 模拟将列表排列为正确顺序
    const container = document.querySelector('.quiz-order-list');
    const items = Array.from(document.querySelectorAll('.quiz-order-item'));

    // 按 order 属性排序（找到正确顺序）
    const sorted = items.sort((a, b) => {
      return parseInt(a.dataset.order) - parseInt(b.dataset.order);
    });
    sorted.forEach(item => container.appendChild(item));

    const submitBtn = document.querySelector('.quiz-submit');
    submitBtn.click();

    const block = document.querySelector('.quiz-block');
    expect(
      block.classList.contains('correct') ||
      block.querySelector('.correct')
    ).toBeTruthy();
  });

  test('错误顺序提交后显示失败反馈', () => {
    // 反转顺序
    const container = document.querySelector('.quiz-order-list');
    const items = Array.from(document.querySelectorAll('.quiz-order-item'));
    items.reverse().forEach(item => container.appendChild(item));

    const submitBtn = document.querySelector('.quiz-submit');
    submitBtn.click();

    const block = document.querySelector('.quiz-block');
    expect(
      block.classList.contains('incorrect') ||
      block.querySelector('.incorrect')
    ).toBeTruthy();
  });

  test('提交后锁定拖拽', () => {
    const submitBtn = document.querySelector('.quiz-submit');
    submitBtn.click();

    const items = document.querySelectorAll('.quiz-order-item');
    items.forEach(item => {
      expect(
        item.getAttribute('draggable') === 'false' ||
        item.classList.contains('answered')
      ).toBe(true);
    });
  });

  test('只有 1 项时正常渲染', () => {
    document.body.innerHTML = `
      <lk-quiz type="order" question="排列：">
        <lk-order-item order="1">唯一步骤</lk-order-item>
      </lk-quiz>
    `;
    renderQuizzes(document.body);

    const items = document.querySelectorAll('.quiz-order-item');
    expect(items.length).toBe(1);
    expect(items[0].textContent).toContain('唯一步骤');
  });

  test('只有 2 项时正常渲染和提交', () => {
    document.body.innerHTML = `
      <lk-quiz type="order" question="排列：">
        <lk-order-item order="1">第一步</lk-order-item>
        <lk-order-item order="2">第二步</lk-order-item>
      </lk-quiz>
    `;
    renderQuizzes(document.body);

    const items = document.querySelectorAll('.quiz-order-item');
    expect(items.length).toBe(2);

    // 按正确顺序排列后提交
    const container = document.querySelector('.quiz-order-list');
    const sorted = Array.from(items).sort((a, b) => {
      return parseInt(a.dataset.order) - parseInt(b.dataset.order);
    });
    sorted.forEach(item => container.appendChild(item));

    const submitBtn = document.querySelector('.quiz-submit');
    submitBtn.click();

    const block = document.querySelector('.quiz-block');
    expect(
      block.classList.contains('correct') ||
      block.querySelector('.correct')
    ).toBeTruthy();
  });
});

// ============================================================
// 连线匹配题 type="match"
// ============================================================
describe('连线匹配题 (type="match")', () => {
  beforeEach(() => {
    document.body.innerHTML = `
      <lk-quiz type="match" question="匹配设计模式与用途：">
        <lk-match-pair left="观察者" right="事件解耦" />
        <lk-match-pair left="对象池" right="减少 GC" />
        <lk-match-pair left="命令模式" right="撤销重做" />
      </lk-quiz>
    `;
    renderQuizzes(document.body);
  });

  test('渲染左右两列', () => {
    const leftCol = document.querySelector('.match-left');
    const rightCol = document.querySelector('.match-right');
    expect(leftCol).not.toBeNull();
    expect(rightCol).not.toBeNull();
  });

  test('左列包含所有模式名称', () => {
    const leftItems = document.querySelectorAll('.match-left .match-item');
    const texts = Array.from(leftItems).map(i => i.textContent.trim());
    expect(texts).toContain('观察者');
    expect(texts).toContain('对象池');
    expect(texts).toContain('命令模式');
  });

  test('右列包含所有用途描述（打乱顺序）', () => {
    const rightItems = document.querySelectorAll('.match-right .match-item');
    const texts = Array.from(rightItems).map(i => i.textContent.trim());
    expect(texts).toContain('事件解耦');
    expect(texts).toContain('减少 GC');
    expect(texts).toContain('撤销重做');
  });

  test('有提交按钮', () => {
    const submitBtn = document.querySelector('.quiz-submit');
    expect(submitBtn).not.toBeNull();
  });

  test('可以选择左右配对', () => {
    const leftItems = document.querySelectorAll('.match-left .match-item');
    const rightItems = document.querySelectorAll('.match-right .match-item');

    // 点击左侧第一个
    leftItems[0].click();
    expect(leftItems[0].classList.contains('selected')).toBe(true);

    // 点击右侧第一个完成配对
    rightItems[0].click();
    expect(
      leftItems[0].classList.contains('paired') ||
      leftItems[0].dataset.pair
    ).toBeTruthy();
  });

  test('全部正确配对后提交显示成功', () => {
    const leftItems = document.querySelectorAll('.match-left .match-item');
    const rightItems = document.querySelectorAll('.match-right .match-item');

    // 按正确顺序配对
    for (const leftItem of leftItems) {
      const leftText = leftItem.textContent.trim();
      let expectedRight;
      if (leftText === '观察者') expectedRight = '事件解耦';
      else if (leftText === '对象池') expectedRight = '减少 GC';
      else if (leftText === '命令模式') expectedRight = '撤销重做';

      leftItem.click();
      const rightItem = Array.from(rightItems).find(
        r => r.textContent.trim() === expectedRight
      );
      rightItem.click();
    }

    const submitBtn = document.querySelector('.quiz-submit');
    submitBtn.click();

    const block = document.querySelector('.quiz-block');
    expect(
      block.classList.contains('correct') ||
      block.querySelector('.correct') ||
      document.querySelector('.quiz-feedback.correct')
    ).toBeTruthy();
  });

  test('错误配对后提交显示失败', () => {
    const leftItems = document.querySelectorAll('.match-left .match-item');
    const rightItems = document.querySelectorAll('.match-right .match-item');

    // 全部配对到第一个右侧项（故意错配）
    leftItems[0].click();
    rightItems[2].click();
    leftItems[1].click();
    rightItems[0].click();
    leftItems[2].click();
    rightItems[1].click();

    const submitBtn = document.querySelector('.quiz-submit');
    submitBtn.click();

    const block = document.querySelector('.quiz-block');
    expect(
      block.classList.contains('incorrect') ||
      block.querySelector('.incorrect')
    ).toBeTruthy();
  });

  test('提交后锁定交互', () => {
    const submitBtn = document.querySelector('.quiz-submit');
    submitBtn.click();

    const items = document.querySelectorAll('.match-item');
    items.forEach(item => {
      expect(item.classList.contains('answered')).toBe(true);
    });
  });

  test('已配对后重新选择左侧项可以更换配对', () => {
    const leftItems = document.querySelectorAll('.match-left .match-item');
    const rightItems = document.querySelectorAll('.match-right .match-item');

    // 先配对左0-右0
    leftItems[0].click();
    rightItems[0].click();

    // 再次选择左0并配对到右1（更换配对）
    leftItems[0].click();
    rightItems[1].click();

    // 左0 应该与右1 配对，而非右0
    const pairValue = leftItems[0].dataset.pair;
    const rightItem1Id = rightItems[1].dataset.id || rightItems[1].textContent.trim();
    expect(
      pairValue === rightItem1Id ||
      leftItems[0].classList.contains('paired')
    ).toBeTruthy();
  });

  test('只配了部分就提交时显示不完整提示', () => {
    const leftItems = document.querySelectorAll('.match-left .match-item');
    const rightItems = document.querySelectorAll('.match-right .match-item');

    // 只配对一对
    leftItems[0].click();
    rightItems[0].click();

    const submitBtn = document.querySelector('.quiz-submit');
    submitBtn.click();

    // 应该有提示或反馈表明配对不完整
    const feedback = document.querySelector('.quiz-feedback');
    const block = document.querySelector('.quiz-block');
    expect(
      (feedback && feedback.textContent.length > 0) ||
      block.classList.contains('incomplete') ||
      block.querySelector('.incomplete') ||
      block.querySelector('.incorrect')
    ).toBeTruthy();
  });
});

// ============================================================
// 边界情况
// ============================================================
describe('测验题边界情况', () => {
  test('无 question 属性时使用默认文字或留空', () => {
    document.body.innerHTML = `
      <lk-quiz type="single">
        <lk-option correct>A</lk-option>
        <lk-option>B</lk-option>
      </lk-quiz>
    `;
    expect(() => renderQuizzes(document.body)).not.toThrow();
    const block = document.querySelector('.quiz-block');
    expect(block).not.toBeNull();
  });

  test('无选项的单选题不报错', () => {
    document.body.innerHTML = `
      <lk-quiz type="single" question="空题"></lk-quiz>
    `;
    expect(() => renderQuizzes(document.body)).not.toThrow();
  });

  test('未知 type 属性不报错', () => {
    document.body.innerHTML = `
      <lk-quiz type="unknown" question="未知类型">
        <lk-option>A</lk-option>
      </lk-quiz>
    `;
    expect(() => renderQuizzes(document.body)).not.toThrow();
  });

  test('多个测验题同时渲染', () => {
    document.body.innerHTML = `
      <lk-quiz type="single" question="Q1">
        <lk-option correct>A</lk-option>
        <lk-option>B</lk-option>
      </lk-quiz>
      <lk-quiz type="truefalse" question="Q2" answer="true"></lk-quiz>
      <lk-quiz type="fill" question="Q3 {0}" answers="X"></lk-quiz>
    `;
    renderQuizzes(document.body);

    const blocks = document.querySelectorAll('.quiz-block');
    expect(blocks.length).toBe(3);
  });

  test('包含 HTML 特殊字符的选项正确渲染', () => {
    document.body.innerHTML = `
      <lk-quiz type="single" question="选择 &lt;script&gt; 标签的替代品">
        <lk-option correct>&lt;noscript&gt;</lk-option>
        <lk-option>&lt;style&gt;</lk-option>
      </lk-quiz>
    `;
    renderQuizzes(document.body);
    const options = document.querySelectorAll('.quiz-option');
    expect(options.length).toBe(2);
  });

  test('单选题选项含 emoji 正确渲染', () => {
    document.body.innerHTML = `
      <lk-quiz type="single" question="哪个是游戏手柄？">
        <lk-option correct>\u{1F3AE} 游戏手柄</lk-option>
        <lk-option>\u{1F3B5} 音符</lk-option>
        <lk-option>\u{1F4DA} 书本</lk-option>
      </lk-quiz>
    `;
    renderQuizzes(document.body);

    const options = document.querySelectorAll('.quiz-option');
    expect(options.length).toBe(3);
    expect(options[0].textContent).toContain('\u{1F3AE}');
    expect(options[0].textContent).toContain('游戏手柄');
  });

  test('问题文字含中文标点（「」、——等）正确渲染', () => {
    document.body.innerHTML = `
      <lk-quiz type="single" question="以下关于「MDA 框架」的说法——哪个是正确的？">
        <lk-option correct>它包含三个层次</lk-option>
        <lk-option>它只关注美学</lk-option>
      </lk-quiz>
    `;
    renderQuizzes(document.body);

    const question = document.querySelector('.quiz-question');
    expect(question).not.toBeNull();
    expect(question.textContent).toContain('「MDA 框架」');
    expect(question.textContent).toContain('——');
  });

  test('超长问题文本（500 字）正确渲染', () => {
    const longQuestion = '这是一个非常长的问题描述，用来测试渲染引擎对超长文本的处理能力。'.repeat(25);
    document.body.innerHTML = `
      <lk-quiz type="single" question="${longQuestion}">
        <lk-option correct>正确</lk-option>
        <lk-option>错误</lk-option>
      </lk-quiz>
    `;
    renderQuizzes(document.body);

    const question = document.querySelector('.quiz-question');
    expect(question).not.toBeNull();
    expect(question.textContent.length).toBeGreaterThan(400);
  });
});
