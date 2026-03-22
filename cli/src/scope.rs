use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Scope {
    pub program: String,
    pub title: String,
    pub created: String,
    pub difficulty: Option<String>,
    pub subjects: Vec<Subject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject {
    pub slug: String,
    pub title: String,
    pub lessons: Vec<Lesson>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lesson {
    pub slug: String,
    pub title: String,
    pub sections: Vec<String>,
}

pub fn parse_scope(content: &str) -> anyhow::Result<Scope> {
    // Split on --- delimiters, parse YAML frontmatter
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        anyhow::bail!("Invalid scope.md: missing YAML frontmatter");
    }
    let yaml = parts[1].trim();
    let scope: Scope = serde_yaml::from_str(yaml)?;
    Ok(scope)
}
