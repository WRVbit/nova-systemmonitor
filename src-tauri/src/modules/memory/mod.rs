// Memory Monitoring Module
// Provides RAM and SWAP usage statistics

use serde::{Deserialize, Serialize};
use sysinfo::System;
use std::sync::RwLock;

/// Memory statistics in bytes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_memory: u64,
    pub used_memory: u64,
    pub available_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub memory_usage_percent: f32,
    pub swap_usage_percent: f32,
}

/// Memory Monitor state
pub struct MemoryMonitor {
    system: RwLock<System>,
}

impl MemoryMonitor {
    pub fn new() -> Self {
        let mut system = System::new();
        system.refresh_memory();
        Self {
            system: RwLock::new(system),
        }
    }

    pub fn refresh(&self) -> MemoryInfo {
        let mut sys = self.system.write()
            .expect("Memory monitor RwLock poisoned - this is a fatal error");
        sys.refresh_memory();
        
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let available_memory = sys.available_memory();
        let total_swap = sys.total_swap();
        let used_swap = sys.used_swap();

        let memory_usage_percent = if total_memory > 0 {
            (used_memory as f32 / total_memory as f32) * 100.0
        } else {
            0.0
        };

        let swap_usage_percent = if total_swap > 0 {
            (used_swap as f32 / total_swap as f32) * 100.0
        } else {
            0.0
        };

        MemoryInfo {
            total_memory,
            used_memory,
            available_memory,
            total_swap,
            used_swap,
            memory_usage_percent,
            swap_usage_percent,
        }
    }
}

impl Default for MemoryMonitor {
    fn default() -> Self {
        Self::new()
    }
}
