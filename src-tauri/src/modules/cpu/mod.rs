// CPU Monitoring Module
// Provides CPU usage, frequency, and per-core statistics

use serde::{Deserialize, Serialize};
use sysinfo::System;
use std::sync::RwLock;

/// CPU information for a single core
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuCore {
    pub name: String,
    pub usage: f32,
    pub frequency: u64, // MHz
}

/// Overall CPU information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub name: String,
    pub vendor: String,
    pub brand: String,
    pub physical_cores: usize,
    pub logical_cores: usize,
    pub global_usage: f32,
    pub cores: Vec<CpuCore>,
}

/// CPU Monitor state
pub struct CpuMonitor {
    system: RwLock<System>,
}

impl CpuMonitor {
    pub fn new() -> Self {
        let mut system = System::new();
        system.refresh_cpu_all();
        Self {
            system: RwLock::new(system),
        }
    }

    pub fn refresh(&self) -> CpuInfo {
        let mut sys = self.system.write()
            .expect("CPU monitor RwLock poisoned - this is a fatal error");
        sys.refresh_cpu_all();
        
        let cpus = sys.cpus();
        let cores: Vec<CpuCore> = cpus
            .iter()
            .map(|cpu| CpuCore {
                name: cpu.name().to_string(),
                usage: cpu.cpu_usage(),
                frequency: cpu.frequency(),
            })
            .collect();

        let global_usage = if !cores.is_empty() {
            cores.iter().map(|c| c.usage).sum::<f32>() / cores.len() as f32
        } else {
            0.0
        };

        CpuInfo {
            name: cpus.first().map(|c| c.name().to_string()).unwrap_or_default(),
            vendor: cpus.first().map(|c| c.vendor_id().to_string()).unwrap_or_default(),
            brand: cpus.first().map(|c| c.brand().to_string()).unwrap_or_default(),
            physical_cores: sys.physical_core_count().unwrap_or(0),
            logical_cores: cpus.len(),
            global_usage,
            cores,
        }
    }
}

impl Default for CpuMonitor {
    fn default() -> Self {
        Self::new()
    }
}
