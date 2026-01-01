use anyhow::Result;

#[cfg(target_os = "windows")]
use crate::utils::registry as reg_utils;
#[cfg(target_os = "windows")]
use crate::fixes::backup;
#[cfg(target_os = "windows")]
use windows::Win32::System::Registry::*;

#[cfg(target_os = "windows")]
pub fn write_dword_with_backup(key: HKEY, subkey: &str, value: &str, data: u32) -> Result<()> {
    // Create backup
    if let Ok(old_value) = reg_utils::read_dword(key, subkey, value) {
        backup::backup_registry_value(subkey, value, old_value)?;
    }
    
    // Write new value
    reg_utils::write_dword(key, subkey, value, data)
}

#[cfg(not(target_os = "windows"))]
pub fn write_dword_with_backup(_key: usize, _subkey: &str, _value: &str, _data: u32) -> Result<()> {
    Err(anyhow::anyhow!("Registry operations only supported on Windows"))
}
