// Sensors Monitoring Module
// Provides temperature, fan speed, and sensor readings from hardware

use serde::{Deserialize, Serialize};
use sysinfo::Components;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// Sensor reading type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensorType {
    Temperature,
    Fan,
    Voltage,
    Power,
    Unknown,
}

/// Individual sensor reading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    pub label: String,
    pub sensor_type: SensorType,
    pub value: f32,
    pub max_value: Option<f32>,
    pub critical_value: Option<f32>,
    pub unit: String,
}

/// All sensor readings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorsInfo {
    pub sensors: Vec<SensorReading>,
    pub cpu_temp: Option<f32>,
    pub gpu_temp: Option<f32>,
}

/// Cached sensor data to prevent frequent rescanning
struct SensorCache {
    data: SensorsInfo,
    last_update: Instant,
}

/// Sensors Monitor state with caching
pub struct SensorsMonitor {
    components: RwLock<Option<Components>>,
    cache: RwLock<Option<SensorCache>>,
}

// Minimum time between full sensor refreshes (2 seconds)
const MIN_REFRESH_INTERVAL: Duration = Duration::from_secs(2);

impl SensorsMonitor {
    pub fn new() -> Self {
        // Don't initialize components here - do it lazily on first refresh
        Self {
            components: RwLock::new(None),
            cache: RwLock::new(None),
        }
    }

    pub fn refresh(&self) -> SensorsInfo {
        // Check cache first
        {
            let cache = self.cache.read()
                .expect("Sensors cache RwLock poisoned");
            if let Some(ref cached) = *cache {
                if cached.last_update.elapsed() < MIN_REFRESH_INTERVAL {
                    return cached.data.clone();
                }
            }
        }

        // Need to refresh - get write lock
        let mut components_guard = self.components.write()
            .expect("Sensors monitor RwLock poisoned");
        
        // Initialize components lazily if needed
        if components_guard.is_none() {
            *components_guard = Some(Components::new_with_refreshed_list());
        }
        
        let components = components_guard.as_mut().unwrap();
        
        // Only refresh the list, don't recreate it
        components.refresh_list();
        
        let mut sensors: Vec<SensorReading> = Vec::new();
        let mut cpu_temp: Option<f32> = None;
        let mut gpu_temp: Option<f32> = None;

        for component in components.iter() {
            let label = component.label().to_string();
            let temperature = component.temperature();
            let max = component.max();
            let critical = component.critical();

            // Try to identify CPU and GPU temps with more robust detection
            let label_lower = label.to_lowercase();
            
            // CPU temperature detection
            if cpu_temp.is_none() && (
                label_lower.contains("cpu") ||
                label_lower.contains("core") ||
                label_lower.contains("tctl") ||
                label_lower.contains("tdie") ||
                label_lower.contains("package")
            ) {
                cpu_temp = Some(temperature);
            }
            
            // GPU temperature detection
            if gpu_temp.is_none() && (
                label_lower.contains("gpu") ||
                label_lower.contains("edge") ||
                label_lower.contains("junction") ||
                label_lower.contains("radeon") ||
                label_lower.contains("nvidia") ||
                label_lower.contains("amdgpu")
            ) {
                gpu_temp = Some(temperature);
            }

            sensors.push(SensorReading {
                label,
                sensor_type: SensorType::Temperature,
                value: temperature,
                max_value: Some(max),
                critical_value: critical,
                unit: "°C".to_string(),
            });
        }

        let result = SensorsInfo {
            sensors,
            cpu_temp,
            gpu_temp,
        };

        // Update cache
        {
            let mut cache = self.cache.write()
                .expect("Sensors cache RwLock poisoned");
            *cache = Some(SensorCache {
                data: result.clone(),
                last_update: Instant::now(),
            });
        }

        result
    }
}

impl Default for SensorsMonitor {
    fn default() -> Self {
        Self::new()
    }
}
