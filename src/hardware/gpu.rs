use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: String,
    pub memory: u64,
    pub driver_version: String,
}

pub fn detect() -> Vec<GpuInfo> {
    let mut gpus = Vec::new();
    
    // Try NVML for NVIDIA GPUs
    if let Ok(nvml) = nvml_wrapper::Nvml::init() {
        if let Ok(device_count) = nvml.device_count() {
            for i in 0..device_count {
                if let Ok(device) = nvml.device_by_index(i) {
                    let name = device.name().unwrap_or_else(|_| "Unknown NVIDIA GPU".to_string());
                    let memory = device.memory_info().map(|m| m.total).unwrap_or(0);
                    let driver = nvml.sys_driver_version().unwrap_or_else(|_| "Unknown".to_string());
                    
                    gpus.push(GpuInfo {
                        name,
                        vendor: "NVIDIA".to_string(),
                        memory,
                        driver_version: driver,
                    });
                }
            }
        }
    }
    
    // Fallback for other GPUs using sysinfo or registry
    if gpus.is_empty() {
        gpus.push(GpuInfo {
            name: "Unknown GPU".to_string(),
            vendor: "Unknown".to_string(),
            memory: 0,
            driver_version: "Unknown".to_string(),
        });
    }
    
    gpus
}
