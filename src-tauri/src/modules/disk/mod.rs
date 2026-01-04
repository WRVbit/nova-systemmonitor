// Disk Monitoring Module
// Provides disk usage, I/O statistics, mount point information, and SMART data

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::sync::RwLock;
use std::time::{Duration, Instant};
use sysinfo::Disks;

/// SMART health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SmartHealth {
    Passed,
    Failed,
    Unknown,
}

/// SMART disk information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartInfo {
    pub health: SmartHealth,
    pub temperature: Option<u32>, // Celsius
    pub power_on_hours: Option<u64>,
    pub power_cycle_count: Option<u64>,
}

/// Information about a single disk/partition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub file_system: String,
    pub total_space: u64,
    pub available_space: u64,
    pub used_space: u64,
    pub usage_percent: f32,
    pub is_removable: bool,
    pub read_bytes: u64,
    pub written_bytes: u64,
    pub smart: Option<SmartInfo>,
}

/// Overall disk statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisksInfo {
    pub disks: Vec<DiskInfo>,
    pub total_space: u64,
    pub total_used: u64,
    pub total_available: u64,
}

/// SMART cache entry
struct SmartCache {
    info: Option<SmartInfo>,
    last_update: Instant,
}

/// Disk Monitor state with SMART caching
pub struct DiskMonitor {
    disks: RwLock<Option<Disks>>,
    smart_cache: RwLock<HashMap<String, SmartCache>>,
}

// SMART data cache duration (60 seconds - SMART data doesn't change often)
const SMART_CACHE_DURATION: Duration = Duration::from_secs(60);

impl DiskMonitor {
    pub fn new() -> Self {
        Self {
            disks: RwLock::new(None),
            smart_cache: RwLock::new(HashMap::new()),
        }
    }

    /// Get SMART data for a disk device (with caching)
    fn get_smart_info_cached(&self, device_name: &str) -> Option<SmartInfo> {
        // Check cache first
        {
            let cache = self.smart_cache.read().ok()?;
            if let Some(cached) = cache.get(device_name) {
                if cached.last_update.elapsed() < SMART_CACHE_DURATION {
                    return cached.info.clone();
                }
            }
        }

        // Cache miss or expired - fetch new data
        let smart_info = self.get_smart_info_raw(device_name);

        // Update cache
        if let Ok(mut cache) = self.smart_cache.write() {
            cache.insert(
                device_name.to_string(),
                SmartCache {
                    info: smart_info.clone(),
                    last_update: Instant::now(),
                },
            );
        }

        smart_info
    }

    /// Get SMART data for a disk device (raw, without cache)
    fn get_smart_info_raw(&self, device_name: &str) -> Option<SmartInfo> {
        // Extract device path (e.g., /dev/sda from /dev/sda1)
        let device_path = if device_name.starts_with("/dev/") {
            // Remove partition number (sda1 -> sda, nvme0n1p1 -> nvme0n1)
            let base = device_name.trim_end_matches(|c: char| c.is_ascii_digit());
            let base = base.trim_end_matches('p'); // For nvme0n1p1
            base.to_string()
        } else {
            return None;
        };

        // Run smartctl (requires smartmontools installed)
        let output = Command::new("smartctl")
            .args(["-H", "-A", &device_path])
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Parse health status
        let health = if stdout.contains("PASSED") {
            SmartHealth::Passed
        } else if stdout.contains("FAILED") {
            SmartHealth::Failed
        } else {
            SmartHealth::Unknown
        };

        // Parse temperature (ID 194)
        let temperature = stdout
            .lines()
            .find(|line| {
                line.contains("Temperature_Celsius") || line.contains("Airflow_Temperature")
            })
            .and_then(|line| {
                line.split_whitespace()
                    .nth(9)
                    .and_then(|s| s.parse::<u32>().ok())
            });

        // Parse power on hours (ID 9)
        let power_on_hours = stdout
            .lines()
            .find(|line| line.contains("Power_On_Hours"))
            .and_then(|line| {
                line.split_whitespace()
                    .nth(9)
                    .and_then(|s| s.parse::<u64>().ok())
            });

        // Parse power cycle count (ID 12)
        let power_cycle_count = stdout
            .lines()
            .find(|line| line.contains("Power_Cycle_Count"))
            .and_then(|line| {
                line.split_whitespace()
                    .nth(9)
                    .and_then(|s| s.parse::<u64>().ok())
            });

        Some(SmartInfo {
            health,
            temperature,
            power_on_hours,
            power_cycle_count,
        })
    }

    pub fn refresh(&self) -> DisksInfo {
        let mut disks_handle = self
            .disks
            .write()
            .expect("Disk monitor RwLock poisoned - fatal error");

        // Initialize lazily
        if disks_handle.is_none() {
            *disks_handle = Some(Disks::new_with_refreshed_list());
        }

        let disks_ref = disks_handle.as_mut().unwrap();
        disks_ref.refresh_list();

        let mut disks: Vec<DiskInfo> = Vec::new();
        let mut total_space: u64 = 0;
        let mut total_used: u64 = 0;
        let mut total_available: u64 = 0;

        // Batch read disk stats once
        let io_stats = Self::get_all_disk_io_stats();

        for disk in disks_ref.iter() {
            let disk_total = disk.total_space();
            let disk_available = disk.available_space();
            let disk_used = disk_total.saturating_sub(disk_available);

            let usage_percent = if disk_total > 0 {
                (disk_used as f32 / disk_total as f32) * 100.0
            } else {
                0.0
            };

            let device_name = disk.name().to_string_lossy().to_string();
            // Use cached SMART info to prevent blocking
            let smart = self.get_smart_info_cached(&device_name);

            // Lookup I/O stats from the batch map
            let (read_bytes, written_bytes) = io_stats.get(&device_name).copied().unwrap_or((0, 0));

            disks.push(DiskInfo {
                name: device_name,
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                file_system: disk.file_system().to_string_lossy().to_string(),
                total_space: disk_total,
                available_space: disk_available,
                used_space: disk_used,
                usage_percent,
                is_removable: disk.is_removable(),
                read_bytes,
                written_bytes,
                smart,
            });

            total_space += disk_total;
            total_available += disk_available;
            total_used += disk_used;
        }

        DisksInfo {
            disks,
            total_space,
            total_used,
            total_available,
        }
    }

    /// Read all I/O stats from /proc/diskstats once
    /// Returns a map of device_name -> (read_bytes, written_bytes)
    fn get_all_disk_io_stats() -> HashMap<String, (u64, u64)> {
        let mut stats = HashMap::new();
        if let Ok(content) = fs::read_to_string("/proc/diskstats") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 14 {
                    // Field 3 is device name
                    let device_name = parts[2].to_string();

                    // Field 6: sectors read
                    // Field 10: sectors written
                    let sectors_read = parts[5].parse::<u64>().unwrap_or(0);
                    let sectors_written = parts[9].parse::<u64>().unwrap_or(0);

                    // Assuming 512 byte sectors is the standard unit for diskstats
                    stats.insert(device_name, (sectors_read * 512, sectors_written * 512));
                }
            }
        }
        stats
    }
}

impl Default for DiskMonitor {
    fn default() -> Self {
        Self::new()
    }
}
