// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod monitor;
mod pet;
mod poller;

use commands::AppState;
use std::sync::Arc;

fn main() {
    // Create application state
    let state = Arc::new(AppState::new());

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(state.clone())
        .setup(move |app| {
            // Start background polling
            poller::start_polling(app.handle().clone(), state.clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_metrics,
            commands::get_pet_state,
            commands::cleanup_temp,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
