use std::path::PathBuf;
use anyhow::Result;

pub fn learnkit_root() -> PathBuf {
    // Dev environment: ~/cc/.learnkit/
    // Production: ~/.learnkit/
    let home = dirs_or_fallback();
    if home.join("cc/.learnkit").exists() || cfg!(debug_assertions) {
        home.join("cc/.learnkit")
    } else {
        home.join(".learnkit")
    }
}

pub fn program_root(slug: &str) -> PathBuf {
    learnkit_root().join(slug)
}

pub fn ensure_root() -> Result<PathBuf> {
    let root = learnkit_root();
    if !root.exists() {
        std::fs::create_dir_all(&root)?;
    }
    Ok(root)
}

fn dirs_or_fallback() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}
