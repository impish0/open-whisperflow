use tauri::State;

use crate::audio::AudioRecorder;
use crate::config::AppConfig;
use crate::docker::DockerClient;
use crate::error::{AppError, Result};
use crate::injection::TextInjector;
use crate::llm::LLMService;
use crate::state::{AppState, ProcessingStage, RecordingState};
use crate::transcription::TranscriptionService;

/// Start audio recording
#[tauri::command]
pub async fn start_recording(state: State<'_, AppState>) -> Result<()> {
    log::info!("Command: start_recording");

    // Check if already recording
    if state.is_recording().await {
        return Err(crate::error::AppError::InvalidState(
            "Already recording".to_string(),
        ));
    }

    // Create audio recorder if it doesn't exist and start recording
    {
        let mut recorder_opt = state.audio_recorder.lock().unwrap();
        if recorder_opt.is_none() {
            *recorder_opt = Some(AudioRecorder::new()?);
        }

        // Start recording before releasing lock
        if let Some(ref mut recorder) = *recorder_opt {
            recorder.start_recording()?;
        }
    } // Lock is dropped here

    // Update state after releasing lock
    state
        .set_recording_state(RecordingState::Recording {
            started_at: chrono::Utc::now().timestamp_millis() as u64,
        })
        .await;

    Ok(())
}

/// Stop recording and process audio
#[tauri::command]
pub async fn stop_recording(state: State<'_, AppState>) -> Result<ProcessedResult> {
    log::info!("Command: stop_recording");

    // Stop recording and get audio file path
    let audio_path = {
        let mut recorder_opt = state.audio_recorder.lock().unwrap();
        if let Some(ref mut recorder) = *recorder_opt {
            recorder.stop_recording()?
        } else {
            return Err(AppError::InvalidState("No active recorder".to_string()));
        }
    };

    log::info!("Audio saved to: {}", audio_path.display());

    // Transcribe
    state
        .set_recording_state(RecordingState::Processing {
            stage: ProcessingStage::Transcribing,
        })
        .await;

    let config = state.config.read().await;
    let transcription_service = TranscriptionService::new(&config.transcription).await?;
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
pub async fn cancel_recording(state: State<'_, AppState>) -> Result<()> {
    log::info!("Command: cancel_recording");

    // Stop recording if active and get audio path
    let audio_path_opt = if state.is_recording().await {
        let mut recorder_opt = state.audio_recorder.lock().unwrap();
        if let Some(ref mut recorder) = *recorder_opt {
            Some(recorder.stop_recording()?)
        } else {
            None
        }
    } else {
        None
    }; // Lock is dropped here

    // Delete the audio file if exists (after lock is released)
    if let Some(audio_path) = audio_path_opt {
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

    let (available, message) = match TranscriptionService::new(&config.transcription).await {
        Ok(service) => {
            let is_available = service.transcribe(&std::path::PathBuf::from("")).await.is_ok();
            let msg = match config.transcription.backend {
                crate::config::TranscriptionBackend::OpenAI => {
                    if config.transcription.openai_api_key.is_some() {
                        "Ready".to_string()
                    } else {
                        "API key not configured".to_string()
                    }
                }
                crate::config::TranscriptionBackend::FasterWhisper => {
                    "Docker container ready".to_string()
                }
            };
            (is_available || config.transcription.openai_api_key.is_some(), msg)
        }
        Err(e) => {
            log::error!("Backend check failed: {}", e);
            (false, format!("Error: {}", e))
        }
    };

    Ok(BackendStatus {
        name: "Transcription".to_string(),
        available,
        message,
    })
}

/// Check if LLM backend is available
#[tauri::command]
pub async fn check_llm_backend(state: State<'_, AppState>) -> Result<BackendStatus> {
    let config = state.config.read().await;
    let _service = LLMService::new(&config.llm)?;

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

/// Check Docker status
#[tauri::command]
pub async fn check_docker_status() -> Result<DockerStatus> {
    match DockerClient::new() {
        Ok(docker) => {
            let available = docker.is_available().await;
            let has_gpu = docker.has_nvidia_gpu().await;
            let container_running = docker.is_container_running().await.unwrap_or(false);

            Ok(DockerStatus {
                available,
                container_running,
                has_nvidia_gpu: has_gpu,
                message: if available {
                    "Docker is running".to_string()
                } else {
                    "Docker is not running".to_string()
                },
            })
        }
        Err(e) => Ok(DockerStatus {
            available: false,
            container_running: false,
            has_nvidia_gpu: false,
            message: format!("Docker error: {}", e),
        }),
    }
}

/// Start faster-whisper Docker container
#[tauri::command]
pub async fn start_whisper_container() -> Result<String> {
    log::info!("Command: start_whisper_container");

    let docker = DockerClient::new()?;
    docker.start_container().await?;
    docker.wait_for_ready(30).await?;

    Ok("Container started successfully".to_string())
}

/// Stop faster-whisper Docker container
#[tauri::command]
pub async fn stop_whisper_container() -> Result<String> {
    log::info!("Command: stop_whisper_container");

    let docker = DockerClient::new()?;
    docker.stop_container().await?;

    Ok("Container stopped successfully".to_string())
}

/// Get list of available Whisper models
#[tauri::command]
pub fn get_available_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            name: "tiny".to_string(),
            size: "39 MB".to_string(),
            description: "Fastest, lowest accuracy".to_string(),
            recommended: false,
        },
        ModelInfo {
            name: "base".to_string(),
            size: "74 MB".to_string(),
            description: "Fast, good for testing".to_string(),
            recommended: false,
        },
        ModelInfo {
            name: "small".to_string(),
            size: "244 MB".to_string(),
            description: "Balanced speed and accuracy".to_string(),
            recommended: true,
        },
        ModelInfo {
            name: "medium".to_string(),
            size: "769 MB".to_string(),
            description: "Higher accuracy, slower".to_string(),
            recommended: false,
        },
        ModelInfo {
            name: "large".to_string(),
            size: "1.5 GB".to_string(),
            description: "Best accuracy, slowest".to_string(),
            recommended: false,
        },
    ]
}

