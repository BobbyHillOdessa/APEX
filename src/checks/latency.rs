use crate::config::Config;
use crate::types::{CheckResult, CheckStatus};
use crate::utils::registry;

#[cfg(target_os = "windows")]
use windows::Win32::System::Registry::*;

pub fn run_checks(_config: &Config) -> Vec<CheckResult> {
    vec![
        check_dpc_latency(),
        check_hpet(),
        check_tsc_sync(),
        check_dynamic_tick(),
        check_mmcss_responsiveness(),
        check_network_throttling(),
        check_priority_separation(),
        check_gpu_msi(),
        check_nic_msi(),
        check_gpu_interrupt_affinity(),
        check_nic_interrupt_affinity(),
        check_timer_coalescing(),
        check_smi_detection(),
        check_x2apic(),
        check_pcie_aspm(),
        check_usb_latency(),
        check_sata_link_power(),
        check_audio_buffer(),
        check_directx_flip(),
        check_timer_resolution(),
        check_cpu_idle_latency(),
        check_memory_timing(),
        check_nvme_queue_depth(),
        check_bluetooth_latency(),
        check_hid_polling(),
        check_monitor_sync(),
        check_gpu_preemption(),
        check_shader_cache(),
        check_asset_streaming(),
        check_network_coalescing(),
        check_storage_priority(),
        check_realtime_priority(),
        check_mmcss_thread_priority(),
        check_interrupt_steering(),
        check_global_timer_resolution(),
    ]
}

fn check_dpc_latency() -> CheckResult {
    CheckResult {
        name: "DPC Latency".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "DPC latency measurement requires runtime monitoring".to_string(),
    }
}

#[cfg(target_os = "windows")]
fn check_hpet() -> CheckResult {
    let status = registry::read_dword(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\TimeProviders\\TimerResolution",
        "HPETDisabled"
    ).unwrap_or(0);
    
    CheckResult {
        name: "HPET Status".to_string(),
        category: "Latency".to_string(),
        status: if status == 1 { CheckStatus::Ok } else { CheckStatus::Warn },
        detail: format!("HPET disabled: {}", status == 1),
    }
}

#[cfg(not(target_os = "windows"))]
fn check_hpet() -> CheckResult {
    CheckResult {
        name: "HPET Status".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Windows-only check".to_string(),
    }
}

#[cfg(target_os = "windows")]
fn check_tsc_sync() -> CheckResult {
    let status = registry::read_dword(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\kernel",
        "UsePlatformClock"
    ).unwrap_or(0);
    
    CheckResult {
        name: "TSC Sync Policy".to_string(),
        category: "Latency".to_string(),
        status: if status == 0 { CheckStatus::Ok } else { CheckStatus::Warn },
        detail: format!("TSC synchronized: {}", status == 0),
    }
}

#[cfg(not(target_os = "windows"))]
fn check_tsc_sync() -> CheckResult {
    CheckResult {
        name: "TSC Sync Policy".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Windows-only check".to_string(),
    }
}

#[cfg(target_os = "windows")]
fn check_dynamic_tick() -> CheckResult {
    let status = registry::read_dword(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\kernel",
        "DisableDynamicTick"
    ).unwrap_or(0);
    
    CheckResult {
        name: "Dynamic Tick".to_string(),
        category: "Latency".to_string(),
        status: if status == 1 { CheckStatus::Ok } else { CheckStatus::Warn },
        detail: format!("Dynamic tick disabled: {}", status == 1),
    }
}

#[cfg(not(target_os = "windows"))]
fn check_dynamic_tick() -> CheckResult {
    CheckResult {
        name: "Dynamic Tick".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Windows-only check".to_string(),
    }
}

#[cfg(target_os = "windows")]
fn check_mmcss_responsiveness() -> CheckResult {
    let value = registry::read_dword(
        HKEY_LOCAL_MACHINE,
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
        "SystemResponsiveness"
    ).unwrap_or(20);
    
    CheckResult {
        name: "MMCSS System Responsiveness".to_string(),
        category: "Latency".to_string(),
        status: if value <= 10 { CheckStatus::Ok } else { CheckStatus::Warn },
        detail: format!("System responsiveness: {}%", value),
    }
}

#[cfg(not(target_os = "windows"))]
fn check_mmcss_responsiveness() -> CheckResult {
    CheckResult {
        name: "MMCSS System Responsiveness".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Windows-only check".to_string(),
    }
}

#[cfg(target_os = "windows")]
fn check_network_throttling() -> CheckResult {
    let value = registry::read_dword(
        HKEY_LOCAL_MACHINE,
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
        "NetworkThrottlingIndex"
    ).unwrap_or(10);
    
    CheckResult {
        name: "Network Throttling".to_string(),
        category: "Latency".to_string(),
        status: if value == 0xFFFFFFFF { CheckStatus::Ok } else { CheckStatus::Warn },
        detail: format!("Network throttling: {}", if value == 0xFFFFFFFF { "Disabled" } else { "Enabled" }),
    }
}

#[cfg(not(target_os = "windows"))]
fn check_network_throttling() -> CheckResult {
    CheckResult {
        name: "Network Throttling".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Windows-only check".to_string(),
    }
}

#[cfg(target_os = "windows")]
fn check_priority_separation() -> CheckResult {
    let value = registry::read_dword(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\PriorityControl",
        "Win32PrioritySeparation"
    ).unwrap_or(2);
    
    CheckResult {
        name: "Priority Separation".to_string(),
        category: "Latency".to_string(),
        status: if value == 38 { CheckStatus::Ok } else { CheckStatus::Info },
        detail: format!("Win32PrioritySeparation: {}", value),
    }
}

