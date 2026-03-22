// LearnKit 教案组件渲染系统
// 纯原生 JS，无外部依赖，ES Module

// ============================================================
// 工具函数
// ============================================================

function safeRoot(root) {
  return root != null ? root : null;
}

// ============================================================
// Quiz 组件
// ============================================================

export function renderQuizzes(root) {
  if (!safeRoot(root)) return;

  const quizzes = root.querySelectorAll('lk-quiz:not([data-rendered="true"])');
  quizzes.forEach(renderSingleQuiz);
}

function renderSingleQuiz(lkQuiz) {
  const type = lkQuiz.getAttribute('type') || 'single';
  const question = lkQuiz.getAttribute('question') || '';

  let block;
  switch (type) {
    case 'single':
      block = buildSingleQuiz(lkQuiz, question);
      break;
    case 'multi':
      block = buildMultiQuiz(lkQuiz, question);
      break;
    case 'truefalse':
      block = buildTrueFalseQuiz(lkQuiz, question);
      break;
    case 'fill':
      block = buildFillQuiz(lkQuiz, question);
      break;
    case 'order':
      block = buildOrderQuiz(lkQuiz, question);
      break;
    case 'match':
      block = buildMatchQuiz(lkQuiz, question);
      break;
    default:
      block = buildGenericQuiz(lkQuiz, question, type);
      break;
  }

  if (block) {
    lkQuiz.setAttribute('data-rendered', 'true');
    lkQuiz.style.display = 'none';
    lkQuiz.parentNode.insertBefore(block, lkQuiz.nextSibling);
  }
}

// --- 单选题 ---
function buildSingleQuiz(lkQuiz, question) {
  const block = document.createElement('div');
  block.className = 'quiz-block';

  const header = document.createElement('div');
  header.className = 'quiz-header';
  block.appendChild(header);

  const questionEl = document.createElement('div');
  questionEl.className = 'quiz-question';
  questionEl.textContent = question;
  block.appendChild(questionEl);

  const optionsContainer = document.createElement('div');
  optionsContainer.className = 'quiz-options';

  const lkOptions = lkQuiz.querySelectorAll('lk-option');
  const explanationEl = lkQuiz.querySelector('lk-explanation');
  const explanationText = explanationEl ? explanationEl.textContent.trim() : '';

  const letters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';

  lkOptions.forEach((opt, i) => {
    const optionDiv = document.createElement('div');
    optionDiv.className = 'quiz-option';

    const letterSpan = document.createElement('span');
    letterSpan.className = 'option-letter';
    letterSpan.textContent = letters[i] || '';
    optionDiv.appendChild(letterSpan);

    const textSpan = document.createElement('span');
    textSpan.className = 'option-text';
    textSpan.textContent = opt.textContent;
    optionDiv.appendChild(textSpan);

    optionDiv.dataset.index = String(i);
    optionDiv.dataset.correct = opt.hasAttribute('correct') ? 'true' : 'false';

    optionsContainer.appendChild(optionDiv);
  });

  block.appendChild(optionsContainer);

  const feedback = document.createElement('div');
  feedback.className = 'quiz-feedback';
  block.appendChild(feedback);

  optionsContainer.addEventListener('click', (e) => {
    const optionDiv = e.target.closest('.quiz-option');
    if (!optionDiv) return;
    if (optionDiv.classList.contains('answered')) return;

    const allOptions = optionsContainer.querySelectorAll('.quiz-option');
    const isCorrect = optionDiv.dataset.correct === 'true';

    if (isCorrect) {
      optionDiv.classList.add('correct');
    } else {
      optionDiv.classList.add('incorrect');
      allOptions.forEach(o => {
        if (o.dataset.correct === 'true') o.classList.add('correct');
      });
    }

    allOptions.forEach(o => o.classList.add('answered'));

    if (explanationText) {
      feedback.textContent = explanationText;
    }
  });

  return block;
}

