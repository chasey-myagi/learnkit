# LearnKit 开发规范

## Rust CLI 开发

- **必须使用 TDD 工作流** — `devkit:tdd-workflow`
  - 先写测试 → 验证失败 → 写实现 → 验证通过 → code-review → 提交
- 当前 Phase 1 的命令已有 stub 实现，后续迭代和 bug 修复必须走 TDD

## 前端开发

- **必须使用 frontend-workflow** — `devkit:frontend-workflow`
  - 设计先行，UI cases → 用户确认 → design freeze → 实现
- **所有前端代码必须使用 impeccable skills**
- 严格按 DESIGN.md 冻结规范实现，不得自行"改进"

## 通用

- 每个 PR / 提交前必须通过 `devkit:code-review`
- Bug 修复走 `devkit:issue-fix`（TDD 驱动：先写回归测试 → 修复）
