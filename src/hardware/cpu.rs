use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub name: String,
    pub cores: usize,
    pub threads: usize,
    pub frequency: u64,
    pub vendor: String,
}

pub fn detect() -> CpuInfo {
    let mut sys = System::new_all();
    sys.refresh_cpu_all();
    
    let cpus = sys.cpus();
    let cpu = cpus.first();
    
    CpuInfo {
        name: cpu.map(|c| c.brand().to_string()).unwrap_or_else(|| "Unknown".to_string()),
        cores: sys.physical_core_count().unwrap_or(0),
        threads: cpus.len(),
        frequency: cpu.map(|c| c.frequency()).unwrap_or(0),
        vendor: cpu.map(|c| c.vendor_id().to_string()).unwrap_or_else(|| "Unknown".to_string()),
    }
}
