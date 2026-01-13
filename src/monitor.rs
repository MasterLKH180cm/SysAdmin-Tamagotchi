use sysinfo::System;
use std::time::Instant;

/// System metrics snapshot
#[derive(Debug, Clone)]
pub struct Metrics {
    pub ram_percent: f32,
    pub cpu_percent: f32,
    pub disk_junk_mb: u64,
}

/// System monitor that wraps sysinfo and provides clean metrics API
pub struct SystemMonitor {
    sys: System,
    last_update: Instant,
}

impl SystemMonitor {
    /// Create a new system monitor
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self {
            sys,
            last_update: Instant::now(),
        }
    }

    /// Refresh system information (call before get_metrics)
    pub fn refresh(&mut self) {
        self.sys.refresh_memory();
        self.sys.refresh_cpu_all();
        self.last_update = Instant::now();
    }

    /// Get current system metrics
    pub fn get_metrics(&self) -> Metrics {
        let ram_percent = self.get_ram_percent();
        let cpu_percent = self.get_cpu_percent();
        let disk_junk_mb = self.get_disk_junk_mb();

        Metrics {
            ram_percent,
            cpu_percent,
            disk_junk_mb,
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

    /// Get disk junk size in MB (scans Windows TEMP folder)
    fn get_disk_junk_mb(&self) -> u64 {
        // Get Windows TEMP directory
        let temp_dir = std::env::var("TEMP")
            .or_else(|_| std::env::var("TMP"))
            .unwrap_or_else(|_| "C:\\Windows\\Temp".to_string());

        // Calculate total size of files in TEMP
        match calculate_dir_size(&temp_dir) {
            Ok(bytes) => bytes / (1024 * 1024), // Convert to MB
            Err(_) => 0, // Return 0 if we can't access TEMP
        }
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