// --- 多选题 ---
function buildMultiQuiz(lkQuiz, question) {
  const block = document.createElement('div');
  block.className = 'quiz-block';

  const header = document.createElement('div');
  header.className = 'quiz-header';
  block.appendChild(header);

  const questionEl = document.createElement('div');
  questionEl.className = 'quiz-question';
  questionEl.textContent = question;
  block.appendChild(questionEl);

  const optionsContainer = document.createElement('div');
  optionsContainer.className = 'quiz-options';

  const lkOptions = lkQuiz.querySelectorAll('lk-option');
  const explanationEl = lkQuiz.querySelector('lk-explanation');
  const explanationText = explanationEl ? explanationEl.textContent.trim() : '';

  lkOptions.forEach((opt) => {
    const optionDiv = document.createElement('div');
    optionDiv.className = 'quiz-option';

    const checkbox = document.createElement('input');
    checkbox.type = 'checkbox';
    optionDiv.appendChild(checkbox);

    const textSpan = document.createElement('span');
    textSpan.className = 'option-text';
    textSpan.textContent = opt.textContent;
    optionDiv.appendChild(textSpan);

    optionDiv.dataset.correct = opt.hasAttribute('correct') ? 'true' : 'false';
    optionsContainer.appendChild(optionDiv);
  });

  block.appendChild(optionsContainer);

  const submitBtn = document.createElement('button');
  submitBtn.className = 'quiz-submit';
  submitBtn.textContent = '提交';
  block.appendChild(submitBtn);

  const feedback = document.createElement('div');
  feedback.className = 'quiz-feedback';
  block.appendChild(feedback);

  submitBtn.addEventListener('click', () => {
    const allOptions = optionsContainer.querySelectorAll('.quiz-option');
    allOptions.forEach(opt => {
      const cb = opt.querySelector('input[type="checkbox"]');
      const isCorrect = opt.dataset.correct === 'true';
      const isChecked = cb.checked;

      if (isCorrect && isChecked) {
        opt.classList.add('correct');
      } else if (isCorrect && !isChecked) {
        opt.classList.add('missed');
      } else if (!isCorrect && isChecked) {
        opt.classList.add('incorrect');
      }

      cb.disabled = true;
    });

    if (explanationText) {
      feedback.textContent = explanationText;
    }
  });

  return block;
}

// --- 判断题 ---
function buildTrueFalseQuiz(lkQuiz, question) {
  const block = document.createElement('div');
  block.className = 'quiz-block';

  const header = document.createElement('div');
  header.className = 'quiz-header';
  block.appendChild(header);

  const questionEl = document.createElement('div');
  questionEl.className = 'quiz-question';
  questionEl.textContent = question;
  block.appendChild(questionEl);

  const answer = lkQuiz.getAttribute('answer') || 'true';
  const explanationEl = lkQuiz.querySelector('lk-explanation');
  const explanationText = explanationEl ? explanationEl.textContent.trim() : '';

  const optionsContainer = document.createElement('div');
  optionsContainer.className = 'quiz-options truefalse-options';

  const trueOption = document.createElement('div');
  trueOption.className = 'quiz-option';
  trueOption.textContent = '✓ 正确';
  trueOption.dataset.value = 'true';
  trueOption.dataset.correct = answer === 'true' ? 'true' : 'false';

  const falseOption = document.createElement('div');
  falseOption.className = 'quiz-option';
  falseOption.textContent = '✗ 错误';
  falseOption.dataset.value = 'false';
  falseOption.dataset.correct = answer === 'false' ? 'true' : 'false';

  optionsContainer.appendChild(trueOption);
  optionsContainer.appendChild(falseOption);
  block.appendChild(optionsContainer);

  const feedback = document.createElement('div');
  feedback.className = 'quiz-feedback';
  block.appendChild(feedback);

  optionsContainer.addEventListener('click', (e) => {
    const optionDiv = e.target.closest('.quiz-option');
    if (!optionDiv) return;
    if (optionDiv.classList.contains('answered')) return;

    const allOptions = optionsContainer.querySelectorAll('.quiz-option');
    const isCorrect = optionDiv.dataset.correct === 'true';

    if (isCorrect) {
      optionDiv.classList.add('correct');
    } else {
      optionDiv.classList.add('incorrect');
      allOptions.forEach(o => {
        if (o.dataset.correct === 'true') o.classList.add('correct');
      });
    }

    allOptions.forEach(o => o.classList.add('answered'));

    if (explanationText) {
      feedback.textContent = explanationText;
    }
  });

  return block;
}

// --- 填空题 ---
function buildFillQuiz(lkQuiz, question) {
  const block = document.createElement('div');
  block.className = 'quiz-block';

  const header = document.createElement('div');
  header.className = 'quiz-header';
  block.appendChild(header);

  const questionEl = document.createElement('div');
  questionEl.className = 'quiz-question';

  const answersStr = lkQuiz.getAttribute('answers') || '';
  const answers = answersStr.split(',').map(a => a.trim());
  const explanationEl = lkQuiz.querySelector('lk-explanation');
  const explanationText = explanationEl ? explanationEl.textContent.trim() : '';

  const inputs = [];
  const parts = question.split(/\{(\d+)\}/);
  parts.forEach((part, i) => {
    if (i % 2 === 0) {
      if (part) {
        questionEl.appendChild(document.createTextNode(part));
      }
    } else {
      const input = document.createElement('input');
      input.type = 'text';
      input.className = 'fill-input';
      input.dataset.index = part;
      inputs.push(input);
      questionEl.appendChild(input);
    }
  });

  block.appendChild(questionEl);

  const submitBtn = document.createElement('button');
  submitBtn.className = 'quiz-submit';
  submitBtn.textContent = '提交';
  block.appendChild(submitBtn);

  const feedback = document.createElement('div');
  feedback.className = 'quiz-feedback';
  block.appendChild(feedback);

  submitBtn.addEventListener('click', () => {
    let allCorrect = true;

    inputs.forEach((input, i) => {
      const expected = answers[i] || '';
      const userAnswer = (input.value || '').trim();
      const isCorrect = userAnswer.toLowerCase() === expected.toLowerCase();

      if (isCorrect) {
        input.classList.add('correct');
      } else {
        input.classList.add('incorrect');
        allCorrect = false;
      }

      input.disabled = true;
    });

    if (!allCorrect) {
      block.classList.add('incorrect');
    }

    if (explanationText) {
      feedback.textContent = explanationText;
    }
  });

  return block;
}

