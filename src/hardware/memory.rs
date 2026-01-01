use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total: u64,
    pub available: u64,
    pub used: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

pub fn detect() -> MemoryInfo {
    let sys = System::new_all();
    
    MemoryInfo {
        total: sys.total_memory(),
        available: sys.available_memory(),
        used: sys.used_memory(),
        swap_total: sys.total_swap(),
        swap_used: sys.used_swap(),
    }
}
