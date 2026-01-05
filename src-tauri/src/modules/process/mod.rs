// Process Monitoring Module
// Provides process listing, details, and management with priority control

use crate::modules::MonitorError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;
use sysinfo::{Pid, ProcessStatus, Signal, System};

/// Status of a process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcStatus {
    Running,
    Sleeping,
    Stopped,
    Zombie,
    Dead,
    Unknown,
}

impl From<ProcessStatus> for ProcStatus {
    fn from(status: ProcessStatus) -> Self {
        match status {
            ProcessStatus::Run => ProcStatus::Running,
            ProcessStatus::Sleep => ProcStatus::Sleeping,
            ProcessStatus::Stop => ProcStatus::Stopped,
            ProcessStatus::Zombie => ProcStatus::Zombie,
            ProcessStatus::Dead => ProcStatus::Dead,
            _ => ProcStatus::Unknown,
        }
    }
}

/// Information about a single process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub parent_pid: Option<u32>,
    pub name: String,
    pub exe_path: String,
    pub command: Vec<String>,
    pub status: ProcStatus,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
    pub memory_percent: f32,
    pub start_time: u64,
    pub run_time: u64,
    pub user_id: Option<String>,
    pub nice: i32,
    pub instance_count: Option<u32>, // Number of instances when grouped
}

/// Process list result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessList {
    pub processes: Vec<ProcessInfo>,
    pub total_count: usize,
}

/// Process Monitor state with lazy initialization
pub struct ProcessMonitor {
    system: RwLock<Option<System>>,
}

impl ProcessMonitor {
    pub fn new() -> Self {
        // Don't initialize System here - do it lazily
        Self {
            system: RwLock::new(None),
        }
    }

    pub fn refresh(&self) -> ProcessList {
        let mut sys_guard = self
            .system
            .write()
            .expect("Process monitor RwLock poisoned - fatal error");

        // Lazy initialization
        if sys_guard.is_none() {
            let mut system = System::new_all();
            system.refresh_all();
            *sys_guard = Some(system);
        }

        let sys = sys_guard.as_mut().unwrap();
        sys.refresh_all();

        let total_memory = sys.total_memory();
        let mut processes: Vec<ProcessInfo> = Vec::new();

        for (pid, process) in sys.processes() {
            let memory = process.memory();
            let memory_percent = if total_memory > 0 {
                (memory as f32 / total_memory as f32) * 100.0
            } else {
                0.0
            };

            processes.push(ProcessInfo {
                pid: pid.as_u32(),
                parent_pid: process.parent().map(|p| p.as_u32()),
                name: process.name().to_string_lossy().to_string(),
                exe_path: process
                    .exe()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default(),
                command: process
                    .cmd()
                    .iter()
                    .map(|s| s.to_string_lossy().to_string())
                    .collect(),
                status: process.status().into(),
                cpu_usage: process.cpu_usage(),
                memory_bytes: memory,
                memory_percent,
                start_time: process.start_time(),
                run_time: process.run_time(),
                user_id: process.user_id().map(|u| u.to_string()),
                nice: unsafe {
                    // Clear errno
                    *libc::__errno_location() = 0;
                    let val = libc::getpriority(0, pid.as_u32());
                    if val == -1 && *libc::__errno_location() != 0 {
                        0
                    } else {
                        val
                    }
                },
                instance_count: None, // Will be set if grouped
            });
        }

        // Group processes by name
        let mut groups: std::collections::HashMap<String, ProcessInfo> =
            std::collections::HashMap::new();

        for p in processes {
            groups
                .entry(p.name.clone())
                .and_modify(|e| {
                    // CPU: sum usage
                    e.cpu_usage += p.cpu_usage;
                    // Memory: keep existing (assuming main process/shared memory)
                    // Instance count: increment
                    e.instance_count = Some(e.instance_count.unwrap_or(1) + 1);

                    // Keep lowest PID as representative (usually main thread/process)
                    if p.pid < e.pid {
                        e.pid = p.pid;
                        e.memory_bytes = p.memory_bytes;
                        e.memory_percent = p.memory_percent;
                        e.start_time = p.start_time;
                        e.user_id = p.user_id.clone();
                    }
                })
                .or_insert_with(|| {
                    let mut new_p = p.clone();
                    new_p.instance_count = Some(1);
                    new_p
                });
        }

        let mut grouped_processes: Vec<ProcessInfo> = groups.into_values().collect();

        // Sort by CPU usage descending by default
        grouped_processes.sort_by(|a, b| {
            b.cpu_usage
                .partial_cmp(&a.cpu_usage)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let total_count = grouped_processes.len();
        ProcessList {
            processes: grouped_processes,
            total_count,
        }
    }

    pub fn kill_process(&self, pid: u32, force: bool) -> Result<bool, MonitorError> {
        let sys_guard = self
            .system
            .read()
            .expect("Process monitor RwLock poisoned - fatal error");

        let sys = match sys_guard.as_ref() {
            Some(s) => s,
            None => {
                return Err(MonitorError::SystemAccess(
                    "Process monitor not initialized".to_string(),
                ))
            }
        };

        let pid = Pid::from_u32(pid);

        if let Some(process) = sys.process(pid) {
            let signal = if force { Signal::Kill } else { Signal::Term };
            process.kill_with(signal).ok_or_else(|| {
                MonitorError::PermissionDenied(format!(
                    "Failed to kill process {}: permission denied or process protected",
                    pid.as_u32()
                ))
            })
        } else {
            Err(MonitorError::ProcessNotFound(pid.as_u32()))
        }
    }

    /// Set process priority (nice value)
    /// nice: -20 (highest priority) to 19 (lowest priority)
    /// Requires root/CAP_SYS_NICE for nice < 0
    pub fn set_priority(&self, pid: u32, nice: i32) -> Result<(), MonitorError> {
        if nice < -20 || nice > 19 {
            return Err(MonitorError::PermissionDenied(
                "Nice value must be between -20 and 19".to_string(),
            ));
        }

        unsafe {
            // PRIO_PROCESS = 0
            let result = libc::setpriority(0, pid, nice);
            if result == -1 {
                let errno = *libc::__errno_location();
                match errno {
                    libc::EPERM => Err(MonitorError::PermissionDenied(
                        "Permission denied: negative nice values require CAP_SYS_NICE".to_string(),
                    )),
                    libc::ESRCH => Err(MonitorError::ProcessNotFound(pid)),
                    _ => Err(MonitorError::SystemAccess(format!(
                        "Failed to set priority: errno {}",
                        errno
                    ))),
                }
            } else {
                Ok(())
            }
        }
    }
}

impl Default for ProcessMonitor {
    fn default() -> Self {
        Self::new()
    }
}
