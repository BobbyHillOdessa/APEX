use serde::{Deserialize, Serialize};
use sysinfo::{System, SystemExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub hostname: String,
    pub uptime: u64,
}

pub fn detect() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_system();
    
    SystemInfo {
        os_name: sys.name().unwrap_or_else(|| "Unknown".to_string()),
        os_version: sys.os_version().unwrap_or_else(|| "Unknown".to_string()),
        kernel_version: sys.kernel_version().unwrap_or_else(|| "Unknown".to_string()),
        hostname: sys.host_name().unwrap_or_else(|| "Unknown".to_string()),
        uptime: sys.uptime(),
    }
}