// --- 排序题 ---
function buildOrderQuiz(lkQuiz, question) {
  const block = document.createElement('div');
  block.className = 'quiz-block';

  const header = document.createElement('div');
  header.className = 'quiz-header';
  block.appendChild(header);

  const questionEl = document.createElement('div');
  questionEl.className = 'quiz-question';
  questionEl.textContent = question;
  block.appendChild(questionEl);

  const orderList = document.createElement('div');
  orderList.className = 'quiz-order-list';

  const lkItems = lkQuiz.querySelectorAll('lk-order-item');
  const items = [];

  lkItems.forEach(item => {
    const order = item.getAttribute('order') || '0';
    items.push({ text: item.textContent.trim(), order: parseInt(order, 10) });
  });

  items.forEach(item => {
    const div = document.createElement('div');
    div.className = 'quiz-order-item';
    div.textContent = item.text;
    div.dataset.order = String(item.order);
    div.setAttribute('draggable', 'true');

    div.addEventListener('dragstart', (e) => {
      e.dataTransfer.setData('text/plain', '');
      div.classList.add('dragging');
    });
    div.addEventListener('dragend', () => {
      div.classList.remove('dragging');
    });

    orderList.appendChild(div);
  });

  orderList.addEventListener('dragover', (e) => {
    e.preventDefault();
    const dragging = orderList.querySelector('.dragging');
    if (!dragging) return;
    const afterElement = getDragAfterElement(orderList, e.clientY);
    if (afterElement) {
      orderList.insertBefore(dragging, afterElement);
    } else {
      orderList.appendChild(dragging);
    }
  });

  block.appendChild(orderList);

  const submitBtn = document.createElement('button');
  submitBtn.className = 'quiz-submit';
  submitBtn.textContent = '提交';
  block.appendChild(submitBtn);

  const feedback = document.createElement('div');
  feedback.className = 'quiz-feedback';
  block.appendChild(feedback);

  const explanationEl = lkQuiz.querySelector('lk-explanation');
  const explanationText = explanationEl ? explanationEl.textContent.trim() : '';

  submitBtn.addEventListener('click', () => {
    const currentItems = orderList.querySelectorAll('.quiz-order-item');
    let isCorrect = true;
    let prevOrder = -Infinity;

    currentItems.forEach(item => {
      const order = parseInt(item.dataset.order, 10);
      if (order < prevOrder) isCorrect = false;
      prevOrder = order;
    });

    if (isCorrect) {
      block.classList.add('correct');
    } else {
      block.classList.add('incorrect');
    }

    if (explanationText) {
      feedback.textContent = explanationText;
    }

    currentItems.forEach(item => {
      item.setAttribute('draggable', 'false');
      item.classList.add('answered');
    });
  });

  return block;
}

function getDragAfterElement(container, y) {
  const elements = [...container.querySelectorAll('.quiz-order-item:not(.dragging)')];
  return elements.reduce((closest, child) => {
    const box = child.getBoundingClientRect();
    const offset = y - box.top - box.height / 2;
    if (offset < 0 && offset > closest.offset) {
      return { offset, element: child };
    }
    return closest;
  }, { offset: -Infinity }).element || null;
}

