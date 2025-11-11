use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex as TokioMutex;

use crate::audio::AudioRecorder;
use crate::config::AppConfig;
use crate::error::Result;
use crate::injection::TextInjector;
use crate::llm::LLMService;
use crate::state::{AppState, ProcessingStage, RecordingState};
use crate::transcription::TranscriptionService;

/// Start audio recording
#[tauri::command]
pub async fn start_recording(
    state: State<'_, AppState>,
    recorder: State<'_, Arc<TokioMutex<AudioRecorder>>>,
) -> Result<()> {
    log::info!("Command: start_recording");

    // Check if already recording
    if state.is_recording().await {
        return Err(crate::error::AppError::InvalidState(
            "Already recording".to_string(),
        ));
    }

    // Update state
    state
        .set_recording_state(RecordingState::Recording {
            started_at: chrono::Utc::now().timestamp_millis() as u64,
        })
        .await;

    // Start recording
    let mut rec = recorder.lock().await;
    rec.start_recording()?;

    Ok(())
}

/// Stop recording and process audio
#[tauri::command]
pub async fn stop_recording(
    state: State<'_, AppState>,
    recorder: State<'_, Arc<TokioMutex<AudioRecorder>>>,
) -> Result<ProcessedResult> {
    log::info!("Command: stop_recording");

    // Stop recording and get audio file path
    let audio_path = {
        let mut rec = recorder.lock().await;
        rec.stop_recording()?
    };

    log::info!("Audio saved to: {}", audio_path.display());

    // Transcribe
    state
        .set_recording_state(RecordingState::Processing {
            stage: ProcessingStage::Transcribing,
        })
        .await;

    let config = state.config.read().await;
    let transcription_service = TranscriptionService::new(&config.transcription)?;
    let transcription = transcription_service.transcribe(&audio_path).await?;
    log::info!("Transcription: {}", transcription);

    // Rewrite with LLM
    state
        .set_recording_state(RecordingState::Processing {
            stage: ProcessingStage::Rewriting,
        })
        .await;

    let llm_service = LLMService::new(&config.llm)?;
    let cleaned_text = llm_service.rewrite_text(&transcription).await?;
    log::info!("Cleaned text: {}", cleaned_text);

    // Inject text
    state
        .set_recording_state(RecordingState::Processing {
            stage: ProcessingStage::Injecting,
        })
        .await;

    let mut text_injector = TextInjector::new(&config.injection)?;
    drop(config); // Release lock before async operation

    text_injector.inject_text(&cleaned_text).await?;
    log::info!("Text injected successfully");

    // Cleanup
    crate::utils::secure_delete_file(&audio_path).await.ok();

    // Reset state
    state.set_recording_state(RecordingState::Idle).await;

    Ok(ProcessedResult {
        transcription,
        cleaned_text,
    })
}

/// Cancel current recording
#[tauri::command]
pub async fn cancel_recording(
    state: State<'_, AppState>,
    recorder: State<'_, Arc<TokioMutex<AudioRecorder>>>,
) -> Result<()> {
    log::info!("Command: cancel_recording");

    // Stop recording if active
    if state.is_recording().await {
        let mut rec = recorder.lock().await;
        let audio_path = rec.stop_recording()?;
        // Delete the audio file
        crate::utils::secure_delete_file(&audio_path).await.ok();
    }

    // Reset state
    state.set_recording_state(RecordingState::Idle).await;

    Ok(())
}

/// Get current recording state
#[tauri::command]
pub async fn get_recording_state(state: State<'_, AppState>) -> Result<RecordingState> {
    Ok(state.get_recording_state().await)
}

/// Get application configuration
#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<AppConfig> {
    Ok(state.config.read().await.clone())
}

/// Update application configuration
#[tauri::command]
pub async fn update_config(state: State<'_, AppState>, config: AppConfig) -> Result<()> {
    log::info!("Command: update_config");

    // Save to disk
    config.save()?;

    // Update in-memory state
    *state.config.write().await = config;

    Ok(())
}

/// Get system information
#[tauri::command]
pub fn get_system_info() -> String {
    crate::utils::get_system_info()
}

/// Check if transcription backend is available
#[tauri::command]
pub async fn check_transcription_backend(state: State<'_, AppState>) -> Result<BackendStatus> {
    let config = state.config.read().await;
    let service = TranscriptionService::new(&config.transcription)?;

    // Try to check availability (simplified check for now)
    let available = match config.transcription.backend {
        crate::config::TranscriptionBackend::OpenAI => {
            config.transcription.openai_api_key.is_some()
        }
        crate::config::TranscriptionBackend::FasterWhisper => false, // Not implemented yet
    };

    Ok(BackendStatus {
        name: "Transcription".to_string(),
        available,
        message: if available {
            "Ready".to_string()
        } else {
            "Not configured".to_string()
        },
    })
}

/// Check if LLM backend is available
#[tauri::command]
pub async fn check_llm_backend(state: State<'_, AppState>) -> Result<BackendStatus> {
    let config = state.config.read().await;
    let service = LLMService::new(&config.llm)?;

    // Basic availability check
    let available = match config.llm.backend {
        crate::config::LLMBackend::OpenAI => config.llm.api_key.is_some(),
        crate::config::LLMBackend::Ollama => {
            // Check if Ollama is running
            reqwest::Client::new()
                .get(format!(
                    "{}/api/tags",
                    config.llm.base_url.replace("/v1", "")
                ))
                .send()
                .await
                .is_ok()
        }
        crate::config::LLMBackend::None => false,
    };

    Ok(BackendStatus {
        name: "LLM".to_string(),
        available,
        message: if available {
            "Ready".to_string()
        } else {
            "Not configured or not running".to_string()
        },
    })
}

// Response types
#[derive(Debug, serde::Serialize)]
pub struct ProcessedResult {
    pub transcription: String,
    pub cleaned_text: String,
}

#[derive(Debug, serde::Serialize)]
pub struct BackendStatus {
    pub name: String,
    pub available: bool,
    pub message: String,
}
