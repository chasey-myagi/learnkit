use anyhow::Result;
use crate::config;
use crate::scope;
use crate::db;
use std::fs;

pub fn write(program: &str, file: &str) -> Result<()> {
    // 1. Read the source file
    let content = fs::read_to_string(file)?;

    // 2. Parse and validate YAML frontmatter
    let parsed = scope::parse_scope(&content)?;

    // 3. Copy to {program_root}/scope.md
    let program_root = config::program_root(program);
    if !program_root.exists() {
        anyhow::bail!("Program '{}' does not exist. Run `init` first.", program);
    }
    let scope_path = program_root.join("scope.md");
    fs::write(&scope_path, &content)?;

    // 4. Sync lessons and sections to DB
    let conn = db::open(program)?;
    for subject in &parsed.subjects {
        for lesson in &subject.lessons {
            let lesson_id = format!("{}/{}", subject.slug, lesson.slug);
            db::lessons::insert_lesson(&conn, &lesson_id, &subject.slug, &lesson.slug, &lesson.title)?;
            // 5. Sync sections
            db::sections::insert_sections(&conn, &lesson_id, &lesson.sections)?;
        }
    }

    // 6. Output success
    let total_lessons: usize = parsed.subjects.iter().map(|s| s.lessons.len()).sum();
    let total_sections: usize = parsed.subjects.iter()
        .flat_map(|s| &s.lessons)
        .map(|l| l.sections.len())
        .sum();
    println!("Scope written for '{}': {} subjects, {} lessons, {} sections",
        program, parsed.subjects.len(), total_lessons, total_sections);
    Ok(())
}

pub fn read(program: &str) -> Result<()> {
    let scope_path = config::program_root(program).join("scope.md");
    if !scope_path.exists() {
        anyhow::bail!("No scope.md found for program '{}'", program);
    }
    let content = fs::read_to_string(&scope_path)?;
    let parsed = scope::parse_scope(&content)?;
    let json = serde_json::to_string_pretty(&parsed)?;
    println!("{}", json);
    Ok(())
}
