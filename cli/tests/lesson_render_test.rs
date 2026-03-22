//! 教案动态渲染集成测试
//!
//! 测试重构后的 lesson 渲染流程：
//! - Backend 动态拼装 shell.html + body content → 完整 HTML
//! - 静态伺服 lesson.css / lesson.js
//! - Shell 模板结构验证
//! - 与现有 API 兼容性

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use std::path::Path;
use std::sync::Arc;
use tempfile::TempDir;
use tower::ServiceExt;

use learnkit::server;

// ============================================================
// Helper 函数
// ============================================================

/// 模板目录名
const TEMPLATES_DIR: &str = "templates";

/// Shell 模板内容（模拟生产 shell.html）
fn shell_html_content() -> &'static str {
    r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{title}} - LearnKit</title>
    <link rel="stylesheet" href="/static/lesson.css">
    <script>window.API_BASE = "{{api_base}}";</script>
</head>
<body>
    <nav class="navbar">
        <a href="/">LearnKit</a>
        <div class="breadcrumb">{{breadcrumb}}</div>
        <button id="theme-toggle" aria-label="切换主题">🌓</button>
    </nav>
    <main class="lesson-content">
        {{content}}
    </main>
    <script src="/static/lesson.js"></script>
</body>
</html>"#
}

/// 创建测试路由（使用新的动态渲染路由）
fn create_test_app(root: &Path) -> axum::Router {
    let state = Arc::new(server::state::AppState::with_root(root.to_path_buf()));
    server::create_router(state)
}

/// 在临时目录中创建 shell.html 模板
fn setup_shell_template(dir: &Path) {
    let templates = dir.join(TEMPLATES_DIR);
    std::fs::create_dir_all(&templates).unwrap();

    // shell.html
    std::fs::write(templates.join("shell.html"), shell_html_content()).unwrap();

    // lesson.css
    std::fs::write(
        templates.join("lesson.css"),
        r#"/* LearnKit Lesson Styles */
body { font-family: system-ui, sans-serif; margin: 0; }
.navbar { display: flex; align-items: center; padding: 0.5rem 1rem; }
.breadcrumb { margin-left: 1rem; }
.lesson-content { max-width: 48rem; margin: 0 auto; padding: 2rem; }
"#,
    )
    .unwrap();

    // lesson.js
    std::fs::write(
        templates.join("lesson.js"),
        r#"// LearnKit Lesson Runtime
document.addEventListener('DOMContentLoaded', () => {
    // 处理 lk-* 自定义标签
    document.querySelectorAll('[class^="lk-"]').forEach(el => {
        el.classList.add('lk-initialized');
    });
});
"#,
    )
    .unwrap();
}

/// 在临时目录中创建 body-only lesson 文件
fn setup_lesson_content(
    dir: &Path,
    program: &str,
    subject: &str,
    lesson: &str,
    content: &str,
) {
    let lesson_dir = dir.join(program).join("lessons").join(subject);
    std::fs::create_dir_all(&lesson_dir).unwrap();
    std::fs::write(lesson_dir.join(format!("{lesson}.html")), content).unwrap();
}

/// 创建 scope.md 以支持 program 查询
fn setup_test_program(dir: &Path, slug: &str) {
    let program_dir = dir.join(slug);
    std::fs::create_dir_all(&program_dir).unwrap();

    let scope_content = format!(
        r#"---
program: {slug}
title: Test Program
created: 2026-01-01
subjects:
  - slug: design
    title: Game Design
    lessons:
      - slug: mda
        title: MDA Framework
        sections:
          - Overview
          - Details
---

# Test Program
"#
    );
    std::fs::write(program_dir.join("scope.md"), &scope_content).unwrap();
}

