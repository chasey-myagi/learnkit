use rusqlite::Connection;
use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct LessonRow {
    pub id: String,
    pub subject: String,
    pub lesson: String,
    pub title: String,
    pub status: String,
}

pub fn insert_lesson(conn: &Connection, id: &str, subject: &str, lesson: &str, title: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO lessons (id, subject, lesson, title) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![id, subject, lesson, title],
    )?;
    Ok(())
}

pub fn list_lessons(conn: &Connection, status: Option<&str>) -> Result<Vec<LessonRow>> {
    let mut rows = Vec::new();
    match status {
        Some(s) => {
            let mut stmt = conn.prepare(
                "SELECT id, subject, lesson, title, status FROM lessons WHERE status = ?1 ORDER BY id"
            )?;
            let iter = stmt.query_map(rusqlite::params![s], |row| {
                Ok(LessonRow {
                    id: row.get(0)?,
                    subject: row.get(1)?,
                    lesson: row.get(2)?,
                    title: row.get(3)?,
                    status: row.get(4)?,
                })
            })?;
            for r in iter {
                rows.push(r?);
            }
        }
        None => {
            let mut stmt = conn.prepare(
                "SELECT id, subject, lesson, title, status FROM lessons ORDER BY id"
            )?;
            let iter = stmt.query_map([], |row| {
                Ok(LessonRow {
                    id: row.get(0)?,
                    subject: row.get(1)?,
                    lesson: row.get(2)?,
                    title: row.get(3)?,
                    status: row.get(4)?,
                })
            })?;
            for r in iter {
                rows.push(r?);
            }
        }
    }
    Ok(rows)
}

pub fn count_by_status(conn: &Connection) -> Result<HashMap<String, i64>> {
    let mut map = HashMap::new();
    let mut stmt = conn.prepare("SELECT status, COUNT(*) FROM lessons GROUP BY status")?;
    let iter = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
    })?;
    for r in iter {
        let (status, count) = r?;
        map.insert(status, count);
    }
    Ok(map)
}

/// Get a single lesson by id (format: "subject/lesson")
#[allow(dead_code)]
pub fn get_lesson(conn: &Connection, id: &str) -> Result<Option<LessonRow>> {
    let mut stmt = conn.prepare(
        "SELECT id, subject, lesson, title, status FROM lessons WHERE id = ?1"
    )?;

    let row = stmt.query_row([id], |row| {
        Ok(LessonRow {
            id: row.get(0)?,
            subject: row.get(1)?,
            lesson: row.get(2)?,
            title: row.get(3)?,
            status: row.get(4)?,
        })
    });

    match row {
        Ok(r) => Ok(Some(r)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// Update lesson status with appropriate timestamp
pub fn update_lesson_status(conn: &Connection, id: &str, status: &str) -> Result<()> {
    let affected = match status {
        "in_progress" => {
            conn.execute(
                "UPDATE lessons SET status = ?1, started_at = datetime('now') WHERE id = ?2",
                [status, id],
            )?
        }
        "completed" => {
            conn.execute(
                "UPDATE lessons SET status = ?1, completed_at = datetime('now') WHERE id = ?2",
                [status, id],
            )?
        }
        _ => {
            conn.execute(
                "UPDATE lessons SET status = ?1 WHERE id = ?2",
                [status, id],
            )?
        }
    };

    if affected == 0 {
        anyhow::bail!("Lesson '{}' not found", id);
    }
    Ok(())
}

/// Mark a lesson as prepared with its file path.
/// If the lesson doesn't exist, insert it.
pub fn update_lesson_prepared(conn: &Connection, id: &str, file_path: &str) -> Result<()> {
    let affected = conn.execute(
        "UPDATE lessons SET status = 'prepared', file_path = ?1, prepared_at = datetime('now') WHERE id = ?2",
        [file_path, id],
    )?;

    if affected == 0 {
        // Lesson doesn't exist yet — insert it
        let parts: Vec<&str> = id.splitn(2, '/').collect();
        let (subject, lesson) = if parts.len() == 2 {
            (parts[0], parts[1])
        } else {
            (id, id)
        };

        conn.execute(
            "INSERT INTO lessons (id, subject, lesson, title, status, file_path, prepared_at)
             VALUES (?1, ?2, ?3, ?4, 'prepared', ?5, datetime('now'))",
            rusqlite::params![id, subject, lesson, lesson, file_path],
        )?;
    }

    Ok(())
}

/// Get the next lesson to study: first 'in_progress', then first 'prepared'
pub fn get_next_lesson(conn: &Connection) -> Result<Option<LessonRow>> {
    let mut stmt = conn.prepare(
        "SELECT id, subject, lesson, title, status
         FROM lessons
         WHERE status IN ('in_progress', 'prepared')
         ORDER BY
           CASE status WHEN 'in_progress' THEN 0 WHEN 'prepared' THEN 1 END,
           id
         LIMIT 1"
    )?;

    let row = stmt.query_row([], |row| {
        Ok(LessonRow {
            id: row.get(0)?,
            subject: row.get(1)?,
            lesson: row.get(2)?,
            title: row.get(3)?,
            status: row.get(4)?,
        })
    });

    match row {
        Ok(r) => Ok(Some(r)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// Count lessons that are ready but not yet completed
pub fn count_prepared_unfinished(conn: &Connection) -> Result<i64> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM lessons WHERE status IN ('prepared', 'in_progress')",
        [],
        |row| row.get(0),
    )?;
    Ok(count)
}

/// Get pending lessons (not yet prepared), limited
pub fn get_pending_lessons(conn: &Connection, limit: i64) -> Result<Vec<LessonRow>> {
    let mut stmt = conn.prepare(
        "SELECT id, subject, lesson, title, status
         FROM lessons WHERE status = 'pending' ORDER BY id LIMIT ?1"
    )?;

    let rows = stmt.query_map([limit], |row| {
        Ok(LessonRow {
            id: row.get(0)?,
            subject: row.get(1)?,
            lesson: row.get(2)?,
            title: row.get(3)?,
            status: row.get(4)?,
        })
    })?.collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(rows)
}
