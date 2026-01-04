// GPU Monitoring Module
// Provides comprehensive GPU monitoring for NVIDIA, AMD, and Intel GPUs

use nvml_wrapper::Nvml;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// GPU vendor type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Unknown,
}

/// Information about a single GPU
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub index: u32,
    pub name: String,
    pub vendor: GpuVendor,
    pub uuid: String,
    pub utilization_gpu: u32,    // Percentage
    pub utilization_memory: u32, // Percentage
    pub memory_total: u64,       // Bytes
    pub memory_used: u64,        // Bytes
    pub memory_free: u64,        // Bytes
    pub temperature: u32,        // Celsius
    pub power_usage: u32,        // Milliwatts
    pub power_limit: u32,        // Milliwatts
    pub fan_speed: Option<u32>,  // Percentage
    pub clock_graphics: u32,     // MHz
    pub clock_memory: u32,       // MHz
    pub encoder_utilization: Option<u32>,
    pub decoder_utilization: Option<u32>,
}

/// Overall GPU information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpusInfo {
    pub gpus: Vec<GpuInfo>,
    pub nvidia_available: bool,
    pub amd_available: bool,
    pub intel_available: bool,
    pub driver_version: Option<String>,
    pub errors: Vec<String>,
}

/// Internal GPU state for lazy initialization and history tracking
struct GpuState {
    nvml: Option<Nvml>,
    initialized: bool,
    // Store last RC6 reading and timestamp for Intel GPUs: (card_index) -> (residency_ms, timestamp_ms)
    last_rc6_readings: std::collections::HashMap<u32, (u64, u64)>,
}

/// GPU Monitor state with lazy initialization
pub struct GpuMonitor {
    state: std::sync::RwLock<GpuState>,
}

impl GpuMonitor {
    pub fn new() -> Self {
        // Don't init NVML here - do it lazily on first use
        Self {
            state: std::sync::RwLock::new(GpuState {
                nvml: None,
                initialized: false,
                last_rc6_readings: std::collections::HashMap::new(),
            }),
        }
    }

    /// Get NVIDIA GPU information via NVML
    fn get_nvidia_gpus(
        &self,
        driver_version: &mut Option<String>,
        errors: &mut Vec<String>,
    ) -> Vec<GpuInfo> {
        let mut gpus = Vec::new();

        // Use read lock initially
        if let Ok(state) = self.state.read() {
            if let Some(ref nvml) = state.nvml {
                *driver_version = nvml.sys_driver_version().ok();

                match nvml.device_count() {
                    Ok(device_count) => {
                        for i in 0..device_count {
                            if let Ok(device) = nvml.device_by_index(i) {
                                let name = device
                                    .name()
                                    .unwrap_or_else(|_| "Unknown NVIDIA GPU".to_string());
                                let uuid =
                                    device.uuid().unwrap_or_else(|_| format!("nvidia-{}", i));

                                let (utilization_gpu, utilization_memory) = device
                                    .utilization_rates()
                                    .map(|u| (u.gpu, u.memory))
                                    .unwrap_or((0, 0));

                                let (memory_total, memory_used, memory_free) = device
                                    .memory_info()
                                    .map(|m| (m.total, m.used, m.free))
                                    .unwrap_or((0, 0, 0));

                                let temperature = device
                                    .temperature(
                                        nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu,
                                    )
                                    .unwrap_or(0);

                                let power_usage = device.power_usage().unwrap_or(0);
                                let power_limit = device.power_management_limit().unwrap_or(0);
                                let fan_speed = device.fan_speed(0).ok();

                                let clock_graphics = device
                                    .clock_info(
                                        nvml_wrapper::enum_wrappers::device::Clock::Graphics,
                                    )
                                    .unwrap_or(0);
                                let clock_memory = device
                                    .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory)
                                    .unwrap_or(0);

                                let encoder_utilization =
                                    device.encoder_utilization().ok().map(|e| e.utilization);
                                let decoder_utilization =
                                    device.decoder_utilization().ok().map(|d| d.utilization);

                                gpus.push(GpuInfo {
                                    index: i,
                                    name,
                                    vendor: GpuVendor::Nvidia,
                                    uuid,
                                    utilization_gpu,
                                    utilization_memory,
                                    memory_total,
                                    memory_used,
                                    memory_free,
                                    temperature,
                                    power_usage,
                                    power_limit,
                                    fan_speed,
                                    clock_graphics,
                                    clock_memory,
                                    encoder_utilization,
                                    decoder_utilization,
                                });
                            }
                        }
                    }
                    Err(e) => errors.push(format!("NVIDIA: Failed to get device count: {}", e)),
                }
            } else {
                errors.push("NVIDIA: NVML not initialized".to_string());
            }
        }