/// 读取响应 body 为字符串
async fn read_body_string(response: axum::http::Response<Body>) -> String {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

/// 发送 GET 请求
async fn get(app: axum::Router, uri: &str) -> axum::http::Response<Body> {
    app.oneshot(
        Request::builder()
            .uri(uri)
            .body(Body::empty())
            .unwrap(),
    )
    .await
    .unwrap()
}

/// 解析 JSON 响应
async fn parse_json(response: axum::http::Response<Body>) -> serde_json::Value {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

// ============================================================
// 1. 动态拼装 lesson 页面
// ============================================================

/// GET 请求返回的 HTML 包含 shell 骨架 + body 内容
#[tokio::test]
async fn test_serve_lesson_combines_shell_and_content() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());
    setup_lesson_content(
        tmp.path(),
        "test-prog",
        "design",
        "mda",
        "<h2>MDA Framework</h2><p>Mechanics, Dynamics, Aesthetics</p>",
    );

    let app = create_test_app(tmp.path());
    let resp = get(app, "/lessons/test-prog/lessons/design/mda").await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body = read_body_string(resp).await;

    // 包含 shell 骨架
    assert!(
        body.contains("<!DOCTYPE html>"),
        "应包含 DOCTYPE 声明"
    );
    assert!(
        body.contains("<html"),
        "应包含 <html> 标签"
    );
    assert!(body.contains("</html>"), "应包含闭合 </html> 标签");

    // 包含嵌入的 body 内容
    assert!(
        body.contains("<h2>MDA Framework</h2>"),
        "应包含 lesson body 内容"
    );
    assert!(
        body.contains("Mechanics, Dynamics, Aesthetics"),
        "应包含 lesson 正文段落"
    );
}

/// 返回的 HTML 包含 CSS link
#[tokio::test]
async fn test_serve_lesson_includes_css_link() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());
    setup_lesson_content(tmp.path(), "prog", "sub", "les", "<p>test</p>");

    let app = create_test_app(tmp.path());
    let resp = get(app, "/lessons/prog/lessons/sub/les").await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body = read_body_string(resp).await;
    assert!(
        body.contains("lesson.css"),
        "应包含 lesson.css 的引用，实际: {body}"
    );
    assert!(
        body.contains("<link"),
        "应通过 <link> 标签引入 CSS"
    );
}

/// 返回的 HTML 包含 JS script
#[tokio::test]
async fn test_serve_lesson_includes_js_script() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());
    setup_lesson_content(tmp.path(), "prog", "sub", "les", "<p>test</p>");

    let app = create_test_app(tmp.path());
    let resp = get(app, "/lessons/prog/lessons/sub/les").await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body = read_body_string(resp).await;
    assert!(
        body.contains("lesson.js"),
        "应包含 lesson.js 的引用"
    );
    assert!(
        body.contains("<script"),
        "应通过 <script> 标签引入 JS"
    );
}

/// 页面 title 包含 lesson 名称
#[tokio::test]
async fn test_serve_lesson_sets_correct_title() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());
    setup_lesson_content(tmp.path(), "prog", "design", "mda", "<p>content</p>");

    let app = create_test_app(tmp.path());
    let resp = get(app, "/lessons/prog/lessons/design/mda").await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body = read_body_string(resp).await;

    // title 应该包含 lesson slug 或名称
    assert!(
        body.contains("<title>") && body.contains("</title>"),
        "应包含 <title> 标签"
    );
    // slug "mda" 应出现在 title 中（可能被转换为人类可读的形式）
    let title_start = body.find("<title>").unwrap() + "<title>".len();
    let title_end = body.find("</title>").unwrap();
    let title = &body[title_start..title_end];
    assert!(
        title.to_lowercase().contains("mda"),
        "title 应包含 lesson 标识 'mda'，实际 title: '{title}'"
    );
}

/// 面包屑导航包含 subject 和 lesson 信息
#[tokio::test]
async fn test_serve_lesson_sets_correct_breadcrumb() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());
    setup_lesson_content(tmp.path(), "prog", "design", "mda", "<p>content</p>");

    let app = create_test_app(tmp.path());
    let resp = get(app, "/lessons/prog/lessons/design/mda").await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body = read_body_string(resp).await;

    assert!(
        body.contains("breadcrumb"),
        "应包含面包屑导航区域"
    );
    // 面包屑中应体现 subject 和 lesson
    assert!(
        body.contains("design"),
        "面包屑应包含 subject 信息 'design'"
    );
    assert!(
        body.contains("mda"),
        "面包屑应包含 lesson 信息 'mda'"
    );
}

/// 响应 Content-Type 是 text/html
#[tokio::test]
async fn test_serve_lesson_content_type_html() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());
    setup_lesson_content(tmp.path(), "prog", "sub", "les", "<p>test</p>");

    let app = create_test_app(tmp.path());
    let resp = get(app, "/lessons/prog/lessons/sub/les").await;

    assert_eq!(resp.status(), StatusCode::OK);
    let ct = resp
        .headers()
        .get("content-type")
        .expect("缺少 content-type 头")
        .to_str()
        .unwrap();
    assert!(
        ct.contains("text/html"),
        "Content-Type 应为 text/html，实际: {ct}"
    );
}

