use serde::{Deserialize, Serialize};
use sysinfo::{NetworkExt, System, SystemExt, NetworksExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub name: String,
    pub received: u64,
    pub transmitted: u64,
}

pub fn detect() -> Vec<NetworkInfo> {
    let mut sys = System::new_all();
    sys.refresh_networks();
    
    sys.networks()
        .iter()
        .map(|(name, data)| NetworkInfo {
            name: name.clone(),
            received: data.total_received(),
            transmitted: data.total_transmitted(),
        })
        .collect()
}
