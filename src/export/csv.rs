use crate::types::AuditResults;
use anyhow::Result;
use std::fs;

pub fn export(results: &AuditResults, path: &str) -> Result<()> {
    let mut csv = String::from("Category,Check,Status,Detail\n");
    
    for check in &results.checks {
        csv.push_str(&format!(
            "\"{}\",\"{}\",\"{:?}\",\"{}\"\n",
            check.category,
            check.name,
            check.status,
            check.detail.replace('"', "\"\"")
        ));
    }
    
    fs::write(path, csv)?;
    Ok(())
}
