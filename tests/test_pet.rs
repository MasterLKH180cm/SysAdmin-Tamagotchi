// Integration tests for Pet state transitions
// Tests pet behavior based on metrics

use sysadmin_tamagotchi::{Pet, PetState, Metrics};

#[test]
fn test_pet_creation() {
    let pet = Pet::new();
    assert_eq!(pet.get_state(), PetState::Happy);
    println!("Pet creation test passed: {}", pet.get_icon_description());
}

#[test]
fn test_pet_happy_state() {
    let mut pet = Pet::new();

    // All metrics good (< 70%)
    let metrics = Metrics {
        ram_percent: 50.0,
        cpu_percent: 50.0,
        disk_junk_percent: 3.0,
        disk_junk_mb: 100,
        total_disk_mb: 256000,
    };

    let state = pet.update(&metrics);
    assert_eq!(state, PetState::Happy);
    assert_eq!(pet.get_emoji(), "😊");
    println!("Happy state test passed");
}

#[test]
fn test_pet_okay_state() {
    let mut pet = Pet::new();

    // One metric in warning range (70-85%)
    let metrics = Metrics {
        ram_percent: 75.0,  // Warning
        cpu_percent: 50.0,  // Good
        disk_junk_percent: 3.0,  // Good
        disk_junk_mb: 100,
        total_disk_mb: 256000,
    };

    let state = pet.update(&metrics);
    assert_eq!(state, PetState::Okay);
    assert_eq!(pet.get_emoji(), "😐");
    println!("Okay state test passed");
}

#[test]
fn test_pet_stressed_state() {
    let mut pet = Pet::new();

    // Two metrics in warning range
    let metrics = Metrics {
        ram_percent: 75.0,  // Warning
        cpu_percent: 80.0,  // Warning
        disk_junk_percent: 3.0,  // Good
        disk_junk_mb: 100,
        total_disk_mb: 256000,
    };

    let state = pet.update(&metrics);
    assert_eq!(state, PetState::Stressed);
    assert_eq!(pet.get_emoji(), "😰");
    println!("Stressed state test passed");
}

#[test]
fn test_pet_critical_state_ram() {
    let mut pet = Pet::new();

    // RAM critical (>= 95%)
    let metrics = Metrics {
        ram_percent: 96.0,  // Critical
        cpu_percent: 50.0,  // Good
        disk_junk_percent: 3.0,  // Good
        disk_junk_mb: 100,
        total_disk_mb: 256000,
    };

    let state = pet.update(&metrics);
    assert_eq!(state, PetState::Critical);
    assert_eq!(pet.get_emoji(), "🔥");
    println!("Critical state (RAM) test passed");
}

#[test]
fn test_pet_critical_state_cpu() {
    let mut pet = Pet::new();

    // CPU critical (>= 95%)
    let metrics = Metrics {
        ram_percent: 50.0,  // Good
        cpu_percent: 97.0,  // Critical
        disk_junk_percent: 3.0,  // Good
        disk_junk_mb: 100,
        total_disk_mb: 256000,
    };

    let state = pet.update(&metrics);
    assert_eq!(state, PetState::Critical);
    assert_eq!(pet.get_emoji(), "🔥");
    println!("Critical state (CPU) test passed");
}

#[test]
fn test_pet_critical_state_disk() {
    let mut pet = Pet::new();

    // Disk critical (>= 20%)
    let metrics = Metrics {
        ram_percent: 50.0,  // Good
        cpu_percent: 50.0,  // Good
        disk_junk_percent: 25.0,  // Critical (25% of disk)
        disk_junk_mb: 64000,  // 64GB
        total_disk_mb: 256000,  // 256GB total
    };

    let state = pet.update(&metrics);
    assert_eq!(state, PetState::Critical);
    assert_eq!(pet.get_emoji(), "🔥");
    println!("Critical state (Disk) test passed");
}

#[test]
fn test_pet_state_transitions() {
    let mut pet = Pet::new();

    // Start happy
    let happy_metrics = Metrics {
        ram_percent: 50.0,
        cpu_percent: 50.0,
        disk_junk_percent: 3.0,
        disk_junk_mb: 100,
        total_disk_mb: 256000,
    };
    assert_eq!(pet.update(&happy_metrics), PetState::Happy);

    // Transition to okay
    let okay_metrics = Metrics {
        ram_percent: 75.0,
        cpu_percent: 50.0,
        disk_junk_percent: 3.0,
        disk_junk_mb: 100,
        total_disk_mb: 256000,
    };
    assert_eq!(pet.update(&okay_metrics), PetState::Okay);

    // Transition to stressed
    let stressed_metrics = Metrics {
        ram_percent: 75.0,
        cpu_percent: 80.0,
        disk_junk_percent: 3.0,
        disk_junk_mb: 100,
        total_disk_mb: 256000,
    };
    assert_eq!(pet.update(&stressed_metrics), PetState::Stressed);

    // Transition to critical
    let critical_metrics = Metrics {
        ram_percent: 96.0,
        cpu_percent: 80.0,
        disk_junk_percent: 3.0,
        disk_junk_mb: 100,
        total_disk_mb: 256000,
    };
    assert_eq!(pet.update(&critical_metrics), PetState::Critical);

    // Transition back to happy
    assert_eq!(pet.update(&happy_metrics), PetState::Happy);

    println!("State transition test passed");
}

#[test]
fn test_updated_thresholds() {
    let mut pet = Pet::new();

    // Test new RAM threshold boundaries
    // 69% = Good
    let metrics_69 = Metrics {
        ram_percent: 69.0,
        cpu_percent: 50.0,
        disk_junk_percent: 3.0,
        disk_junk_mb: 100,
        total_disk_mb: 256000,
    };
    pet.update(&metrics_69);
    assert_eq!(pet.get_state(), PetState::Happy);

    // 70% = Warning (Okay)
    let metrics_70 = Metrics {
        ram_percent: 70.0,
        cpu_percent: 50.0,
        disk_junk_percent: 3.0,
        disk_junk_mb: 100,
        total_disk_mb: 256000,
    };
    pet.update(&metrics_70);
    assert_eq!(pet.get_state(), PetState::Okay);

    // 95% = Critical
    let metrics_95 = Metrics {
        ram_percent: 95.0,
        cpu_percent: 50.0,
        disk_junk_percent: 3.0,
        disk_junk_mb: 100,
        total_disk_mb: 256000,
    };
    pet.update(&metrics_95);
    assert_eq!(pet.get_state(), PetState::Critical);

    println!("Updated thresholds test passed");
    println!("  70%+ = Warning (Okay)");
    println!("  85%+ = Warning (Stressed if multiple)");
    println!("  95%+ = Critical");
}
