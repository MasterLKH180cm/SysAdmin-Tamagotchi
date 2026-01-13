use serde::{Deserialize, Serialize};
use sysinfo::System;
use std::collections::VecDeque;
use std::time::Instant;

/// System metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub ram_percent: f32,
    pub cpu_percent: f32,
    pub disk_junk_percent: f32,
    pub disk_junk_mb: u64,
    pub total_disk_mb: u64,
}

/// System monitor that wraps sysinfo and provides clean metrics API
pub struct SystemMonitor {
    sys: System,
    last_update: Instant,
    // CPU smoothing buffer - stores last 30 seconds of readings (6 samples at 5s interval)
    cpu_history: VecDeque<f32>,
    max_history_size: usize,
}

impl SystemMonitor {
    /// Create a new system monitor
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self {
            sys,
            last_update: Instant::now(),
            cpu_history: VecDeque::with_capacity(6), // 30 seconds / 5 seconds = 6 samples
            max_history_size: 6,
        }
    }

    /// Refresh system information (call before get_metrics)
    pub fn refresh(&mut self) {
        self.sys.refresh_memory();
        self.sys.refresh_cpu_all();
        self.last_update = Instant::now();
    }

    /// Get current system metrics with temporal smoothing for CPU
    pub fn get_metrics(&mut self) -> Metrics {
        let ram_percent = self.get_ram_percent();
        let raw_cpu = self.get_cpu_percent();

        // Add to CPU history buffer
        self.cpu_history.push_back(raw_cpu);
        if self.cpu_history.len() > self.max_history_size {
            self.cpu_history.pop_front();
        }

        // Calculate smoothed CPU (average of last 30 seconds)
        let cpu_percent = if !self.cpu_history.is_empty() {
            self.cpu_history.iter().sum::<f32>() / self.cpu_history.len() as f32
        } else {
            raw_cpu
        };

        let (disk_junk_mb, total_disk_mb) = self.get_disk_junk();
        let disk_junk_percent = if total_disk_mb > 0 {
            (disk_junk_mb as f32 / total_disk_mb as f32) * 100.0
        } else {
            0.0
        };

        Metrics {
            ram_percent,
            cpu_percent,
            disk_junk_percent,
            disk_junk_mb,
            total_disk_mb,
        }
    }

    /// Get RAM usage as percentage (0-100)
    fn get_ram_percent(&self) -> f32 {
        let total = self.sys.total_memory() as f64;
        let used = self.sys.used_memory() as f64;

        if total > 0.0 {
            ((used / total) * 100.0) as f32
        } else {
            0.0
        }
    }

    /// Get CPU usage as percentage (0-100)
    fn get_cpu_percent(&self) -> f32 {
        self.sys.global_cpu_usage()
    }

    /// Get disk junk size in MB and total disk size (scans Windows TEMP folder)
    /// Returns (junk_mb, total_disk_mb)
    fn get_disk_junk(&self) -> (u64, u64) {
        // Get Windows TEMP directory
        let temp_dir = std::env::var("TEMP")
            .or_else(|_| std::env::var("TMP"))
            .unwrap_or_else(|_| "C:\\Windows\\Temp".to_string());

        // Get total disk size for the drive containing TEMP
        let total_disk_mb = self.get_total_disk_size(&temp_dir);

        // Calculate total size of files in TEMP
        let junk_bytes = match calculate_dir_size(&temp_dir) {
            Ok(bytes) => bytes,
            Err(_) => 0,
        };

        let junk_mb = junk_bytes / (1024 * 1024);

        (junk_mb, total_disk_mb)
    }

    /// Get total disk size in MB for the drive containing the given path
    fn get_total_disk_size(&self, _path: &str) -> u64 {
        // Get disk information from sysinfo
        let disks = sysinfo::Disks::new_with_refreshed_list();

        // For simplicity, get the C: drive total space
        // In production, we'd parse the path to determine which disk
        for disk in disks.list() {
            if disk.mount_point().to_string_lossy().starts_with("C:") {
                return disk.total_space() / (1024 * 1024); // Convert to MB
            }
        }

        // Default fallback: 256GB
        256 * 1024
    }

    /// Clean up temp files (Windows TEMP directory)
    pub fn cleanup_temp() -> Result<u64, std::io::Error> {
        let temp_dir = std::env::var("TEMP")
            .or_else(|_| std::env::var("TMP"))
            .unwrap_or_else(|_| "C:\\Windows\\Temp".to_string());

        let mut deleted_bytes = 0u64;

        if let Ok(entries) = std::fs::read_dir(&temp_dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        let size = metadata.len();
                        // Attempt to delete file (ignore errors for locked files)
                        if std::fs::remove_file(entry.path()).is_ok() {
                            deleted_bytes += size;
                        }
                    }
                }
            }
        }

        Ok(deleted_bytes / (1024 * 1024)) // Return MB deleted
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate total size of a directory in bytes
fn calculate_dir_size(path: &str) -> std::io::Result<u64> {
    let mut total_size = 0u64;

    // Use a simple recursive calculation with basic error handling
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    total_size += metadata.len();
                } else if metadata.is_dir() {
                    // Recursively calculate subdirectories
                    // Note: This is a simplified version. Could add depth limit
                    // or timeout for production use to avoid hanging on large dirs
                    if let Some(path_str) = entry.path().to_str().map(String::from) {
                        total_size += calculate_dir_size(&path_str).unwrap_or(0);
                    }
                }
            }
        }
    }

    Ok(total_size)
}
