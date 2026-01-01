use anyhow::Result;
use std::process::Command;

pub fn set_power_setting(scheme: &str, subgroup: &str, setting: &str, value: &str) -> Result<()> {
    Command::new("powercfg")
        .args(["/setacvalueindex", scheme, subgroup, setting, value])
        .output()?;
    
    Command::new("powercfg")
        .args(["/setdcvalueindex", scheme, subgroup, setting, value])
        .output()?;
    
    Command::new("powercfg")
        .args(["/setactive", scheme])
        .output()?;
    
    Ok(())
}
