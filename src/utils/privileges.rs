#[cfg(target_os = "windows")]
use windows::Win32::Foundation::BOOL;
#[cfg(target_os = "windows")]
use windows::Win32::Security::IsUserAnAdmin;

#[cfg(target_os = "windows")]
pub fn is_admin() -> bool {
    unsafe {
        IsUserAnAdmin() == BOOL(1)
    }
}

#[cfg(not(target_os = "windows"))]
pub fn is_admin() -> bool {
    false
}

pub fn require_admin() -> anyhow::Result<()> {
    if !is_admin() {
        Err(anyhow::anyhow!("Administrator privileges required. Please run as administrator."))
    } else {
        Ok(())
    }
}
