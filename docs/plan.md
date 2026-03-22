# LearnKit Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a personal learning tool that transforms any topic into structured, interactive HTML lessons — powered by Claude Code Skills + Rust CLI/Backend + React WebUI.

**Architecture:** Single Rust binary (`learnkit`) serves as both CLI (for Claude Code) and HTTP backend (for WebUI). React frontend is built to static files and served by the backend. Claude Code generates lesson content via Skills, writes via CLI commands. Communication uses process spawn + filesystem + exit codes.

**Tech Stack:** Rust (Axum, rusqlite, clap, serde_yaml), React (Vite, TypeScript), SQLite, HTML lesson templates

**Spec:** `docs/architecture.md`

---

## Phase 1: Rust CLI Foundation

### Task 1: Initialize Rust project

**Files:**
- Create: `cli/Cargo.toml`
- Create: `cli/src/main.rs`
- Create: `cli/src/config.rs`

- [ ] **Step 1:** Initialize Cargo project with dependencies

```bash
cd /Users/chasey/cc/projects/frameworks/learnkit
cargo init cli
```

Add to `Cargo.toml`:
```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
rusqlite = { version = "0.31", features = ["bundled"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower-http = { version = "0.5", features = ["fs", "cors"] }
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1"
```

- [ ] **Step 2:** Define CLI subcommand structure in `main.rs` using clap derive

Define all subcommands as enums: `Serve`, `Init`, `List`, `Info`, `ScopeWrite`, `ScopeRead`, `LessonWrite`, `LessonVerify`, `LessonList`, `LessonOpen`, `Next`, `ResourceAdd`, `ResourceList`, `Progress`, `ProgressUpdate`, `CheckPrepare`, `AnswerWrite`, `QaHistory`

- [ ] **Step 3:** Define `config.rs` with workspace root path resolution

`learnkit_root()` returns `~/.learnkit/` (or `~/cc/.learnkit/` for dev)
`program_root(slug)` returns `{root}/{slug}/`

- [ ] **Step 4:** Verify it compiles and runs `learnkit --help`

```bash
cd cli && cargo run -- --help
```

- [ ] **Step 5:** Commit

---

### Task 2: SQLite database module

**Files:**
- Create: `cli/src/db/mod.rs`
- Create: `cli/src/db/schema.rs`
- Create: `cli/src/db/lessons.rs`
- Create: `cli/src/db/sections.rs`
- Create: `cli/src/db/qa.rs`
- Create: `cli/src/db/resources.rs`

- [ ] **Step 1:** Define schema creation in `schema.rs`

Four tables: `lessons`, `sections`, `qa_history`, `resources` — exactly as defined in architecture.md Section 8.1. Include WAL mode pragma.

- [ ] **Step 2:** Implement `db::open(program_slug)` in `mod.rs`

Opens/creates `~/.learnkit/{slug}/learnkit.db`, runs migrations, returns connection.

- [ ] **Step 3:** Implement `lessons.rs` CRUD

Functions: `insert_lesson`, `update_lesson_status`, `get_lesson`, `list_lessons`, `count_by_status`

- [ ] **Step 4:** Implement `sections.rs` CRUD

Functions: `insert_sections`, `mark_section_read`, `get_sections_for_lesson`

- [ ] **Step 5:** Implement `qa.rs` CRUD

Functions: `insert_qa`, `list_qa_for_lesson`, `list_qa_all`

- [ ] **Step 6:** Implement `resources.rs` CRUD

Functions: `insert_resource`, `list_resources`

- [ ] **Step 7:** Write tests for all DB operations

```bash
cargo test db::
```

- [ ] **Step 8:** Commit

---

### Task 3: Scope parser

**Files:**
- Create: `cli/src/scope.rs`

- [ ] **Step 1:** Define scope data structures

```rust
struct Scope { program: String, title: String, created: String, difficulty: String, subjects: Vec<Subject> }
struct Subject { slug: String, title: String, lessons: Vec<Lesson> }
struct Lesson { slug: String, title: String, sections: Vec<String> }
```

- [ ] **Step 2:** Implement `parse_scope(path) -> Scope`

Read file, split on `---` delimiters, parse YAML frontmatter with serde_yaml.

- [ ] **Step 3:** Implement `scope_to_json(scope) -> String`

For `scope-read` command output.

- [ ] **Step 4:** Write tests with sample scope.md

- [ ] **Step 5:** Commit

---

### Task 4: Workspace management commands