#[cfg(not(target_os = "windows"))]
fn check_priority_separation() -> CheckResult {
    CheckResult {
        name: "Priority Separation".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Windows-only check".to_string(),
    }
}

fn check_gpu_msi() -> CheckResult {
    CheckResult {
        name: "GPU MSI-X Mode".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "GPU MSI-X status requires device enumeration".to_string(),
    }
}

fn check_nic_msi() -> CheckResult {
    CheckResult {
        name: "NIC MSI-X Mode".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "NIC MSI-X status requires device enumeration".to_string(),
    }
}

fn check_gpu_interrupt_affinity() -> CheckResult {
    CheckResult {
        name: "GPU Interrupt Affinity".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "GPU interrupt affinity requires device enumeration".to_string(),
    }
}

fn check_nic_interrupt_affinity() -> CheckResult {
    CheckResult {
        name: "NIC Interrupt Affinity".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "NIC interrupt affinity requires device enumeration".to_string(),
    }
}

#[cfg(target_os = "windows")]
fn check_timer_coalescing() -> CheckResult {
    let value = registry::read_dword(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\kernel",
        "CoalescingTimerInterval"
    ).unwrap_or(0);
    
    CheckResult {
        name: "Timer Coalescing".to_string(),
        category: "Latency".to_string(),
        status: if value == 0 { CheckStatus::Ok } else { CheckStatus::Warn },
        detail: format!("Timer coalescing disabled: {}", value == 0),
    }
}

#[cfg(not(target_os = "windows"))]
fn check_timer_coalescing() -> CheckResult {
    CheckResult {
        name: "Timer Coalescing".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Windows-only check".to_string(),
    }
}

fn check_smi_detection() -> CheckResult {
    CheckResult {
        name: "SMI Detection".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "SMI detection requires hardware monitoring".to_string(),
    }
}

fn check_x2apic() -> CheckResult {
    CheckResult {
        name: "x2APIC Mode".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "x2APIC status requires CPU feature detection".to_string(),
    }
}

fn check_pcie_aspm() -> CheckResult {
    CheckResult {
        name: "PCIe ASPM".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "PCIe ASPM status requires device enumeration".to_string(),
    }
}

fn check_usb_latency() -> CheckResult {
    CheckResult {
        name: "USB Controller Latency".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "USB latency requires controller enumeration".to_string(),
    }
}

fn check_sata_link_power() -> CheckResult {
    CheckResult {
        name: "SATA Link Power Management".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "SATA link power requires device enumeration".to_string(),
    }
}

fn check_audio_buffer() -> CheckResult {
    CheckResult {
        name: "Audio Endpoint Buffer".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Audio buffer size requires audio device enumeration".to_string(),
    }
}

fn check_directx_flip() -> CheckResult {
    CheckResult {
        name: "DirectX Flip Model".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "DirectX flip model requires runtime detection".to_string(),
    }
}

fn check_timer_resolution() -> CheckResult {
    CheckResult {
        name: "Kernel Timer Resolution".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Timer resolution requires NtQueryTimerResolution call".to_string(),
    }
}

fn check_cpu_idle_latency() -> CheckResult {
    CheckResult {
        name: "CPU Idle Latency".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "CPU idle latency requires power state monitoring".to_string(),
    }
}

fn check_memory_timing() -> CheckResult {
    CheckResult {
        name: "Memory Controller Timing".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Memory timing requires SPD reading".to_string(),
    }
}

fn check_nvme_queue_depth() -> CheckResult {
    CheckResult {
        name: "NVMe Queue Depth".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "NVMe queue depth requires device enumeration".to_string(),
    }
}

fn check_bluetooth_latency() -> CheckResult {
    CheckResult {
        name: "Bluetooth Audio Latency".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Bluetooth latency requires device enumeration".to_string(),
    }
}

fn check_hid_polling() -> CheckResult {
    CheckResult {
        name: "HID Polling Rate".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "HID polling rate requires device enumeration".to_string(),
    }
}

fn check_monitor_sync() -> CheckResult {
    CheckResult {
        name: "Monitor Refresh Sync".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Monitor sync requires display enumeration".to_string(),
    }
}

fn check_gpu_preemption() -> CheckResult {
    CheckResult {
        name: "GPU Preemption Granularity".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "GPU preemption requires driver API".to_string(),
    }
}

fn check_shader_cache() -> CheckResult {
    CheckResult {
        name: "Shader Cache".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Shader cache status requires driver enumeration".to_string(),
    }
}

fn check_asset_streaming() -> CheckResult {
    CheckResult {
        name: "Asset Streaming".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Asset streaming requires game detection".to_string(),
    }
}

fn check_network_coalescing() -> CheckResult {
    CheckResult {
        name: "Network Interrupt Coalescing".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Network coalescing requires adapter enumeration".to_string(),
    }
}

fn check_storage_priority() -> CheckResult {
    CheckResult {
        name: "Storage I/O Priority".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Storage priority requires device enumeration".to_string(),
    }
}

fn check_realtime_priority() -> CheckResult {
    CheckResult {
        name: "Real-time Priority Processes".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Real-time priority requires process enumeration".to_string(),
    }
}

fn check_mmcss_thread_priority() -> CheckResult {
    CheckResult {
        name: "MMCSS Thread Priority".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "MMCSS thread priority requires service enumeration".to_string(),
    }
}

fn check_interrupt_steering() -> CheckResult {
    CheckResult {
        name: "Interrupt Steering".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Interrupt steering requires device enumeration".to_string(),
    }
}

fn check_global_timer_resolution() -> CheckResult {
    CheckResult {
        name: "Global Timer Resolution".to_string(),
        category: "Latency".to_string(),
        status: CheckStatus::Info,
        detail: "Global timer resolution requires system monitoring".to_string(),
    }
}
