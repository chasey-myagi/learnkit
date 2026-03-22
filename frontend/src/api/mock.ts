import type { Program, ScopeData, LessonRow, ProgressData } from './client';

export const mockPrograms: Program[] = [
  { slug: 'game-dev', title: '游戏开发', path: '/data/programs/game-dev' },
  { slug: 'reinforcement-learning', title: '强化学习', path: '/data/programs/reinforcement-learning' },
  { slug: 'rust-systems', title: 'Rust 系统编程', path: '/data/programs/rust-systems' },
];

export const mockScopes: Record<string, ScopeData> = {
  'game-dev': {
    program: 'game-dev',
    title: '游戏开发',
    created: '2026-03-22',
    difficulty: 'intermediate',
    subjects: [
      {
        slug: 'game-design',
        title: '游戏设计',
        lessons: [
          { slug: 'intro', title: '游戏设计概论', sections: ['概念', '历史', '分类'] },
          { slug: 'psychology', title: '玩家心理学', sections: ['动机', '心流', '反馈循环', '留存'] },
          { slug: 'core-loop', title: '核心游戏循环', sections: ['定义', '案例分析', '设计实践'] },
          { slug: 'level-design', title: '关卡设计方法论', sections: ['节奏', '难度曲线', '引导', '测试', '工具'] },
          { slug: 'prototyping', title: '原型设计与迭代', sections: ['快速原型', '测试方法'] },
        ],
      },
      {
        slug: 'game-programming',
        title: '游戏编程',
        lessons: [
          { slug: 'lang-choice', title: '编程语言选择', sections: ['对比', '生态', '性能'] },
          { slug: 'engine-arch', title: '游戏引擎架构', sections: ['ECS', '场景图', '资源管理'] },
          { slug: 'rendering', title: '渲染管线基础', sections: ['顶点处理', '光栅化', '着色'] },
          { slug: 'physics', title: '物理引擎入门', sections: ['碰撞检测', '刚体', '约束'] },
        ],
      },
      {
        slug: 'game-art',
        title: '游戏美术',
        lessons: [
          { slug: 'pixel-art', title: '像素艺术基础', sections: ['工具', '调色板', '动画帧', '导出'] },
          { slug: 'character-anim', title: '角色动画设计', sections: ['骨骼', '状态机'] },
          { slug: 'ui-visual', title: 'UI 与视觉设计', sections: ['布局', 'HUD', '菜单'] },
        ],
      },
    ],
  },
  'reinforcement-learning': {
    program: 'reinforcement-learning',
    title: '强化学习',
    created: '2026-03-20',
    difficulty: 'advanced',
    subjects: [
      {
        slug: 'foundations',
        title: '基础理论',
        lessons: [
          { slug: 'mdp', title: '马尔可夫决策过程', sections: ['状态', '动作', '奖励', '策略'] },
          { slug: 'bellman', title: 'Bellman 方程', sections: ['推导', '求解', '迭代'] },
          { slug: 'dp', title: '动态规划方法', sections: ['策略迭代', '值迭代'] },
          { slug: 'mc', title: '蒙特卡洛方法', sections: ['采样', '估计', '控制'] },
        ],
      },
      {
        slug: 'advanced',
        title: '进阶方法',
        lessons: [
          { slug: 'td', title: '时序差分学习', sections: ['TD(0)', 'SARSA', 'Q-Learning'] },
          { slug: 'pg', title: '策略梯度', sections: ['REINFORCE', 'Actor-Critic'] },
          { slug: 'drl', title: '深度强化学习', sections: ['DQN', 'A3C', 'PPO'] },
          { slug: 'env', title: '环境建模', sections: ['Gym', '自定义环境', '奖励设计'] },
        ],
      },
    ],
  },
  'rust-systems': {
    program: 'rust-systems',
    title: 'Rust 系统编程',
    created: '2026-03-15',
    difficulty: 'advanced',
    subjects: [
      {
        slug: 'ownership',
        title: '所有权系统',
        lessons: [
          { slug: 'basics', title: '所有权基础', sections: ['移动', '借用', '生命周期'] },
          { slug: 'borrowing', title: '借用检查器', sections: ['可变借用', '不可变借用', '悬垂引用'] },
          { slug: 'lifetime', title: '生命周期标注', sections: ['函数', '结构体', '高阶'] },
          { slug: 'smart-ptr', title: '智能指针', sections: ['Box', 'Rc', 'Arc', 'RefCell'] },
        ],
      },
      {
        slug: 'systems',
        title: '系统设计',
        lessons: [
          { slug: 'concurrency', title: '并发编程', sections: ['线程', '消息传递', '共享状态'] },
          { slug: 'async', title: '异步运行时', sections: ['Future', 'Tokio', 'async/await'] },
          { slug: 'ffi', title: 'FFI 与互操作', sections: ['C ABI', '安全封装', 'bindgen'] },
          { slug: 'perf', title: '性能优化', sections: ['零成本抽象', 'SIMD', '内存布局'] },
        ],
      },
    ],
  },
};