**Files:**
- Create: `cli/src/commands/mod.rs`
- Create: `cli/src/commands/init.rs`
- Create: `cli/src/commands/list.rs`
- Create: `cli/src/commands/info.rs`
- Create: `cli/src/commands/scope.rs`

- [ ] **Step 1:** Implement `init` — create workspace directory structure + empty DB

- [ ] **Step 2:** Implement `list` — scan `~/.learnkit/` for program dirs, output summary

- [ ] **Step 3:** Implement `info` — read scope.md + progress from DB, output JSON

- [ ] **Step 4:** Implement `scope-write` — copy provided file to `{workspace}/scope.md`, sync lessons to DB

- [ ] **Step 5:** Implement `scope-read` — parse scope.md, output JSON

- [ ] **Step 6:** Test all commands end-to-end

- [ ] **Step 7:** Commit

---

### Task 5: Lesson management commands

**Files:**
- Create: `cli/src/commands/lesson.rs`
- Create: `cli/src/template.rs`
- Create: `cli/src/verify.rs`

- [ ] **Step 1:** Create a minimal `_template.html` placeholder

Include `{{content}}`, `{{title}}`, `{{program}}`, `{{subject}}`, `{{lesson}}`, `{{prev_link}}`, `{{next_link}}` placeholders. Add placeholder script tags for ask/progress JS.

- [ ] **Step 2:** Implement `template::inject(template, content, metadata) -> String`

Replace placeholders with actual values.

- [ ] **Step 3:** Implement `verify::verify_lesson(html, scope_lesson) -> Result<(), Vec<String>>`

Check: template skeleton DOM nodes, script presence, section anchors match scope, valid HTML structure.

- [ ] **Step 4:** Implement `lesson-write` command

Read content file → inject into template → verify → write HTML → insert into DB.

- [ ] **Step 5:** Implement `lesson-verify` command (standalone)

- [ ] **Step 6:** Implement `lesson-list` command with status filter

- [ ] **Step 7:** Implement `lesson-open` — open URL in default browser

- [ ] **Step 8:** Implement `next` — find next pending/prepared lesson from scope order

- [ ] **Step 9:** Test with mock content

- [ ] **Step 10:** Commit

---

### Task 6: Progress and answer commands

**Files:**
- Create: `cli/src/commands/progress.rs`
- Create: `cli/src/commands/answer.rs`
- Create: `cli/src/commands/resource.rs`

- [ ] **Step 1:** Implement `progress` — output JSON from DB

- [ ] **Step 2:** Implement `progress-update` — update lesson status in DB

- [ ] **Step 3:** Implement `check-prepare` — count prepared-but-unfinished, exit code 0 or 1

- [ ] **Step 4:** Implement `answer-write` — write `answers/{request-id}.json` + insert into qa_history

- [ ] **Step 5:** Implement `qa-history` — list from DB, optional lesson filter

- [ ] **Step 6:** Implement `resource-add` — download URL to `resources/`, insert DB

- [ ] **Step 7:** Implement `resource-list` — list from DB

- [ ] **Step 8:** Test all commands

- [ ] **Step 9:** Commit

---

## Phase 2: Rust Backend

### Task 7: Axum HTTP server

**Files:**
- Create: `cli/src/server/mod.rs`
- Create: `cli/src/server/routes.rs`
- Create: `cli/src/server/state.rs`

- [ ] **Step 1:** Define `AppState` — db pool, config, `preparing_lessons: HashSet`

- [ ] **Step 2:** Implement `GET /api/health` endpoint

- [ ] **Step 3:** Implement `GET /api/programs` — list all programs

- [ ] **Step 4:** Implement `GET /api/programs/:slug/scope` — scope JSON

- [ ] **Step 5:** Implement `GET /api/programs/:slug/lessons` — lessons list

- [ ] **Step 6:** Implement `GET/POST /api/programs/:slug/progress` — read/write progress

- [ ] **Step 7:** Implement `GET /api/programs/:slug/qa-history` — QA history

- [ ] **Step 8:** Set up static file serving for `/lessons/` and React build

- [ ] **Step 9:** Wire into `learnkit serve` command

- [ ] **Step 10:** Test all endpoints with curl

- [ ] **Step 11:** Commit

---

### Task 8: Ask API (async + Claude Code spawn)

**Files:**
- Create: `cli/src/server/ask.rs`
- Create: `cli/src/server/claude.rs`

- [ ] **Step 1:** Implement `claude::spawn_claude(prompt, task_type) -> JoinHandle`

Spawn `claude -p --output-format stream-json --permission-mode bypassPermissions` as child process. Read stdout chunks as heartbeat. Handle idle/total timeouts. Return exit code.

