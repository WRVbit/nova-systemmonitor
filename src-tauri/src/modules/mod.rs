// Nova System Monitor - Modules
// Core system monitoring functionality

pub mod cpu;
pub mod disk;
pub mod gpu;
pub mod memory;
pub mod network;
pub mod process;
pub mod sensors;
pub mod system;

use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum MonitorError {
    #[error("Failed to access system information: {0}")]
    SystemAccess(String),

    #[error("GPU not available: {0}")]
    GpuNotAvailable(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Process not found: {0}")]
    ProcessNotFound(u32),
}

impl From<MonitorError> for String {
    fn from(err: MonitorError) -> Self {
        err.to_string()
    }
}

// Common traits or structs can go here if needed across modules