export const mockLessons: Record<string, LessonRow[]> = {
  'game-dev': [
    { id: 'game-design/intro', subject: 'game-design', lesson: 'intro', title: '游戏设计概论', status: 'completed', file_path: '/lessons/game-dev/game-design/intro.html', prepared_at: '2026-03-18', started_at: '2026-03-19', completed_at: '2026-03-20' },
    { id: 'game-design/psychology', subject: 'game-design', lesson: 'psychology', title: '玩家心理学', status: 'completed', file_path: '/lessons/game-dev/game-design/psychology.html', prepared_at: '2026-03-19', started_at: '2026-03-20', completed_at: '2026-03-21' },
    { id: 'game-design/core-loop', subject: 'game-design', lesson: 'core-loop', title: '核心游戏循环', status: 'in_progress', file_path: '/lessons/game-dev/game-design/core-loop.html', prepared_at: '2026-03-20', started_at: '2026-03-22', completed_at: null },
    { id: 'game-design/level-design', subject: 'game-design', lesson: 'level-design', title: '关卡设计方法论', status: 'prepared', file_path: '/lessons/game-dev/game-design/level-design.html', prepared_at: '2026-03-21', started_at: null, completed_at: null },
    { id: 'game-design/prototyping', subject: 'game-design', lesson: 'prototyping', title: '原型设计与迭代', status: 'pending', file_path: null, prepared_at: null, started_at: null, completed_at: null },
    { id: 'game-programming/lang-choice', subject: 'game-programming', lesson: 'lang-choice', title: '编程语言选择', status: 'completed', file_path: '/lessons/game-dev/game-programming/lang-choice.html', prepared_at: '2026-03-17', started_at: '2026-03-18', completed_at: '2026-03-19' },
    { id: 'game-programming/engine-arch', subject: 'game-programming', lesson: 'engine-arch', title: '游戏引擎架构', status: 'pending', file_path: null, prepared_at: null, started_at: null, completed_at: null },
    { id: 'game-programming/rendering', subject: 'game-programming', lesson: 'rendering', title: '渲染管线基础', status: 'pending', file_path: null, prepared_at: null, started_at: null, completed_at: null },
    { id: 'game-programming/physics', subject: 'game-programming', lesson: 'physics', title: '物理引擎入门', status: 'pending', file_path: null, prepared_at: null, started_at: null, completed_at: null },
    { id: 'game-art/pixel-art', subject: 'game-art', lesson: 'pixel-art', title: '像素艺术基础', status: 'prepared', file_path: '/lessons/game-dev/game-art/pixel-art.html', prepared_at: '2026-03-21', started_at: null, completed_at: null },
    { id: 'game-art/character-anim', subject: 'game-art', lesson: 'character-anim', title: '角色动画设计', status: 'pending', file_path: null, prepared_at: null, started_at: null, completed_at: null },
    { id: 'game-art/ui-visual', subject: 'game-art', lesson: 'ui-visual', title: 'UI 与视觉设计', status: 'pending', file_path: null, prepared_at: null, started_at: null, completed_at: null },
  ],
  'reinforcement-learning': [
    { id: 'foundations/mdp', subject: 'foundations', lesson: 'mdp', title: '马尔可夫决策过程', status: 'pending', file_path: null, prepared_at: null, started_at: null, completed_at: null },
    { id: 'foundations/bellman', subject: 'foundations', lesson: 'bellman', title: 'Bellman 方程', status: 'pending', file_path: null, prepared_at: null, started_at: null, completed_at: null },
    { id: 'foundations/dp', subject: 'foundations', lesson: 'dp', title: '动态规划方法', status: 'pending', file_path: null, prepared_at: null, started_at: null, completed_at: null },
    { id: 'foundations/mc', subject: 'foundations', lesson: 'mc', title: '蒙特卡洛方法', status: 'pending', file_path: null, prepared_at: null, started_at: null, completed_at: null },
    { id: 'advanced/td', subject: 'advanced', lesson: 'td', title: '时序差分学习', status: 'pending', file_path: null, prepared_at: null, started_at: null, completed_at: null },
    { id: 'advanced/pg', subject: 'advanced', lesson: 'pg', title: '策略梯度', status: 'pending', file_path: null, prepared_at: null, started_at: null, completed_at: null },
    { id: 'advanced/drl', subject: 'advanced', lesson: 'drl', title: '深度强化学习', status: 'pending', file_path: null, prepared_at: null, started_at: null, completed_at: null },
    { id: 'advanced/env', subject: 'advanced', lesson: 'env', title: '环境建模', status: 'pending', file_path: null, prepared_at: null, started_at: null, completed_at: null },
  ],
  'rust-systems': [
    { id: 'ownership/basics', subject: 'ownership', lesson: 'basics', title: '所有权基础', status: 'completed', file_path: '/lessons/rust-systems/ownership/basics.html', prepared_at: '2026-03-10', started_at: '2026-03-11', completed_at: '2026-03-12' },
    { id: 'ownership/borrowing', subject: 'ownership', lesson: 'borrowing', title: '借用检查器', status: 'completed', file_path: '/lessons/rust-systems/ownership/borrowing.html', prepared_at: '2026-03-11', started_at: '2026-03-12', completed_at: '2026-03-13' },
    { id: 'ownership/lifetime', subject: 'ownership', lesson: 'lifetime', title: '生命周期标注', status: 'completed', file_path: '/lessons/rust-systems/ownership/lifetime.html', prepared_at: '2026-03-12', started_at: '2026-03-13', completed_at: '2026-03-14' },
    { id: 'ownership/smart-ptr', subject: 'ownership', lesson: 'smart-ptr', title: '智能指针', status: 'completed', file_path: '/lessons/rust-systems/ownership/smart-ptr.html', prepared_at: '2026-03-13', started_at: '2026-03-14', completed_at: '2026-03-15' },
    { id: 'systems/concurrency', subject: 'systems', lesson: 'concurrency', title: '并发编程', status: 'completed', file_path: '/lessons/rust-systems/systems/concurrency.html', prepared_at: '2026-03-13', started_at: '2026-03-14', completed_at: '2026-03-15' },
    { id: 'systems/async', subject: 'systems', lesson: 'async', title: '异步运行时', status: 'completed', file_path: '/lessons/rust-systems/systems/async.html', prepared_at: '2026-03-14', started_at: '2026-03-15', completed_at: '2026-03-15' },
    { id: 'systems/ffi', subject: 'systems', lesson: 'ffi', title: 'FFI 与互操作', status: 'completed', file_path: '/lessons/rust-systems/systems/ffi.html', prepared_at: '2026-03-14', started_at: '2026-03-15', completed_at: '2026-03-15' },
    { id: 'systems/perf', subject: 'systems', lesson: 'perf', title: '性能优化', status: 'completed', file_path: '/lessons/rust-systems/systems/perf.html', prepared_at: '2026-03-14', started_at: '2026-03-15', completed_at: '2026-03-15' },
  ],
};

export const mockProgress: Record<string, ProgressData> = {
  'game-dev': {
    lessons: { completed: 3, in_progress: 1, prepared: 2, pending: 6 },
    sections: { read: 8, total: 35 },
  },
  'reinforcement-learning': {
    lessons: { completed: 0, in_progress: 0, prepared: 0, pending: 8 },
    sections: { read: 0, total: 28 },
  },
  'rust-systems': {
    lessons: { completed: 8, in_progress: 0, prepared: 0, pending: 0 },
    sections: { read: 24, total: 24 },
  },
};
