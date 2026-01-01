use serde::{Deserialize, Serialize};
use sysinfo::{DiskExt, System, SystemExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    pub name: String,
    pub mount_point: String,
    pub total: u64,
    pub available: u64,
    pub filesystem: String,
}

pub fn detect() -> Vec<StorageInfo> {
    let mut sys = System::new_all();
    sys.refresh_disks();
    
    sys.disks()
        .iter()
        .map(|disk| StorageInfo {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            total: disk.total_space(),
            available: disk.available_space(),
            filesystem: String::from_utf8_lossy(disk.file_system()).to_string(),
        })
        .collect()
}