/// 请求不存在的 lesson 返回 404
#[tokio::test]
async fn test_serve_lesson_not_found() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());
    // 不创建任何 lesson 文件

    let app = create_test_app(tmp.path());
    let resp = get(app, "/lessons/no-prog/lessons/no-sub/no-lesson").await;

    assert_eq!(
        resp.status(),
        StatusCode::NOT_FOUND,
        "不存在的 lesson 应返回 404"
    );
}

/// 路径穿越攻击被阻止
#[tokio::test]
async fn test_serve_lesson_path_traversal_blocked() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());

    let app = create_test_app(tmp.path());

    // 尝试路径穿越
    let traversal_paths = [
        "/lessons/../../../etc/passwd/lessons/x/y",
        "/lessons/prog/lessons/../../etc/passwd",
        "/lessons/prog/lessons/../../../etc/shadow",
        "/lessons/..%2F..%2Fetc/lessons/passwd/x",
    ];

    for path in &traversal_paths {
        let app_clone = app.clone();
        let resp = get(app_clone, path).await;
        let status = resp.status();
        assert!(
            status == StatusCode::BAD_REQUEST || status == StatusCode::NOT_FOUND,
            "路径穿越 '{path}' 应被阻止（400 或 404），实际: {status}"
        );
    }
}

/// slug 包含特殊字符时正确处理
#[tokio::test]
async fn test_serve_lesson_special_chars_in_slug() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());

    let app = create_test_app(tmp.path());

    // 包含特殊字符的 slug — 有些在 URI 层就会被拒绝，有些能到路由层
    // 所有这些路径都不应成功返回 200
    let special_slugs = [
        "/lessons/prog/lessons/sub/%3Cscript%3Ealert(1)%3C%2Fscript%3E",
        "/lessons/prog/lessons/sub/les%20son",
        "/lessons/prog/lessons/sub/%00hidden",
        "/lessons/prog/lessons/sub/a%2Fb",
    ];

    for path in &special_slugs {
        let app_clone = app.clone();
        let req = Request::builder().uri(*path).body(Body::empty());
        match req {
            Ok(request) => {
                let resp = app_clone.oneshot(request).await.unwrap();
                let status = resp.status();
                // 不应成功返回内容（应为 400/404/500 中的某种错误码）
                assert_ne!(
                    status,
                    StatusCode::OK,
                    "特殊字符路径 '{path}' 不应返回 200"
                );
            }
            Err(_) => {
                // URI 构建失败也是可以接受的防御——无效字符在 HTTP 层被拒绝
            }
        }
    }
}

/// body 内容中的 <lk-*> 标签被保留（留给客户端 JS 处理）
#[tokio::test]
async fn test_serve_lesson_preserves_lk_tags() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());

    let lk_content = r#"<h2>Quiz</h2>
<lk-quiz data-type="multiple-choice">
    <lk-question>What is MDA?</lk-question>
    <lk-option correct>Mechanics, Dynamics, Aesthetics</lk-option>
    <lk-option>Model, Design, Architecture</lk-option>
</lk-quiz>
<lk-progress section="overview"></lk-progress>"#;

    setup_lesson_content(tmp.path(), "prog", "design", "quiz-lesson", lk_content);

    let app = create_test_app(tmp.path());
    let resp = get(app, "/lessons/prog/lessons/design/quiz-lesson").await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body = read_body_string(resp).await;

    assert!(
        body.contains("<lk-quiz"),
        "应保留 <lk-quiz> 自定义标签"
    );
    assert!(
        body.contains("<lk-question>"),
        "应保留 <lk-question> 自定义标签"
    );
    assert!(
        body.contains("<lk-option"),
        "应保留 <lk-option> 自定义标签"
    );
    assert!(
        body.contains("<lk-progress"),
        "应保留 <lk-progress> 自定义标签"
    );
}

/// 拼装的 HTML 中 API_BASE 变量被正确设置
#[tokio::test]
async fn test_serve_lesson_injects_api_base() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());
    setup_lesson_content(tmp.path(), "prog", "sub", "les", "<p>test</p>");

    let app = create_test_app(tmp.path());
    let resp = get(app, "/lessons/prog/lessons/sub/les").await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body = read_body_string(resp).await;

    assert!(
        body.contains("API_BASE"),
        "应包含 API_BASE 变量设置"
    );
    // API_BASE 占位符应该被替换为实际值，不应保留 {{api_base}} 原始占位符
    assert!(
        !body.contains("{{api_base}}"),
        "{{api_base}} 占位符应已被替换为实际值"
    );
}

