// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

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

use docker::DockerClient;
use state::AppState;

/// Prepare backends on startup for faster first use
async fn prepare_backends(state: AppState) {
    log::info!("Preparing backends in background...");

    // Load config
    let config = state.config.read().await;

    // If using faster-whisper, pre-start the container
    if matches!(config.transcription.backend, config::TranscriptionBackend::FasterWhisper) {
        log::info!("Detected faster-whisper backend, pre-starting Docker container...");

        match DockerClient::new() {
            Ok(docker) => {
                if docker.is_available().await {
                    match docker.is_container_running().await {
                        Ok(false) => {
                            log::info!("Starting faster-whisper container...");
                            if let Err(e) = docker.start_container().await {
                                log::warn!("Failed to pre-start container: {}. Will retry on first use.", e);
                            } else {
                                match docker.wait_for_ready(30).await {
                                    Ok(_) => log::info!("Container ready for instant transcription!"),
                                    Err(e) => log::warn!("Container started but health check failed: {}", e),
                                }
                            }
                        }
                        Ok(true) => {
                            log::info!("Container already running");
                        }
                        Err(e) => {
                            log::warn!("Failed to check container status: {}", e);
                        }
                    }
                } else {
                    log::warn!("Docker not available, container will start on first transcription");
                }
            }
            Err(e) => {
                log::warn!("Failed to initialize Docker client: {}", e);
            }
        }
    }

    log::info!("Backend preparation complete");
}

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

            // Manage state
            app.manage(app_state.clone());

            log::info!("Application setup complete");
            log::info!("System info:\n{}", utils::get_system_info());

            // Spawn background task to prepare Docker container if needed
            let state_clone = app_state.clone();
            tauri::async_runtime::spawn(async move {
                prepare_backends(state_clone).await;
            });

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
            commands::check_ollama_status,
            commands::get_ollama_models,
            commands::get_recommended_ollama_models,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    log::info!("Application shutting down");
}
