use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use chrono::Local;

#[derive(Debug, Serialize, Deserialize)]
struct RegistryBackup {
    timestamp: String,
    subkey: String,
    value: String,
    data: u32,
}

pub fn backup_registry_value(subkey: &str, value: &str, data: u32) -> Result<()> {
    let backup_dir = PathBuf::from("backups");
    fs::create_dir_all(&backup_dir)?;
    
    let backup = RegistryBackup {
        timestamp: Local::now().format("%Y%m%d_%H%M%S").to_string(),
        subkey: subkey.to_string(),
        value: value.to_string(),
        data,
    };
    
    let filename = format!("backup_{}_{}.json", 
        backup.timestamp,
        value.replace('\\', "_")
    );
    
    let path = backup_dir.join(filename);
    let json = serde_json::to_string_pretty(&backup)?;
    fs::write(path, json)?;
    
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn rollback_from_file(backup_file: &str) -> Result<()> {
    use windows::Win32::System::Registry::*;
    
    let json = fs::read_to_string(backup_file)?;
    let backup: RegistryBackup = serde_json::from_str(&json)?;
    
    // Restore registry value
    crate::utils::registry::write_dword(
        HKEY_LOCAL_MACHINE,
        &backup.subkey,
        &backup.value,
        backup.data,
    )?;
    
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn rollback_from_file(_backup_file: &str) -> Result<()> {
    Err(anyhow::anyhow!("Rollback only supported on Windows"))
}
