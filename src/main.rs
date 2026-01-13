mod monitor;
mod pet;

use monitor::SystemMonitor;
use pet::Pet;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayIconBuilder,
};
use std::time::{Duration, Instant};

/// Main application state
struct App {
    monitor: SystemMonitor,
    pet: Pet,
}

impl App {
    /// Create a new application instance
    fn new() -> Self {
        Self {
            monitor: SystemMonitor::new(),
            pet: Pet::new(),
        }
    }

    /// Update system metrics and pet state
    /// Returns true if pet state changed
    fn update(&mut self) -> bool {
        // Refresh system information
        self.monitor.refresh();

        // Get current metrics
        let metrics = self.monitor.get_metrics();

        // Update pet state
        let old_state = self.pet.get_state();
        let new_state = self.pet.update(&metrics);

        // Print current status (for debugging)
        println!(
            "RAM: {:.1}% | CPU: {:.1}% | Junk: {}MB | State: {:?}",
            metrics.ram_percent, metrics.cpu_percent, metrics.disk_junk_mb, new_state
        );

        // Return true if state changed
        old_state != new_state
    }
}

fn main() {
    println!("Starting SysAdmin Tamagotchi...");

    // Create application state
    let mut app = App::new();

    // Create tray menu
    let menu = create_menu();

    // Create tray icon
    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("SysAdmin Tamagotchi")
        .build()
        .expect("Failed to create tray icon");

    println!("Tray icon created. Pet is running in your system tray!");
    println!("Right-click the tray icon to exit.");

    // Menu event receiver
    let menu_channel = MenuEvent::receiver();

    // Simple polling loop
    let mut last_poll = Instant::now();
    let poll_interval = Duration::from_secs(5);

    loop {
        // Check for menu events
        if let Ok(event) = menu_channel.try_recv() {
            if event.id == "exit" {
                println!("Exiting SysAdmin Tamagotchi...");
                break;
            }
        }

        // Update pet state every 5 seconds
        if last_poll.elapsed() >= poll_interval {
            let state_changed = app.update();

            if state_changed {
                // In a full implementation, we would update the tray icon here
                // based on app.pet.get_state()
                println!("Pet state changed: {}", app.pet.get_icon_description());
            }

            last_poll = Instant::now();
        }

        // Sleep briefly to avoid spinning CPU
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// Create the tray icon context menu
fn create_menu() -> Menu {
    let menu = Menu::new();

    // Exit menu item
    let exit_item = MenuItem::with_id("exit", "Exit", true, None);

    menu.append(&exit_item).expect("Failed to add menu item");

    menu
}
