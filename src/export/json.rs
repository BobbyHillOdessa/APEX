use crate::types::AuditResults;
use anyhow::Result;
use std::fs;

pub fn export(results: &AuditResults, path: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(results)?;
    fs::write(path, json)?;
    Ok(())
}
