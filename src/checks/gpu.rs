use crate::config::Config;
use crate::types::{CheckResult, CheckStatus};
use crate::utils::registry;

#[cfg(target_os = "windows")]
use windows::Win32::System::Registry::*;

pub fn run_checks(_config: &Config) -> Vec<CheckResult> {
    vec![
        check_hags(),
        check_tdr_level(),
        check_tdr_delay(),
        check_gamedvr(),
        check_mpo(),
        check_fso(),
        check_dwm(),
        check_nvidia_scheduling(),
        check_amd_prerender(),
        check_rebar(),
        check_gpu_preemption(),
        check_shader_cache(),
        check_anisotropic(),
        check_prerendered_frames(),
        check_texture_filtering(),
        check_vsync(),
        check_triple_buffering(),
        check_frame_limiter(),
        check_low_latency_mode(),
        check_power_management(),
        check_pcie_power_state(),
        check_color_depth(),
        check_hdr(),
        check_vrr(),
        check_memory_clock(),
        check_core_clock(),
        check_voltage(),
        check_wddm_version(),
        check_directx_level(),
        check_vulkan(),
        check_opengl_icd(),
        check_compute_shader(),
        check_ray_tracing(),
        check_dlss_fsr(),
        check_scheduled_priority(),
        check_display_mode(),
    ]
}

#[cfg(target_os = "windows")]
fn check_hags() -> CheckResult {
    let status = registry::read_dword(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
        "HwSchMode"
    ).unwrap_or(1);
    
    CheckResult {
        name: "Hardware Accelerated GPU Scheduling".to_string(),
        category: "GPU".to_string(),
        status: if status == 2 { CheckStatus::Ok } else { CheckStatus::Info },
        detail: format!("HAGS enabled: {}", status == 2),
    }
}

#[cfg(not(target_os = "windows"))]
fn check_hags() -> CheckResult {
    CheckResult {
        name: "Hardware Accelerated GPU Scheduling".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Windows-only check".to_string(),
    }
}

#[cfg(target_os = "windows")]
fn check_tdr_level() -> CheckResult {
    let level = registry::read_dword(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
        "TdrLevel"
    ).unwrap_or(3);
    
    CheckResult {
        name: "TDR Level".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: format!("TDR level: {}", level),
    }
}

#[cfg(not(target_os = "windows"))]
fn check_tdr_level() -> CheckResult {
    CheckResult {
        name: "TDR Level".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Windows-only check".to_string(),
    }
}

#[cfg(target_os = "windows")]
fn check_tdr_delay() -> CheckResult {
    let delay = registry::read_dword(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
        "TdrDelay"
    ).unwrap_or(2);
    
    CheckResult {
        name: "TDR Delay".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: format!("TDR delay: {} seconds", delay),
    }
}

#[cfg(not(target_os = "windows"))]
fn check_tdr_delay() -> CheckResult {
    CheckResult {
        name: "TDR Delay".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Windows-only check".to_string(),
    }
}

#[cfg(target_os = "windows")]
fn check_gamedvr() -> CheckResult {
    let status = registry::read_dword(
        HKEY_CURRENT_USER,
        "System\\GameConfigStore",
        "GameDVR_Enabled"
    ).unwrap_or(0);
    
    CheckResult {
        name: "Game DVR".to_string(),
        category: "GPU".to_string(),
        status: if status == 0 { CheckStatus::Ok } else { CheckStatus::Warn },
        detail: format!("Game DVR disabled: {}", status == 0),
    }
}

#[cfg(not(target_os = "windows"))]
fn check_gamedvr() -> CheckResult {
    CheckResult {
        name: "Game DVR".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Windows-only check".to_string(),
    }
}

// Remaining functions - simplified for now
fn check_mpo() -> CheckResult {
    CheckResult {
        name: "Multi-Plane Overlay".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "MPO requires detailed registry check".to_string(),
    }
}

fn check_fso() -> CheckResult {
    CheckResult {
        name: "Fullscreen Optimizations".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "FSO status per-application".to_string(),
    }
}

fn check_dwm() -> CheckResult {
    CheckResult {
        name: "Desktop Window Manager".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "DWM always enabled on Windows 10+".to_string(),
    }
}

fn check_nvidia_scheduling() -> CheckResult {
    CheckResult {
        name: "NVIDIA Multi-threaded Optimization".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "NVIDIA scheduling requires driver detection".to_string(),
    }
}

fn check_amd_prerender() -> CheckResult {
    CheckResult {
        name: "AMD Anti-Lag".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "AMD prerender requires driver detection".to_string(),
    }
}