/// Check Ollama status
#[tauri::command]
pub async fn check_ollama_status(state: State<'_, AppState>) -> Result<OllamaStatus> {
    let config = state.config.read().await;
    let ollama_url = config.llm.base_url.replace("/v1", "");

    // Check if Ollama is running
    let available = match reqwest::Client::new()
        .get(format!("{}/api/tags", ollama_url))
        .timeout(std::time::Duration::from_secs(2))
        .send()
        .await
    {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    };

    Ok(OllamaStatus {
        available,
        base_url: ollama_url.clone(),
        message: if available {
            "Ollama is running".to_string()
        } else {
            "Ollama is not running or not installed".to_string()
        },
    })
}

/// Get list of installed Ollama models
#[tauri::command]
pub async fn get_ollama_models(state: State<'_, AppState>) -> Result<Vec<OllamaModelInfo>> {
    let config = state.config.read().await;
    let ollama_url = config.llm.base_url.replace("/v1", "");

    #[derive(serde::Deserialize)]
    struct OllamaModelsResponse {
        models: Vec<OllamaModel>,
    }

    #[derive(serde::Deserialize)]
    struct OllamaModel {
        name: String,
        size: i64,
        modified_at: String,
    }

    let response = reqwest::Client::new()
        .get(format!("{}/api/tags", ollama_url))
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .map_err(|e| AppError::LLMProcessing(format!("Failed to get Ollama models: {}", e)))?;

    if !response.status().is_success() {
        return Err(AppError::LLMProcessing(
            "Failed to fetch Ollama models".to_string(),
        ));
    }

    let data: OllamaModelsResponse = response.json().await?;

    let models = data
        .models
        .into_iter()
        .map(|m| {
            let size_gb = (m.size as f64) / 1_073_741_824.0;
            let size_str = if size_gb >= 1.0 {
                format!("{:.1} GB", size_gb)
            } else {
                format!("{} MB", (m.size as f64) / 1_048_576.0)
            };

            // Determine if recommended based on model name
            let recommended = m.name.contains("llama3.2:3b")
                || m.name.contains("llama3:8b")
                || m.name.contains("mistral:7b");

            OllamaModelInfo {
                name: m.name,
                size: size_str,
                modified_at: m.modified_at,
                recommended,
            }
        })
        .collect();

    Ok(models)
}

/// Get recommended Ollama models for rewriting
#[tauri::command]
pub fn get_recommended_ollama_models() -> Vec<OllamaModelInfo> {
    vec![
        OllamaModelInfo {
            name: "llama3.2:3b".to_string(),
            size: "2 GB".to_string(),
            modified_at: String::new(),
            recommended: true,
        },
        OllamaModelInfo {
            name: "llama3:8b".to_string(),
            size: "4.7 GB".to_string(),
            modified_at: String::new(),
            recommended: true,
        },
        OllamaModelInfo {
            name: "mistral:7b".to_string(),
            size: "4.1 GB".to_string(),
            modified_at: String::new(),
            recommended: true,
        },
        OllamaModelInfo {
            name: "phi3:3.8b".to_string(),
            size: "2.3 GB".to_string(),
            modified_at: String::new(),
            recommended: false,
        },
        OllamaModelInfo {
            name: "gemma2:9b".to_string(),
            size: "5.5 GB".to_string(),
            modified_at: String::new(),
            recommended: false,
        },
    ]
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

#[derive(Debug, serde::Serialize)]
pub struct DockerStatus {
    pub available: bool,
    pub container_running: bool,
    pub has_nvidia_gpu: bool,
    pub message: String,
}

#[derive(Debug, serde::Serialize)]
pub struct ModelInfo {
    pub name: String,
    pub size: String,
    pub description: String,
    pub recommended: bool,
}

#[derive(Debug, serde::Serialize)]
pub struct OllamaStatus {
    pub available: bool,
    pub base_url: String,
    pub message: String,
}

#[derive(Debug, serde::Serialize)]
pub struct OllamaModelInfo {
    pub name: String,
    pub size: String,
    pub modified_at: String,
    pub recommended: bool,
}
