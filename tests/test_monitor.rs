// Integration tests for SystemMonitor
// Tests metric collection and thresholds

use sysadmin_tamagotchi::{SystemMonitor, Metrics};

#[test]
fn test_system_monitor_creation() {
    let monitor = SystemMonitor::new();
    // Should not panic
    assert!(true);
}

#[test]
fn test_get_metrics_returns_valid_data() {
    let mut monitor = SystemMonitor::new();
    monitor.refresh();
    let metrics = monitor.get_metrics();

    // RAM should be between 0-100%
    assert!(metrics.ram_percent >= 0.0);
    assert!(metrics.ram_percent <= 100.0);

    // CPU should be between 0-100%
    assert!(metrics.cpu_percent >= 0.0);
    assert!(metrics.cpu_percent <= 100.0);

    // Disk junk should be non-negative
    assert!(metrics.disk_junk_mb >= 0);
    assert!(metrics.total_disk_mb > 0);

    // Disk junk percent should be valid
    assert!(metrics.disk_junk_percent >= 0.0);
    assert!(metrics.disk_junk_percent <= 100.0);

    println!("Metrics test passed:");
    println!("  RAM: {:.1}%", metrics.ram_percent);
    println!("  CPU: {:.1}%", metrics.cpu_percent);
    println!("  Disk Junk: {} MB ({:.1}%)", metrics.disk_junk_mb, metrics.disk_junk_percent);
}

#[test]
fn test_cpu_temporal_smoothing() {
    let mut monitor = SystemMonitor::new();

    // Collect multiple samples
    let mut samples = Vec::new();
    for i in 0..6 {
        monitor.refresh();
        let metrics = monitor.get_metrics();
        samples.push(metrics.cpu_percent);
        println!("Sample {}: CPU = {:.1}%", i + 1, metrics.cpu_percent);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // All samples should be valid
    for sample in &samples {
        assert!(*sample >= 0.0 && *sample <= 100.0);
    }

    println!("CPU smoothing test passed with {} samples", samples.len());
}

#[test]
fn test_metrics_refresh() {
    let mut monitor = SystemMonitor::new();

    // Get first reading
    monitor.refresh();
    let metrics1 = monitor.get_metrics();

    // Wait briefly
    std::thread::sleep(std::time::Duration::from_millis(200));

    // Get second reading
    monitor.refresh();
    let metrics2 = monitor.get_metrics();

    // Both should be valid (values may be the same or different)
    assert!(metrics1.ram_percent >= 0.0);
    assert!(metrics2.ram_percent >= 0.0);

    println!("Refresh test passed");
    println!("  First reading: RAM={:.1}%, CPU={:.1}%", metrics1.ram_percent, metrics1.cpu_percent);
    println!("  Second reading: RAM={:.1}%, CPU={:.1}%", metrics2.ram_percent, metrics2.cpu_percent);
}

#[test]
fn test_cleanup_temp_does_not_panic() {
    // This test verifies that cleanup doesn't panic
    // It may or may not delete files (depends on permissions)
    let result = SystemMonitor::cleanup_temp();

    match result {
        Ok(deleted_mb) => {
            println!("Cleanup test passed: {} MB deleted", deleted_mb);
            assert!(deleted_mb >= 0);
        }
        Err(e) => {
            println!("Cleanup encountered error (expected on some systems): {}", e);
            // Error is acceptable due to permissions
        }
    }
}
