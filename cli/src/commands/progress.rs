use anyhow::Result;

use crate::db;

/// Show learning progress as JSON
pub fn show(program: &str) -> Result<()> {
    let conn = db::open(program)?;

    let status_counts = db::lessons::count_by_status(&conn)?;
    let (sections_read, sections_total) = db::sections::get_section_progress(&conn)?;

    let progress = serde_json::json!({
        "lessons": status_counts,
        "sections": {
            "read": sections_read,
            "total": sections_total,
        }
    });

    let json = serde_json::to_string_pretty(&progress)?;
    println!("{}", json);
    Ok(())
}

/// Update lesson progress status
pub fn update(program: &str, subject: &str, lesson: &str, status: &str) -> Result<()> {
    let conn = db::open(program)?;
    let id = format!("{}/{}", subject, lesson);

    // Validate status
    match status {
        "pending" | "prepared" | "in_progress" | "completed" => {}
        _ => anyhow::bail!("Invalid status '{}'. Must be one of: pending, prepared, in_progress, completed", status),
    }

    db::lessons::update_lesson_status(&conn, &id, status)?;

    println!("Updated {}: status = {}", id, status);
    Ok(())
}

/// Check if more lessons need to be prepared
pub fn check_prepare(program: &str) -> Result<()> {
    let conn = db::open(program)?;

    let ready_count = db::lessons::count_prepared_unfinished(&conn)?;

    if ready_count <= 1 {
        // Need more lessons to be prepared
        let pending = db::lessons::get_pending_lessons(&conn, 5)?;
        println!("NEED_PREPARE");
        if !pending.is_empty() {
            println!("Next pending lessons:");
            for l in &pending {
                println!("  {} - {}", l.id, l.title);
            }
        } else {
            println!("No more pending lessons.");
        }
        std::process::exit(1);
    }

    println!("OK");
    Ok(())
}