// --- 连线匹配题 ---
function buildMatchQuiz(lkQuiz, question) {
  const block = document.createElement('div');
  block.className = 'quiz-block';

  const header = document.createElement('div');
  header.className = 'quiz-header';
  block.appendChild(header);

  const questionEl = document.createElement('div');
  questionEl.className = 'quiz-question';
  questionEl.textContent = question;
  block.appendChild(questionEl);

  const matchContainer = document.createElement('div');
  matchContainer.className = 'match-container';

  const leftCol = document.createElement('div');
  leftCol.className = 'match-left';
  const rightCol = document.createElement('div');
  rightCol.className = 'match-right';

  const pairs = lkQuiz.querySelectorAll('lk-match-pair');
  const leftItems = [];
  const rightItems = [];

  pairs.forEach((pair, i) => {
    const leftText = pair.getAttribute('left') || '';
    const rightText = pair.getAttribute('right') || '';

    const leftDiv = document.createElement('div');
    leftDiv.className = 'match-item';
    leftDiv.textContent = leftText;
    leftDiv.dataset.id = 'left-' + i;
    leftDiv.dataset.match = rightText;
    leftItems.push(leftDiv);

    const rightDiv = document.createElement('div');
    rightDiv.className = 'match-item';
    rightDiv.textContent = rightText;
    rightDiv.dataset.id = 'right-' + i;
    rightItems.push(rightDiv);
  });

  // Reverse the right items so they're not in the same order as left
  const displayRight = [...rightItems].reverse();

  leftItems.forEach(item => leftCol.appendChild(item));
  displayRight.forEach(item => rightCol.appendChild(item));

  matchContainer.appendChild(leftCol);
  matchContainer.appendChild(rightCol);
  block.appendChild(matchContainer);

  // Match interaction state
  let selectedLeft = null;

  matchContainer.addEventListener('click', (e) => {
    const item = e.target.closest('.match-item');
    if (!item) return;
    if (item.classList.contains('answered')) return;

    const isLeft = item.closest('.match-left') !== null;
    const isRight = item.closest('.match-right') !== null;

    if (isLeft) {
      if (selectedLeft) selectedLeft.classList.remove('selected');
      selectedLeft = item;
      item.classList.add('selected');
    } else if (isRight && selectedLeft) {
      // Remove old pair for this right item (if any left was paired to it)
      leftCol.querySelectorAll('.match-item').forEach(li => {
        if (li.dataset.pair === item.dataset.id) {
          li.classList.remove('paired');
          delete li.dataset.pair;
        }
      });

      // Remove old pair from the previously paired right item
      const oldPairId = selectedLeft.dataset.pair;
      if (oldPairId) {
        const oldRight = rightCol.querySelector(`.match-item[data-id="${oldPairId}"]`);
        if (oldRight) oldRight.classList.remove('paired');
      }

      selectedLeft.dataset.pair = item.dataset.id;
      selectedLeft.classList.add('paired');
      selectedLeft.classList.remove('selected');
      item.classList.add('paired');
      selectedLeft = null;
    }
  });

  const submitBtn = document.createElement('button');
  submitBtn.className = 'quiz-submit';
  submitBtn.textContent = '提交';
  block.appendChild(submitBtn);

  const feedback = document.createElement('div');
  feedback.className = 'quiz-feedback';
  block.appendChild(feedback);

  const explanationEl = lkQuiz.querySelector('lk-explanation');
  const explanationText = explanationEl ? explanationEl.textContent.trim() : '';

  submitBtn.addEventListener('click', () => {
    const allLeft = leftCol.querySelectorAll('.match-item');
    let allCorrect = true;
    let allPaired = true;

    allLeft.forEach(leftItem => {
      const expectedRight = leftItem.dataset.match;
      const pairedId = leftItem.dataset.pair;

      if (!pairedId) {
        allPaired = false;
        allCorrect = false;
        return;
      }

      const rightItem = rightCol.querySelector(`.match-item[data-id="${pairedId}"]`);
      if (!rightItem || rightItem.textContent.trim() !== expectedRight) {
        allCorrect = false;
      }
    });

    if (!allPaired) {
      block.classList.add('incomplete');
      feedback.textContent = '请完成所有配对';
    }

    if (allCorrect && allPaired) {
      block.classList.add('correct');
      feedback.classList.add('correct');
    } else {
      block.classList.add('incorrect');
    }

    if (explanationText) {
      feedback.textContent = explanationText;
    }

    const allItems = matchContainer.querySelectorAll('.match-item');
    allItems.forEach(item => item.classList.add('answered'));
  });

  return block;
}

// --- 未知类型 ---
function buildGenericQuiz(lkQuiz, question) {
  const block = document.createElement('div');
  block.className = 'quiz-block';

  const header = document.createElement('div');
  header.className = 'quiz-header';
  block.appendChild(header);

  const questionEl = document.createElement('div');
  questionEl.className = 'quiz-question';
  questionEl.textContent = question;
  block.appendChild(questionEl);

  const optionsContainer = document.createElement('div');
  optionsContainer.className = 'quiz-options';
  block.appendChild(optionsContainer);

  const feedback = document.createElement('div');
  feedback.className = 'quiz-feedback';
  block.appendChild(feedback);

  return block;
}

// ============================================================
// Code 组件 (lk-code, lk-code-tabs, lk-diff, lk-filetree)
// ============================================================

