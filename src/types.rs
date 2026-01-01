use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

impl Severity {
    pub fn penalty(&self) -> i32 {
        match self {
            Severity::Critical => 20,
            Severity::High => 12,
            Severity::Medium => 6,
            Severity::Low => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckStatus {
    Ok,
    Warn,
    Bad,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub num: u32,
    pub category: String,
    pub problem: String,
    pub solution: String,
    pub severity: Severity,
    pub safe: bool,
    pub reboot: bool,
    #[serde(skip)]
    pub fix_fn: Option<fn() -> anyhow::Result<()>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warning {
    pub category: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub name: String,
    pub category: String,
    pub status: CheckStatus,
    pub detail: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AuditResults {
    pub issues: Vec<Issue>,
    pub warnings: Vec<Warning>,
    pub checks: Vec<CheckResult>,
    pub check_count: usize,
    pub score: i32,
}

impl AuditResults {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_issue(&mut self, issue: Issue) {
        self.issues.push(issue);
    }

    pub fn add_warning(&mut self, warning: Warning) {
        self.warnings.push(warning);
    }

    pub fn add_check(&mut self, check: CheckResult) {
        self.checks.push(check);
        self.check_count += 1;
    }

    pub fn calculate_score(&mut self) {
        let base_score = 100;
        let penalty: i32 = self.issues.iter().map(|i| i.severity.penalty()).sum();
        self.score = (base_score - penalty).max(0);
    }
}
