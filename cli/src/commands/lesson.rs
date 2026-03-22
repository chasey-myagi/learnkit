use anyhow::Result;
use std::fs;
use std::path::PathBuf;

use crate::config;
use crate::db;

/// Build the lesson file path: {program_root}/lessons/{subject}/{lesson}.html
fn lesson_file_path(program: &str, subject: &str, lesson: &str) -> PathBuf {
    config::program_root(program)
        .join("lessons")
        .join(subject)
        .join(format!("{}.html", lesson))
}

/// Build a lesson id: "subject/lesson"
fn lesson_id(subject: &str, lesson: &str) -> String {
    format!("{}/{}", subject, lesson)
}

/// Write a lesson body-only HTML file from content-file and register in DB.
///
/// The content file should contain a body fragment (not a full HTML page).
/// Template wrapping is handled at render time by the backend.
pub fn write(program: &str, subject: &str, lesson: &str, content_file: &str) -> Result<()> {
    // 1. Read the content file
    let raw = fs::read_to_string(content_file)
        .map_err(|e| anyhow::anyhow!("Failed to read content file '{}': {}", content_file, e))?;

    // 2. Strip UTF-8 BOM if present
    let content = raw.strip_prefix('\u{FEFF}').unwrap_or(&raw);

    // 3. Validate content
    validate_content(content)?;

    // 4. Create directory and write file (body only, no template)
    let file_path = lesson_file_path(program, subject, lesson);
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&file_path, content)?;

    // 5. Update DB
    let conn = db::open(program)?;
    let id = lesson_id(subject, lesson);
    let path_str = file_path.to_string_lossy().to_string();
    db::lessons::update_lesson_prepared(&conn, &id, &path_str)?;

    println!("Lesson written: {}", file_path.display());
    Ok(())
}

/// Validate that content is a proper body fragment.
///
/// Rules:
/// - Must not be empty or whitespace-only
/// - Must not contain `<html` (should be a body fragment, not a full page)
/// - Must not contain `<style` (styles are managed by the shell template)
/// - Must not contain `<script` (scripts are managed by the shell template)
/// - Must contain at least one `<h2` (at least one section heading)
pub fn validate_content(content: &str) -> Result<()> {
    if content.is_empty() {
        anyhow::bail!("Content is empty");
    }
    if content.trim().is_empty() {
        anyhow::bail!("Content is whitespace-only");
    }

    let lower = content.to_lowercase();
    if lower.contains("<html") {
        anyhow::bail!("Content should be a body fragment, not a full html page");
    }
    if lower.contains("<style") {
        anyhow::bail!("Content should not contain <style> tags; styles are managed by the shell template");
    }
    if lower.contains("<script") {
        anyhow::bail!("Content should not contain <script> tags; scripts are managed by the shell template");
    }
    if !lower.contains("<h2") {
        anyhow::bail!("Content must contain at least one <h2> section heading");
    }

    Ok(())
}

/// Apply template substitutions.
///
/// IMPORTANT: `{{content}}` must be replaced LAST because the content itself
/// may contain placeholder-like strings (e.g., `{{program}}`) that must not
/// be substituted by the template engine.
fn apply_template(
    template: &str,
    content: &str,
    title: &str,
    subject: &str,
    program: &str,
    lesson: &str,
) -> String {
    template
        .replace("{{lesson_title}}", title)
        .replace("{{subject_title}}", subject)
        .replace("{{program_title}}", program)
        .replace("{{program}}", program)
        .replace("{{subject}}", subject)
        .replace("{{lesson}}", lesson)
        .replace("{{prev_link}}", "")
        .replace("{{next_link}}", "")
        .replace("{{prev_title}}", "")
        .replace("{{next_title}}", "")
        .replace("{{api_base}}", &format!("/api/programs/{}", program))
        .replace("{{content}}", content)
}

/// Verify a lesson body-fragment file's integrity
pub fn verify(program: &str, subject: &str, lesson: &str) -> Result<()> {
    let file_path = lesson_file_path(program, subject, lesson);

    // Check file exists
    if !file_path.exists() {
        anyhow::bail!("FAIL: file not found: {}", file_path.display());
    }

    // Check file size > 0
    let metadata = fs::metadata(&file_path)?;
    if metadata.len() == 0 {
        anyhow::bail!("FAIL: file is empty: {}", file_path.display());
    }

    // Check contains <h2 (body fragment basic validation)
    let content = fs::read_to_string(&file_path)?;
    if !content.contains("<h2") {
        anyhow::bail!("FAIL: missing <h2> section heading");
    }

    println!("OK");
    Ok(())
}

