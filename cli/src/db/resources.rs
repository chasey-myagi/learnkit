use rusqlite::Connection;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResourceRow {
    pub id: i64,
    pub url: String,
    pub r#type: String,
    pub local_path: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
}

pub fn insert_resource(conn: &Connection, url: &str, r#type: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO resources (url, type) VALUES (?1, ?2)",
        rusqlite::params![url, r#type],
    )?;
    Ok(())
}

pub fn list_resources(conn: &Connection) -> Result<Vec<ResourceRow>> {
    let mut stmt = conn.prepare("SELECT id, url, type, local_path, description, created_at FROM resources ORDER BY created_at DESC")?;
    let rows = stmt.query_map([], |row| {
        Ok(ResourceRow {
            id: row.get(0)?, url: row.get(1)?, r#type: row.get(2)?,
            local_path: row.get(3)?, description: row.get(4)?, created_at: row.get(5)?,
        })
    })?.collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}
