use anyhow::Result;
use crate::config;
use std::fs;

pub fn run(slug: &str) -> Result<()> {
    let root = config::program_root(slug);

    if root.exists() {
        anyhow::bail!("Program '{}' already exists at {}", slug, root.display());
    }

    // Create directory structure
    fs::create_dir_all(root.join("lessons"))?;
    fs::create_dir_all(root.join("resources/docs"))?;
    fs::create_dir_all(root.join("resources/repos"))?;
    fs::create_dir_all(root.join("answers"))?;

    // Create empty scope.md
    fs::write(
        root.join("scope.md"),
        format!(
            "---\nprogram: {slug}\ntitle: \ncreated: \nsubjects: []\n---\n"
        ),
    )?;

    // Create resources index
    fs::write(
        root.join("resources/index.md"),
        "# 教学资源\n\n暂无资源。\n",
    )?;

    // Initialize database
    let _conn = crate::db::open(slug)?;

    println!("Program '{}' initialized at {}", slug, root.display());
    Ok(())
}
