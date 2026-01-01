pub mod cpu;
pub mod gpu;
pub mod memory;
pub mod storage;
pub mod network;
pub mod system;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub cpu: cpu::CpuInfo,
    pub gpu: Vec<gpu::GpuInfo>,
    pub memory: memory::MemoryInfo,
    pub storage: Vec<storage::StorageInfo>,
    pub network: Vec<network::NetworkInfo>,
    pub system: system::SystemInfo,
}

impl HardwareInfo {
    pub fn detect() -> Self {
        Self {
            cpu: cpu::detect(),
            gpu: gpu::detect(),
            memory: memory::detect(),
            storage: storage::detect(),
            network: network::detect(),
            system: system::detect(),
        }
    }
}
