use crate::config::Config;
use crate::types::{CheckResult, CheckStatus};
use crate::utils::registry;

#[cfg(target_os = "windows")]
use windows::Win32::System::Registry::*;

pub fn run_checks(_config: &Config) -> Vec<CheckResult> {
    vec![
        check_power_plan(),
        check_c_states(),
        check_core_parking(),
        check_boost_mode(),
        check_throttle_min(),
        check_throttle_max(),
        check_vbs(),
        check_hvci(),
        check_mitigations(),
        check_heterogeneous_scheduler(),
        check_smt(),
        check_speedshift(),
        check_pbo(),
        check_per_core_pstates(),
        check_tvb(),
        check_cppc(),
        check_cache_prefetcher(),
        check_thread_director(),
        check_efficiency_parking(),
        check_performance_priority(),
        check_curve_optimizer(),
        check_affinity(),
        check_scheduling_class(),
        check_quantum(),
        check_priority_boost(),
        check_background_throttling(),
        check_foreground_separation(),
        check_cpu_set(),
        check_idle_states(),
        check_power_limits(),
        check_frequency_scaling(),
    ]
}

fn check_power_plan() -> CheckResult {
    CheckResult {
        name: "Power Plan".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Power plan requires powercfg enumeration".to_string(),
    }
}

fn check_c_states() -> CheckResult {
    CheckResult {
        name: "C-States".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "C-States require power configuration check".to_string(),
    }
}

fn check_core_parking() -> CheckResult {
    CheckResult {
        name: "Core Parking".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Core parking requires power configuration check".to_string(),
    }
}

fn check_boost_mode() -> CheckResult {
    CheckResult {
        name: "Turbo Boost".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Boost mode requires power configuration check".to_string(),
    }
}

fn check_throttle_min() -> CheckResult {
    CheckResult {
        name: "Minimum Processor State".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Min throttle requires power configuration check".to_string(),
    }
}

fn check_throttle_max() -> CheckResult {
    CheckResult {
        name: "Maximum Processor State".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Max throttle requires power configuration check".to_string(),
    }
}

#[cfg(target_os = "windows")]
fn check_vbs() -> CheckResult {
    let status = registry::read_dword(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\DeviceGuard",
        "EnableVirtualizationBasedSecurity"
    ).unwrap_or(0);
    
    CheckResult {
        name: "Virtualization Based Security".to_string(),
        category: "CPU".to_string(),
        status: if status == 0 { CheckStatus::Ok } else { CheckStatus::Warn },
        detail: format!("VBS enabled: {}", status == 1),
    }
}

#[cfg(not(target_os = "windows"))]
fn check_vbs() -> CheckResult {
    CheckResult {
        name: "Virtualization Based Security".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Windows-only check".to_string(),
    }
}

#[cfg(target_os = "windows")]
fn check_hvci() -> CheckResult {
    let status = registry::read_dword(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\DeviceGuard\\Scenarios\\HypervisorEnforcedCodeIntegrity",
        "Enabled"
    ).unwrap_or(0);
    
    CheckResult {
        name: "Hypervisor-Enforced Code Integrity".to_string(),
        category: "CPU".to_string(),
        status: if status == 0 { CheckStatus::Ok } else { CheckStatus::Warn },
        detail: format!("HVCI enabled: {}", status == 1),
    }
}

#[cfg(not(target_os = "windows"))]
fn check_hvci() -> CheckResult {
    CheckResult {
        name: "Hypervisor-Enforced Code Integrity".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Windows-only check".to_string(),
    }
}

#[cfg(target_os = "windows")]
fn check_mitigations() -> CheckResult {
    let status = registry::read_dword(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management",
        "FeatureSettingsOverride"
    ).unwrap_or(0);
    
    CheckResult {
        name: "CPU Mitigations".to_string(),
        category: "CPU".to_string(),
        status: if status == 3 { CheckStatus::Ok } else { CheckStatus::Info },
        detail: format!("Mitigations override: {}", status),
    }
}

#[cfg(not(target_os = "windows"))]
fn check_mitigations() -> CheckResult {
    CheckResult {
        name: "CPU Mitigations".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Windows-only check".to_string(),
    }
}

