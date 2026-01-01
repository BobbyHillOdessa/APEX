use windows::Win32::Foundation::BOOL;
use windows::Win32::Security::IsUserAnAdmin;

pub fn is_admin() -> bool {
    unsafe {
        IsUserAnAdmin() == BOOL(1)
    }
}

pub fn require_admin() -> anyhow::Result<()> {
    if !is_admin() {
        Err(anyhow::anyhow!("Administrator privileges required. Please run as administrator."))
    } else {
        Ok(())
    }
}
