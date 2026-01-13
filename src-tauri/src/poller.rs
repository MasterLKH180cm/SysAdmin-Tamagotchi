use crate::commands::AppState;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::time;

/// Start background polling of system metrics
/// Polls every 5 seconds and emits events to frontend
pub fn start_polling(app_handle: AppHandle, state: Arc<AppState>) {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5));

        loop {
            interval.tick().await;

            // Refresh metrics
            if let Ok(mut monitor) = state.monitor.lock() {
                monitor.refresh();
                let metrics = monitor.get_metrics();

                // Update pet state
                if let Ok(mut pet) = state.pet.lock() {
                    let pet_state = pet.update(&metrics);

                    // Emit event to frontend with updated metrics
                    let event_data = serde_json::json!({
                        "metrics": metrics,
                        "pet_state": pet_state,
                        "pet_emoji": pet.get_emoji(),
                        "pet_description": pet.get_icon_description(),
                    });

                    // Emit to all windows
                    if let Err(e) = app_handle.emit("metrics-update", event_data) {
                        eprintln!("Failed to emit metrics-update event: {}", e);
                    }
                }
            }
        }
    });
}