// ============================================================
// 2. 静态资源伺服
// ============================================================

/// GET /static/lesson.css 返回 CSS 文件，Content-Type 正确
#[tokio::test]
async fn test_serve_lesson_css() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());

    let app = create_test_app(tmp.path());
    let resp = get(app, "/static/lesson.css").await;

    assert_eq!(resp.status(), StatusCode::OK, "应能获取 lesson.css");
    let ct = resp
        .headers()
        .get("content-type")
        .expect("缺少 content-type")
        .to_str()
        .unwrap();
    assert!(
        ct.contains("text/css"),
        "lesson.css 的 Content-Type 应为 text/css，实际: {ct}"
    );

    let body = read_body_string(resp).await;
    assert!(
        body.contains("font-family"),
        "CSS 文件应包含样式规则"
    );
}

/// GET /static/lesson.js 返回 JS 文件，Content-Type 正确
#[tokio::test]
async fn test_serve_lesson_js() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());

    let app = create_test_app(tmp.path());
    let resp = get(app, "/static/lesson.js").await;

    assert_eq!(resp.status(), StatusCode::OK, "应能获取 lesson.js");
    let ct = resp
        .headers()
        .get("content-type")
        .expect("缺少 content-type")
        .to_str()
        .unwrap();
    assert!(
        ct.contains("javascript"),
        "lesson.js 的 Content-Type 应包含 javascript，实际: {ct}"
    );

    let body = read_body_string(resp).await;
    assert!(
        body.contains("DOMContentLoaded"),
        "JS 文件应包含运行时逻辑"
    );
}

/// 请求不存在的静态文件返回 404
#[tokio::test]
async fn test_serve_static_not_found() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());

    let app = create_test_app(tmp.path());
    let resp = get(app, "/static/nonexistent.css").await;

    assert_eq!(
        resp.status(),
        StatusCode::NOT_FOUND,
        "不存在的静态文件应返回 404"
    );
}

/// 请求 /static/ 目录不返回文件列表
#[tokio::test]
async fn test_serve_static_no_directory_listing() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());

    let app = create_test_app(tmp.path());
    let resp = get(app, "/static/").await;

    let status = resp.status();
    let body = read_body_string(resp).await;

    // 不应返回目录列表（不应包含 lesson.css 和 lesson.js 的文件名列表）
    let lists_files = body.contains("lesson.css") && body.contains("lesson.js");
    assert!(
        !lists_files || status == StatusCode::NOT_FOUND,
        "/static/ 不应暴露目录列表"
    );
}

// ============================================================
// 3. Shell 模板验证
// ============================================================

/// shell.html 是有效的 HTML 文档
#[tokio::test]
async fn test_shell_template_is_valid_html() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());

    let template_path = tmp.path().join(TEMPLATES_DIR).join("shell.html");
    let content = std::fs::read_to_string(&template_path).unwrap();

    assert!(
        content.contains("<!DOCTYPE html>"),
        "shell.html 应以 DOCTYPE 声明开头"
    );
    assert!(
        content.contains("<html"),
        "应包含 <html> 标签"
    );
    assert!(
        content.contains("</html>"),
        "应包含闭合 </html>"
    );
    assert!(
        content.contains("<head>") || content.contains("<head "),
        "应包含 <head> 标签"
    );
    assert!(
        content.contains("</head>"),
        "应包含闭合 </head>"
    );
    assert!(
        content.contains("<body>") || content.contains("<body "),
        "应包含 <body> 标签"
    );
    assert!(
        content.contains("</body>"),
        "应包含闭合 </body>"
    );
}

/// shell.html 包含 {{content}} 占位符
#[tokio::test]
async fn test_shell_template_has_content_placeholder() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());

    let template_path = tmp.path().join(TEMPLATES_DIR).join("shell.html");
    let content = std::fs::read_to_string(&template_path).unwrap();

    assert!(
        content.contains("{{content}}"),
        "shell.html 应包含 {{{{content}}}} 占位符用于注入 lesson body"
    );
}

