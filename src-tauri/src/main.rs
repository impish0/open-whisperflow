// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

mod audio;
mod commands;
mod config;
mod docker;
mod error;
mod hotkeys;
mod injection;
mod llm;
mod state;
mod transcription;
mod utils;

use audio::AudioRecorder;
use state::AppState;

fn main() {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting Open WhisperFlow v{}", env!("CARGO_PKG_VERSION"));

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            log::info!("Setting up application");

            // Initialize application state
            let app_state = AppState::new().expect("Failed to create app state");

            // Initialize audio recorder
            let audio_recorder = Arc::new(TokioMutex::new(
                AudioRecorder::new().expect("Failed to create audio recorder"),
            ));

            // Manage state
            app.manage(app_state);
            app.manage(audio_recorder);

            log::info!("Application setup complete");
            log::info!("System info:\n{}", utils::get_system_info());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_recording,
            commands::stop_recording,
            commands::cancel_recording,
            commands::get_recording_state,
            commands::get_config,
            commands::update_config,
            commands::get_system_info,
            commands::check_transcription_backend,
            commands::check_llm_backend,
            commands::check_docker_status,
            commands::start_whisper_container,
            commands::stop_whisper_container,
            commands::get_available_models,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    log::info!("Application shutting down");
}