        gpus
    }

    /// Get AMD GPU information via sysfs
    fn get_amd_gpus(&self, _errors: &mut Vec<String>) -> Vec<GpuInfo> {
        let mut gpus = Vec::new();
        let drm_path = Path::new("/sys/class/drm");

        if !drm_path.exists() {
            return gpus;
        }

        if let Ok(entries) = fs::read_dir(drm_path) {
            let mut index = 0;
            for entry in entries.flatten() {
                let path = entry.path();
                let device_path = path.join("device");

                // Check if this is an AMD GPU
                let vendor_path = device_path.join("vendor");
                if let Ok(vendor) = fs::read_to_string(&vendor_path) {
                    // AMD vendor ID: 0x1002
                    if !vendor.trim().eq_ignore_ascii_case("0x1002") {
                        continue;
                    }
                } else {
                    continue;
                }

                // Get GPU name - better detection for iGPU
                let device_id = fs::read_to_string(device_path.join("device"))
                    .ok()
                    .and_then(|s| s.trim().to_string().into());

                let name = if let Some(ref id) = device_id {
                    match id.as_str() {
                        "0x1638" => "AMD Radeon Graphics (Ryzen 5000 Series iGPU)".to_string(),
                        "0x1506" => "AMD Radeon Graphics (Ryzen 7000 Series iGPU)".to_string(),
                        _ => format!("AMD Radeon Graphics (Device {})", id),
                    }
                } else {
                    "AMD Radeon Graphics".to_string()
                };

                // Get utilization (gpu_busy_percent)
                let utilization_gpu = fs::read_to_string(device_path.join("gpu_busy_percent"))
                    .ok()
                    .and_then(|s| s.trim().parse::<u32>().ok())
                    .unwrap_or(0);

                // Get VRAM usage from sysfs
                let memory_total = fs::read_to_string(device_path.join("mem_info_vram_total"))
                    .ok()
                    .and_then(|s| s.trim().parse::<u64>().ok())
                    .unwrap_or(0);

                let memory_used = fs::read_to_string(device_path.join("mem_info_vram_used"))
                    .ok()
                    .and_then(|s| s.trim().parse::<u64>().ok())
                    .unwrap_or(0);

                let memory_free = memory_total.saturating_sub(memory_used);
                let utilization_memory = if memory_total > 0 {
                    ((memory_used as f64 / memory_total as f64) * 100.0) as u32
                } else {
                    0
                };

                let temperature = Self::find_amd_temperature(&device_path).unwrap_or(0);
                let power_usage = Self::find_amd_power(&device_path).unwrap_or(0);
                let clock_graphics = Self::find_amd_clock(&device_path, "pp_dpm_sclk").unwrap_or(0);
                let clock_memory = Self::find_amd_clock(&device_path, "pp_dpm_mclk").unwrap_or(0);

                gpus.push(GpuInfo {
                    index,
                    name,
                    vendor: GpuVendor::Amd,
                    uuid: format!("amd-{}", index),
                    utilization_gpu,
                    utilization_memory,
                    memory_total,
                    memory_used,
                    memory_free,
                    temperature,
                    power_usage,
                    power_limit: 0,
                    fan_speed: None,
                    clock_graphics,
                    clock_memory,
                    encoder_utilization: None,
                    decoder_utilization: None,
                });

                index += 1;
            }
        }

        gpus
    }

    fn find_amd_temperature(device_path: &Path) -> Option<u32> {
        let hwmon_path = device_path.join("hwmon");
        if let Ok(entries) = fs::read_dir(&hwmon_path) {
            for entry in entries.flatten() {
                if let Ok(temp_str) = fs::read_to_string(entry.path().join("temp1_input")) {
                    if let Ok(temp_millidegrees) = temp_str.trim().parse::<u32>() {
                        return Some(temp_millidegrees / 1000);
                    }
                }
            }
        }
        None
    }

    fn find_amd_power(device_path: &Path) -> Option<u32> {
        let hwmon_path = device_path.join("hwmon");
        if let Ok(entries) = fs::read_dir(&hwmon_path) {
            for entry in entries.flatten() {
                if let Ok(power_str) = fs::read_to_string(entry.path().join("power1_average")) {
                    if let Ok(power_microwatts) = power_str.trim().parse::<u32>() {
                        return Some(power_microwatts / 1000);
                    }
                }
            }
        }
        None
    }

    fn find_amd_clock(device_path: &Path, clock_file: &str) -> Option<u32> {
        if let Ok(clock_str) = fs::read_to_string(device_path.join(clock_file)) {
            for line in clock_str.lines() {
                if line.contains('*') {
                    if let Some(freq) = line.split_whitespace().nth(1) {
                        if let Some(num_str) = freq
                            .strip_suffix("Mhz")
                            .or_else(|| freq.strip_suffix("MHz"))
                        {
                            return num_str.parse::<u32>().ok();
                        }
                    }
                }
            }
        }
        None
    }

    /// Get Intel GPU information via sysfs/RC6
    fn get_intel_gpus(&self, _errors: &mut Vec<String>) -> Vec<GpuInfo> {
        let mut gpus = Vec::new();
        let drm_path = Path::new("/sys/class/drm");

        if !drm_path.exists() {
            return gpus;
        }

        if let Ok(entries) = fs::read_dir(drm_path) {
            let mut index = 0;
            for entry in entries.flatten() {
                let path = entry.path();
                let device_path = path.join("device");

                // Check if this is an Intel GPU (vendor 0x8086)
                let vendor_path = device_path.join("vendor");
                if let Ok(vendor) = fs::read_to_string(&vendor_path) {
                    if !vendor.trim().eq_ignore_ascii_case("0x8086") {
                        continue;
                    }
                } else {
                    continue;
                }

                let name = "Intel Integrated GPU".to_string();

                // Intel GPU frequency (current)
                let clock_graphics = fs::read_to_string(path.join("gt/gt0/rps_cur_freq_mhz"))
                    .ok()
                    .and_then(|s| s.trim().parse::<u32>().ok())
                    .unwrap_or(0);

                // Check for RC6 residency to calculate utilization
                // Path: /sys/class/drm/cardX/gt/gt0/rc6_residency_ms
                let rc6_path = path.join("gt/gt0/rc6_residency_ms");
                let mut utilization_gpu = 0;

                if rc6_path.exists() {
                    let current_time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as u64;

                    if let Ok(rc6_str) = fs::read_to_string(&rc6_path) {
                        if let Ok(rc6_ms) = rc6_str.trim().parse::<u64>() {
                            // Calculate delta
                            // We need a write lock to update state, but we only have `&self`.
                            // This means we need to upgrade or just use the lock.
                            if let Ok(mut state) = self.state.write() {
                                if let Some((last_rc6, last_time)) =
                                    state.last_rc6_readings.get(&index)
                                {
                                    let delta_time = current_time.saturating_sub(*last_time);
                                    let delta_rc6 = rc6_ms.saturating_sub(*last_rc6);

                                    if delta_time > 0 {
                                        // RC6 is "idle time". Utilization = 1.0 - (idle / total)
                                        // Cap at 100% just in case
                                        let idle_percent =
                                            (delta_rc6 as f64 / delta_time as f64) * 100.0;
                                        utilization_gpu =
                                            (100.0 - idle_percent).clamp(0.0, 100.0) as u32;
                                    }
                                }
                                // Update state
                                state
                                    .last_rc6_readings
                                    .insert(index, (rc6_ms, current_time));
                            }
                        }
                    }
                }

                gpus.push(GpuInfo {
                    index,
                    name,
                    vendor: GpuVendor::Intel,
                    uuid: format!("intel-{}", index),
                    utilization_gpu,
                    utilization_memory: 0,
                    memory_total: 0,
                    memory_used: 0,
                    memory_free: 0,
                    temperature: 0,
                    power_usage: 0,
                    power_limit: 0,
                    fan_speed: None,
                    clock_graphics,
                    clock_memory: 0,
                    encoder_utilization: None,
                    decoder_utilization: None,
                });

                index += 1;
            }
        }

        gpus
    }

    pub fn refresh(&self) -> GpusInfo {
        // Lazy initialize NVML
        {
            let mut state = self.state.write().expect("GPU state RwLock poisoned");
            if !state.initialized {
                state.nvml = Nvml::init().ok();
                state.initialized = true;
            }
        }

        let mut all_gpus = Vec::new();
        let mut driver_version: Option<String> = None;
        let mut errors = Vec::new();

        all_gpus.extend(self.get_nvidia_gpus(&mut driver_version, &mut errors));
        all_gpus.extend(self.get_amd_gpus(&mut errors));
        all_gpus.extend(self.get_intel_gpus(&mut errors));

        let nvidia_available = !all_gpus
            .iter()
            .all(|g| matches!(g.vendor, GpuVendor::Unknown));
        let amd_available = all_gpus.iter().any(|g| matches!(g.vendor, GpuVendor::Amd));
        let intel_available = all_gpus
            .iter()
            .any(|g| matches!(g.vendor, GpuVendor::Intel));

        GpusInfo {
            gpus: all_gpus,
            nvidia_available,
            amd_available,
            intel_available,
            driver_version,
            errors,
        }
    }
}

impl Default for GpuMonitor {
    fn default() -> Self {
        Self::new()
    }
}
