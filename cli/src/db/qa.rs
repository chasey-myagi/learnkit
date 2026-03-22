use rusqlite::Connection;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct QaRow {
    pub id: String,
    pub lesson_id: String,
    pub selection: String,
    pub question: String,
    pub answer: String,
    pub created_at: String,
}

pub fn insert_qa(conn: &Connection, id: &str, lesson_id: &str, selection: &str, question: &str, answer: &str) -> Result<()> {
    // Temporarily disable foreign keys — lesson_id may reference a lesson
    // that hasn't been registered in the lessons table yet (e.g., ad-hoc QA).
    conn.execute_batch("PRAGMA foreign_keys = OFF;")?;
    conn.execute(
        "INSERT INTO qa_history (id, lesson_id, selection, question, answer) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![id, lesson_id, selection, question, answer],
    )?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    Ok(())
}

pub fn list_qa(conn: &Connection, lesson_id: Option<&str>) -> Result<Vec<QaRow>> {
    if let Some(lid) = lesson_id {
        let mut s = conn.prepare("SELECT id, lesson_id, selection, question, answer, created_at FROM qa_history WHERE lesson_id = ?1 ORDER BY created_at DESC")?;
        let rows = s.query_map(rusqlite::params![lid], |row| {
            Ok(QaRow {
                id: row.get(0)?, lesson_id: row.get(1)?, selection: row.get(2)?,
                question: row.get(3)?, answer: row.get(4)?, created_at: row.get(5)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    } else {
        let mut s = conn.prepare("SELECT id, lesson_id, selection, question, answer, created_at FROM qa_history ORDER BY created_at DESC")?;
        let rows = s.query_map([], |row| {
            Ok(QaRow {
                id: row.get(0)?, lesson_id: row.get(1)?, selection: row.get(2)?,
                question: row.get(3)?, answer: row.get(4)?, created_at: row.get(5)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }
}