export function renderCodeBlocks(root) {
  if (!safeRoot(root)) return;

  // lk-code
  root.querySelectorAll('lk-code:not([data-rendered="true"])').forEach(lkCode => {
    const lang = lkCode.getAttribute('lang') || '';
    const title = lkCode.getAttribute('title') || '';
    const content = lkCode.textContent;

    const block = document.createElement('div');
    block.className = 'code-block';

    if (title) {
      const titleEl = document.createElement('div');
      titleEl.className = 'code-title';
      titleEl.textContent = title;
      block.appendChild(titleEl);
    }

    if (lang) {
      const langLabel = document.createElement('span');
      langLabel.className = 'code-lang';
      langLabel.textContent = lang;
      block.appendChild(langLabel);
    }

    const pre = document.createElement('pre');
    const code = document.createElement('code');
    code.textContent = content;
    pre.appendChild(code);
    block.appendChild(pre);

    lkCode.setAttribute('data-rendered', 'true');
    lkCode.style.display = 'none';
    lkCode.parentNode.insertBefore(block, lkCode.nextSibling);
  });

  // lk-code-tabs
  root.querySelectorAll('lk-code-tabs:not([data-rendered="true"])').forEach(lkCodeTabs => {
    const tabs = lkCodeTabs.querySelectorAll('lk-code-tab');

    const container = document.createElement('div');
    container.className = 'code-tabs';

    const tabHeader = document.createElement('div');
    tabHeader.className = 'code-tab-header';

    const tabPanels = [];

    tabs.forEach((tab, i) => {
      const label = tab.getAttribute('label') || '';
      const content = tab.textContent;

      const btn = document.createElement('button');
      btn.className = 'code-tab-btn';
      btn.textContent = label;
      if (i === 0) btn.classList.add('active');

      const panel = document.createElement('div');
      panel.className = 'code-tab-panel';
      panel.textContent = content;
      if (i !== 0) panel.style.display = 'none';

      btn.addEventListener('click', () => {
        tabHeader.querySelectorAll('.code-tab-btn').forEach(b => b.classList.remove('active'));
        btn.classList.add('active');
        tabPanels.forEach(p => p.style.display = 'none');
        panel.style.display = '';
      });

      tabHeader.appendChild(btn);
      tabPanels.push(panel);
    });

    container.appendChild(tabHeader);
    tabPanels.forEach(p => container.appendChild(p));

    lkCodeTabs.setAttribute('data-rendered', 'true');
    lkCodeTabs.style.display = 'none';
    lkCodeTabs.parentNode.insertBefore(container, lkCodeTabs.nextSibling);
  });

  // lk-diff
  root.querySelectorAll('lk-diff:not([data-rendered="true"])').forEach(lkDiff => {
    const title = lkDiff.getAttribute('title') || '';
    const content = lkDiff.textContent;

    const block = document.createElement('div');
    block.className = 'diff-block';

    if (title) {
      const titleEl = document.createElement('div');
      titleEl.className = 'diff-title';
      titleEl.textContent = title;
      block.appendChild(titleEl);
    }

    const lines = content.split('\n');
    lines.forEach(line => {
      if (line.startsWith('+')) {
        const lineDiv = document.createElement('div');
        lineDiv.className = 'diff-line added';
        lineDiv.textContent = line.substring(1);
        block.appendChild(lineDiv);
      } else if (line.startsWith('-')) {
        const lineDiv = document.createElement('div');
        lineDiv.className = 'diff-line removed';
        lineDiv.textContent = line.substring(1);
        block.appendChild(lineDiv);
      } else if (line.startsWith(' ') || line.trim().length > 0) {
        const lineDiv = document.createElement('div');
        lineDiv.className = 'diff-line unchanged';
        lineDiv.textContent = line.startsWith(' ') ? line.substring(1) : line;
        block.appendChild(lineDiv);
      }
    });

    lkDiff.setAttribute('data-rendered', 'true');
    lkDiff.style.display = 'none';
    lkDiff.parentNode.insertBefore(block, lkDiff.nextSibling);
  });

  // lk-filetree
  root.querySelectorAll('lk-filetree:not([data-rendered="true"])').forEach(lkTree => {
    const title = lkTree.getAttribute('title') || '';
    const content = lkTree.textContent;

    const block = document.createElement('div');
    block.className = 'filetree-block';

    if (title) {
      const titleEl = document.createElement('div');
      titleEl.className = 'filetree-title';
      titleEl.textContent = title;
      block.appendChild(titleEl);
    }

    const pre = document.createElement('pre');
    pre.className = 'filetree-content';
    pre.textContent = content;
    block.appendChild(pre);

    lkTree.setAttribute('data-rendered', 'true');
    lkTree.style.display = 'none';
    lkTree.parentNode.insertBefore(block, lkTree.nextSibling);
  });
}

