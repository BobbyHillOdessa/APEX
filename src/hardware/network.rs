use serde::{Deserialize, Serialize};
use sysinfo::Networks;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub name: String,
    pub received: u64,
    pub transmitted: u64,
}

pub fn detect() -> Vec<NetworkInfo> {
    let networks = Networks::new_with_refreshed_list();
    
    networks
        .iter()
        .map(|(name, data)| NetworkInfo {
            name: name.clone(),
            received: data.total_received(),
            transmitted: data.total_transmitted(),
        })
        .collect()
}