- [ ] **Step 2:** Implement `POST /api/programs/:slug/ask` — async

Generate request_id, spawn claude process in background, return `{ requestId }` immediately.

- [ ] **Step 3:** Implement `GET /api/programs/:slug/answer/:requestId` — polling

Check `answers/{requestId}.json` exists. Return pending/done/error.

- [ ] **Step 4:** Implement smart retry logic in `claude.rs`

- [ ] **Step 5:** Test with mock claude command

- [ ] **Step 6:** Commit

---

### Task 9: Auto-prepare detection

**Files:**
- Create: `cli/src/server/prepare.rs`

- [ ] **Step 1:** Implement `check_and_prepare(state)` — periodic check logic

Per-subject detection, `preparing_lessons` dedup, concurrent spawn.

- [ ] **Step 2:** Implement `POST /api/programs/:slug/prepare` — manual trigger

- [ ] **Step 3:** Implement `GET /api/programs/:slug/prepare-status`

- [ ] **Step 4:** Wire periodic check into server startup (tokio::spawn interval)

- [ ] **Step 5:** Implement auto-start server from CLI commands

Check `GET localhost:3377/api/health`, if fails spawn `learnkit serve` in background.

- [ ] **Step 6:** Test auto-prepare with mock data

- [ ] **Step 7:** Commit

---

## Phase 3: Claude Code Skills

### Task 10: learn-create skill

**Files:**
- Create: `skills/learn-create/skill.md`

- [ ] **Step 1:** Write skill definition

Prompt: accept topic → discuss scope with user → call `learnkit init` → write scope.md → call `learnkit scope-write`.

- [ ] **Step 2:** Test manually in Claude Code: `/learn-create 游戏开发`

- [ ] **Step 3:** Commit

---

### Task 11: learn-research skill

**Files:**
- Create: `skills/learn-research/skill.md`

- [ ] **Step 1:** Write skill definition

Prompt: read scope.md → search resources → confirm with user → call `learnkit resource-add`.

- [ ] **Step 2:** Test manually

- [ ] **Step 3:** Commit

---

### Task 12: learn-prepare skill

**Files:**
- Create: `skills/learn-prepare/skill.md`

- [ ] **Step 1:** Write skill definition

Prompt: accept program/subject/lesson → read scope + resources → generate ONE lesson HTML body → call `learnkit lesson-write` → handle verify errors and retry.

- [ ] **Step 2:** Test: `claude -p "/learn-prepare game-dev game-design mda"`

- [ ] **Step 3:** Test verify failure + retry flow

- [ ] **Step 4:** Commit

---

### Task 13: learn-answer skill

**Files:**
- Create: `skills/learn-answer/skill.md`

- [ ] **Step 1:** Write skill definition

Prompt: accept request-id + program + lesson + selection + question → read lesson HTML for context → generate answer → call `learnkit answer-write`.

- [ ] **Step 2:** Test: `claude -p "/learn-answer --request-id test --program game-dev ..."`

- [ ] **Step 3:** Commit

---

## Phase 4: Frontend UI/UX Design

> **CRITICAL:** Before writing ANY frontend code, complete the UI/UX design process below.
> All design work MUST use `impeccable` skills for guidance.

### Task 14: Establish design system

**Files:**
- Create: `docs/DESIGN.md` — LearnKit 教案 UI/UX 设计规范
- Create: `frontend/design/` — design cases directory

- [ ] **Step 1:** Run `/teach-impeccable` to establish LearnKit-specific design context

Create a LearnKit-specific `.impeccable.md` or section in the existing one. Define: users (Chasey, self-learner), brand personality, aesthetic direction for the learning tool.

- [ ] **Step 2:** Write `docs/DESIGN.md` initial draft

Define: color system, typography, spacing, component patterns, dark/light modes, the principle of "multiple styles but consistent within a lesson".

- [ ] **Step 3:** Commit

---

### Task 15: Application shell component cases

> Each component gets a `cases.html` with at least 5 style variations.
> Use `impeccable:frontend-design` skill for EVERY cases.html file.

**Files:**
- Create: `frontend/design/program-card-cases.html` — Program 列表卡片 (5+ 方案)
- Create: `frontend/design/lesson-list-cases.html` — Lesson 列表项 (5+ 方案)
- Create: `frontend/design/progress-bar-cases.html` — 进度条/指示器 (5+ 方案)
- Create: `frontend/design/status-badge-cases.html` — 状态标签 (5+ 方案)
- Create: `frontend/design/nav-header-cases.html` — 应用顶栏 (5+ 方案)

