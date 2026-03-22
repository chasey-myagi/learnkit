use anyhow::Result;
use crate::config;
use crate::scope;
use crate::db;
use serde::Serialize;
use std::fs;
use std::collections::HashMap;

#[derive(Serialize)]
struct ProgramInfo {
    program: String,
    title: String,
    created: String,
    subjects_count: usize,
    lessons_count: usize,
    progress: HashMap<String, i64>,
}

pub fn run(program: &str) -> Result<()> {
    let scope_path = config::program_root(program).join("scope.md");
    if !scope_path.exists() {
        anyhow::bail!("No scope.md found for program '{}'", program);
    }
    let content = fs::read_to_string(&scope_path)?;
    let parsed = scope::parse_scope(&content)?;

    let total_lessons: usize = parsed.subjects.iter().map(|s| s.lessons.len()).sum();

    // Read progress from DB
    let progress = match db::open(program) {
        Ok(conn) => db::lessons::count_by_status(&conn).unwrap_or_default(),
        Err(_) => HashMap::new(),
    };

    let info = ProgramInfo {
        program: parsed.program,
        title: parsed.title,
        created: parsed.created,
        subjects_count: parsed.subjects.len(),
        lessons_count: total_lessons,
        progress,
    };

    let json = serde_json::to_string_pretty(&info)?;
    println!("{}", json);
    Ok(())
}
