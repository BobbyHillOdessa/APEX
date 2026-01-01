use crate::utils::registry as reg_utils;
use crate::fixes::backup;
use anyhow::Result;
use windows::Win32::System::Registry::*;

pub fn write_dword_with_backup(key: HKEY, subkey: &str, value: &str, data: u32) -> Result<()> {
    // Create backup
    if let Ok(old_value) = reg_utils::read_dword(key, subkey, value) {
        backup::backup_registry_value(subkey, value, old_value)?;
    }
    
    // Write new value
    reg_utils::write_dword(key, subkey, value, data)
}
