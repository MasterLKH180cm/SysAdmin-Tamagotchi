use crate::monitor::{Metrics, SystemMonitor};
use crate::pet::{Pet, PetState};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

/// Application state shared across Tauri commands
pub struct AppState {
    pub monitor: Mutex<SystemMonitor>,
    pub pet: Mutex<Pet>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            monitor: Mutex::new(SystemMonitor::new()),
            pet: Mutex::new(Pet::new()),
        }
    }
}

/// Response containing current system metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsResponse {
    pub metrics: Metrics,
    pub pet_state: PetState,
    pub pet_emoji: String,
    pub pet_description: String,
}

/// Tauri command: Get current system metrics and pet state
#[tauri::command]
pub async fn get_metrics(state: State<'_, AppState>) -> Result<MetricsResponse, String> {
    // Refresh and get metrics
    let mut monitor = state.monitor.lock().map_err(|e| e.to_string())?;
    monitor.refresh();
    let metrics = monitor.get_metrics();

    // Update pet state
    let mut pet = state.pet.lock().map_err(|e| e.to_string())?;
    let pet_state = pet.update(&metrics);

    Ok(MetricsResponse {
        metrics,
        pet_state,
        pet_emoji: pet.get_emoji().to_string(),
        pet_description: pet.get_icon_description().to_string(),
    })
}

/// Tauri command: Get current pet state without refreshing metrics
#[tauri::command]
pub async fn get_pet_state(state: State<'_, AppState>) -> Result<PetStateResponse, String> {
    let pet = state.pet.lock().map_err(|e| e.to_string())?;

    Ok(PetStateResponse {
        state: pet.get_state(),
        emoji: pet.get_emoji().to_string(),
        description: pet.get_icon_description().to_string(),
    })
}

/// Response containing pet state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetStateResponse {
    pub state: PetState,
    pub emoji: String,
    pub description: String,
}

/// Tauri command: Clean up temporary files
#[tauri::command]
pub async fn cleanup_temp() -> Result<CleanupResponse, String> {
    match SystemMonitor::cleanup_temp() {
        Ok(deleted_mb) => Ok(CleanupResponse {
            success: true,
            deleted_mb,
            message: format!("Successfully cleaned up {} MB of temporary files", deleted_mb),
        }),
        Err(e) => Ok(CleanupResponse {
            success: false,
            deleted_mb: 0,
            message: format!("Cleanup encountered errors: {}", e),
        }),
    }
}

/// Response from cleanup operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupResponse {
    pub success: bool,
    pub deleted_mb: u64,
    pub message: String,
}