// ============================================================
// Callout 组件
// ============================================================

const CALLOUT_ICONS = {
  tip: '\u{1F4A1}',
  warning: '\u26A0\uFE0F',
  note: '\u{1F4DD}',
  important: '\u2757',
};

export function renderCallouts(root) {
  if (!safeRoot(root)) return;

  root.querySelectorAll('lk-callout:not([data-rendered="true"])').forEach(lkCallout => {
    const type = lkCallout.getAttribute('type') || 'note';
    const content = lkCallout.textContent;

    const callout = document.createElement('div');
    callout.className = 'callout callout-' + type;

    const iconSpan = document.createElement('span');
    iconSpan.className = 'callout-icon';
    iconSpan.textContent = CALLOUT_ICONS[type] || CALLOUT_ICONS.note;
    callout.appendChild(iconSpan);

    const bodySpan = document.createElement('span');
    bodySpan.className = 'callout-body';
    bodySpan.textContent = content;
    callout.appendChild(bodySpan);

    lkCallout.setAttribute('data-rendered', 'true');
    lkCallout.style.display = 'none';
    lkCallout.parentNode.insertBefore(callout, lkCallout.nextSibling);
  });
}

// ============================================================
// Definition 组件
// ============================================================

export function renderDefinitions(root) {
  if (!safeRoot(root)) return;

  root.querySelectorAll('lk-definition:not([data-rendered="true"])').forEach(lkDef => {
    const term = lkDef.getAttribute('term') || '';
    const content = lkDef.textContent;

    const card = document.createElement('div');
    card.className = 'definition-card';

    const termEl = document.createElement('div');
    termEl.className = 'definition-term';
    termEl.textContent = term;
    card.appendChild(termEl);

    const bodyEl = document.createElement('div');
    bodyEl.className = 'definition-body';
    bodyEl.textContent = content;
    card.appendChild(bodyEl);

    lkDef.setAttribute('data-rendered', 'true');
    lkDef.style.display = 'none';
    lkDef.parentNode.insertBefore(card, lkDef.nextSibling);
  });
}

// ============================================================
// Expandable 组件
// ============================================================

export function renderExpandables(root) {
  if (!safeRoot(root)) return;

  root.querySelectorAll('lk-expandable:not([data-rendered="true"])').forEach(lkExp => {
    const title = lkExp.getAttribute('title') || '';
    const content = lkExp.innerHTML;

    const block = document.createElement('div');
    block.className = 'expandable';

    const headerEl = document.createElement('div');
    headerEl.className = 'expandable-header';
    headerEl.textContent = title;
    block.appendChild(headerEl);

    const contentEl = document.createElement('div');
    contentEl.className = 'expandable-content';
    contentEl.innerHTML = content;
    contentEl.style.display = 'none';
    block.appendChild(contentEl);

    headerEl.addEventListener('click', () => {
      const isOpen = block.classList.contains('open');
      if (isOpen) {
        block.classList.remove('open');
        contentEl.style.display = 'none';
      } else {
        block.classList.add('open');
        contentEl.style.display = '';
      }
    });

    lkExp.setAttribute('data-rendered', 'true');
    lkExp.style.display = 'none';
    lkExp.parentNode.insertBefore(block, lkExp.nextSibling);
  });
}

// ============================================================
// Comparisons (vs-card) 组件
// ============================================================

export function renderComparisons(root) {
  if (!safeRoot(root)) return;

  root.querySelectorAll('lk-vs:not([data-rendered="true"])').forEach(lkVs => {
    const sides = lkVs.querySelectorAll('lk-vs-side');

    const block = document.createElement('div');
    block.className = 'vs-block';

    sides.forEach(side => {
      const sideTitle = side.getAttribute('title') || '';

      const sideDiv = document.createElement('div');
      sideDiv.className = 'vs-side';

      const titleEl = document.createElement('div');
      titleEl.className = 'vs-side-title';
      titleEl.textContent = sideTitle;
      sideDiv.appendChild(titleEl);

      const contentEl = document.createElement('div');
      contentEl.className = 'vs-side-content';
      contentEl.innerHTML = side.innerHTML;
      sideDiv.appendChild(contentEl);

      block.appendChild(sideDiv);
    });

    lkVs.setAttribute('data-rendered', 'true');
    lkVs.style.display = 'none';
    lkVs.parentNode.insertBefore(block, lkVs.nextSibling);
  });
}

// ============================================================
// Tabs 组件 (content-tabs)
// ============================================================

