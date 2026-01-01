use crate::config::Config;
use crate::types::{CheckResult, CheckStatus};

pub fn run_checks(_config: &Config) -> Vec<CheckResult> {
    (1..=20).map(|i| CheckResult {
        name: format!("Thermal Check {}", i),
        category: "Thermal".to_string(),
        status: CheckStatus::Info,
        detail: "Thermal checks require detailed implementation".to_string(),
    }).collect()
}
