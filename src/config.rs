pub struct Config {
    pub dpc_warn: f64,
    pub dpc_bad: f64,
    pub temp_warn: i32,
    pub temp_crit: i32,
    pub cpu_usage_warn: f32,
    pub memory_usage_warn: f32,
    pub disk_usage_warn: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dpc_warn: 1.5,
            dpc_bad: 3.0,
            temp_warn: 80,
            temp_crit: 90,
            cpu_usage_warn: 90.0,
            memory_usage_warn: 90.0,
            disk_usage_warn: 85.0,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}