/// List lessons from DB, optionally filtered by status, output as JSON
pub fn list(program: &str, status: Option<&str>) -> Result<()> {
    let conn = db::open(program)?;
    let lessons = db::lessons::list_lessons(&conn, status)?;
    let json = serde_json::to_string_pretty(&lessons)?;
    println!("{}", json);
    Ok(())
}

/// Open a lesson in the browser via the backend HTTP server
pub fn open(program: &str, subject: &str, lesson: &str) -> Result<()> {
    let url = format!(
        "http://localhost:13135/lessons/{}/lessons/{}/{}",
        program, subject, lesson
    );

    // Use `open` command on macOS
    let status = std::process::Command::new("open")
        .arg(&url)
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to open lesson in browser");
    }

    println!("Opened: {}", url);
    Ok(())
}

/// Get the next unfinished lesson
pub fn next(program: &str) -> Result<()> {
    let conn = db::open(program)?;

    // Try to find an in_progress or prepared lesson
    if let Some(lesson) = db::lessons::get_next_lesson(&conn)? {
        let json = serde_json::to_string_pretty(&lesson)?;
        println!("{}", json);
        return Ok(());
    }

    // No prepared lessons — check if there are pending ones
    let pending = db::lessons::get_pending_lessons(&conn, 3)?;
    if !pending.is_empty() {
        let ids: Vec<String> = pending.iter().map(|l| format!("  - {} ({})", l.id, l.title)).collect();
        anyhow::bail!(
            "No prepared lessons. {} pending lesson(s) need preparation:\n{}",
            pending.len(),
            ids.join("\n")
        );
    }

    eprintln!("All lessons completed!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_template_content_with_placeholder_text() {
        // Regression test for Issue 4: if lesson content contains {{program}},
        // it should NOT be replaced by template substitution.
        let template = "<html><body>{{content}}</body><script>var p='{{program}}';</script></html>";
        let content = "This lesson teaches you about {{program}} placeholder syntax.";

        let result = apply_template(template, content, "My Lesson", "math", "test-prog", "intro");

        // The content should be injected as-is, preserving {{program}} in the content
        assert!(
            result.contains("about {{program}} placeholder syntax"),
            "Content's {{{{program}}}} was wrongly replaced. Got: {}",
            result
        );
        // But the template's own {{program}} should be replaced
        assert!(
            result.contains("var p='test-prog'"),
            "Template's {{{{program}}}} was not replaced. Got: {}",
            result
        );
    }

    #[test]
    fn test_apply_template_basic() {
        let template = "<title>{{lesson_title}} — {{subject_title}}</title><div>{{content}}</div>";
        let result = apply_template(template, "Hello World", "My Lesson", "math", "prog", "intro");
        assert!(result.contains("<title>My Lesson — math</title>"));
        assert!(result.contains("<div>Hello World</div>"));
    }

    // ========================================================================
    // 以下为 lesson-write 重构的 TDD 测试
    // 重构目标：write() 不再注入模板，只存储 body 内容片段
    // ========================================================================

    /// 辅助函数：设置隔离的测试环境，返回 (tempdir, program, subject, lesson)
    /// 将 HOME 指向临时目录，使 config::program_root 在临时目录下工作
    fn setup_test_env() -> (tempfile::TempDir, String, String, String) {
        let dir = tempfile::tempdir().unwrap();
        std::env::set_var("HOME", dir.path());
        // 创建 cc/.learnkit 目录让 debug 模式使用
        std::fs::create_dir_all(dir.path().join("cc/.learnkit")).unwrap();
        let program = "test-prog".to_string();
        let subject = "math".to_string();
        let lesson = "intro-to-algebra".to_string();
        (dir, program, subject, lesson)
    }

    /// 辅助函数：创建一个临时内容文件并返回路径
    fn create_content_file(dir: &std::path::Path, content: &str) -> String {
        let content_path = dir.join("content.html");
        std::fs::write(&content_path, content).unwrap();
        content_path.to_string_lossy().to_string()
    }

    // --- write() 函数重构测试 ---

    #[test]
    fn test_write_stores_body_only() {
        // 重构后 write() 应该只存储 body 内容，不包含模板标签
        let (dir, program, subject, lesson) = setup_test_env();
        let body_content = r#"<h2 id="basics">Basics</h2>
<p>Learn algebra fundamentals.</p>
<lk-quiz data-id="q1">What is x+1=2?</lk-quiz>"#;
        let content_file = create_content_file(dir.path(), body_content);

        write(&program, &subject, &lesson, &content_file).unwrap();

        let output_path = lesson_file_path(&program, &subject, &lesson);
        let written = std::fs::read_to_string(&output_path).unwrap();

        // 不应包含模板标签
        assert!(!written.contains("<html"), "Output should not contain <html tag");
        assert!(!written.contains("</html>"), "Output should not contain </html> tag");
        assert!(!written.contains("<head"), "Output should not contain <head tag");
        assert!(!written.contains("<body"), "Output should not contain <body tag");
        assert!(!written.contains("</body>"), "Output should not contain </body> tag");
    }

    #[test]
    fn test_write_preserves_content_exactly() {
        // 写入的内容应与输入文件内容完全一致
        let (dir, program, subject, lesson) = setup_test_env();
        let body_content = r#"<h2 id="section-one">Section One</h2>
<p>Some content with <strong>formatting</strong>.</p>
<lk-code lang="python">
def hello():
    print("world")
</lk-code>
<h2 id="section-two">Section Two</h2>
<p>More content here.</p>"#;
        let content_file = create_content_file(dir.path(), body_content);

        write(&program, &subject, &lesson, &content_file).unwrap();

        let output_path = lesson_file_path(&program, &subject, &lesson);
        let written = std::fs::read_to_string(&output_path).unwrap();
        assert_eq!(written, body_content, "Written content must exactly match input");
    }

    #[test]
    fn test_write_creates_directory_structure() {
        // write() 应该自动创建 lessons/{subject}/ 目录
        let (dir, program, subject, lesson) = setup_test_env();
        let body_content = "<h2 id=\"intro\">Intro</h2>\n<p>Hello</p>";
        let content_file = create_content_file(dir.path(), body_content);

        let output_path = lesson_file_path(&program, &subject, &lesson);
        // 确认目录不存在
        assert!(!output_path.parent().unwrap().exists(), "Directory should not exist before write");

        write(&program, &subject, &lesson, &content_file).unwrap();

        // 目录和文件应该被创建
        assert!(output_path.parent().unwrap().exists(), "Directory should be created");
        assert!(output_path.exists(), "Lesson file should be created");
    }

    #[test]
    fn test_write_updates_db_status() {
        // 成功写入后 DB 中 lesson 状态应更新为 "prepared"
        let (dir, program, subject, lesson) = setup_test_env();
        let body_content = "<h2 id=\"intro\">Intro</h2>\n<p>Hello</p>";
        let content_file = create_content_file(dir.path(), body_content);

        write(&program, &subject, &lesson, &content_file).unwrap();

        let conn = db::open(&program).unwrap();
        let id = lesson_id(&subject, &lesson);
        let row = db::lessons::get_lesson(&conn, &id).unwrap();
        assert!(row.is_some(), "Lesson should exist in DB after write");
        assert_eq!(row.unwrap().status, "prepared", "Lesson status should be 'prepared'");
    }

    #[test]
    fn test_write_missing_content_file_error() {
        // 内容文件不存在时应返回错误
        let (_dir, program, subject, lesson) = setup_test_env();

        let result = write(&program, &subject, &lesson, "/nonexistent/path/content.html");
        assert!(result.is_err(), "Should error when content file does not exist");
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("content file") || err_msg.contains("not found") || err_msg.contains("No such file"),
            "Error should mention missing file, got: {}", err_msg
        );
    }

    #[test]
    fn test_write_empty_content_file_error() {
        // 重构后 write() 应拒绝空内容文件
        let (dir, program, subject, lesson) = setup_test_env();
        let content_file = create_content_file(dir.path(), "");

        let result = write(&program, &subject, &lesson, &content_file);
        assert!(result.is_err(), "Should error when content file is empty");
    }

    #[test]
    fn test_write_overwrites_existing_file() {
        // 重复写入应覆盖已有文件
        let (dir, program, subject, lesson) = setup_test_env();

        // 第一次写入
        let content_v1 = "<h2 id=\"v1\">Version 1</h2>\n<p>Original content</p>";
        let content_file_v1 = create_content_file(dir.path(), content_v1);
        write(&program, &subject, &lesson, &content_file_v1).unwrap();

        // 第二次写入不同内容
        let content_v2 = "<h2 id=\"v2\">Version 2</h2>\n<p>Updated content</p>";
        let content_file_v2 = create_content_file(dir.path(), content_v2);
        write(&program, &subject, &lesson, &content_file_v2).unwrap();

        let output_path = lesson_file_path(&program, &subject, &lesson);
        let written = std::fs::read_to_string(&output_path).unwrap();
        assert!(written.contains("Version 2"), "File should contain the newer content");
        assert!(!written.contains("Version 1"), "File should not contain the older content");
    }

    #[test]
    fn test_write_content_with_lk_tags() {
        // 包含 <lk-*> 自定义标签的内容应正确保存
        let (dir, program, subject, lesson) = setup_test_env();
        let body_content = r#"<h2 id="interactive">Interactive Section</h2>
<lk-quiz data-id="q1" data-type="multiple-choice">
  <lk-question>What is 2+2?</lk-question>
  <lk-option correct>4</lk-option>
  <lk-option>5</lk-option>
</lk-quiz>
<lk-code lang="rust" editable>
fn main() {
    println!("Hello!");
}
</lk-code>
<lk-note type="warning">Be careful with overflow.</lk-note>"#;
        let content_file = create_content_file(dir.path(), body_content);

        write(&program, &subject, &lesson, &content_file).unwrap();

        let output_path = lesson_file_path(&program, &subject, &lesson);
        let written = std::fs::read_to_string(&output_path).unwrap();
        assert!(written.contains("<lk-quiz"), "Should preserve lk-quiz tag");
        assert!(written.contains("<lk-code"), "Should preserve lk-code tag");
        assert!(written.contains("<lk-note"), "Should preserve lk-note tag");
        assert!(written.contains("<lk-question>"), "Should preserve lk-question tag");
        assert!(written.contains("<lk-option correct>"), "Should preserve lk-option tag with attributes");
    }

    // --- validate_content() 新函数测试 ---
    // 注意：validate_content() 目前还不存在，这些测试预期编译失败（TDD）

    #[test]
    fn test_validate_content_with_h2_sections() {
        // 包含 h2 标签的内容应通过验证
        let content = r#"<h2 id="basics">Basics</h2>
<p>Some content here.</p>
<h2 id="advanced">Advanced</h2>
<p>More content.</p>"#;
        let result = validate_content(content);
        assert!(result.is_ok(), "Content with h2 sections should pass validation");
    }

    #[test]
    fn test_validate_content_without_h2_fails() {
        // 不包含任何 h2 标签的内容应验证失败
        let content = "<p>Just a paragraph without any sections.</p>";
        let result = validate_content(content);
        assert!(result.is_err(), "Content without h2 should fail validation");
    }

    #[test]
    fn test_validate_content_empty_fails() {
        // 空内容应验证失败
        let result = validate_content("");
        assert!(result.is_err(), "Empty content should fail validation");
    }

    #[test]
    fn test_validate_content_with_only_whitespace_fails() {
        // 只有空白字符的内容应验证失败
        let result = validate_content("   \n\t\n   ");
        assert!(result.is_err(), "Whitespace-only content should fail validation");
    }

    #[test]
    fn test_validate_rejects_full_html_page() {
        // 包含 <html> 标签的完整页面应被拒绝（body 片段不应包含文档级标签）
        let content = r#"<!DOCTYPE html>
<html lang="en">
<head><title>Test</title></head>
<body>
<h2 id="intro">Intro</h2>
<p>Content</p>
</body>
</html>"#;
        let result = validate_content(content);
        assert!(result.is_err(), "Full HTML page should be rejected");
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.to_lowercase().contains("html") || err_msg.contains("body fragment"),
            "Error should mention html/body fragment issue, got: {}", err_msg
        );
    }

    #[test]
    fn test_validate_rejects_style_tags() {
        // 包含 <style> 标签的内容应被拒绝（样式由模板统一管理）
        let content = r#"<style>h2 { color: red; }</style>
<h2 id="intro">Intro</h2>
<p>Content</p>"#;
        let result = validate_content(content);
        assert!(result.is_err(), "Content with <style> tag should be rejected");
    }

    #[test]
    fn test_validate_rejects_script_tags() {
        // 包含 <script> 标签的内容应被拒绝（脚本由模板统一管理）
        let content = r#"<h2 id="intro">Intro</h2>
<p>Content</p>
<script>alert('xss')</script>"#;
        let result = validate_content(content);
        assert!(result.is_err(), "Content with <script> tag should be rejected");
    }

    // --- verify() 函数更新测试 ---

    #[test]
    fn test_verify_body_only_file() {
        // 重构后 verify() 应该接受 body-only 文件（不再要求 <html> 标签）
        let (_dir, program, subject, lesson) = setup_test_env();

        // 创建一个 body-only 文件
        let output_path = lesson_file_path(&program, &subject, &lesson);
        std::fs::create_dir_all(output_path.parent().unwrap()).unwrap();
        let body_content = "<h2 id=\"intro\">Intro</h2>\n<p>Body only content.</p>";
        std::fs::write(&output_path, body_content).unwrap();

        let result = verify(&program, &subject, &lesson);
        assert!(result.is_ok(), "Body-only file should pass verification, got: {:?}", result.err());
    }

    #[test]
    fn test_verify_missing_file_fails() {
        // 文件不存在时验证应失败
        let (_dir, program, subject, lesson) = setup_test_env();

        let result = verify(&program, &subject, &lesson);
        assert!(result.is_err(), "Missing file should fail verification");
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("not found") || err_msg.contains("FAIL"),
            "Error should indicate file not found, got: {}", err_msg);
    }

    #[test]
    fn test_verify_empty_file_fails() {
        // 空文件验证应失败
        let (_dir, program, subject, lesson) = setup_test_env();

        let output_path = lesson_file_path(&program, &subject, &lesson);
        std::fs::create_dir_all(output_path.parent().unwrap()).unwrap();
        std::fs::write(&output_path, "").unwrap();

        let result = verify(&program, &subject, &lesson);
        assert!(result.is_err(), "Empty file should fail verification");
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("empty") || err_msg.contains("FAIL"),
            "Error should indicate empty file, got: {}", err_msg);
    }

    // --- CLI 边界测试 ---

    #[test]
    fn test_write_content_with_utf8_bom() {
        // content 文件含 UTF-8 BOM 头时应正确处理（去除 BOM 或正常工作）
        let (dir, program, subject, lesson) = setup_test_env();

        // UTF-8 BOM = EF BB BF
        let bom = b"\xEF\xBB\xBF";
        let body_content = b"<h2 id=\"intro\">Intro</h2>\n<p>Content with BOM</p>";
        let mut content_with_bom = Vec::new();
        content_with_bom.extend_from_slice(bom);
        content_with_bom.extend_from_slice(body_content);

        let content_path = dir.path().join("content_bom.html");
        std::fs::write(&content_path, &content_with_bom).unwrap();
        let content_file = content_path.to_string_lossy().to_string();

        let result = write(&program, &subject, &lesson, &content_file);
        // 应该成功写入，不因 BOM 而失败
        assert!(result.is_ok(), "Content with UTF-8 BOM should be handled, got: {:?}", result.err());

        let output_path = lesson_file_path(&program, &subject, &lesson);
        let written = std::fs::read_to_string(&output_path).unwrap();
        // 写入的内容应包含实际内容
        assert!(
            written.contains("Content with BOM"),
            "Written content should contain the actual text"
        );
    }

    #[test]
    fn test_slug_with_chinese_characters() {
        // subject/lesson slug 含中文字符时的行为
        let (dir, program, _subject, _lesson) = setup_test_env();
        let chinese_subject = "数学基础";
        let chinese_lesson = "代数入门";

        let body_content = "<h2 id=\"intro\">代数入门</h2>\n<p>学习基础代数。</p>";
        let content_file = create_content_file(dir.path(), body_content);

        let result = write(&program, chinese_subject, chinese_lesson, &content_file);
        // 中文 slug 应该能正常工作（创建目录和文件），或者返回明确的错误
        if result.is_ok() {
            let output_path = lesson_file_path(&program, chinese_subject, chinese_lesson);
            assert!(output_path.exists(), "File with Chinese slug should be created");
            let written = std::fs::read_to_string(&output_path).unwrap();
            assert!(written.contains("代数入门"), "Content should be preserved with Chinese slug");
        }
        // 如果返回错误，确保是有意义的错误消息而非 panic
    }
}
