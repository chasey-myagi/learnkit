use rusqlite::Connection;
use anyhow::Result;
use serde::Serialize;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct SectionRow {
    pub id: i64,
    pub lesson_id: String,
    pub title: String,
    pub read: bool,
    pub read_at: Option<String>,
}

pub fn insert_sections(conn: &Connection, lesson_id: &str, titles: &[String]) -> Result<()> {
    // Delete existing sections for this lesson, then re-insert
    conn.execute(
        "DELETE FROM sections WHERE lesson_id = ?1",
        rusqlite::params![lesson_id],
    )?;
    let mut stmt = conn.prepare(
        "INSERT INTO sections (lesson_id, title) VALUES (?1, ?2)"
    )?;
    for title in titles {
        stmt.execute(rusqlite::params![lesson_id, title])?;
    }
    Ok(())
}

/// Mark a section as read
#[allow(dead_code)]
pub fn mark_section_read(conn: &Connection, lesson_id: &str, title: &str) -> Result<()> {
    let affected = conn.execute(
        "UPDATE sections SET read = 1, read_at = datetime('now') WHERE lesson_id = ?1 AND title = ?2",
        [lesson_id, title],
    )?;

    if affected == 0 {
        anyhow::bail!("Section '{}' not found in lesson '{}'", title, lesson_id);
    }
    Ok(())
}

/// Get all sections for a lesson
#[allow(dead_code)]
pub fn get_sections_for_lesson(conn: &Connection, lesson_id: &str) -> Result<Vec<SectionRow>> {
    let mut stmt = conn.prepare(
        "SELECT id, lesson_id, title, read, read_at FROM sections WHERE lesson_id = ?1 ORDER BY id"
    )?;

    let rows = stmt.query_map([lesson_id], |row| {
        Ok(SectionRow {
            id: row.get(0)?,
            lesson_id: row.get(1)?,
            title: row.get(2)?,
            read: row.get::<_, i32>(3)? != 0,
            read_at: row.get(4)?,
        })
    })?.collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(rows)
}

/// Get section reading progress: (read_count, total_count)
pub fn get_section_progress(conn: &Connection) -> Result<(i64, i64)> {
    let total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sections",
        [],
        |row| row.get(0),
    )?;
    let read: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sections WHERE read = 1",
        [],
        |row| row.get(0),
    )?;
    Ok((read, total))
}
