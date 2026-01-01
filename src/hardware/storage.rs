use serde::{Deserialize, Serialize};
use sysinfo::{Disks};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    pub name: String,
    pub mount_point: String,
    pub total: u64,
    pub available: u64,
    pub filesystem: String,
}

pub fn detect() -> Vec<StorageInfo> {
    let disks = Disks::new_with_refreshed_list();
    
    disks
        .iter()
        .map(|disk| StorageInfo {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            total: disk.total_space(),
            available: disk.available_space(),
            filesystem: disk.file_system().to_string_lossy().to_string(),
        })
        .collect()
}
