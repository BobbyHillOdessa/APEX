pub mod latency;
pub mod cpu;
pub mod gpu;
pub mod memory;
pub mod storage;
pub mod network;
pub mod audio;
pub mod input;
pub mod stability;
pub mod services;
pub mod security;
pub mod platform;
pub mod thermal;
pub mod power;

use crate::config::Config;
use crate::types::{AuditResults, CheckResult};
use rayon::prelude::*;

pub fn run_all_checks(config: &Config) -> AuditResults {
    let mut results = AuditResults::new();
    
    // Run all check categories in parallel
    let check_results: Vec<Vec<CheckResult>> = vec![
        latency::run_checks(config),
        cpu::run_checks(config),
        gpu::run_checks(config),
        memory::run_checks(config),
        storage::run_checks(config),
        network::run_checks(config),
        audio::run_checks(config),
        input::run_checks(config),
        stability::run_checks(config),
        services::run_checks(config),
        security::run_checks(config),
        platform::run_checks(config),
        thermal::run_checks(config),
        power::run_checks(config),
    ].into_par_iter()
        .map(|checks| checks)
        .collect();
    
    // Flatten results
    for category_checks in check_results {
        for check in category_checks {
            results.add_check(check);
        }
    }
    
    results.calculate_score();
    results
}
