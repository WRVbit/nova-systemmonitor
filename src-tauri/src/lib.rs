// Nova System Monitor - Main Library
// Tauri v2 commands for system monitoring with thread-safe shared state

mod modules;

use modules::{
    cpu::CpuMonitor,
    memory::MemoryMonitor,
    disk::DiskMonitor,
    network::NetworkMonitor,
    process::ProcessMonitor,
    gpu::GpuMonitor,
    sensors::SensorsMonitor,
    system::SystemMonitor,
};
use std::sync::Arc;
use tauri::State;

/// Application state containing all monitors (thread-safe)
pub struct AppState {
    pub cpu: Arc<CpuMonitor>,
    pub memory: Arc<MemoryMonitor>,
    pub disk: Arc<DiskMonitor>,
    pub network: Arc<NetworkMonitor>,
    pub process: Arc<ProcessMonitor>,
    pub gpu: Arc<GpuMonitor>,
    pub sensors: Arc<SensorsMonitor>,
    pub system: Arc<SystemMonitor>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            cpu: Arc::new(CpuMonitor::new()),
            memory: Arc::new(MemoryMonitor::new()),
            disk: Arc::new(DiskMonitor::new()),
            network: Arc::new(NetworkMonitor::new()),
            process: Arc::new(ProcessMonitor::new()),
            gpu: Arc::new(GpuMonitor::new()),
            sensors: Arc::new(SensorsMonitor::new()),
            system: Arc::new(SystemMonitor::new()),
        }
    }
}

// ============================================================================
// CPU Commands (Async)
// ============================================================================

#[tauri::command]
async fn get_cpu_info(state: State<'_, AppState>) -> Result<modules::cpu::CpuInfo, String> {
    let cpu = Arc::clone(&state.cpu);
    tokio::task::spawn_blocking(move || {
        Ok(cpu.refresh())
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

// ============================================================================
// Memory Commands (Async)
// ============================================================================

#[tauri::command]
async fn get_memory_info(state: State<'_, AppState>) -> Result<modules::memory::MemoryInfo, String> {
    let memory = Arc::clone(&state.memory);
    tokio::task::spawn_blocking(move || {
        Ok(memory.refresh())
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

// ============================================================================
// Disk Commands (Async)
// ============================================================================

#[tauri::command]
async fn get_disk_info(state: State<'_, AppState>) -> Result<modules::disk::DisksInfo, String> {
    let disk = Arc::clone(&state.disk);
    tokio::task::spawn_blocking(move || {
        Ok(disk.refresh())
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

// ============================================================================
// Network Commands (Async)
// ============================================================================

#[tauri::command]
async fn get_network_info(state: State<'_, AppState>) -> Result<modules::network::NetworkInfo, String> {
    let network = Arc::clone(&state.network);
    tokio::task::spawn_blocking(move || {
        Ok(network.refresh())
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

// ============================================================================
// Process Commands (Async)
// ============================================================================

#[tauri::command]
async fn get_processes(state: State<'_, AppState>) -> Result<modules::process::ProcessList, String> {
    let process = Arc::clone(&state.process);
    tokio::task::spawn_blocking(move || {
        Ok(process.refresh())
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

#[tauri::command]
async fn kill_process(state: State<'_, AppState>, pid: u32, force: bool) -> Result<bool, String> {
    let process = Arc::clone(&state.process);
    tokio::task::spawn_blocking(move || {
        process.kill_process(pid, force).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

#[tauri::command]
async fn set_process_priority(state: State<'_, AppState>, pid: u32, nice: i32) -> Result<(), String> {
    let process = Arc::clone(&state.process);
    tokio::task::spawn_blocking(move || {
        process.set_priority(pid, nice).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

// ============================================================================
// GPU Commands (Async)
// ============================================================================

#[tauri::command]
async fn get_gpu_info(state: State<'_, AppState>) -> Result<modules::gpu::GpusInfo, String> {
    let gpu = Arc::clone(&state.gpu);
    tokio::task::spawn_blocking(move || {
        Ok(gpu.refresh())
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

// ============================================================================
// Sensors Commands (Async)
// ============================================================================

#[tauri::command]
async fn get_sensors_info(state: State<'_, AppState>) -> Result<modules::sensors::SensorsInfo, String> {
    let sensors = Arc::clone(&state.sensors);
    tokio::task::spawn_blocking(move || {
        Ok(sensors.refresh())
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

// ============================================================================
// System Commands (Sync - no blocking I/O)
// ============================================================================

#[tauri::command]
fn get_system_info(state: State<'_, AppState>) -> modules::system::SystemInfo {
    state.system.refresh()
}

// ============================================================================
// Application Entry Point
// ============================================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            // CPU
            get_cpu_info,
            // Memory
            get_memory_info,
            // Disk
            get_disk_info,
            // Network
            get_network_info,
            // Process
            get_processes,
            kill_process,
            set_process_priority,
            // GPU
            get_gpu_info,
            // Sensors
            get_sensors_info,
            // System
            get_system_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Nova System Monitor");
}