export function renderTabs(root) {
  if (!safeRoot(root)) return;

  root.querySelectorAll('lk-tabs:not([data-rendered="true"])').forEach(lkTabs => {
    const tabs = lkTabs.querySelectorAll('lk-tab');

    const container = document.createElement('div');
    container.className = 'tabs-block';

    const tabHeader = document.createElement('div');
    tabHeader.className = 'tab-header';

    const panels = [];

    tabs.forEach((tab, i) => {
      const label = tab.getAttribute('label') || '';
      const content = tab.innerHTML;

      const btn = document.createElement('button');
      btn.className = 'tab-btn';
      btn.textContent = label;
      if (i === 0) btn.classList.add('active');

      const panel = document.createElement('div');
      panel.className = 'tab-panel';
      panel.innerHTML = content;
      if (i !== 0) panel.style.display = 'none';

      btn.addEventListener('click', () => {
        tabHeader.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
        btn.classList.add('active');
        panels.forEach(p => p.style.display = 'none');
        panel.style.display = '';
      });

      tabHeader.appendChild(btn);
      panels.push(panel);
    });

    container.appendChild(tabHeader);
    panels.forEach(p => container.appendChild(p));

    lkTabs.setAttribute('data-rendered', 'true');
    lkTabs.style.display = 'none';
    lkTabs.parentNode.insertBefore(container, lkTabs.nextSibling);
  });
}

// ============================================================
// Steps 组件
// ============================================================

export function renderSteps(root) {
  if (!safeRoot(root)) return;

  root.querySelectorAll('lk-steps:not([data-rendered="true"])').forEach(lkSteps => {
    const steps = lkSteps.querySelectorAll('lk-step');

    const block = document.createElement('div');
    block.className = 'steps-block';

    steps.forEach((step, i) => {
      const title = step.getAttribute('title') || '';

      const item = document.createElement('div');
      item.className = 'step-item';

      const numberEl = document.createElement('div');
      numberEl.className = 'step-number';
      numberEl.textContent = String(i + 1);
      item.appendChild(numberEl);

      const titleEl = document.createElement('div');
      titleEl.className = 'step-title';
      titleEl.textContent = title;
      item.appendChild(titleEl);

      const contentEl = document.createElement('div');
      contentEl.className = 'step-content';
      contentEl.innerHTML = step.innerHTML;
      item.appendChild(contentEl);

      block.appendChild(item);
    });

    lkSteps.setAttribute('data-rendered', 'true');
    lkSteps.style.display = 'none';
    lkSteps.parentNode.insertBefore(block, lkSteps.nextSibling);
  });
}

// ============================================================
// Timeline 组件
// ============================================================

export function renderTimelines(root) {
  if (!safeRoot(root)) return;

  root.querySelectorAll('lk-timeline:not([data-rendered="true"])').forEach(lkTimeline => {
    const items = lkTimeline.querySelectorAll('lk-timeline-item');

    const block = document.createElement('div');
    block.className = 'timeline-block';

    items.forEach(item => {
      const date = item.getAttribute('date') || '';
      const title = item.getAttribute('title') || '';
      const content = item.textContent;

      const timelineItem = document.createElement('div');
      timelineItem.className = 'timeline-item';

      const dateEl = document.createElement('div');
      dateEl.className = 'timeline-date';
      dateEl.textContent = date;
      timelineItem.appendChild(dateEl);

      const titleEl = document.createElement('div');
      titleEl.className = 'timeline-title';
      titleEl.textContent = title;
      timelineItem.appendChild(titleEl);

      const bodyEl = document.createElement('div');
      bodyEl.className = 'timeline-body';
      bodyEl.textContent = content;
      timelineItem.appendChild(bodyEl);

      block.appendChild(timelineItem);
    });

    lkTimeline.setAttribute('data-rendered', 'true');
    lkTimeline.style.display = 'none';
    lkTimeline.parentNode.insertBefore(block, lkTimeline.nextSibling);
  });
}

// ============================================================
// Formula 组件
// ============================================================

export function renderFormulas(root) {
  if (!safeRoot(root)) return;

  // Block formulas
  root.querySelectorAll('lk-formula:not([data-rendered="true"])').forEach(lkFormula => {
    const caption = lkFormula.getAttribute('caption') || '';
    const content = lkFormula.textContent;

    const block = document.createElement('div');
    block.className = 'formula-block';

    if (caption) {
      const captionEl = document.createElement('div');
      captionEl.className = 'formula-caption';
      captionEl.textContent = caption;
      block.appendChild(captionEl);
    }

    const contentEl = document.createElement('div');
    contentEl.className = 'formula-content';
    contentEl.textContent = content;
    block.appendChild(contentEl);

    lkFormula.setAttribute('data-rendered', 'true');
    lkFormula.style.display = 'none';
    lkFormula.parentNode.insertBefore(block, lkFormula.nextSibling);
  });

  // Inline formulas
  root.querySelectorAll('lk-formula-inline:not([data-rendered="true"])').forEach(lkInline => {
    const content = lkInline.textContent;

    const span = document.createElement('span');
    span.className = 'formula-inline';
    span.textContent = content;

    lkInline.setAttribute('data-rendered', 'true');
    lkInline.style.display = 'none';
    lkInline.parentNode.insertBefore(span, lkInline.nextSibling);
  });
}