- [ ] **Step 1:** Generate program-card-cases.html

Use `impeccable:frontend-design` — 5+ styles for program card (title, progress, subjects count, created date). Present to user for selection.

- [ ] **Step 2:** Generate lesson-list-cases.html

5+ styles for lesson list item (title, status badge, subject grouping).

- [ ] **Step 3:** Generate progress-bar-cases.html

5+ styles for progress indicators (bar, ring, dots, segments, etc.)

- [ ] **Step 4:** Generate status-badge-cases.html

5+ styles for status labels (pending/prepared/in_progress/completed/failed).

- [ ] **Step 5:** Generate nav-header-cases.html

5+ styles for the application top bar.

- [ ] **Step 6:** User reviews all cases and picks preferred styles

- [ ] **Step 7:** Commit

---

### Task 16: Lesson template component cases

> The lesson template is independent HTML, not React. Cases still use impeccable.

**Files:**
- Create: `frontend/design/lesson-nav-cases.html` — 教案导航栏 (5+ 方案)
- Create: `frontend/design/lesson-toc-cases.html` — 教案目录 (5+ 方案)
- Create: `frontend/design/ask-toolbar-cases.html` — 划词提问工具栏 (5+ 方案)
- Create: `frontend/design/answer-card-cases.html` — 回答悬浮卡片 (5+ 方案)
- Create: `frontend/design/qa-history-cases.html` — 问答历史面板 (5+ 方案)

- [ ] **Step 1:** Generate lesson-nav-cases.html — 5+ styles

- [ ] **Step 2:** Generate lesson-toc-cases.html — 5+ styles

- [ ] **Step 3:** Generate ask-toolbar-cases.html — 5+ styles for text selection toolbar + input popup

- [ ] **Step 4:** Generate answer-card-cases.html — 5+ styles for floating answer cards

- [ ] **Step 5:** Generate qa-history-cases.html — 5+ styles for history panel

- [ ] **Step 6:** User reviews all cases and picks preferred styles

- [ ] **Step 7:** Commit

---

### Task 17: Confirm final UI/UX

**Files:**
- Create: `frontend/design/UI.html` — 应用壳最终 UI 全貌
- Create: `frontend/design/UX.html` — 交互模式与动效
- Create: `frontend/design/lesson-template-final.html` — 教案模板最终设计
- Update: `docs/DESIGN.md` — 补充确认后的规范

- [ ] **Step 1:** Compose UI.html — assemble selected components into full application mockup

Use `impeccable:frontend-design`. Show Program list page + Program detail page in one HTML.

- [ ] **Step 2:** Compose UX.html — interactive prototype showing transitions, hover states, loading states

- [ ] **Step 3:** Compose lesson-template-final.html — full lesson reading experience with ask/progress/QA

- [ ] **Step 4:** User final review and approval

- [ ] **Step 5:** Update DESIGN.md with finalized specs (colors, fonts, spacing, component styles)

- [ ] **Step 6:** Commit

---

## Phase 5: Frontend Implementation

> All frontend code MUST follow confirmed UI/UX from Phase 4.
> Use `impeccable` skills during development.

### Task 18: React project setup

**Files:**
- Create: `frontend/package.json`
- Create: `frontend/vite.config.ts`
- Create: `frontend/tsconfig.json`
- Create: `frontend/src/main.tsx`
- Create: `frontend/src/App.tsx`
- Create: `frontend/src/api/client.ts`

- [ ] **Step 1:** Initialize Vite + React + TypeScript project

```bash
cd /Users/chasey/cc/projects/frameworks/learnkit
npm create vite@latest frontend -- --template react-ts
cd frontend && npm install
npm install react-router-dom
```

- [ ] **Step 2:** Configure vite proxy to backend (`localhost:3377`)

- [ ] **Step 3:** Implement API client (`api/client.ts`) with typed endpoints

- [ ] **Step 4:** Set up router with two routes: `/` and `/program/:slug`

- [ ] **Step 5:** Verify dev server runs

- [ ] **Step 6:** Commit

---

### Task 19: Program list page

**Files:**
- Create: `frontend/src/pages/ProgramList.tsx`
- Create: `frontend/src/components/ProgramCard.tsx`

- [ ] **Step 1:** Implement ProgramCard component following confirmed UI.html design

Use `impeccable:frontend-design` to match the selected program card style.

- [ ] **Step 2:** Implement ProgramList page — fetch `/api/programs`, render cards

- [ ] **Step 3:** Use `impeccable:polish` for final pass

- [ ] **Step 4:** Commit

---

