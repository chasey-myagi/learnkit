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

    // 3. Wrap in simple HTML template
    let html = format!(
        "<html>\n<head><title>{title}</title></head>\n<body>\n{content}\n</body>\n</html>"
    );

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

/// Verify a lesson HTML file's integrity
pub fn verify(program: &str, subject: &str, lesson: &str) -> Result<()> {
    let file_path = lesson_file_path(program, subject, lesson);

    // Check file exists
    if !file_path.exists() {
        eprintln!("FAIL: file not found: {}", file_path.display());
        std::process::exit(1);
    }

    // Check file size > 0
    let metadata = fs::metadata(&file_path)?;
    if metadata.len() == 0 {
        eprintln!("FAIL: file is empty: {}", file_path.display());
        std::process::exit(1);
    }

    // Check contains <html and </html> tags
    let content = fs::read_to_string(&file_path)?;
    if !content.contains("<html") {
        eprintln!("FAIL: missing <html tag");
        std::process::exit(1);
    }
    if !content.contains("</html>") {
        eprintln!("FAIL: missing </html> tag");
        std::process::exit(1);
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

/// Open a lesson HTML file in the browser
pub fn open(program: &str, subject: &str, lesson: &str) -> Result<()> {
    let file_path = lesson_file_path(program, subject, lesson);

    if !file_path.exists() {
        anyhow::bail!("Lesson file not found: {}", file_path.display());
    }

    // Use `open` command on macOS
    let status = std::process::Command::new("open")
        .arg(&file_path)
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to open lesson in browser");
    }

    println!("Opened: {}", file_path.display());
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
        eprintln!("No prepared lessons. {} pending lesson(s) need preparation:", pending.len());
        for l in &pending {
            eprintln!("  - {} ({})", l.id, l.title);
        }
        std::process::exit(1);
    }

    eprintln!("All lessons completed!");
    Ok(())
}