// ============================================================
// Diagram 组件
// ============================================================

export function renderDiagrams(root) {
  if (!safeRoot(root)) return;

  root.querySelectorAll('lk-diagram:not([data-rendered="true"])').forEach(lkDiagram => {
    const title = lkDiagram.getAttribute('title') || '';
    const content = lkDiagram.textContent;

    const block = document.createElement('div');
    block.className = 'diagram-block';

    if (title) {
      const titleEl = document.createElement('div');
      titleEl.className = 'diagram-title';
      titleEl.textContent = title;
      block.appendChild(titleEl);
    }

    const pre = document.createElement('pre');
    pre.className = 'diagram-content';
    pre.textContent = content;
    block.appendChild(pre);

    lkDiagram.setAttribute('data-rendered', 'true');
    lkDiagram.style.display = 'none';
    lkDiagram.parentNode.insertBefore(block, lkDiagram.nextSibling);
  });
}

// ============================================================
// Figure 组件
// ============================================================

export function renderFigures(root) {
  if (!safeRoot(root)) return;

  root.querySelectorAll('lk-figure:not([data-rendered="true"])').forEach(lkFigure => {
    const src = lkFigure.getAttribute('src') || '';
    const alt = lkFigure.getAttribute('alt') || '';
    const caption = lkFigure.getAttribute('caption') || '';

    const figure = document.createElement('figure');

    const img = document.createElement('img');
    img.setAttribute('src', src);
    img.setAttribute('alt', alt);
    figure.appendChild(img);

    if (caption) {
      const figcaption = document.createElement('figcaption');
      figcaption.textContent = caption;
      figure.appendChild(figcaption);
    }

    lkFigure.setAttribute('data-rendered', 'true');
    lkFigure.style.display = 'none';
    lkFigure.parentNode.insertBefore(figure, lkFigure.nextSibling);
  });
}

// ============================================================
// Flowchart 组件
// ============================================================

export function renderFlowcharts(root) {
  if (!safeRoot(root)) return;

  root.querySelectorAll('lk-flowchart:not([data-rendered="true"])').forEach(lkFlowchart => {
    const title = lkFlowchart.getAttribute('title') || '';

    const container = document.createElement('div');
    container.className = 'flowchart';

    if (title) {
      const titleEl = document.createElement('div');
      titleEl.className = 'flowchart-title';
      titleEl.textContent = title;
      container.appendChild(titleEl);
    }

    const children = lkFlowchart.children;
    for (let i = 0; i < children.length; i++) {
      const child = children[i];
      const tagName = child.tagName.toLowerCase();

      if (tagName === 'lk-flowchart-node') {
        const nodeDiv = document.createElement('div');
        nodeDiv.className = 'flowchart-node';
        nodeDiv.textContent = child.textContent;
        if (child.hasAttribute('active')) {
          nodeDiv.classList.add('flowchart-node-active');
        }
        container.appendChild(nodeDiv);
      } else if (tagName === 'lk-flowchart-arrow') {
        const arrowDiv = document.createElement('div');
        arrowDiv.className = 'flowchart-arrow';
        arrowDiv.textContent = child.textContent;
        container.appendChild(arrowDiv);
      }
    }

    lkFlowchart.setAttribute('data-rendered', 'true');
    lkFlowchart.style.display = 'none';
    lkFlowchart.parentNode.insertBefore(container, lkFlowchart.nextSibling);
  });
}

// ============================================================
// renderAllComponents - 渲染所有组件
// ============================================================

export function renderAllComponents(root) {
  if (!safeRoot(root)) return;

  // 先渲染叶子组件（可能嵌套在容器组件内）
  renderCallouts(root);
  renderDefinitions(root);
  renderFormulas(root);
  renderDiagrams(root);
  renderFigures(root);
  renderFlowcharts(root);
  renderCodeBlocks(root);
  renderQuizzes(root);

  // 再渲染容器组件（它们会通过 innerHTML 捕获已渲染的嵌套内容）
  renderExpandables(root);
  renderComparisons(root);
  renderTabs(root);
  renderSteps(root);
  renderTimelines(root);

  // 二次渲染：处理容器组件复制 innerHTML 后新出现的未渲染组件
  renderCallouts(root);
  renderDefinitions(root);
  renderFormulas(root);
  renderDiagrams(root);
  renderFigures(root);
  renderFlowcharts(root);
  renderCodeBlocks(root);
  renderQuizzes(root);
}