/// shell.html 包含主题切换功能
#[tokio::test]
async fn test_shell_template_has_theme_toggle() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());

    let template_path = tmp.path().join(TEMPLATES_DIR).join("shell.html");
    let content = std::fs::read_to_string(&template_path).unwrap();

    assert!(
        content.contains("theme-toggle") || content.contains("theme_toggle"),
        "shell.html 应包含主题切换元素（id='theme-toggle' 或类似标识）"
    );
}

/// shell.html 包含导航栏结构
#[tokio::test]
async fn test_shell_template_has_nav() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());

    let template_path = tmp.path().join(TEMPLATES_DIR).join("shell.html");
    let content = std::fs::read_to_string(&template_path).unwrap();

    assert!(
        content.contains("<nav") || content.contains("navbar"),
        "shell.html 应包含导航栏结构"
    );
}

// ============================================================
// 4. 与现有 API 的兼容性
// ============================================================

/// /api/health 等 API 不受影响
#[tokio::test]
async fn test_existing_api_still_works() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());
    setup_test_program(tmp.path(), "compat-test");

    let app = create_test_app(tmp.path());

    // /api/health 仍正常
    let resp = get(app.clone(), "/api/health").await;
    assert_eq!(resp.status(), StatusCode::OK, "/api/health 应正常返回 200");
    let body = parse_json(resp).await;
    assert_eq!(body["status"], "ok");

    // /api/programs 仍正常
    let resp = get(app.clone(), "/api/programs").await;
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "/api/programs 应正常返回 200"
    );
    let body = parse_json(resp).await;
    let programs = body.as_array().unwrap();
    assert!(
        !programs.is_empty(),
        "/api/programs 应返回已创建的 program"
    );

    // /api/programs/:slug/scope 仍正常
    let resp = get(app.clone(), "/api/programs/compat-test/scope").await;
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "/api/programs/:slug/scope 应正常返回 200"
    );
    let body = parse_json(resp).await;
    assert_eq!(body["program"], "compat-test");
}

/// 进度 API 正常工作
#[tokio::test]
async fn test_progress_api_still_works() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());
    setup_test_program(tmp.path(), "progress-test");

    let app = create_test_app(tmp.path());

    // GET /api/programs/:slug/progress — 无 DB 时应返回空进度
    let resp = get(app.clone(), "/api/programs/progress-test/progress").await;
    let status = resp.status();
    assert!(
        status == StatusCode::OK || status == StatusCode::NOT_FOUND,
        "/api/programs/:slug/progress 应正常工作，实际: {status}"
    );

    // GET /api/programs/:slug/lessons — 无 DB 时应返回空数组
    let resp = get(app.clone(), "/api/programs/progress-test/lessons").await;
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "/api/programs/:slug/lessons 应返回 200"
    );
    let body = parse_json(resp).await;
    assert!(
        body.as_array().unwrap().is_empty(),
        "无 lesson 时应返回空数组"
    );
}

// ============================================================
// 5. Backend 错误处理
// ============================================================

/// shell.html 不存在时返回 500 或合适的错误
#[tokio::test]
async fn test_serve_lesson_missing_shell_template() {
    let tmp = TempDir::new().unwrap();
    // 不调用 setup_shell_template，但创建 lesson 内容
    setup_lesson_content(tmp.path(), "prog", "sub", "les", "<p>content</p>");

    let app = create_test_app(tmp.path());
    let resp = get(app, "/lessons/prog/lessons/sub/les").await;

    let status = resp.status();
    assert!(
        status == StatusCode::INTERNAL_SERVER_ERROR || status == StatusCode::NOT_FOUND,
        "shell.html 不存在时应返回 500 或 404，实际: {status}"
    );
}

/// lesson 文件读取失败时返回适当错误
#[tokio::test]
async fn test_serve_lesson_content_read_failure() {
    let tmp = TempDir::new().unwrap();
    setup_shell_template(tmp.path());
    // 创建 lesson 目录但不创建文件，或创建一个无法读取的文件
    let lesson_dir = tmp.path().join("prog").join("lessons").join("sub");
    std::fs::create_dir_all(&lesson_dir).unwrap();
    // 不创建 les.html 文件 — 请求时应返回 404 而非 panic

    let app = create_test_app(tmp.path());
    let resp = get(app, "/lessons/prog/lessons/sub/les").await;

    let status = resp.status();
    assert!(
        status == StatusCode::NOT_FOUND || status == StatusCode::INTERNAL_SERVER_ERROR,
        "lesson 文件不存在时应返回 404 或 500，实际: {status}"
    );
}
