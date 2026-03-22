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

/// Write a lesson HTML file from content-file, wrap in template, and register in DB
pub fn write(program: &str, subject: &str, lesson: &str, content_file: &str) -> Result<()> {
    // 1. Read the content file
    let content = fs::read_to_string(content_file)
        .map_err(|e| anyhow::anyhow!("Failed to read content file '{}': {}", content_file, e))?;

    // 2. Build title from lesson slug
    let title = lesson.replace('-', " ").replace('_', " ");

    // 3. Load template: LEARNKIT_TEMPLATE_DIR override > embedded template
    const EMBEDDED_TEMPLATE: &str = include_str!("../../templates/_template.html");

    let html = {
        let template = if let Ok(dir) = std::env::var("LEARNKIT_TEMPLATE_DIR") {
            let override_path = std::path::Path::new(&dir).join("_template.html");
            if override_path.exists() {
                fs::read_to_string(&override_path)?
            } else {
                EMBEDDED_TEMPLATE.to_string()
            }
        } else {
            EMBEDDED_TEMPLATE.to_string()
        };
        apply_template(&template, &content, &title, subject, program, lesson)
    };

    // 4. Create directory and write file
    let file_path = lesson_file_path(program, subject, lesson);
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&file_path, &html)?;

    // 5. Update DB
    let conn = db::open(program)?;
    let id = lesson_id(subject, lesson);
    let path_str = file_path.to_string_lossy().to_string();
    db::lessons::update_lesson_prepared(&conn, &id, &path_str)?;

    println!("Lesson written: {}", file_path.display());
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

/// Verify a lesson HTML file's integrity
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

    // Check contains <html and </html> tags
    let content = fs::read_to_string(&file_path)?;
    if !content.contains("<html") {
        anyhow::bail!("FAIL: missing <html tag");
    }
    if !content.contains("</html>") {
        anyhow::bail!("FAIL: missing </html> tag");
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
        "http://localhost:13135/lessons/{}/lessons/{}/{}.html",
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
}
