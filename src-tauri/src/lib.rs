// Library module for SysAdmin Tamagotchi
// Exposes modules for testing

pub mod commands;
pub mod monitor;
pub mod pet;
pub mod poller;

// Re-export key types for testing
pub use monitor::{Metrics, SystemMonitor};
pub use pet::{Pet, PetState};
