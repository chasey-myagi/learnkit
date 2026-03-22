use anyhow::Result;

pub fn add(program: &str, url: &str, r#type: &str) -> Result<()> {
    // Validate type
    match r#type {
        "doc" | "repo" | "pdf" => {}
        other => anyhow::bail!("Unknown resource type '{}'. Supported: doc, repo, pdf", other),
    }

    // Insert into DB (no actual download yet)
    let conn = crate::db::open(program)?;
    crate::db::resources::insert_resource(&conn, url, r#type)?;

    println!("OK: resource added [{}] {}", r#type, url);
    Ok(())
}

pub fn list(program: &str) -> Result<()> {
    let conn = crate::db::open(program)?;
    let rows = crate::db::resources::list_resources(&conn)?;
    println!("{}", serde_json::to_string_pretty(&rows)?);
    Ok(())
}
