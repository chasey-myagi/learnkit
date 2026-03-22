use anyhow::Result;
use crate::config;
use std::fs;

pub fn write(
    program: &str,
    request_id: &str,
    lesson: &str,
    selection: &str,
    question: &str,
    answer: &str,
) -> Result<()> {
    let root = config::program_root(program);
    let answers_dir = root.join("answers");

    // 1. Create answers/ directory if not exists
    if !answers_dir.exists() {
        fs::create_dir_all(&answers_dir)?;
    }

    // 2. Build answer JSON
    let answer_json = serde_json::json!({
        "request_id": request_id,
        "program": program,
        "lesson": lesson,
        "selection": selection,
        "question": question,
        "answer": answer,
    });

    // 3. Write to answers/{request_id}.json
    let file_path = answers_dir.join(format!("{}.json", request_id));
    fs::write(&file_path, serde_json::to_string_pretty(&answer_json)?)?;

    // 4. Insert into DB qa_history
    let conn = crate::db::open(program)?;
    crate::db::qa::insert_qa(&conn, request_id, lesson, selection, question, answer)?;

    // 5. Output confirmation
    println!("OK: answer written to {}", file_path.display());
    Ok(())
}

pub fn history(program: &str, lesson: Option<&str>) -> Result<()> {
    let conn = crate::db::open(program)?;
    let rows = crate::db::qa::list_qa(&conn, lesson)?;
    println!("{}", serde_json::to_string_pretty(&rows)?);
    Ok(())
}
