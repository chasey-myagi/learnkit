use rusqlite::Connection;
use anyhow::Result;

pub fn ensure_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS lessons (
            id TEXT PRIMARY KEY,
            subject TEXT NOT NULL,
            lesson TEXT NOT NULL,
            title TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            file_path TEXT,
            prepared_at TEXT,
            started_at TEXT,
            completed_at TEXT
        );

        CREATE TABLE IF NOT EXISTS sections (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            lesson_id TEXT NOT NULL REFERENCES lessons(id),
            title TEXT NOT NULL,
            read INTEGER NOT NULL DEFAULT 0,
            read_at TEXT
        );

        CREATE TABLE IF NOT EXISTS qa_history (
            id TEXT PRIMARY KEY,
            lesson_id TEXT NOT NULL REFERENCES lessons(id),
            selection TEXT NOT NULL,
            question TEXT NOT NULL,
            answer TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS resources (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            url TEXT NOT NULL,
            type TEXT NOT NULL,
            local_path TEXT,
            description TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );
    ")?;
    Ok(())
}
