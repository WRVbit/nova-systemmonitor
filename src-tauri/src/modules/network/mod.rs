// Network Monitoring Module
// Provides per-interface network statistics with real-time rate calculation

use serde::{Deserialize, Serialize};
use sysinfo::Networks;
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

/// Network rate sample for calculating speed
#[derive(Debug, Clone)]
struct NetworkSample {
    timestamp: u64,  // Milliseconds
    received: u64,
    transmitted: u64,
}

/// Information about a single network interface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub mac_address: String,
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
    pub received_packets: u64,
    pub transmitted_packets: u64,
    pub errors_in: u64,
    pub errors_out: u64,
    // Real-time rates
    pub download_rate_bps: f64,  // Bytes per second
    pub upload_rate_bps: f64,    // Bytes per second
}

/// Overall network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub interfaces: Vec<NetworkInterface>,
    pub total_received: u64,
    pub total_transmitted: u64,
    pub total_download_rate: f64,  // Bytes per second
    pub total_upload_rate: f64,    // Bytes per second
}

/// Network Monitor state
pub struct NetworkMonitor {
    networks: RwLock<Networks>,
    last_samples: RwLock<HashMap<String, NetworkSample>>,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        Self {
            networks: RwLock::new(Networks::new_with_refreshed_list()),
            last_samples: RwLock::new(HashMap::new()),
        }
    }

    fn current_timestamp_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before UNIX epoch")
            .as_millis() as u64
    }

    pub fn refresh(&self) -> NetworkInfo {
        let mut networks_handle = self.networks.write()
            .expect("Network monitor RwLock poisoned - fatal error");
        networks_handle.refresh();
        
        let mut last_samples = self.last_samples.write()
            .expect("Network samples RwLock poisoned - fatal error");
        
        let current_time = Self::current_timestamp_ms();
        
        let mut interfaces: Vec<NetworkInterface> = Vec::new();
        let mut total_received: u64 = 0;
        let mut total_transmitted: u64 = 0;
        let mut total_download_rate: f64 = 0.0;
        let mut total_upload_rate: f64 = 0.0;

        for (name, network) in networks_handle.iter() {
            let received = network.total_received();
            let transmitted = network.total_transmitted();
            
            // Calculate rates
            let (download_rate, upload_rate) = if let Some(last_sample) = last_samples.get(name) {
                let time_delta = (current_time - last_sample.timestamp) as f64 / 1000.0; // Convert to seconds
                
                if time_delta > 0.0 {
                    let rx_delta = received.saturating_sub(last_sample.received) as f64;
                    let tx_delta = transmitted.saturating_sub(last_sample.transmitted) as f64;
                    
                    (rx_delta / time_delta, tx_delta / time_delta)
                } else {
                    (0.0, 0.0)
                }
            } else {
                (0.0, 0.0)
            };
            
            // Update sample
            last_samples.insert(name.clone(), NetworkSample {
                timestamp: current_time,
                received,
                transmitted,
            });
            
            interfaces.push(NetworkInterface {
                name: name.clone(),
                mac_address: network.mac_address().to_string(),
                received_bytes: received,
                transmitted_bytes: transmitted,
                received_packets: network.total_packets_received(),
                transmitted_packets: network.total_packets_transmitted(),
                errors_in: network.total_errors_on_received(),
                errors_out: network.total_errors_on_transmitted(),
                download_rate_bps: download_rate,
                upload_rate_bps: upload_rate,
            });

            total_received += received;
            total_transmitted += transmitted;
            total_download_rate += download_rate;
            total_upload_rate += upload_rate;
        }

        NetworkInfo {
            interfaces,
            total_received,
            total_transmitted,
            total_download_rate,
            total_upload_rate,
        }
    }
}

impl Default for NetworkMonitor {
    fn default() -> Self {
        Self::new()
    }
}