fn check_heterogeneous_scheduler() -> CheckResult {
    CheckResult {
        name: "Heterogeneous Scheduler".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Heterogeneous scheduler requires CPU detection".to_string(),
    }
}

fn check_smt() -> CheckResult {
    CheckResult {
        name: "Simultaneous Multithreading".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "SMT status requires CPU enumeration".to_string(),
    }
}

fn check_speedshift() -> CheckResult {
    CheckResult {
        name: "Intel Speed Shift".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Speed Shift requires Intel CPU detection".to_string(),
    }
}

fn check_pbo() -> CheckResult {
    CheckResult {
        name: "Precision Boost Overdrive".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "PBO requires AMD CPU detection".to_string(),
    }
}

fn check_per_core_pstates() -> CheckResult {
    CheckResult {
        name: "Per-Core P-States".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Per-core P-states require CPU enumeration".to_string(),
    }
}

fn check_tvb() -> CheckResult {
    CheckResult {
        name: "Thermal Velocity Boost".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "TVB requires Intel CPU detection".to_string(),
    }
}

fn check_cppc() -> CheckResult {
    CheckResult {
        name: "Collaborative Processor Performance Control".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "CPPC requires power configuration check".to_string(),
    }
}

fn check_cache_prefetcher() -> CheckResult {
    CheckResult {
        name: "CPU Cache Prefetcher".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Cache prefetcher requires MSR access".to_string(),
    }
}

fn check_thread_director() -> CheckResult {
    CheckResult {
        name: "Intel Thread Director".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Thread Director requires Intel 12th gen+ detection".to_string(),
    }
}

fn check_efficiency_parking() -> CheckResult {
    CheckResult {
        name: "Efficiency Core Parking".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "E-core parking requires heterogeneous CPU detection".to_string(),
    }
}

fn check_performance_priority() -> CheckResult {
    CheckResult {
        name: "Performance Core Priority".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "P-core priority requires heterogeneous CPU detection".to_string(),
    }
}

fn check_curve_optimizer() -> CheckResult {
    CheckResult {
        name: "AMD Curve Optimizer".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Curve Optimizer requires AMD Ryzen 5000+ detection".to_string(),
    }
}

fn check_affinity() -> CheckResult {
    CheckResult {
        name: "Process Affinity".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Process affinity requires process enumeration".to_string(),
    }
}

fn check_scheduling_class() -> CheckResult {
    CheckResult {
        name: "Scheduling Class".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Scheduling class requires process enumeration".to_string(),
    }
}

fn check_quantum() -> CheckResult {
    CheckResult {
        name: "Thread Quantum".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Thread quantum requires system configuration check".to_string(),
    }
}

fn check_priority_boost() -> CheckResult {
    CheckResult {
        name: "Priority Boost".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Priority boost requires process enumeration".to_string(),
    }
}

fn check_background_throttling() -> CheckResult {
    CheckResult {
        name: "Background Process Throttling".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Background throttling requires power configuration check".to_string(),
    }
}

#[cfg(target_os = "windows")]
fn check_foreground_separation() -> CheckResult {
    let value = registry::read_dword(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Control\\PriorityControl",
        "Win32PrioritySeparation"
    ).unwrap_or(2);
    
    CheckResult {
        name: "Foreground/Background Separation".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: format!("Priority separation: {}", value),
    }
}

#[cfg(not(target_os = "windows"))]
fn check_foreground_separation() -> CheckResult {
    CheckResult {
        name: "Foreground/Background Separation".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Windows-only check".to_string(),
    }
}

fn check_cpu_set() -> CheckResult {
    CheckResult {
        name: "CPU Sets".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "CPU sets require process enumeration".to_string(),
    }
}

fn check_idle_states() -> CheckResult {
    CheckResult {
        name: "Idle States".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Idle states require power configuration check".to_string(),
    }
}

fn check_power_limits() -> CheckResult {
    CheckResult {
        name: "Power Limits".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Power limits require CPU monitoring".to_string(),
    }
}

fn check_frequency_scaling() -> CheckResult {
    CheckResult {
        name: "Frequency Scaling".to_string(),
        category: "CPU".to_string(),
        status: CheckStatus::Info,
        detail: "Frequency scaling requires power configuration check".to_string(),
    }
}