fn check_rebar() -> CheckResult {
    CheckResult {
        name: "Resizable BAR".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "ReBAR requires PCIe enumeration".to_string(),
    }
}

fn check_gpu_preemption() -> CheckResult {
    CheckResult {
        name: "GPU Preemption Mode".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "GPU preemption requires driver API".to_string(),
    }
}

fn check_shader_cache() -> CheckResult {
    CheckResult {
        name: "Shader Cache".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Shader cache location varies by driver".to_string(),
    }
}

fn check_anisotropic() -> CheckResult {
    CheckResult {
        name: "Anisotropic Filtering".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Anisotropic filtering per-application".to_string(),
    }
}

fn check_prerendered_frames() -> CheckResult {
    CheckResult {
        name: "Maximum Pre-rendered Frames".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Pre-rendered frames requires driver detection".to_string(),
    }
}

fn check_texture_filtering() -> CheckResult {
    CheckResult {
        name: "Texture Filtering Quality".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Texture filtering per-application".to_string(),
    }
}

fn check_vsync() -> CheckResult {
    CheckResult {
        name: "VSync".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "VSync per-application".to_string(),
    }
}

fn check_triple_buffering() -> CheckResult {
    CheckResult {
        name: "Triple Buffering".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Triple buffering per-application".to_string(),
    }
}

fn check_frame_limiter() -> CheckResult {
    CheckResult {
        name: "Frame Rate Limiter".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Frame limiter per-application".to_string(),
    }
}

fn check_low_latency_mode() -> CheckResult {
    CheckResult {
        name: "Low Latency Mode".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Low latency mode requires driver detection".to_string(),
    }
}

fn check_power_management() -> CheckResult {
    CheckResult {
        name: "GPU Power Management".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "GPU power management requires driver detection".to_string(),
    }
}

fn check_pcie_power_state() -> CheckResult {
    CheckResult {
        name: "PCIe Link State Power Management".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "PCIe power state requires device enumeration".to_string(),
    }
}

fn check_color_depth() -> CheckResult {
    CheckResult {
        name: "Display Color Depth".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Color depth requires display enumeration".to_string(),
    }
}

fn check_hdr() -> CheckResult {
    CheckResult {
        name: "HDR Support".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "HDR requires display enumeration".to_string(),
    }
}

fn check_vrr() -> CheckResult {
    CheckResult {
        name: "Variable Refresh Rate".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "VRR/FreeSync/G-Sync requires display enumeration".to_string(),
    }
}

fn check_memory_clock() -> CheckResult {
    CheckResult {
        name: "GPU Memory Clock".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Memory clock requires GPU monitoring".to_string(),
    }
}

fn check_core_clock() -> CheckResult {
    CheckResult {
        name: "GPU Core Clock".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Core clock requires GPU monitoring".to_string(),
    }
}

fn check_voltage() -> CheckResult {
    CheckResult {
        name: "GPU Voltage".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Voltage requires GPU monitoring".to_string(),
    }
}

fn check_wddm_version() -> CheckResult {
    CheckResult {
        name: "WDDM Version".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "WDDM version requires driver detection".to_string(),
    }
}

fn check_directx_level() -> CheckResult {
    CheckResult {
        name: "DirectX Feature Level".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "DirectX level requires GPU detection".to_string(),
    }
}

fn check_vulkan() -> CheckResult {
    CheckResult {
        name: "Vulkan Support".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Vulkan requires runtime detection".to_string(),
    }
}

fn check_opengl_icd() -> CheckResult {
    CheckResult {
        name: "OpenGL ICD".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "OpenGL ICD requires driver detection".to_string(),
    }
}

fn check_compute_shader() -> CheckResult {
    CheckResult {
        name: "Compute Shader Support".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Compute shader requires GPU detection".to_string(),
    }
}

fn check_ray_tracing() -> CheckResult {
    CheckResult {
        name: "Ray Tracing Support".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Ray tracing requires GPU detection".to_string(),
    }
}

fn check_dlss_fsr() -> CheckResult {
    CheckResult {
        name: "DLSS/FSR Support".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "DLSS/FSR requires GPU detection".to_string(),
    }
}

fn check_scheduled_priority() -> CheckResult {
    CheckResult {
        name: "GPU Scheduled Priority".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "GPU priority requires process enumeration".to_string(),
    }
}

fn check_display_mode() -> CheckResult {
    CheckResult {
        name: "Display Mode".to_string(),
        category: "GPU".to_string(),
        status: CheckStatus::Info,
        detail: "Display mode requires enumeration".to_string(),
    }
}
