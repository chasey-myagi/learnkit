use anyhow::Result;
use crate::config;
use crate::scope;
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
struct ProgramEntry {
    slug: String,
    title: String,
    path: String,
}

pub fn run() -> Result<()> {
    let root = config::learnkit_root();
    if !root.exists() {
        println!("[]");
        return Ok(());
    }

    let mut programs = Vec::new();

    for entry in fs::read_dir(&root)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let scope_path = path.join("scope.md");
        if !scope_path.exists() {
            continue;
        }
        let slug = entry.file_name().to_string_lossy().to_string();
        let title = match fs::read_to_string(&scope_path) {
            Ok(content) => match scope::parse_scope(&content) {
                Ok(s) => s.title,
                Err(_) => String::new(),
            },
            Err(_) => String::new(),
        };
        programs.push(ProgramEntry {
            slug,
            title,
            path: path.to_string_lossy().to_string(),
        });
    }

    programs.sort_by(|a, b| a.slug.cmp(&b.slug));
    let json = serde_json::to_string_pretty(&programs)?;
    println!("{}", json);
    Ok(())
}