### Task 20: Program detail page (lesson list)

**Files:**
- Create: `frontend/src/pages/ProgramDetail.tsx`
- Create: `frontend/src/components/LessonItem.tsx`
- Create: `frontend/src/components/ProgressBar.tsx`
- Create: `frontend/src/components/StatusBadge.tsx`

- [ ] **Step 1:** Implement StatusBadge, ProgressBar following confirmed designs

- [ ] **Step 2:** Implement LessonItem with status, click-to-navigate

- [ ] **Step 3:** Implement ProgramDetail page — fetch scope + lessons + progress, render grouped by subject

- [ ] **Step 4:** Link lesson click → navigate to `/lessons/:slug/:subject/:lesson.html`

- [ ] **Step 5:** Use `impeccable:polish` for final pass

- [ ] **Step 6:** Commit

---

### Task 21: Build and integrate with Backend

- [ ] **Step 1:** Configure `vite build` output to `frontend/dist/`

- [ ] **Step 2:** Update Backend to serve `frontend/dist/` at root for non-API routes

- [ ] **Step 3:** Test full flow: `learnkit serve` → open browser → see program list

- [ ] **Step 4:** Commit

---

## Phase 6: Lesson Template

> Use `impeccable` skills for all template work.

### Task 22: Base lesson template

**Files:**
- Create: `cli/templates/_template.html`

- [ ] **Step 1:** Build template based on confirmed `lesson-template-final.html` from Phase 4

Include: navigation bar, TOC sidebar, content area, footer navigation. All with `{{placeholder}}` tags.

- [ ] **Step 2:** Add theme toggle (dark/light) with localStorage

- [ ] **Step 3:** Commit

---

### Task 23: Text selection + ask interaction

**Files:**
- Modify: `cli/templates/_template.html`

- [ ] **Step 1:** Implement text selection detection JS

On `mouseup`/`touchend`, detect selection, show ask toolbar at selection position.

- [ ] **Step 2:** Implement ask input popup

Click toolbar → show input with quoted selection → submit button.

- [ ] **Step 3:** Implement async ask flow

POST `/api/programs/:slug/ask` → show loading → poll `/api/programs/:slug/answer/:id` → show floating card.

- [ ] **Step 4:** Implement floating answer card (close, pin, position near selection)

- [ ] **Step 5:** Use `impeccable:polish` for interactions

- [ ] **Step 6:** Commit

---

### Task 24: Progress reporting + QA history

**Files:**
- Modify: `cli/templates/_template.html`

- [ ] **Step 1:** Implement scroll-based section reading detection

IntersectionObserver on section headings → POST progress when section enters viewport.

- [ ] **Step 2:** Implement QA history panel

Slide-out panel showing past questions for this lesson, fetched from `/api/programs/:slug/qa-history`.

- [ ] **Step 3:** Commit

---

### Task 25: Integrate template with lesson-write

- [ ] **Step 1:** Move finalized template to `cli/templates/_template.html`

- [ ] **Step 2:** Update `template::inject()` to handle all placeholders + JS URL injection

- [ ] **Step 3:** Update `verify::verify_lesson()` checks against finalized template

- [ ] **Step 4:** Test: write a mock lesson → verify → open in browser → test ask

- [ ] **Step 5:** Commit

---

## Phase 7: Integration & End-to-End

### Task 26: Full loop integration test

- [ ] **Step 1:** Start `learnkit serve`

- [ ] **Step 2:** Run `/learn-create` in Claude Code to create a test program

- [ ] **Step 3:** Run `/learn-prepare` to generate lessons

- [ ] **Step 4:** Open WebUI → browse program → read lesson → ask question → verify answer

- [ ] **Step 5:** Verify auto-prepare triggers when lessons are running low

- [ ] **Step 6:** Fix any integration issues

- [ ] **Step 7:** Commit

---

### Task 27: Concurrent prepare + error handling

- [ ] **Step 1:** Test concurrent `claude -p` spawning (3+ sessions)

- [ ] **Step 2:** Test timeout handling (simulate stuck process)

- [ ] **Step 3:** Test retry logic (simulate failures)

- [ ] **Step 4:** Test lesson-verify rejection + Claude Code retry

- [ ] **Step 5:** Fix issues and commit

---

### Task 28: Documentation and cleanup

- [ ] **Step 1:** Update README.md with installation and usage instructions

- [ ] **Step 2:** Clean up test-mvp/ artifacts

- [ ] **Step 3:** Remove unused skills from project structure (learn-review, learn-progress, learn-next)

- [ ] **Step 4:** Final commit
